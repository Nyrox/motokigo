#![feature(debug_non_exhaustive)]

pub mod camera;
pub mod mesh;
pub mod shader;
pub mod transform;

use motokigo::{
	compiler, parser,
	vm::{self, *},
};

use camera::Camera;
use cgmath::{Deg, Matrix4, PerspectiveFov, Rad, Vector2, Vector3, Vector4};
use transform::Transform;

use std::path::PathBuf;

use gl::types::*;
use shader::Uniform;
use std::{ffi::CString, time::*};

fn edge(p: Vector2<f32>, v0: Vector2<f32>, v1: Vector2<f32>) -> f32 {
	(p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;

#[derive(Copy, Clone, Debug)]
pub struct Tri2(Vector2f, Vector2f, Vector2f);
#[derive(Copy, Clone, Debug)]
pub struct Tri3(Vector3f, Vector3f, Vector3f);

impl Tri3 {
	pub fn truncate(self) -> Tri2 {
		Tri2(self.0.truncate(), self.1.truncate(), self.2.truncate())
	}
}

#[derive(Copy, Clone)]
pub struct Bounds2 {
	min_x: f32,
	min_y: f32,
	width: f32,
	height: f32,
}

pub fn calculate_triangle_bounds(tri: Tri2) -> Bounds2 {
	let points = [tri.0, tri.1, tri.2];

	let mut min_x = 42000.0;
	let mut max_x = 0.0;
	let mut min_y = 42000.0;
	let mut max_y = 0.0;

	for p in points.iter() {
		if p.x < min_x {
			min_x = p.x;
		}
		if p.x > max_x {
			max_x = p.x;
		}
		if p.y < min_y {
			min_y = p.y;
		}
		if p.y > max_y {
			max_y = p.y;
		}
	}

	Bounds2 {
		min_x,
		min_y,
		width: max_x - min_x,
		height: max_y - min_y,
	}
}

///
///
///
pub fn rasterize_window_space<F>(tri: Tri3, mut cb: F)
where
	F: FnMut((u32, u32), (f32, f32, f32)) -> (),
{
	let bounds = calculate_triangle_bounds(tri.truncate());
	let rast_min_x = bounds.min_x as u32;
	let rast_max_x = (bounds.min_x + bounds.width + 1.0) as u32;
	let rast_min_y = bounds.min_y as u32;
	let rast_max_y = (bounds.min_y + bounds.height + 1.0) as u32;

	for x in rast_min_x..rast_max_x {
		for y in rast_min_y..rast_max_y {
			let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

			let v0 = tri.0.truncate();
			let v1 = tri.1.truncate();
			let v2 = tri.2.truncate();

			let area = edge(v0, v1, v2);

			let w0 = edge(p, v1, v2);
			let w1 = edge(p, v2, v0);
			let w2 = edge(p, v0, v1);

			if (w0 <= 0.0) && (w1 <= 0.0) && (w2 <= 0.0) {
				cb((x, y), (w0 / area, w1 / area, w2 / area))
			}
		}
	}
}

impl Uniform for Matrix4<f32> {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniformMatrix4fv(handle, location, 1, gl::FALSE, ::std::mem::transmute(self));
		}
	}
}

impl Uniform for Vector3<f32> {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniform3fv(handle, location, 1, ::std::mem::transmute(self));
		}
	}
}

impl Uniform for Vector2<i32> {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniform2iv(handle, location, 1, ::std::mem::transmute(self));
		}
	}
}

impl Uniform for i32 {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniform1i(handle, location, *self);
		}
	}
}

impl Uniform for f32 {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniform1f(handle, location, *self);
		}
	}
}

fn main() {
	let im_dims = (800, 600);

	let events_loop = glutin::event_loop::EventLoop::new();

	let wb = glutin::window::WindowBuilder::new()
		.with_inner_size(glutin::dpi::LogicalSize::new(im_dims.0 as f32, im_dims.1 as f32))
		.with_title("Hello world");

	let context = glutin::ContextBuilder::new().build_windowed(wb, &events_loop).unwrap();
	let context = unsafe { context.make_current().unwrap() };

	gl::load_with(|s| context.get_proc_address(s));

	let mut imgbuf = image::ImageBuffer::new(im_dims.0, im_dims.1);

	let mut camera = Camera::new(
		Transform::default(),
		PerspectiveFov {
			fovy: Rad::from(Deg(75.0)),
			aspect: 1280.0 / 720.0,
			near: 0.1,
			far: 1000.0,
		},
	);
	camera.transform.position.z = -3.0;

	let view = camera.get_view_matrix();
	let proj = camera.get_projection_matrix();

	let mesh = mesh::load_ply(PathBuf::from("res/mesh/monkey.ply"));

	let mut depth = vec![1.0; im_dims.0 as usize * im_dims.1 as usize];

	struct Viewport {
		x: i32,
		y: i32,
		width: u32,
		height: u32,
	}

	let mut vao = 0;
	let mut vbo = 0;

	unsafe {
		gl::CreateVertexArrays(1, &mut vao);
		gl::CreateBuffers(1, &mut vbo);
		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::NamedBufferData(
			vbo,
			(std::mem::size_of::<mesh::Triangle>() * mesh.triangles.len()) as isize,
			mesh.triangles.as_ptr() as *const GLvoid,
			gl::STATIC_DRAW,
		);

		gl::VertexAttribPointer(
			0,
			3,
			gl::FLOAT,
			0,
			std::mem::size_of::<mesh::Vertex>() as i32,
			std::ptr::null(),
		);
		gl::EnableVertexAttribArray(0);

		macro_rules! offset_of {
			($ty:ty, $field:ident) => {
				&(*(0 as *const $ty)).$field as *const _ as usize
			};
		}

		gl::VertexAttribPointer(
			1,
			3,
			gl::FLOAT,
			0,
			std::mem::size_of::<mesh::Vertex>() as i32,
			offset_of!(mesh::Vertex, normal) as *const GLvoid,
		);
		gl::EnableVertexAttribArray(1);

		gl::BindVertexArray(0);
	}

	use std::io::Read;
	pub fn read_file_contents(filename: &str) -> String {
		let mut f = std::fs::File::open(filename).unwrap();
		let mut buffer = String::new();
		f.read_to_string(&mut buffer).unwrap();
		buffer
	}

	let shadelang_shader = {
		let src = read_file_contents("res/shaders/shadelang/basic.sl");
		std::fs::create_dir_all("debug/shaders/basic/").ok();

		let program = parser::parse(&src).unwrap();
		std::fs::write("debug/shaders/basic/ast.rson", format!("{:#?}", program)).ok();
		{
			let mut program = program.clone();
			motokigo::compiler::resolve_types::resolve(
				&mut program,
				&mut motokigo::compiler::program_data::ProgramData::new(),
			)
			.unwrap();
			let glsl = motokigo::glsl::generate_glsl(program.clone());
			std::fs::write("debug/shaders/basic/compiled.glsl", glsl.clone()).ok();
			std::fs::write("res/shaders/glsl/basic.fs", glsl).unwrap();
		}
		let compiled = compiler::compile(program);
		std::fs::write("debug/shaders/basic/code.ron", format!("{:#?}", compiled)).ok();
		compiled
	};
	let shadelang_vm = vm::VirtualMachine::new(&shadelang_shader);
	let shader = shader::Shader::new();
	shader
		.attach(&read_file_contents("res/shaders/glsl/basic.vs"), gl::VERTEX_SHADER)
		.unwrap();
	shader
		.attach(&read_file_contents("res/shaders/glsl/basic.fs"), gl::FRAGMENT_SHADER)
		.unwrap();
	shader.compile().unwrap();
	shader.bind();
	let viewport = Viewport {
		x: 0,
		y: 0,
		width: im_dims.0,
		height: im_dims.1,
	};

	// clear
	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let r = (0.3 * x as f32) as u8;
		let b = (0.3 * y as f32) as u8;
		*pixel = image::Rgb([r, 0, b]);
	}

	let begin = std::time::Instant::now();
	for tri in mesh.triangles.iter() {
		let mut t1_ndc = [
			(proj * view * tri.0.position.extend(1.0)),
			(proj * view * tri.1.position.extend(1.0)),
			(proj * view * tri.2.position.extend(1.0)),
		];

		for p in t1_ndc.iter_mut() {
			p.x = p.x / p.w;
			p.y = p.y / p.w;
			p.z = p.z / p.w;
		}

		let t1_ndc: Vec<_> = t1_ndc.iter().map(|v| Vector4::truncate(*v)).collect();

		let near_val = 0.0;
		let far_val = 1.0;

		let ndc_to_wnd = |p: Vector3<f32>| {
			let (x_ndc, y_ndc, z_ndc) = p.into();
			Vector3::new(
				(viewport.width / 2) as f32 * x_ndc + viewport.x as f32 + (viewport.width / 2) as f32,
				(viewport.height / 2) as f32 * y_ndc + viewport.y as f32 + (viewport.height / 2) as f32,
				((far_val - near_val) / 2.0) * z_ndc + ((far_val + near_val) / 2.0),
			)
		};

		let t1_wnd = [ndc_to_wnd(t1_ndc[0]), ndc_to_wnd(t1_ndc[1]), ndc_to_wnd(t1_ndc[2])];

		let t1_wnd = Tri3(t1_wnd[0], t1_wnd[1], t1_wnd[2]);

		rasterize_window_space(t1_wnd, |(x, y), (w0, w1, w2)| {
			let mut vm = shadelang_vm.clone();

			let i = im_dims.0 * y + x;

			let interpolate_inverse = |(a, b, c), (w0, w1, w2)| {
				let i = (1.0 / a) * (w0) + (1.0 / b) * (w1) + (1.0 / c) * (w2);

				1.0 / i
			};

			let d = interpolate_inverse((t1_wnd.0.z, t1_wnd.1.z, t1_wnd.2.z), (w0, w1, w2));

			let n = (tri.0.normal / t1_wnd.0.z) * w0 * d
				+ (tri.1.normal / t1_wnd.1.z) * w1 * d
				+ (tri.2.normal / t1_wnd.2.z) * w2 * d;

			if d < depth[i as usize] {
				// shadelang_vm.set_in_float("nx", n.x);
				// shadelang_vm.set_in_float("ny", n.y);
				// shadelang_vm.set_in_float("nz", n.z);

				vm.set_global("normal", [n.x, n.y, n.z]);

				let mut result = vm.run_fn("main", vec![]);
				let mut vm = loop {
					match result {
						VMState::BreakpointEncountered(s) => {
							dbg!(s.breakpoint());
							let stack = s.generate_stack_view();
							dbg!(stack.current_fn);
							dbg!(x, y);
							stack.symbols.iter().for_each(|(id, (tk, bytes))| {
								println!(
									"{} [{:?}]: {}",
									id,
									tk,
									match tk {
										motokigo::ast::TypeKind::F32 =>
											format!("{}", bytemuck::from_bytes::<f32>(&bytes)),
										motokigo::ast::TypeKind::I32 =>
											format!("{}", bytemuck::from_bytes::<i32>(&bytes)),
										motokigo::ast::TypeKind::Vector(_, _) => panic!(),
										_ => panic!(),
									}
								);
							});
							std::io::stdin().read_line(&mut String::new()).ok();
							result = s.resume();
						}
						VMState::VMRunFinished(s) => break s.0,
					};
				};

				let color: [f32; 3] = unsafe { vm.pop_stack() };
				// let cr = shadelang_vm.get_out_float("cr");
				// let cg = shadelang_vm.get_out_float("cg");
				// let cb = shadelang_vm.get_out_float("cb");

				// let color = Vector3::new(cr, cg, cb);
				// let color = Vector3::new(1.0, 0.0, 0.0);

				*(imgbuf.get_pixel_mut(x, im_dims.1 - (y + 1))) = image::Rgb([
					(color[0] * 255.0) as u8,
					(color[1] * 255.0) as u8,
					(color[2] * 255.0) as u8,
				]);
				depth[i as usize] = d;
			}
		})
	}

	println!("{:?}", Instant::now().duration_since(begin));
	imgbuf.save("output.png").unwrap();

	use glutin::{
		event::{Event, WindowEvent},
		event_loop::ControlFlow,
	};
	events_loop.run(move |event, _, control_flow| match event {
		Event::LoopDestroyed => return,
		Event::WindowEvent { event, .. } => match event {
			WindowEvent::Resized(physical_size) => context.resize(physical_size),
			WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
			_ => (),
		},
		Event::RedrawRequested(_) => {
			unsafe {
				gl::ClearColor(0.3, 0.0, 0.3, 1.0);
				gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
			}

			shader.bind();
			shader.set_uniform("view", camera.get_view_matrix());
			shader.set_uniform("proj", camera.get_projection_matrix());

			unsafe {
				gl::Enable(gl::DEPTH_TEST);
				gl::DepthFunc(gl::LESS);

				gl::BindVertexArray(vao);
				gl::DrawArrays(gl::TRIANGLES, 0, (mesh.triangles.len() * 3) as i32);
			}

			context.swap_buffers().unwrap();
			context.window().request_redraw();
		}
		_ => (),
	});
}

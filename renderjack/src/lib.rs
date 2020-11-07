use cgmath::prelude::*;
use cgmath::{Vector3, Vector2};



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

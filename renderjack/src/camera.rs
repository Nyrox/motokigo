use crate::transform::Transform;
use cgmath::Rotation;
use cgmath::{Matrix4, PerspectiveFov, Vector3};

#[derive(Debug, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub projection: PerspectiveFov<f32>,
}

impl Camera {
    pub fn new(transform: Transform, projection: PerspectiveFov<f32>) -> Camera {
        Camera {
            transform,
            projection,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_dir(
            self.transform.position,
            self.transform
                .rotation
                .rotate_vector(Vector3::new(0.0, 0.0, 1.0)),
            self.transform
                .rotation
                .rotate_vector(Vector3::new(0.0, 1.0, 0.0)),
        )
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::from(self.projection);
        m = Matrix4::from_nonuniform_scale(-1.0, -1.0, 1.0) * m;
        // m[3][3] *= -1.0;
        m
    }
}

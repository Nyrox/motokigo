use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::{Point3, Quaternion, Vector3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::from_angle_x(Deg(0.0)),
        }
    }
}

impl Transform {
    pub fn forward(&self) -> Vector3<f32> {
        self.rotation.rotate_vector(Vector3::new(0.0, 0.0, 1.0))
    }

    pub fn right(&self) -> Vector3<f32> {
        self.rotation.rotate_vector(Vector3::new(1.0, 0.0, 0.0))
    }

    pub fn up(&self) -> Vector3<f32> {
        self.rotation.rotate_vector(Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn down(&self) -> Vector3<f32> {
        -self.up()
    }
}

use crate::resources::uniform_blocks::CameraUniform;
use cgmath::{Matrix4, Point3, Rad, Vector3};

#[derive(Clone)]
pub enum ProjectionKind {
    Perspective {
        fov_y: f32,
        aspect: f32,
        near: f32,
        far: f32,
    },
    Ortho {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

#[derive(Clone)]
pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub proj: ProjectionKind,
}

impl Camera {
    pub fn new_persp() -> Self {
        Self {
            position: [0.0, 0.0, 3.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            proj: ProjectionKind::Perspective {
                fov_y: 60.0_f32.to_radians(),
                aspect: 1.0,
                near: 0.1,
                far: 100.0,
            },
        }
    }

    pub fn new_ortho() -> Self {
        Self {
            position: [0.0, 0.0, 3.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            proj: ProjectionKind::Ortho {
                left: -1.0,
                right: 1.0,
                bottom: -1.0,
                top: 1.0,
                near: 0.1,
                far: 100.0,
            },
        }
    }

    fn view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            Point3::new(self.position[0], self.position[1], self.position[2]),
            Point3::new(self.target[0], self.target[1], self.target[2]),
            Vector3::new(self.up[0], self.up[1], self.up[2]),
        )
    }

    fn proj(&self) -> Matrix4<f32> {
        match self.proj {
            ProjectionKind::Perspective {
                fov_y,
                aspect,
                near,
                far,
            } => cgmath::perspective(Rad(fov_y), aspect, near, far),
            ProjectionKind::Ortho {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Matrix4::new(
                2.0 / (right - left),
                0.0,
                0.0,
                -(right + left) / (right - left),
                0.0,
                2.0 / (top - bottom),
                0.0,
                -(top + bottom) / (top - bottom),
                0.0,
                0.0,
                -2.0 / (far - near),
                -(far + near) / (far - near),
                0.0,
                0.0,
                0.0,
                1.0,
            ),
        }
    }

    pub fn build_uniform(&self) -> CameraUniform {
        let vp = self.proj() * self.view();
        CameraUniform {
            view_proj: vp.into(),
        }
    }
}

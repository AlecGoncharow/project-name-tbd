pub mod color;
pub mod mesh;
pub mod mode;
pub mod renderer;
pub mod texture;
pub mod vertex;

pub use color::Color;
pub use mode::DrawMode;
pub use mode::PolygonMode;
pub use mode::Topology;

use crate::math::Mat4;
use crate::math::Vec3;

pub trait CameraProjection {
    fn projection_view_matrix(&self) -> Mat4;
    fn projection_matrix(&self) -> Mat4;
    fn view_matrix(&self) -> Mat4;
}

pub struct DefaultCamera {}

impl CameraProjection for DefaultCamera {
    fn projection_view_matrix(&self) -> Mat4 {
        Mat4::identity()
    }
    fn projection_matrix(&self) -> Mat4 {
        Mat4::identity()
    }
    fn view_matrix(&self) -> Mat4 {
        Mat4::identity()
    }
}

pub trait Drawable {
    /// R*T Matrix to translate model from model space to world space
    fn model_matrix(&self) -> Mat4 {
        Mat4::identity()
    }

    fn draw_mode(&self) -> DrawMode {
        DrawMode::Normal(Topology::TriangleList(PolygonMode::Fill))
    }

    fn rotate(&mut self, _theta: f32, _axis: Vec3) {}
    fn translate(&mut self, (_x_tr, _y_tr, _z_tr): (f32, f32, f32)) {}
}

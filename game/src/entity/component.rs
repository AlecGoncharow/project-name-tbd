use super::Context;
use crate::camera::Camera;
use engine::math::Vec3;

pub trait AsComponent: AsDrawable + AsMouseable {}

pub trait DrawComponent {
    fn draw(&mut self, ctx: &mut Context);
}

pub trait AsDrawable {
    fn as_drawable(&mut self) -> Option<&mut dyn DrawComponent> {
        None
    }
}

/// this is useful because it allows 3D picking to ignore entities which aren't part of the
/// clickable environment
pub trait MouseComponent {
    // TODO think about x/y/z and hover events
    fn click_start(&mut self, ctx: &mut Context);
    fn click_end(&mut self, ctx: &mut Context);
    fn mouse_over(&mut self, ctx: &mut Context, pos: Vec3, cam: &Camera);
    fn check_collision(
        &mut self,
        ctx: &mut Context,
        camera_origin: Vec3,
        mouse_direction: Vec3,
    ) -> Option<(&mut dyn MouseComponent, Vec3, f32)>;
}

pub trait AsMouseable {
    fn as_mouseable(&mut self) -> Option<&mut dyn MouseComponent> {
        None
    }
}

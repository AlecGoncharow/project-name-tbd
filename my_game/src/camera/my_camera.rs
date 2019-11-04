use my_engine::math::Mat4;
use my_engine::math::Vec2;
use my_engine::math::Vec3;
use my_engine::math::Vec4;
use my_engine::winit::VirtualKeyCode;

//const YAW_DEFAULT: f64 = -90.0;
//const PITCH_DEFAULT: f64 = 0.0;
const MOVE_DEFAULT: f64 = 0.1;
const MOUSE_DEFAULT: f64 = 0.5;
const ZOOM_DEFAULT: f64 = 45.0;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,

    pub world_up: Vec3,

    pub yaw: f64,
    pub pitch: f64,

    pub move_speed: f64,
    pub mouse_speed: f64,
    pub zoom: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        world_up: Vec3,
        vfov: f64,
        width: f64,
        height: f64,
        //near_plane: f64,
        //far_plane: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let aspect = width / height;

        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).make_unit_vector();
        let u = (w.cross(&world_up)).make_unit_vector();
        let v = u.cross(&w).make_unit_vector();

        println!("u: {:#?}, v: {:#?}, w: {:#?}", u, v, w);
        let pitch = w.y.asin();
        let yaw = {
            if pitch.cos() != 0.0 {
                let cos_yaw = w.x / pitch.cos();
                cos_yaw.acos().to_degrees()
            } else {
                // degenerate case idk what to do with this
                1.0f64.to_degrees()
            }
        };
        let pitch = pitch.to_degrees();
        println!("pitch: {:#?}, yaw: {:#?}", pitch, yaw);

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            world_up,

            yaw,
            pitch,

            move_speed: MOVE_DEFAULT,
            mouse_speed: MOUSE_DEFAULT,
            zoom: ZOOM_DEFAULT,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let rotation = Mat4::new(
            Vec4::new(self.u.x, self.u.y, self.u.z, 0.0),
            Vec4::new(self.v.x, self.v.y, self.v.z, 0.0),
            Vec4::new(self.w.x, self.w.y, self.w.z, 0.0),
            (0, 0, 0, 1).into(),
        );

        let negative_from = -1.0 * self.origin;
        let translation = Mat4::translation(negative_from.x, negative_from.y, negative_from.z);

        rotation * translation
    }

    // reference https://learnopengl.com/code_viewer_gh.php?code=includes/learnopengl/camera.h
    pub fn update_orientation(&mut self) {
        // front vector
        self.w.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.w.y = self.pitch.to_radians().sin();
        self.w.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.w = self.w.make_unit_vector();

        self.u = self.w.cross(&self.world_up).make_unit_vector();
        self.v = self.u.cross(&self.w).make_unit_vector();
    }

    pub fn process_keypress(&mut self, key: VirtualKeyCode) {
        let speed = 0.05;

        match key {
            VirtualKeyCode::W => {
                self.origin -= speed * self.w;
            }
            VirtualKeyCode::A => {
                self.origin -= speed * self.u;
            }
            VirtualKeyCode::S => {
                self.origin += speed * self.w;
            }
            VirtualKeyCode::D => {
                self.origin += speed * self.u;
            }
            _ => (),
        }
    }

    pub fn process_mouse_move(&mut self, delta: Vec2) {
        let x_offset = delta.x * self.mouse_speed;
        let y_offset = delta.y * self.mouse_speed;

        self.yaw += x_offset;
        self.pitch += y_offset;

        self.pitch = if self.pitch > 89.0 {
            89.0
        } else if self.pitch < -89.0 {
            -89.0
        } else {
            self.pitch
        };

        self.update_orientation();
    }
}
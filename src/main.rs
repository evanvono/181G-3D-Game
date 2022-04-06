#![allow(dead_code)]

use color_eyre::eyre::Result;
use std::sync::{Arc, Mutex};
use winit::event::VirtualKeyCode;

mod camera;
mod engine;
mod image;
mod input;
mod renderer;
mod types;
mod vulkan;
use types::*;

const DT: f32 = 1.0 / 60.0;

#[derive(Debug)]
struct GenericGameThing {}
impl engine::GameThing for GenericGameThing {}

fn main() -> Result<()> {
    color_eyre::install()?;

    // If GameThing variants differ widely in size, consider using
    // Box<GameThing>
    let mut engine: engine::Engine = engine::Engine::new(engine::WindowSettings::default());

    engine.set_camera(camera::Camera::look_at(
        Vec3::new(0., -2., -10.),
        Vec3::zero(),
        Vec3::unit_y(),
    ));
    let tex = engine.load_texture(std::path::Path::new("content/robot.png"))?;
    let mesh = engine.load_mesh(std::path::Path::new("content/characterSmall.fbx"), 0.1)?;
    let model = engine.create_model(&mesh, &tex);

    engine.create_game_object(
        Some(&model),
        Isometry3::new(Vec3::new(0.0, -12.5, 25.0), Rotor3::identity()),
        Box::new(GenericGameThing {}),
        None,
    );

    let mut cam_x = Arc::new(Mutex::new(0.));
    let mut cam_y = Arc::new(Mutex::new(-2.));
    let mut cam_degrees_y = Arc::new(Mutex::new(0.));
    let mut cam_degrees_x = Arc::new(Mutex::new(0.));
    let mut cam_to_z = Arc::new(Mutex::new(0.));
    let mouse_move_scale = 200.;

    engine.play(move |_engine| {
        let mut cam_x_lock = cam_x.lock().unwrap();
        let mut cam_y_lock = cam_y.lock().unwrap();
        let mut cam_degrees_y_lock = cam_degrees_y.lock().unwrap();
        let mut cam_degrees_x_lock = cam_degrees_x.lock().unwrap();
        let mut cam_to_z_lock = cam_to_z.lock().unwrap();
        let input = _engine.get_inputs();

        if input.is_key_down(VirtualKeyCode::Up) {
            *cam_x_lock -= 1.;
        }
        if input.is_key_down(VirtualKeyCode::Down) {
            *cam_x_lock += 1.;
        }

        if input.is_key_down(VirtualKeyCode::Left) {
            *cam_y_lock -= 1.;
        }
        if input.is_key_down(VirtualKeyCode::Right) {
            *cam_y_lock += 1.;
        }
        let mouse_coords = input.get_mouse_position();
        let prev_mouse_coords = input.get_prev_mouse_position();

        let x_diff = mouse_coords.x - prev_mouse_coords.x;
        let y_diff = mouse_coords.y - prev_mouse_coords.y;

        *cam_degrees_y_lock -= (x_diff / mouse_move_scale) as f32;
        *cam_degrees_x_lock -= (y_diff / mouse_move_scale) as f32;

        let cam_eye = Vec3::new(*cam_x_lock, *cam_y_lock, -10.);
        let camera = camera::Camera::look_at_degrees(
            cam_eye,
            Vec3::unit_y(),
            (*cam_degrees_y_lock, *cam_degrees_x_lock),
        );
        _engine.set_camera(camera);
    })
}

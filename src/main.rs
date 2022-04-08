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

    let cam_degrees_y = Arc::new(Mutex::new(0.));
    let cam_degrees_x = Arc::new(Mutex::new(0.));

    let mouse_move_scale = 2.;
    let move_spd = 1.0;

    // mutex / lock it, modify it and create new eye each time
    let cam_pos = Arc::new(Mutex::new(Vec3::new(0., -2., -10.)));
    let up = Vec3::unit_y();

    engine.play(move |_engine| {
        let input = _engine.get_inputs();

        let delta = input.get_mouse_delta();

        let mut cam_degrees_x_lock = cam_degrees_x.lock().unwrap();
        let mut cam_degrees_y_lock = cam_degrees_y.lock().unwrap();

        *cam_degrees_x_lock -= (delta.x / mouse_move_scale) as f32;
        *cam_degrees_y_lock += (delta.y / mouse_move_scale) as f32;

        if *cam_degrees_y_lock > 89.999 {
            *cam_degrees_y_lock = 89.999
        }
        if *cam_degrees_y_lock < -89.999 {
            *cam_degrees_y_lock = -89.999
        }
        let mut cam_pos_lock = cam_pos.lock().unwrap();
        let mut is_moving = false;
        let mut directions: Vec<camera::Direction> = Vec::new();
        if input.is_key_down(VirtualKeyCode::Up) {
            directions.push(camera::Direction::Forward);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Down) {
            directions.push(camera::Direction::Backward);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Left) {
            directions.push(camera::Direction::Left);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Right) {
            directions.push(camera::Direction::Right);
            is_moving = true;
        }
        let new_cam = camera::Camera::move_direction(
            &mut cam_pos_lock,
            up,
            (*cam_degrees_x_lock, *cam_degrees_y_lock),
            if is_moving { move_spd } else { 0. },
            directions,
        );
        _engine.set_camera(new_cam);
    })
}

fn move_camera() {}

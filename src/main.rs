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

    let mouse_move_scale = 10.;
    let move_spd = 1.0;

    // mutex / lock it, modify it and create new eye each time
    let cam_pos = Arc::new(Mutex::new(Vec3::new(0.,-2., -10.)));
    let up = Vec3::unit_y();


    engine.play(move |_engine| {
        let input = _engine.get_inputs();

        let mouse_coords = input.get_mouse_position();
        let prev_mouse_coords = input.get_prev_mouse_position();

        let x_diff = mouse_coords.x - prev_mouse_coords.x;
        let y_diff = mouse_coords.y - prev_mouse_coords.y;

        let mut cam_degrees_x_lock = cam_degrees_x.lock().unwrap();
        let mut cam_degrees_y_lock = cam_degrees_y.lock().unwrap();

        *cam_degrees_x_lock -= (x_diff / mouse_move_scale) as f32;
        *cam_degrees_y_lock += (y_diff / mouse_move_scale) as f32;

        let new_cam: camera::Camera;
        let mut cam_pos_lock = cam_pos.lock().unwrap();


        if input.is_key_down(VirtualKeyCode::Up) {
            let cam_new = camera::Camera::move_direction(&mut cam_pos_lock, up, (*cam_degrees_x_lock, *cam_degrees_y_lock), move_spd, camera::Direction::Forward);
            new_cam = cam_new;
        } else if input.is_key_down(VirtualKeyCode::Down) {
            let cam_new = camera::Camera::move_direction(&mut cam_pos_lock, up, (*cam_degrees_x_lock, *cam_degrees_y_lock), move_spd, camera::Direction::Backward);
            new_cam = cam_new;
        } else if input.is_key_down(VirtualKeyCode::Left) {
            let cam_new = camera::Camera::move_direction(&mut cam_pos_lock, up, (*cam_degrees_x_lock, *cam_degrees_y_lock), move_spd, camera::Direction::Left);
            new_cam = cam_new;
        } else if input.is_key_down(VirtualKeyCode::Right) {
            let cam_new = camera::Camera::move_direction(&mut cam_pos_lock, up, (*cam_degrees_x_lock, *cam_degrees_y_lock), move_spd, camera::Direction::Right);
            new_cam = cam_new;
        } else {
            let cam_new = camera::Camera::move_direction(&mut cam_pos_lock, up, (*cam_degrees_x_lock, *cam_degrees_y_lock), 0., camera::Direction::Forward);
            new_cam = cam_new;
        }

        _engine.set_camera(new_cam);
    })
}

fn move_camera() {

}

#![allow(dead_code)]

use color_eyre::eyre::Result;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};
use std::collections::HashMap;
use winit::event::VirtualKeyCode;


mod camera;
mod engine;
mod image;
mod input;
mod renderer;
mod types;
mod vulkan;
mod object; 

const DT:f32 = 1.0/60.0;
const GOAL_CLUES:usize = 10;
const START_ROOM:usize = 0;
use types::*;

#[derive(Debug)]
struct GenericGameThing {}
impl engine::GameThing for GenericGameThing {}

pub struct World{
    rooms: Vec<object::Room>,
    objects: HashMap<usize, Box<dyn object::Object>> //key usize is the object id; to get room call contiainer
    //osborn has work arounds??? ^^^
}
impl World{
    fn new (rooms: Vec<object::Room>, objects: HashMap<usize, Box<dyn object::Object>>) -> World{
   World{rooms, objects} 
}

    fn found_clue (&mut self, obj_key: usize) ->(){
       if let Some(some_obj) = self.objects.get_mut(&obj_key) {
           if let object::ObjType::Clue = some_obj.otype{
               some_obj.found = true;
           }
       }
    }
}

pub fn player_vol () -> types::RPrism{
    types::RPrism{
        pos: Vec3::new(0.0,0.0,0.0), sz: Vec3::new(2.0, 2.0,2.0)
    }
}
/**
 * make world, set current room, 
 */

pub struct GameState{
    world: World,
    current_room: usize,
    player: object::Player,
    film_used: usize,
    clues_found: Vec<usize>,
    goal_clues: usize
}
impl GameState{
    fn new (world: World, current_room: usize, goal_clues: usize ) -> GameState{
        GameState{
            world, current_room, 
            player: object::Player::new(0, Some(current_room), player_vol()), //this is temp CHANGE
            film_used: 0,
            clues_found: Vec::new(),
            goal_clues
        }
    }

}

/**
 * try adding ibjec tot world
 * try getting to render
 */ 

 /**
  * side note: clues can maybe have descriptions to explaine
  after find all clues, some type of "Here's what happened" -> cutscene of crime *cries in animation*
  */
fn main() -> Result<()> {
    let world = World::new(vec![], vec![]); //figure this out
    let game_state = GameState::new(world, START_ROOM, GOAL_CLUES);

    
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

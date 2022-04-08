#![allow(dead_code)]

use color_eyre::eyre::Result;
use winit::event::VirtualKeyCode;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};
use std::collections::HashMap;

mod engine;
mod image;
mod input;
mod types;
mod camera;
mod vulkan;
mod renderer;
mod object; 

const DT:f32 = 1.0/60.0;
const GOAL_CLUES:usize = 10;
const START_ROOM:usize = 0;

#[derive(Debug)]
struct GenericGameThing {}
impl engine::GameThing for GenericGameThing {
}

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
xs
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
    world = World::new(); //figure this out
    game_state = GameState::new(world, START_ROOM, GOAL_CLUES);

    let mut engine:engine::Engine = engine::Engine::new(engine::WindowSettings::default());
    engine.set_camera(camera::Camera::look_at(Vec3::new(0.,-2.,-10.), Vec3::zero(), Vec3::unit_y()));
    
    //main body of game 
    engine.play(move |_engine| {
// stuff here

})

    
}

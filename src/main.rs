#![allow(dead_code)]

use crate::engine::Engine;
use crate::engine::WindowSettings;
pub use color_eyre;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};
use crate::camera::Camera;
use winit::event::VirtualKeyCode;

mod animation;
mod assets;
mod camera;
mod engine;
mod image;
mod input;
mod object;
mod renderer;
mod types;
mod vulkan;
use object::*;
use std::rc::Rc;
use types::*;

const DT: f64 = 1.0 / 60.0;
const GOAL_CLUES: usize = 10;
const START_ROOM: usize = 0;
const PLAYER_MOVE_SPD: f32 = 0.25;
const SNAP: VirtualKeyCode = VirtualKeyCode::S;
const NEXT: VirtualKeyCode = VirtualKeyCode::Space;

pub enum GameMode{
    StartScene,
    GamePlay,
    ClueDisplay,
    EndScene
}

#[derive(Debug)]
struct GenericGameThing {}
impl engine::GameThing for GenericGameThing {}

struct GameObject {
    trf: Similarity3,
    model: Rc<renderer::skinned::Model>,
    animation: assets::AnimRef,
    state: animation::AnimationState,
}
impl GameObject {
    fn tick_animation(&mut self) {
        self.state.tick(DT);
    }
}
struct Sprite {
    trf: Isometry3,
    tex: assets::TextureRef,
    cel: Rect,
    size: Vec2,
}
struct Flat {
    trf: Similarity3,
    model: Rc<renderer::flat::Model>,
}
struct Textured {
    trf: Similarity3,
    model: Rc<renderer::textured::Model>,
}

pub struct GameStuff {
    rooms: Vec<object::Room>,
    objects: HashMap<usize, Box<dyn object::Object>>, //key usize is the object id; to get room call contiainer
    //osborn has work arounds??? ^^^
    textured: Vec<Textured>,
    flats: Vec<Flat>,
    textures: Vec<assets::TextureRef>,
}
impl GameStuff {
    // fn new(rooms: Vec<object::Room>, objects: HashMap<usize, Box<dyn object::Object>>) -> GameStuff {
    //     GameStuff { rooms, objects }
    // }

    // fn found_clue(&mut self, obj_key: usize) -> () {
    //     if let Some(some_obj) = self.objects.get_mut(&obj_key) {
    //         if let object::ObjType::Clue = some_obj.otype {
    //             some_obj.found = true;
    //         }
    //     }
    // }
}

pub fn player_vol() -> types::RPrism {
    types::RPrism {
        pos: Vec3::new(0.0, 0.0, 0.0),
        sz: Vec3::new(2.0, 2.0, 2.0),
    }
}
/**
 * make world, set current room,
 */

pub struct GameState {
    stuff: GameStuff,
    current_room: usize,
    player: object::Player,
    film_used: usize,
    clues_found: Vec<usize>,
    goal_clues: usize,
    game_mode: GameMode
}

impl GameState {
    fn new(stuff: GameStuff, current_room: usize, goal_clues: usize) -> GameState {
        GameState {
            stuff,
            current_room,
            player: object::Player::new(
                0,
                Some(current_room),
                player_vol(),
                (0., 0.),
                PLAYER_MOVE_SPD,
            ), //this is temp CHANGE
            film_used: 0,
            clues_found: Vec::new(),
            goal_clues,
            game_mode: GameMode::GamePlay
        }
    }
}

impl engine::World for GameState {
    fn update(&mut self, input: &input::Input, _assets: &mut assets::Assets) {
        let player = &mut self.player;
        player.move_with_input(input);

        if input.is_key_pressed(SNAP){
            self.game_mode = GameMode::ClueDisplay;
            //do query stuff
        }
        
        if let GameMode::ClueDisplay = self.game_mode{
                if input.is_key_pressed(NEXT){
                            self.game_mode = GameMode::GamePlay;
                        }
        }

        
    }


    fn render(&mut self, _a: &mut assets::Assets, rs: &mut renderer::RenderState) {
        let camera; 
        match self.game_mode{
            GameMode::ClueDisplay => {
                camera = camera::Camera::look_at_degrees(-Vec3::unit_x(), Vec3::unit_y(), (0.0, 0.0));

                self.game_mode = GameMode::GamePlay;
            }
            GameMode::GamePlay => {
                camera = self.player.get_camera();
            }
            GameMode::StartScene => {
                camera = self.player.get_camera();
            }
             GameMode::EndScene => {
                camera = self.player.get_camera();
            }
        } 
        rs.set_camera(camera);

        // for (obj_i, obj) in self.things.iter_mut().enumerate() {
        //     rs.render_skinned(obj.model.clone(), obj.animation, obj.state, obj.trf, obj_i);
        // }

        
        for (m_i, m) in self.stuff.flats.iter_mut().enumerate() {
            rs.render_flat(m.model.clone(), m.trf, m_i);
        }
        for (t_i, t) in self.stuff.textured.iter_mut().enumerate() {
            rs.render_textured(t.model.clone(), t.trf, t_i);
        }

        for (s_i, s) in self.stuff.textures.iter_mut().enumerate() {
            rs.render_sprite(*s, Rect{pos: Vec2::new(0.0, 0.0), sz: Vec2::new(480.0, 480.0)}, Isometry3::default(), Vec2::new(480.0, 480.0), s_i);
        }
    }

}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut engine: Engine = Engine::new(WindowSettings::default(), DT);

    let camera = camera::Camera::look_at(Vec3::new(0., -2., -10.), Vec3::zero(), Vec3::unit_y());
    engine.set_camera(camera);
    let mut stuff = GameStuff {
        rooms: vec![],
        objects: HashMap::from([]),
        textured: vec![],
        flats: vec![],
        textures: vec![],
    };


    //load animation stuff 

    let flat_model = engine.load_flat(std::path::Path::new("content/livingroom.glb"))?;
    stuff.flats.push(Flat {
        trf: Similarity3::new(Vec3::new(0.0, 0.0, 10.0), Rotor3::from_rotation_yz(90.0f32.to_radians()), 1.0),
        model: flat_model,
    });
    let texture = engine.load_texture(std::path::Path::new("content/title_scene.png"))?;
    stuff.textures.push(texture);

    // let tex = engine.load_texture(std::path::Path::new("content/skins/robot3.png"))?;
    // let meshes = engine.load_textured(
    //     std::path::Path::new("content/characterSmall.fbx")
    // )?;
    // let robot = engine.create_textured_model(meshes, vec![tex]);

    // stuff.textured.push(Textured {
    //     trf: Similarity3::new(Vec3::new(0.0, 0.0, -10.0), Rotor3::identity(), 5.0),
    //     model: robot,
    // });
    let game_state = GameState::new(stuff, START_ROOM, GOAL_CLUES);
    engine.play_world(game_state)
}

#![allow(dead_code)]

use crate::engine::Engine;
use crate::engine::WindowSettings;
use crate::renderer::flat::NUM_CLUES;
pub use color_eyre;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};
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
const TAKE_PIC: VirtualKeyCode = VirtualKeyCode::Space;
const CLUE_FOUND_MIN_PIXELS: u32 = 1000;

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
pub struct Sprite {
    trf: Isometry3,
    tex: assets::TextureRef,
    cel: Rect,
    size: Vec2,
}
pub struct Flat {
    trf: Similarity3,
    model: Rc<renderer::flat::Model>,
}
pub struct Textured {
    trf: Similarity3,
    model: Rc<renderer::textured::Model>,
}

pub struct GameStuff {
    rooms: Vec<object::Room>,
    objects: HashMap<usize, Box<dyn object::Object>>, //key usize is the object id; to get room call contiainer
    //osborn has work arounds??? ^^^
    textured: Vec<Textured>,
    flats: Vec<Flat>,
    sprites: Vec<Sprite>,
}
impl GameStuff {
    fn new(
        rooms: Vec<object::Room>,
        objects: HashMap<usize, Box<dyn object::Object>>,
        textured: Vec<Textured>,
        flats: Vec<Flat>,
        sprites: Vec<Sprite>,
    ) -> GameStuff {
        GameStuff {
            rooms,
            objects,
            textured,
            flats,
            sprites,
        }
    }

    // fn found_clue(&mut self, obj_key: usize) -> () {
    //     if let Some(some_obj) = self.objects.get_mut(&obj_key) {
    //         if let object::ObjType::Clue = some_obj.otype {
    //             *some_obj.into_raw().found = true;
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
    clues_found: [bool; NUM_CLUES],
    goal_clues: usize,
    check_clues: bool,
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
            clues_found: [false; NUM_CLUES],
            goal_clues,
            check_clues: false,
        }
    }
}

impl engine::World for GameState {
    fn update(&mut self, input: &input::Input, _assets: &mut assets::Assets) {
        let player = &mut self.player;
        player.move_with_input(input);

        if input.is_key_pressed(TAKE_PIC) && self.film_used < self.player.film_capacity {
            self.check_clues = true;
            self.film_used += 1;
        }
    }
    fn render(&mut self, _a: &mut assets::Assets, rs: &mut renderer::RenderState) {
        let camera = self.player.get_camera();
        rs.set_camera(camera);

        for (_, object) in self.stuff.objects.iter() {
            match object.get_renderable() {
                Some(RenderType::Flat(id)) => {
                    let m = self.stuff.flats.get(id).unwrap();
                    rs.render_flat(m.model.clone(), m.trf, id); // id is the key
                }
                Some(RenderType::Textured(id)) => {
                    let t = self.stuff.textured.get(id).unwrap();
                    rs.render_textured(t.model.clone(), t.trf, id);
                }
                Some(RenderType::Sprite(id)) => {
                    let s = self.stuff.sprites.get(id).unwrap();
                    rs.render_sprite(s.tex, s.cel, s.trf, s.size, id);
                }
                None => (),
            }
        }
    }
    fn handle_query_pool_results(&mut self, query_pool_results: &[u32; NUM_CLUES]) {
        if self.check_clues {
            query_pool_results.iter().enumerate().for_each(|(index, num_pixels)| {
                if *num_pixels >= CLUE_FOUND_MIN_PIXELS {
                    self.clues_found[index] = true;
                    println!{"(Found clue #{:?}) Fascinating! I wonder if this is part of the crime...", index};
                }
            });
            println!{"Film remaining: {:?}", self.player.film_capacity - self.film_used };
            self.check_clues = false
        }
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "FULL");
    color_eyre::install()?;

    let mut engine: Engine = Engine::new(WindowSettings::default(), DT);

    let camera = camera::Camera::look_at(Vec3::new(0., -2., -10.), Vec3::zero(), Vec3::unit_y());
    engine.set_camera(camera);
    let mut stuff = GameStuff::new(vec![], HashMap::from([]), vec![], vec![], vec![]);

    let flat_model = engine.load_flat(std::path::Path::new("content/livingroom.glb"))?;
    stuff.flats.push(Flat {
        trf: Similarity3::new(
            Vec3::new(0.0, 0.0, 10.0),
            Rotor3::from_rotation_yz(90.0f32.to_radians()),
            1.0,
        ),
        model: flat_model,
    });
    stuff.objects.insert(
        0,
        Box::new(NotClue::new(
            0,
            None,
            RPrism {
                pos: Vec3::new(0., 0., 10.),
                sz: Vec3::new(0., 0., 0.),
            },
            RenderType::Flat(0),
        )),
    );

    let clue_model = engine.load_flat(std::path::Path::new("content/detail_crystal.glb"))?;
    stuff.flats.push(Flat {
        trf: Similarity3::new(Vec3::new(-5., 5., 20.), Rotor3::identity(), 1.0),
        model: clue_model,
    });
    stuff.objects.insert(
        1,
        Box::new(Clue::new(
            1,
            None,
            RPrism {
                pos: Vec3::new(-5., 5., 20.),
                sz: Vec3::new(0., 0., 0.),
            },
            RenderType::Flat(1),
        )),
    );

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

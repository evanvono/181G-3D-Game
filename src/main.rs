#![allow(dead_code)]

use crate::camera::Camera;
use crate::engine::Engine;
use crate::engine::WindowSettings;
use crate::renderer::flat::NUM_CLUES;
pub use color_eyre;
use color_eyre::eyre::Result;
use renderer::sprites::SingleRenderState as FSprite;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};
pub use ultraviolet::Bivec3;
use winit::event::VirtualKeyCode;

mod anim_2d;
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
const WIDTH: f32 = 1024.0;
const CLUE_FOUND_MIN_PIXELS: u32 = 1000;
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const TAKE_PIC: VirtualKeyCode = VirtualKeyCode::Space;
const SNAP: VirtualKeyCode = VirtualKeyCode::S;
const NEXT: VirtualKeyCode = VirtualKeyCode::Return;

#[derive(Debug, Clone)]
pub enum GameMode {
    StartScene,
    GamePlay,
    ClueDisplay,
    EndScene,
}

fn get_clues() -> Vec<ClueStuff> {
    return vec![
        ClueStuff{ asset: String::from("content/clues/fan.glb"), found_me: String::from("Huh, it looks like the ceiling fan fell down, and I think those are feathers among the debris..."), coords: Vec3::new(-18.418575,12.0,-196.45444), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/bed.glb"), found_me: String::from("A pretty simple bed for such a famous fowl."), coords: Vec3::new(-128.2365, 1.5, -116.138794), rotation: Rotor3::from_euler_angles(0.0,90.0f32.to_radians(), 90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/flower-out.glb"), found_me: String::from("Didn't they just send out a flier to the neighborhood about toxic plants for pets…I think some of these were on there"), coords: Vec3::new(-98.629196,1.5,-120.80953), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/flower-in.glb"), found_me: String::from("Wow these flowers smell pretty strong, I think some of them are from the garden"), coords: Vec3::new(-11.0, 10.0, -109.594604), rotation: Rotor3::from_euler_angles(0.0,90.0f32.to_radians(), 90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/shower.glb"), found_me: String::from("I don't think birds take showers, but there's no bathtub either"), coords: Vec3::new(1.,2.0,12.), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/mirror.glb"), found_me: String::from("These mirrors make it hard to take pictures, I almost can't look anywhere without seeing a reflection"), coords: Vec3::new(46.970825,12.0,-186.90634), rotation: Rotor3::from_euler_angles(0.0,90.0f32.to_radians(), 90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/crystal.glb"), found_me: String::from("Whoops almost missed this, the Panther Crystal Award for Excellence in Mystery with a Message, presented to Cecil Cedric Coulson IV on behalf of 'Cecil Seeks the Truth'"), coords: Vec3::new(40.632133, 1.5, -119.08712), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/bust.glb"), found_me: String::from("An animal bust which apparently was a gift from Scarlet Firefinn… interesting taste for an animal lover"), coords: Vec3::new(-53.680847, 1.5, -176.36743), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/fridge.glb"), found_me: String::from("Lots of meat and poultry, I thought actors, animal advocates, and birds alike preferred salads."), coords: Vec3::new(-30.202045, 3.0, -129.64348), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
        ClueStuff{ asset: String::from("content/clues/grass.glb"), found_me: String::from("This grass is like a jungle, I think it's taller than Cecil!"), coords: Vec3::new(-114.653435,1.5,-145.70198), rotation: Rotor3::from_rotation_yz(90.0f32.to_radians()), scale: 2.8},
    ];
}

struct ClueStuff {
    asset: String,
    found_me: String,
    coords: Vec3,
    rotation: Rotor3,
    scale: f32
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
    textures: Vec<Sprite>,
}
impl GameStuff {
    fn new(
        rooms: Vec<object::Room>,
        objects: HashMap<usize, Box<dyn object::Object>>,
        textured: Vec<Textured>,
        flats: Vec<Flat>,
        textures: Vec<Sprite>,
    ) -> GameStuff {
        GameStuff {
            rooms,
            objects,
            textured,
            flats,
            textures,
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
    prev_clues_found: [bool; NUM_CLUES],
    clues_found: [bool; NUM_CLUES],
    goal_clues: usize,
    game_mode: GameMode,
    check_clues: bool,
    display_texture: usize,
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
            prev_clues_found: [false; NUM_CLUES],
            clues_found: [false; NUM_CLUES],
            goal_clues,
            game_mode: GameMode::StartScene,
            check_clues: false,
            display_texture: 0,
        }
    }
}

impl engine::World for GameState {
    fn update(&mut self, input: &input::Input, _assets: &mut assets::Assets) {
        let player = &mut self.player;
        player.move_with_input(input);

        match self.game_mode {
            GameMode::StartScene => {
                if input.is_key_pressed(NEXT) {
                    self.display_texture += 1;
                    if self.display_texture >= 3 {
                        self.game_mode = GameMode::GamePlay;
                        self.player.reset_player();
                    }
                }
            }
            GameMode::ClueDisplay => {
                if self.display_texture == 5 {
                    if input.is_key_pressed(VirtualKeyCode::Key4) {
                        self.display_texture = 6;
                    } else if input.is_key_pressed(VirtualKeyCode::Key3)
                        || input.is_key_pressed(VirtualKeyCode::Key5)
                        || input.is_key_pressed(VirtualKeyCode::Key6)
                        || input.is_key_pressed(VirtualKeyCode::Key7)
                        || input.is_key_pressed(VirtualKeyCode::Key8)
                    {
                        self.display_texture = 7;
                    }
                } else if input.is_key_pressed(NEXT) {
                    self.display_texture += 1;
                    if self.display_texture >= 7  {
                        self.display_texture = 8;
                        self.game_mode = GameMode::EndScene;
                    }
                }
            }
            GameMode::EndScene => {
                if input.is_key_pressed(NEXT) && self.display_texture <= 9 {
                    self.display_texture += 1;
                }
            }
            GameMode::GamePlay => {
                if input.is_key_pressed(TAKE_PIC) && self.film_used < self.player.film_capacity {
                    //self.game_mode = GameMode::ClueDisplay;
                    //self.player.pause_rotation();
                    self.check_clues = true;
                    self.film_used += 1;
                }
                if self.film_used >= self.player.film_capacity {
                    self.game_mode = GameMode::ClueDisplay;
                }
            }
        }
    }

    fn render(&mut self, _a: &mut assets::Assets, rs: &mut renderer::RenderState) {
        let camera;
        match self.game_mode {
            GameMode::GamePlay => {
                camera = self.player.get_camera();
            }
            _ => {
                /*camera = self.player.get_camera();
                camera.projection = camera::Projection::Orthographic {
                        width: 1000.0,
                        depth: 1000.0,
                    };*/

                //for the first panel
                camera = Camera {
                    transform: Similarity3 {
                        translation: Vec3 {
                            x: 285.9119,
                            y: -2.0,
                            z: -75.11849,
                        },
                        rotation: Rotor3 {
                            s: 0.999229,
                            bv: Bivec3 {
                                xy: -0.0,
                                xz: 0.03926036,
                                yz: 0.0,
                            },
                        },
                        scale: 1.0,
                    },
                    ratio: 1.3333334,
                    projection: camera::Projection::Orthographic {
                        width: 1000.0,
                        depth: 1000.0,
                    },
                }
            }
        }

        rs.set_camera(camera);

        // for (obj_i, obj) in self.things.iter_mut().enumerate() {
        //     rs.render_skinned(obj.model.clone(), obj.animation, obj.state, obj.trf, obj_i);
        // }

        // dbg!(self.game_mode.clone());

        match self.game_mode {
            GameMode::GamePlay => {
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
                        Some(RenderType::Sprite(id)) => (),
                        // {
                        //     let s = self.stuff.sprites.get(id).unwrap();
                        //     rs.render_sprite(s.tex, s.cel, s.trf, s.size, id);
                        // }
                        None => (),
                    }
                }
            },
            _ => {
                for (s_i, s) in self.stuff.textures.iter_mut().enumerate() {
                    if s_i == self.display_texture{
                        rs.render_sprite(s.tex, s.cel, s.trf, s.size, s_i);
                    }
                }
            }
        }
    }

    fn handle_query_pool_results(&mut self, query_pool_results: &[u32; NUM_CLUES]) {
        if self.check_clues {
            self.prev_clues_found = self.clues_found;
            query_pool_results.iter().enumerate().for_each(|(index, num_pixels)| {
                if *num_pixels >= CLUE_FOUND_MIN_PIXELS && !self.prev_clues_found[index] {
                    self.clues_found[index] = true;
                    let clues = get_clues();
                    println!{"{:?}", clues[index].found_me}
                }
            });
            println! {"Film remaining: {:?}", self.player.film_capacity - self.film_used };
            self.check_clues = false
        }
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "FULL");
    color_eyre::install()?;

    let mut engine: Engine = Engine::new(WindowSettings::default(), DT);

    let camera = camera::Camera::look_at(
        Vec3::new(0., -2., -10.),
        Vec3::zero(),
        Vec3::unit_y(),
        camera::Projection::Perspective { fov: PI / 2.0 },
    );
    engine.set_camera(camera);

    let mut stuff = GameStuff {
        rooms: vec![],
        objects: HashMap::from([]),
        textured: vec![],
        flats: vec![],
        textures: vec![],
    };

    let flat_model = engine.load_flat(std::path::Path::new("content/livingroom-base.glb"))?;
    stuff.flats.push(Flat {
        trf: Similarity3::new(
            Vec3::new(0.0, 0.0, 0.0),
            Rotor3::from_rotation_yz(90.0f32.to_radians()),
            1.0,
        ),
        model: flat_model,
    });

    get_clues().iter().enumerate().for_each(|(index, clue)| {
        let clue_model = engine.load_flat(std::path::Path::new(&clue.asset));
        stuff.flats.push(Flat {
            trf: Similarity3::new(clue.coords, clue.rotation, clue.scale),
            model: clue_model.unwrap(),
        });
        stuff.objects.insert(
            1 + index,
            Box::new(Clue::new(
                1 + index,
                None,
                RPrism {
                    pos: clue.coords,
                    sz: Vec3::new(0., 0., 0.),
                },
                RenderType::Flat(1 + index),
            )),
        );
    });

    //dbg!(stuff.flats[0].trf);
    let intro = engine.load_texture(std::path::Path::new("content/comic_panel_1.png"))?;
    let bios = engine.load_texture(std::path::Path::new("content/comic_bios.png"))?;
    let news = engine.load_texture(std::path::Path::new("content/comic_news.png"))?;
    let clues1 = engine.load_texture(std::path::Path::new("content/comic_clue_1.png"))?;
    let clues2 = engine.load_texture(std::path::Path::new("content/comic_clue_2.png"))?;
    let whodunnit = engine.load_texture(std::path::Path::new("content/comic_guess.png"))?;
    let correct = engine.load_texture(std::path::Path::new("content/comic_guess_correct.png"))?;
    let wrong = engine.load_texture(std::path::Path::new("content/comic_guess_wrong.png"))?;
    let hap = engine.load_texture(std::path::Path::new("content/comic_what_happened.png"))?;
    let hap1 = engine.load_texture(std::path::Path::new("content/comic_happened_1.png"))?;
    let hap2 = engine.load_texture(std::path::Path::new("content/comic_happened_2.png"))?;

    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: news,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: intro,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: bios,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: clues1,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: clues2,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: whodunnit,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: correct,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: wrong,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: hap,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: hap1,
    });
    stuff.textures.push(Sprite {
        trf: Isometry3::new(Vec3::new(-300.0, 0.0, 0.0), Rotor3::identity()),
        size: Vec2::new(900.0, 500.0),
        cel: Rect::new(0.0, 0.0, 1.0, 1.0),
        tex: hap2,
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


    let game_state = GameState::new(stuff, START_ROOM, GOAL_CLUES);
    engine.play_world(game_state)
}

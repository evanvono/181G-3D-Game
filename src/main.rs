#![allow(dead_code)]

use crate::engine::Engine;
use crate::engine::WindowSettings;
pub use color_eyre;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub use ultraviolet::vec::{Vec2, Vec3};

mod camera;
mod engine;
mod image;
mod input;
mod object;
mod renderer;
mod types;
mod vulkan;
mod animation;
mod assets;
use object::*;
use types::*;
use std::rc::Rc;

const DT: f64 = 1.0 / 60.0;
const GOAL_CLUES: usize = 10;
const START_ROOM: usize = 0;


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
    textured: Vec<Textured> 
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
}

impl GameState {
    fn new(stuff: GameStuff, current_room: usize, goal_clues: usize) -> GameState {
        GameState {
            stuff,
            current_room,
            player: object::Player::new(0, Some(current_room), player_vol(), (0.,0.)), //this is temp CHANGE
            film_used: 0,
            clues_found: Vec::new(),
            goal_clues,
        }
    }
}

impl engine::World for GameState {
    fn update(&mut self, input: &input::Input, _assets: &mut assets::Assets) { 
        let player = &mut self.player;
        player.move_with_input(input);


    }
    fn render(&mut self, _a: &mut assets::Assets, rs: &mut renderer::RenderState) { 
        let camera = self.player.get_camera();
        dbg!(camera);
        rs.set_camera(camera);

        // for (obj_i, obj) in self.things.iter_mut().enumerate() {
        //     rs.render_skinned(obj.model.clone(), obj.animation, obj.state, obj.trf, obj_i);
        // }
        // for (s_i, s) in self.sprites.iter_mut().enumerate() {
        //     rs.render_sprite(s.tex, s.cel, s.trf, s.size, s_i);
        // }
        // for (m_i, m) in self.flats.iter_mut().enumerate() {
        //     rs.render_flat(m.model.clone(), m.trf, m_i);
        // }
        for (t_i, t) in self.stuff.textured.iter_mut().enumerate() {
            rs.render_textured(t.model.clone(), t.trf, t_i);
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut engine: Engine = Engine::new(WindowSettings::default(), DT);

    let camera = camera::Camera::look_at(
        Vec3::new(0., -2., -10.),
        Vec3::zero(),
        Vec3::unit_y(),
    );
    engine.set_camera(camera);
    let mut stuff = GameStuff {rooms: vec![], objects: HashMap::from([]), textured: vec![]};
    let tex = engine.load_texture(std::path::Path::new("content/robot.png"))?;
    let meshes = engine.load_textured(
        std::path::Path::new("content/characterSmall.fbx")
    )?;
    let robot = engine.create_textured_model(meshes, vec![tex]);

    stuff.textured.push(Textured {
        trf: Similarity3::new(Vec3::new(0.0, 0.0, -10.0), Rotor3::identity(), 5.0),
        model: robot,
    });
    let game_state = GameState::new(stuff, START_ROOM, GOAL_CLUES);
    engine.play_world(game_state)
}

/*
 * try adding ibjec tot world
 * try getting to render
 */

/*
* side note: clues can maybe have descriptions to explaine
after find all clues, some type of "Here's what happened" -> cutscene of crime *cries in animation*
*/
/*
fn main() -> Result<()> {
    let world = World::new(vec![], vec![]); //figure this out
    let game_state = GameState::new(world, START_ROOM, GOAL_CLUES);

    color_eyre::install()?;

    // If GameThing variants differ widely in size, consider using
    // Box<GameThing>
    let mut engine: Engine = Engine::new(WindowSettings::default(), DT);
    engine.set_camera(camera::Camera::look_at(
        Vec3::new(0., -2., -10.),
        Vec3::zero(),
        Vec3::unit_y(),
    ));

    /* BELOW SNIPPET CREATES THE ROBOT DUDE USING THE SCENE3D RENDERER
    
        let tex = engine.load_texture(std::path::Path::new("content/robot.png"))?;
        let mesh = engine.load_mesh(std::path::Path::new("content/characterSmall.fbx"), 0.1)?;
        let model = engine.create_model(&mesh, &tex);

        engine.create_game_object(
            Some(&model),
            Isometry3::new(Vec3::new(0.0, -12.5, 25.0), Rotor3::identity()),
            Box::new(GenericGameThing {}),
            None,
        );

    */

    
    engine.play(move |_engine| {
        let player = game_state.player;
        player.move_with_input(&_engine);

        _engine.set_camera(player.get_camera());
    })
}
*/
// TODO: Break this up? Move to Player?

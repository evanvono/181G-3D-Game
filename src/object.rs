use super::types::*;

pub use std::f32::consts::PI;
pub use ultraviolet::mat::Mat4;
pub use ultraviolet::rotor::Rotor3;
pub use ultraviolet::transform::Isometry3;
pub use ultraviolet::vec::{Vec2, Vec3};
use crate::camera;
use crate::input;
use winit::event::VirtualKeyCode;


pub enum ObjType {
    Room,
    Clue,
    NotClue,
    Player,
}

pub trait Object {
    
    fn get_id(&self) -> usize;
    fn get_type(&self) -> ObjType;
    fn get_container(&self)-> Option<usize>;
    fn get_volume(&self)-> RPrism;
    fn get_pos(&self) -> Vec3;
    fn set_pos(&self,pos:Vec3);
}

pub struct Room{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub is_occupied: bool,
}
impl Room{
fn new(id: usize, container: Option<usize>, volume: RPrism) -> Room{
        Room{
            id, otype: ObjType::Room, container, volume, is_occupied: false
        }
    }
}
impl Object for Room{
    fn get_id(&self) -> usize
    { 
        self.id
    }
    fn get_type(&self) -> ObjType{
        self.otype
    }
    fn get_container(&self)-> Option<usize>
    {
        self.container
    }
    fn get_volume(&self)-> RPrism
    {
        self.volume
    }
    fn get_pos(&self) -> Vec3{
        self.volume.pos
    }
    fn set_pos(&self, pos: Vec3) {
        self.volume.pos = pos
    }
}

pub struct Clue{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub found: bool,
}
impl Clue{
     fn new(id: usize, container: Option<usize>, volume: RPrism) -> Clue{
        Clue{
            id, otype: ObjType::Clue, container, volume, found: false
        }
    }
}
impl Object for Clue{
    fn get_id(&self) -> usize
    { 
        self.id
    }
    fn get_type(&self) -> ObjType{
        self.otype
    }
    fn get_container(&self)-> Option<usize>
    {
        self.container
    }
    fn get_volume(&self)-> RPrism
    {
        self.volume
    }
    fn get_pos(&self) -> Vec3{
        self.volume.pos
    }
    fn set_pos(&self, pos: Vec3) {
        self.volume.pos = pos
    }
}

pub struct NotClue{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
}
impl NotClue{
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> NotClue{
        NotClue{
            id, otype: ObjType::NotClue, container, volume
        }
    }
}
impl Object for NotClue{
    fn get_id(&self) -> usize
    { 
        self.id
    }
    fn get_type(&self) -> ObjType{
        self.otype
    }
    fn get_container(&self)-> Option<usize>
    {
        self.container
    }
    fn get_volume(&self)-> RPrism
    {
        self.volume
    }
    fn get_pos(&self) -> Vec3{
        self.volume.pos
    }
    fn set_pos(&self, pos: Vec3) {
        self.volume.pos = pos
    }
}
pub struct Player{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub film_capacity: usize,
    pub perspective_deg: (f32, f32),
    pub move_spd: f32
}
impl Player{
    pub fn new(id: usize, container: Option<usize>, volume: RPrism, perspective_deg: (f32, f32)) -> Player{
        Player{
            id, otype: ObjType::Room, container, volume,
            film_capacity: 10, perspective_deg, move_spd: 1.0
        }
    }
    pub fn get_camera(&self) -> camera::Camera {
        camera::Camera::look_at_degrees(self.get_pos(), Vec3::unit_y(), self.get_deg())
    }
    pub fn get_deg(&self) -> (f32,f32) {
        self.perspective_deg
    }
    pub fn set_deg(&self, deg: (f32,f32)) {
        self.perspective_deg = deg
    }

    pub fn move_with_input(&self, input: &input::Input) {
        let delta = input.get_mouse_delta();
    
        let mut is_moving = false;
        let mut directions: Vec<Direction> = Vec::new();
    
        if input.is_key_down(VirtualKeyCode::Up) || input.is_key_down(VirtualKeyCode::W) {
            directions.push(Direction::Forward);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Down) || input.is_key_down(VirtualKeyCode::S) {
            directions.push(Direction::Backward);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Left) || input.is_key_down(VirtualKeyCode::A) {
            directions.push(Direction::Left);
            is_moving = true;
        }
        if input.is_key_down(VirtualKeyCode::Right) || input.is_key_down(VirtualKeyCode::D) {
            directions.push(Direction::Right);
            is_moving = true;
        }
    
        let (mut cam_degrees_x, mut cam_degrees_y) = self.get_deg();
    
        cam_degrees_x = cam_degrees_x - (delta.x as f32 / input::Input::get_mouse_move_scale());
        cam_degrees_y = cam_degrees_y - (delta.y as f32 / input::Input::get_mouse_move_scale());
    
        if cam_degrees_y > 89.999 {
            cam_degrees_y = 89.999
        }
        if cam_degrees_y < -89.999 {
            cam_degrees_y = -89.999
        }
    
        let mut theta = cam_degrees_x;
        directions.iter().for_each(|direction| {
            match direction {
                Direction::Right => theta -= 90.0,
                Direction:: Left => theta += 90.0,
                Direction::Backward => theta += 180.0,
                _ => ()
            }
        });
    
        let distance = if is_moving { self.move_spd } else { 0. };
    
        let pos = self.get_pos();
        pos.z += (distance * theta.to_radians().cos()) as f32;
        pos.x += (distance * theta.to_radians().sin()) as f32;
        
        self.set_pos(pos);
        self.set_deg((cam_degrees_x, cam_degrees_y));
    }
}
impl Object for Player{
    
    fn get_id(&self) -> usize
    { 
        self.id
    }
    fn get_type(&self) -> ObjType{
        self.otype
    }
    fn get_container(&self)-> Option<usize>
    {
        self.container
    }
    fn get_volume(&self)-> RPrism
    {
        self.volume
    }
    fn get_pos(&self) -> Vec3{
        self.volume.pos
    }
    fn set_pos(&self, pos: Vec3) {
        self.volume.pos = pos;
    }
}


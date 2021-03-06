use super::types::*;

pub use std::f32::consts::PI;
pub use ultraviolet::mat::Mat4;
pub use ultraviolet::rotor::Rotor3;
pub use ultraviolet::transform::Isometry3;
pub use ultraviolet::vec::{Vec2, Vec3};
use crate::camera;
use crate::input;
use winit::event::VirtualKeyCode;

const PLAYER_HEIGHT: f32 = 2.;
const PAUSE: VirtualKeyCode = VirtualKeyCode::Key1;
const UNPAUSE: VirtualKeyCode = VirtualKeyCode::Key0;
const RESET_PLAYER: VirtualKeyCode = VirtualKeyCode::Q;
const RESET_POS: VirtualKeyCode = VirtualKeyCode::W;
const RESET_DEG: VirtualKeyCode = VirtualKeyCode::E;

#[derive(Copy, Clone)]
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
    fn set_pos(&mut self,pos:Vec3);
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
    fn set_pos(&mut self, pos: Vec3) {
        self.volume.pos = pos;
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
    fn set_pos(&mut self, pos: Vec3) {
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
    fn set_pos(&mut self, pos: Vec3) {
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
    pub move_spd: f32,
    pub pause_rot: bool,
    default_pos: Vec3,
    default_deg: (f32, f32)
}
impl Player{
    pub fn new(id: usize, container: Option<usize>, volume: RPrism, perspective_deg: (f32, f32), move_spd: f32) -> Player{
        Player{
            id, otype: ObjType::Room, container, volume,
            film_capacity: 10, perspective_deg, move_spd, 
            pause_rot: false, default_pos: volume.pos, default_deg: perspective_deg
        }
    }
    pub fn get_camera(&self) -> camera::Camera {
        let player_at = self.get_pos();
        let at = Vec3::new(player_at.x, PLAYER_HEIGHT, player_at.z);
        camera::Camera::look_at_degrees(at, Vec3::unit_y(), self.get_deg())
    }

    pub fn get_default_camera(&self) -> camera::Camera {
        let player_at = self.default_pos;
        let at = Vec3::new(player_at.x, PLAYER_HEIGHT, player_at.z);
        camera::Camera::look_at_degrees(at, Vec3::unit_y(), self.default_deg)
    }
   
    pub fn get_deg(&self) -> (f32,f32) {
        self.perspective_deg
    }
    pub fn set_deg(&mut self, deg: (f32,f32)) {
        self.perspective_deg = deg
    }

    pub fn pause_rotation(&mut self){
        self.pause_rot = true;
    }

    pub fn unpause_rotation(&mut self){
        self.pause_rot = false;
    }

    pub fn move_with_input(&mut self, input: &input::Input) {
        let delta = input.get_mouse_delta();
    
        let mut is_moving = false;
        let mut directions: Vec<Direction> = Vec::new();

        if input.is_key_pressed(PAUSE){
            self.pause_rotation();
        }
        else if input.is_key_pressed(UNPAUSE){
            self.unpause_rotation();
        }
    
        if input.is_key_down(VirtualKeyCode::Up) || input.is_key_down(VirtualKeyCode::W) {
            directions.push(Direction::Forward);
            is_moving = true;
        }
        else if input.is_key_down(VirtualKeyCode::Down) || input.is_key_down(VirtualKeyCode::S) {
            directions.push(Direction::Backward);
            is_moving = true;
        }
        else if input.is_key_down(VirtualKeyCode::Right) || input.is_key_down(VirtualKeyCode::D) {
            directions.push(Direction::Left);
            is_moving = true;
        }
        else if input.is_key_down(VirtualKeyCode::Left) || input.is_key_down(VirtualKeyCode::A) {
            directions.push(Direction::Right);
            is_moving = true;
        }
    
        let (mut cam_degrees_x, mut cam_degrees_y) = self.get_deg();
    
        if !self.pause_rot {
            cam_degrees_x -= delta.x as f32 / input::Input::get_mouse_move_scale();
            cam_degrees_y += delta.y as f32 / input::Input::get_mouse_move_scale();
        }
        
        if cam_degrees_y > 89.0 {
            cam_degrees_y = 89.0
        }
        if cam_degrees_y < -89.0 {
            cam_degrees_y = -89.0
        }
    
        let mut theta = cam_degrees_x;
        directions.iter().for_each(|direction| {
            match direction {
                Direction::Left => theta -= 90.0,
                Direction::Right => theta += 90.0,
                Direction::Backward => theta += 180.0,
                _ => ()
            }
        });
    
        let distance = if is_moving { self.move_spd } else { 0. };
    
        let mut pos = self.get_pos();
        pos.z += (distance * theta.to_radians().cos()) as f32;
        pos.x += (distance * theta.to_radians().sin()) as f32;
        
        if input.is_key_pressed(RESET_PLAYER){
            self.set_pos(self.default_pos);
            self.set_deg(self.default_deg);
        }
        else if input.is_key_pressed(RESET_POS){
            self.set_pos(self.default_pos);
            self.set_deg((cam_degrees_x, cam_degrees_y));
        }
        else if input.is_key_pressed(RESET_DEG){
            self.set_pos(pos);
            self.set_deg(self.default_deg);
        }
        else {
            self.set_pos(pos);
            self.set_deg((cam_degrees_x, cam_degrees_y));
        }
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
    fn set_pos(&mut self, pos: Vec3) {
        self.volume.pos = pos;
    }
}


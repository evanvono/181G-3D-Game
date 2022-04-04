use super::types::*;
use bytemuck::{Pod, Zeroable};
pub use std::f32::consts::PI;
pub use ultraviolet::mat::Mat4;
pub use ultraviolet::rotor::Rotor3;
pub use ultraviolet::transform::Isometry3;
pub use ultraviolet::vec::{Vec2, Vec3};

pub enum ObjType {
    Room,
    Clue,
    NotClue,
    Player,
}

pub trait Object {
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> Self;
    fn get_id(&self) -> usize;
    fn get_type(&self) -> ObjType;
    fn get_container(&self)-> Option<usize>;
    fn get_volume(&self)-> RPrism;
    fn get_pos(&self) -> Vec3;
}

pub struct Room{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub is_occupied: bool,
}
impl Object for Room{
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> Room{
        Room{
            id, otype: ObjType::Room, container, volume, is_occupied: false
        }
    };
    fn get_id(&self) -> usize
    { 
        self.id
    };
    fn get_type(&self) -> ObjType{
        self.otype
    };
    fn get_container(&self)-> Option<usize>
    {
        self.container
    };
    fn get_volume(&self)-> RPrism
    {
        self.volume
    };
    fn get_pos(&self) -> Vec3{
        self.volume.pos;
    };
}

pub struct Clue{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub found: bool,
}
impl Object for Clue{
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> Clue{
        Clue{
            id, otype: ObjType::Clue, container, volume, found: false
        }
    };
    fn get_id(&self) -> usize
    { 
        self.id
    };
    fn get_type(&self) -> ObjType{
        self.otype
    };
    fn get_container(&self)-> Option<usize>
    {
        self.container
    };
    fn get_volume(&self)-> RPrism
    {
        self.volume
    };
    fn get_pos(&self) -> Vec3{
        self.volume.pos;
    };
}

pub struct NotClue{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
}
impl Object for NotClue{
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> NotClue{
        Room{
            id, otype: ObjType::NotClue, container, volume
        }
    };
    fn get_id(&self) -> usize
    { 
        self.id
    };
    fn get_type(&self) -> ObjType{
        self.otype
    };
    fn get_container(&self)-> Option<usize>
    {
        self.container
    };
    fn get_volume(&self)-> RPrism
    {
        self.volume
    };
    fn get_pos(&self) -> Vec3{
        self.volume.pos;
    };
}
pub struct Player{
    pub id: usize,
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub film_used: usize,
    pub film_capacity: usize,
    pub clues_found: Vec<usize>,
}
impl Object for Player{
    fn new(id: usize, container: Option<usize>, volume: RPrism) -> Player{
        Player{
            id, otype: ObjType::Room, container, volume, film_used: 0, 
            film_capacity: 10, clues_found: 0
        }
    };
    fn get_id(&self) -> usize
    { 
        self.id
    };
    fn get_type(&self) -> ObjType{
        self.otype
    };
    fn get_container(&self)-> Option<usize>
    {
        self.container
    };
    fn get_volume(&self)-> RPrism
    {
        self.volume
    };
    fn get_pos(&self) -> Vec3{
        self.volume.pos;
    };
}


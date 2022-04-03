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
}

/**
 * make object trait? have clue extend that, so that can have "found"
 */
pub struct Object{
    pub id: usize
    pub otype: ObjType,
    pub container: Option<usize>,
    pub volume: RPrism,
    pub 
}
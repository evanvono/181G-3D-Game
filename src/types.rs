use bytemuck::{Pod, Zeroable};
pub use std::f32::consts::PI;
pub use ultraviolet::mat::Mat4;
pub use ultraviolet::rotor::Rotor3;
pub use ultraviolet::transform::Isometry3;
pub use ultraviolet::vec::{Vec2, Vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub pos: Vec2,
    pub sz: Vec2,
}

impl Rect {
    pub fn contains(&self, other: Rect) -> bool {
        let br = self.pos + self.sz;
        let obr = other.pos + other.sz;
        self.pos.x <= other.pos.x && self.pos.y <= other.pos.y && obr.x <= br.x && obr.y <= br.y
    }
}

/**
 * Rectangular Prism
 */
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RPrism {
    pub pos: Vec3,
    pub sz: Vec3,
}
impl RPrism {
    pub fn contains(&self, other: RPrism) -> bool {
        let br = self.pos + self.sz;
        let obr = other.pos + other.sz;
        self.pos.x <= other.pos.x
            && self.pos.y <= other.pos.y
            && self.pos.z <= other.pos.z
            && &&obr.x <= br.x
            && obr.y <= br.y
            && obr.z <= br.z
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Zeroable, Pod)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

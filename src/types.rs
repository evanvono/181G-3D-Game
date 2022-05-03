use bytemuck::{Pod, Zeroable};
pub use std::f32::consts::PI;
pub use ultraviolet::mat::Mat4;
pub use ultraviolet::rotor::Rotor3;
pub use ultraviolet::transform::{Isometry3, Similarity3};
pub use ultraviolet::vec::{Vec2, Vec3, Vec4};
pub use ultraviolet::Lerp;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub pos: Vec2,
    pub sz: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            sz: Vec2::new(w, h),
        }
    } 
    pub fn contains(&self, other: Rect) -> bool {
        let br = self.pos + self.sz;
        let obr = other.pos + other.sz;
        self.pos.x <= other.pos.x && self.pos.y <= other.pos.y && obr.x <= br.x && obr.y <= br.y
    }
}

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
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
            && &&obr.x <= &&br.x
            && obr.y <= br.y
            && obr.z <= br.z
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Zeroable, Pod)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

pub trait LerpF {
    fn lerp(&self, other: &Self, r: f32) -> Self;
}
impl LerpF for Similarity3 {
    fn lerp(&self, other: &Self, r: f32) -> Self {
        Self::new(
            self.translation.lerp(other.translation, r),
            self.rotation.lerp(other.rotation, r).normalized(),
            self.scale.lerp(other.scale, r),
        )
    }
}
impl LerpF for Isometry3 {
    fn lerp(&self, other: &Self, r: f32) -> Self {
        Self::new(
            self.translation.lerp(other.translation, r),
            self.rotation.lerp(other.rotation, r).normalized(),
        )
    }
}

impl LerpF for Rect {
    fn lerp(&self, other: &Self, r: f32) -> Self {
        Self {
            pos: self.pos.lerp(other.pos, r),
            sz: self.sz.lerp(other.sz, r),
        }
    }
}

pub trait Interpolate {
    fn interpolate(&self, other: Self, r: f32) -> Self;
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self;
}
impl Interpolate for f32 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        self.lerp(other, r)
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        if (other - self).abs() >= lim {
            other
        } else {
            self.interpolate(other, r)
        }
    }
}
impl Interpolate for Similarity3 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        Self::new(
            self.translation.interpolate(other.translation, r),
            self.rotation.interpolate(other.rotation, r),
            self.scale.interpolate(other.scale, r),
        )
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        Self::new(
            self.translation
                .interpolate_limit(other.translation, r, lim),
            self.rotation.interpolate_limit(other.rotation, r, PI / 4.0),
            self.scale.interpolate_limit(other.scale, r, 0.5),
        )
    }
}
impl Interpolate for Isometry3 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        Self::new(
            self.translation.interpolate(other.translation, r),
            self.rotation.interpolate(other.rotation, r),
        )
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        Self::new(
            self.translation
                .interpolate_limit(other.translation, r, lim),
            self.rotation.interpolate_limit(other.rotation, r, PI / 4.0),
        )
    }
}

impl Interpolate for Rect {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        Self {
            pos: self.pos.interpolate(other.pos, r),
            sz: self.sz.interpolate(other.sz, r),
        }
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        Self {
            pos: self.pos.interpolate_limit(other.pos, r, lim),
            sz: self.sz.interpolate_limit(other.sz, r, 0.5),
        }
    }
}

impl Interpolate for Vec2 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        Vec2::lerp(self, other, r)
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        if (other - *self).mag_sq() >= lim * lim {
            other
        } else {
            self.interpolate(other, r)
        }
    }
}

impl Interpolate for Vec3 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        Vec3::lerp(self, other, r)
    }
    fn interpolate_limit(&self, other: Self, r: f32, lim: f32) -> Self {
        if (other - *self).mag_sq() >= lim * lim {
            other
        } else {
            self.interpolate(other, r)
        }
    }
}

impl Interpolate for Rotor3 {
    fn interpolate(&self, other: Self, r: f32) -> Self {
        self.lerp(other, r).normalized()
    }
    fn interpolate_limit(&self, other: Self, r: f32, _lim: f32) -> Self {
        self.interpolate(other, r)
    }
}

use crate::types::*;
pub struct Camera {
    eye:Vec3, // Position
    at:Vec3, // Where we're looking
    up:Vec3, // Which way is up
    fov:f32,
    ratio:f32,
    z_far:f32
}
impl Camera {
    pub fn look_at(eye:Vec3, at:Vec3, up:Vec3) -> Camera {
        Camera{eye, at, up, fov:PI/2.0, ratio:4.0/3.0, z_far:1000.0}
    }

    pub fn look_at_degrees(eye: Vec3, up: Vec3, deg: (f32,f32)) -> Camera {
        // for changes in the x axis rotation, we change y and z values
        // for changes in the y axis rotation, we change x and z values
        let theta = deg.0;
        let rho = deg.1;
        let x = eye.x + rho.sin() * theta.sin();
        let y = eye.y + rho.cos();
        let z = eye.z + rho.sin() * theta.cos();
        let at = Vec3 { x, y, z };
        Self::look_at(eye, at, up)
    }

    pub fn as_matrix(&self) -> Mat4 {
        // projection * view
        let proj = ultraviolet::projection::perspective_vk(self.fov, self.ratio, 0.01, self.z_far);
        proj * Mat4::look_at(self.eye, self.at, self.up)
    }
}
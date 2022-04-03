use crate::image::Vec2i;
use winit::event::ElementState;
use winit::event::VirtualKeyCode;

#[derive(Debug, Clone)]
pub struct Input {
    now_keys: Box<[bool]>,
    prev_keys: Box<[bool]>,
    mouse_down: bool,
    prev_mouse_down: bool,
    mouse_position: winit::dpi::PhysicalPosition<f64>,
    prev_mouse_position: winit::dpi::PhysicalPosition<f64>,
}
impl Input {
    pub(crate) fn new() -> Self {
        Self {
            now_keys: vec![false; 255].into_boxed_slice(),
            prev_keys: vec![false; 255].into_boxed_slice(),
            mouse_down: false,
            prev_mouse_down: false,
            mouse_position: winit::dpi::PhysicalPosition { x: 0.0, y: 0.0 },
            prev_mouse_position: winit::dpi::PhysicalPosition { x: 0.0, y: 0.0 },
        }
    }
    pub fn is_key_down(&self, kc: VirtualKeyCode) -> bool {
        self.now_keys[kc as usize]
    }
    pub fn is_key_up(&self, kc: VirtualKeyCode) -> bool {
        !self.now_keys[kc as usize]
    }
    pub fn is_key_pressed(&self, kc: VirtualKeyCode) -> bool {
        self.now_keys[kc as usize] && !self.prev_keys[kc as usize]
    }
    pub fn is_key_released(&self, kc: VirtualKeyCode) -> bool {
        !self.now_keys[kc as usize] && self.prev_keys[kc as usize]
    }
    pub(crate) fn next_frame(&mut self) {
        self.prev_keys.copy_from_slice(&self.now_keys);
        self.prev_mouse_down = self.mouse_down;
        self.prev_mouse_position = self.mouse_position;
    }
    pub(crate) fn handle_key_event(&mut self, ke: winit::event::KeyboardInput) {
        if let winit::event::KeyboardInput {
            virtual_keycode: Some(keycode),
            state,
            ..
        } = ke
        {
            match state {
                winit::event::ElementState::Pressed => {
                    self.now_keys[keycode as usize] = true;
                }
                winit::event::ElementState::Released => {
                    self.now_keys[keycode as usize] = false;
                }
            }
        }
    }
    pub(crate) fn handle_left_mouse_event(&mut self, ms: ElementState) {
        self.mouse_down = ms == ElementState::Pressed;
    }
    pub(crate) fn handle_cursor_moved_event(&mut self, cp: winit::dpi::PhysicalPosition<f64>) {
        self.mouse_position = cp;
    }

    pub fn get_mouse_position(&self) -> winit::dpi::PhysicalPosition<f64> {
        self.mouse_position
    }

    pub fn get_prev_mouse_position(&self) -> winit::dpi::PhysicalPosition<f64> {
        self.prev_mouse_position
    }
}

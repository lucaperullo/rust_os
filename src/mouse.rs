// src/mouse.rs
use crate::graphics::Color;

#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct Mouse {
    pub x: usize,
    pub y: usize,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub scroll_delta: i32,
    pub click_count: u32,
    pub last_click_time: u32,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            x: 320,
            y: 240,
            left_button: false,
            right_button: false,
            middle_button: false,
            scroll_delta: 0,
            click_count: 0,
            last_click_time: 0,
        }
    }
    
    pub fn button_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_button = true,
            MouseButton::Right => self.right_button = true,
            MouseButton::Middle => self.middle_button = true,
        }
    }
    
    pub fn button_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_button = false,
            MouseButton::Right => self.right_button = false,
            MouseButton::Middle => self.middle_button = false,
        }
    }
    
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    
    pub fn scroll(&mut self, delta: i32) {
        self.scroll_delta = delta;
    }
}
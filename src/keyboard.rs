// src/keyboard.rs
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Digit0, Digit1, Digit2, Digit3, Digit4,
    Digit5, Digit6, Digit7, Digit8, Digit9,
    Space, Enter, Backspace, Tab, Escape,
    LeftShift, RightShift, LeftCtrl, RightCtrl,
    LeftAlt, RightAlt, LeftCmd, RightCmd,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

pub struct Keyboard {
    pressed_keys: Vec<Key>,
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    cmd_pressed: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new(),
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            cmd_pressed: false,
        }
    }
    
    pub fn key_down(&mut self, key: Key) -> KeyEvent {
        if !self.pressed_keys.contains(&key) {
            self.pressed_keys.push(key);
        }
        
        match key {
            Key::LeftShift | Key::RightShift => self.shift_pressed = true,
            Key::LeftCtrl | Key::RightCtrl => self.ctrl_pressed = true,
            Key::LeftAlt | Key::RightAlt => self.alt_pressed = true,
            Key::LeftCmd | Key::RightCmd => self.cmd_pressed = true,
            _ => {}
        }
        
        KeyEvent {
            key,
            pressed: true,
            shift: self.shift_pressed,
            ctrl: self.ctrl_pressed,
            alt: self.alt_pressed,
            cmd: self.cmd_pressed,
        }
    }
    
    pub fn key_up(&mut self, key: Key) -> KeyEvent {
        self.pressed_keys.retain(|&k| k != key);
        
        match key {
            Key::LeftShift | Key::RightShift => self.shift_pressed = false,
            Key::LeftCtrl | Key::RightCtrl => self.ctrl_pressed = false,
            Key::LeftAlt | Key::RightAlt => self.alt_pressed = false,
            Key::LeftCmd | Key::RightCmd => self.cmd_pressed = false,
            _ => {}
        }
        
        KeyEvent {
            key,
            pressed: false,
            shift: self.shift_pressed,
            ctrl: self.ctrl_pressed,
            alt: self.alt_pressed,
            cmd: self.cmd_pressed,
        }
    }
    
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }
}
// src/animations.rs
use crate::graphics::{Graphics, Color};

#[derive(Clone, Copy)]
pub enum EasingType {
    EaseInOut,
    EaseIn,
    EaseOut,
    Linear,
}

pub struct Animation {
    pub start_value: f32,
    pub end_value: f32,
    pub duration: u32,
    pub current_time: u32,
    pub easing: EasingType,
    pub is_complete: bool,
}

impl Animation {
    pub fn new(start: f32, end: f32, duration: u32, easing: EasingType) -> Self {
        Self {
            start_value: start,
            end_value: end,
            duration,
            current_time: 0,
            easing,
            is_complete: false,
        }
    }
    
    pub fn update(&mut self) -> f32 {
        if self.is_complete {
            return self.end_value;
        }
        
        self.current_time += 1;
        
        if self.current_time >= self.duration {
            self.is_complete = true;
            return self.end_value;
        }
        
        let t = self.current_time as f32 / self.duration as f32;
        let progress = match self.easing {
            EasingType::Linear => t,
            EasingType::EaseIn => t * t,
            EasingType::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
        };
        
        self.start_value + (self.end_value - self.start_value) * progress
    }
}

pub struct WindowAnimation {
    pub x: Animation,
    pub y: Animation,
    pub width: Animation,
    pub height: Animation,
    pub alpha: Animation,
}

impl WindowAnimation {
    pub fn minimize_to_dock(start_x: f32, start_y: f32, start_w: f32, start_h: f32, dock_x: f32, dock_y: f32) -> Self {
        Self {
            x: Animation::new(start_x, dock_x, 30, EasingType::EaseInOut),
            y: Animation::new(start_y, dock_y, 30, EasingType::EaseInOut),
            width: Animation::new(start_w, 64.0, 30, EasingType::EaseInOut),
            height: Animation::new(start_h, 64.0, 30, EasingType::EaseInOut),
            alpha: Animation::new(1.0, 0.8, 30, EasingType::EaseOut),
        }
    }
    
    pub fn spring_open(start_x: f32, start_y: f32, end_w: f32, end_h: f32) -> Self {
        Self {
            x: Animation::new(start_x, start_x, 20, EasingType::EaseOut),
            y: Animation::new(start_y, start_y, 20, EasingType::EaseOut),
            width: Animation::new(0.0, end_w, 20, EasingType::EaseOut),
            height: Animation::new(0.0, end_h, 20, EasingType::EaseOut),
            alpha: Animation::new(0.0, 1.0, 20, EasingType::EaseOut),
        }
    }
    
    pub fn update(&mut self) -> (f32, f32, f32, f32, f32) {
        (
            self.x.update(),
            self.y.update(),
            self.width.update(),
            self.height.update(),
            self.alpha.update()
        )
    }
    
    pub fn is_complete(&self) -> bool {
        self.x.is_complete && self.y.is_complete && 
        self.width.is_complete && self.height.is_complete
    }
}
// src/notifications.rs
use crate::graphics::{Graphics, Color};
use crate::animations::{Animation, EasingType};
use alloc::string::String;
use alloc::vec::Vec;

pub struct Notification {
    pub title: String,
    pub message: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub animation: Animation,
    pub lifetime: u32,
    pub age: u32,
}

impl Notification {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            x: 640.0,
            y: 50.0,
            width: 300.0,
            height: 80.0,
            animation: Animation::new(640.0, 320.0, 30, EasingType::EaseOut),
            lifetime: 300,
            age: 0,
        }
    }
    
    pub fn update(&mut self) {
        self.x = self.animation.update();
        self.age += 1;
    }
    
    pub fn draw(&self, graphics: &mut Graphics) {
        let x = self.x as usize;
        let y = self.y as usize;
        let w = self.width as usize;
        let h = self.height as usize;
        
        graphics.draw_rounded_rect(x, y, w, h, Color::new(248, 248, 248));
        graphics.draw_rect_outline(x, y, w, h, Color::new(200, 200, 200));
        
        graphics.draw_text(&self.title, x + 15, y + 15, Color::BLACK);
        graphics.draw_text(&self.message, x + 15, y + 35, Color::DARK_GRAY);
        
        graphics.draw_rounded_rect(x + w - 50, y + 15, 30, 30, Color::BLUE);
    }
    
    pub fn is_expired(&self) -> bool {
        self.age > self.lifetime
    }
}

pub struct NotificationCenter {
    notifications: Vec<Notification>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    
    pub fn show_notification(&mut self, title: String, message: String) {
        let mut notification = Notification::new(title, message);
        
        let stack_offset = self.notifications.len() as f32 * 90.0;
        notification.y += stack_offset;
        notification.animation = Animation::new(640.0, 320.0, 30, EasingType::EaseOut);
        
        self.notifications.push(notification);
    }
    
    pub fn update(&mut self) {
        for notification in &mut self.notifications {
            notification.update();
        }
        
        self.notifications.retain(|n| !n.is_expired());
    }
    
    pub fn draw(&self, graphics: &mut Graphics) {
        for notification in &self.notifications {
            notification.draw(graphics);
        }
    }
}
// src/mission_control.rs
use crate::graphics::{Graphics, Color};
use crate::window_manager::{WindowManager};
use alloc::vec::Vec;

pub struct MissionControl {
    pub is_visible: bool,
    pub animation_progress: f32,
    pub desktop_spaces: Vec<DesktopSpace>,
    pub current_space: usize,
}

pub struct DesktopSpace {
    pub id: usize,
    pub windows: Vec<usize>,
    pub wallpaper_color: Color,
}

impl MissionControl {
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        spaces.push(DesktopSpace {
            id: 0,
            windows: Vec::new(),
            wallpaper_color: Color::new(30, 130, 180),
        });
        spaces.push(DesktopSpace {
            id: 1,
            windows: Vec::new(),
            wallpaper_color: Color::new(180, 30, 130),
        });
        
        Self {
            is_visible: false,
            animation_progress: 0.0,
            desktop_spaces: spaces,
            current_space: 0,
        }
    }
    
    pub fn show(&mut self) {
        self.is_visible = true;
        self.animation_progress = 0.0;
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
    
    pub fn update(&mut self) {
        if self.is_visible && self.animation_progress < 1.0 {
            self.animation_progress += 0.05;
            if self.animation_progress > 1.0 {
                self.animation_progress = 1.0;
            }
        }
    }
    
    pub fn draw(&self, graphics: &mut Graphics, _window_manager: &WindowManager) {
        if !self.is_visible {
            return;
        }
        
        graphics.draw_rect(0, 0, 640, 480, Color::new(20, 20, 20));
        
        let space_width = 200;
        let space_height = 150;
        let space_spacing = 220;
        let start_x = (640 - (self.desktop_spaces.len() * space_spacing - 20)) / 2;
        let start_y = 100;
        
        for (i, space) in self.desktop_spaces.iter().enumerate() {
            let x = start_x + i * space_spacing;
            let y = start_y;
            
            let border_color = if i == self.current_space { Color::BLUE } else { Color::GRAY };
            graphics.draw_rect_outline(x - 2, y - 2, space_width + 4, space_height + 4, border_color);
            graphics.draw_rect(x, y, space_width, space_height, space.wallpaper_color);
            
            graphics.draw_rect(x + 20, y + 20, 60, 40, Color::WHITE);
            graphics.draw_rect(x + 90, y + 30, 80, 50, Color::BLACK);
            
            let label = if i == self.current_space { "Current Desktop" } else { "Desktop" };
            graphics.draw_text(label, x + 60, y + space_height + 10, Color::WHITE);
        }
        
        graphics.draw_text("Use arrow keys to switch spaces, ESC to exit", 200, 400, Color::LIGHT_GRAY);
    }
    
    pub fn switch_space(&mut self, direction: i32) {
        if direction > 0 && self.current_space < self.desktop_spaces.len() - 1 {
            self.current_space += 1;
        } else if direction < 0 && self.current_space > 0 {
            self.current_space -= 1;
        }
    }
}
// src/spotlight.rs
use crate::graphics::{Graphics, Color};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpotlightResult {
    pub title: String,
    pub subtitle: String,
    pub icon: char,
}

pub struct Spotlight {
    pub is_visible: bool,
    pub search_query: String,
    pub results: Vec<SpotlightResult>,
    pub selected_index: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Spotlight {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            search_query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            x: 120,
            y: 100,
            width: 400,
            height: 300,
        }
    }
    
    pub fn show(&mut self) {
        self.is_visible = true;
        self.search_query.clear();
        self.update_results();
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
    
    pub fn add_character(&mut self, ch: char) {
        self.search_query.push(ch);
        self.update_results();
    }
    
    pub fn backspace(&mut self) {
        self.search_query.pop();
        self.update_results();
    }
    
    fn update_results(&mut self) {
        self.results.clear();
        
        if self.search_query.is_empty() {
            self.results.push(SpotlightResult {
                title: "Terminal".to_string(),
                subtitle: "Utilities".to_string(),
                icon: '💻',
            });
            self.results.push(SpotlightResult {
                title: "Finder".to_string(),
                subtitle: "System".to_string(),
                icon: '📁',
            });
            self.results.push(SpotlightResult {
                title: "System Preferences".to_string(),
                subtitle: "System".to_string(),
                icon: '⚙️',
            });
        } else {
            let query_lower = self.search_query.to_lowercase();
            
            if "terminal".starts_with(&query_lower) {
                self.results.push(SpotlightResult {
                    title: "Terminal".to_string(),
                    subtitle: "Utilities".to_string(),
                    icon: '💻',
                });
            }
            
            if "finder".starts_with(&query_lower) {
                self.results.push(SpotlightResult {
                    title: "Finder".to_string(),
                    subtitle: "System".to_string(),
                    icon: '📁',
                });
            }
            
            if "system".starts_with(&query_lower) || "preferences".starts_with(&query_lower) {
                self.results.push(SpotlightResult {
                    title: "System Preferences".to_string(),
                    subtitle: "System".to_string(),
                    icon: '⚙️',
                });
            }
        }
        
        self.selected_index = 0;
    }
    
    pub fn move_selection(&mut self, direction: i32) {
        if self.results.is_empty() {
            return;
        }
        
        if direction > 0 {
            self.selected_index = (self.selected_index + 1) % self.results.len();
        } else if direction < 0 {
            self.selected_index = if self.selected_index == 0 {
                self.results.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
    
    pub fn draw(&self, graphics: &mut Graphics) {
        if !self.is_visible {
            return;
        }
        
        graphics.draw_rect(0, 0, 640, 480, Color::new(0, 0, 0));
        
        graphics.draw_rounded_rect(self.x, self.y, self.width, self.height, Color::new(245, 245, 245));
        graphics.draw_rect_outline(self.x, self.y, self.width, self.height, Color::new(200, 200, 200));
        
        graphics.draw_rounded_rect(self.x + 20, self.y + 20, self.width - 40, 40, Color::WHITE);
        graphics.draw_rect_outline(self.x + 20, self.y + 20, self.width - 40, 40, Color::new(180, 180, 180));
        
        graphics.draw_text("🔍", self.x + 30, self.y + 35, Color::GRAY);
        
        graphics.draw_text(&self.search_query, self.x + 60, self.y + 35, Color::BLACK);
        
        let cursor_x = self.x + 60 + self.search_query.len() * 8;
        graphics.draw_rect(cursor_x, self.y + 32, 2, 16, Color::BLUE);
        
        let result_start_y = self.y + 80;
        for (i, result) in self.results.iter().enumerate() {
            let result_y = result_start_y + i * 50;
            
            if i == self.selected_index {
                graphics.draw_rounded_rect(self.x + 10, result_y - 5, self.width - 20, 40, Color::BLUE);
            }
            
            graphics.draw_text(&result.icon.to_string(), self.x + 25, result_y + 10, Color::BLACK);
            
            let text_color = if i == self.selected_index { Color::WHITE } else { Color::BLACK };
            let subtitle_color = if i == self.selected_index { Color::new(200, 200, 200) } else { Color::GRAY };
            
            graphics.draw_text(&result.title, self.x + 60, result_y + 5, text_color);
            graphics.draw_text(&result.subtitle, self.x + 60, result_y + 20, subtitle_color);
        }
    }
}
// src/graphics.rs
use volatile::Volatile;

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 480;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const GRAY: Color = Color::new(128, 128, 128);
    pub const LIGHT_GRAY: Color = Color::new(240, 240, 245);
    pub const DARK_GRAY: Color = Color::new(60, 60, 60);
    pub const BLUE: Color = Color::new(0, 122, 255);
    pub const RED: Color = Color::new(255, 59, 48);
    pub const GREEN: Color = Color::new(52, 199, 89);
    pub const YELLOW: Color = Color::new(255, 204, 0);
    pub const TRANSPARENT: Color = Color::new(0, 0, 1);
}

pub struct Graphics {
    framebuffer: &'static mut [Volatile<u8>],
}

impl Graphics {
    pub fn new() -> Self {
        let framebuffer = unsafe {
            core::slice::from_raw_parts_mut(
                0xA0000 as *mut Volatile<u8>,
                SCREEN_WIDTH * SCREEN_HEIGHT,
            )
        };
        
        Self { framebuffer }
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            let offset = y * SCREEN_WIDTH + x;
            let vga_color = self.rgb_to_vga(color);
            self.framebuffer[offset].write(vga_color);
        }
    }
    
    pub fn clear_screen(&mut self, color: Color) {
        let vga_color = self.rgb_to_vga(color);
        for pixel in self.framebuffer.iter_mut() {
            pixel.write(vga_color);
        }
    }
    
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    pub fn draw_rect_outline(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for dx in 0..width {
            self.set_pixel(x + dx, y, color);
            self.set_pixel(x + dx, y + height - 1, color);
        }
        for dy in 0..height {
            self.set_pixel(x, y + dy, color);
            self.set_pixel(x + width - 1, y + dy, color);
        }
    }
    
    pub fn draw_rounded_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        self.draw_rect(x, y, width, height, color);
        
        if width > 4 && height > 4 {
            let bg = Color::LIGHT_GRAY;
            self.set_pixel(x, y, bg);
            self.set_pixel(x + 1, y, bg);
            self.set_pixel(x, y + 1, bg);
            
            self.set_pixel(x + width - 1, y, bg);
            self.set_pixel(x + width - 2, y, bg);
            self.set_pixel(x + width - 1, y + 1, bg);
            
            self.set_pixel(x, y + height - 1, bg);
            self.set_pixel(x + 1, y + height - 1, bg);
            self.set_pixel(x, y + height - 2, bg);
            
            self.set_pixel(x + width - 1, y + height - 1, bg);
            self.set_pixel(x + width - 2, y + height - 1, bg);
            self.set_pixel(x + width - 1, y + height - 2, bg);
        }
    }
    
    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, color: Color) {
        let mut dx = 0;
        for ch in text.chars() {
            self.draw_char(ch, x + dx, y, color);
            dx += 8;
        }
    }
    
    fn draw_char(&mut self, ch: char, x: usize, y: usize, color: Color) {
        let font_data = self.get_font_data(ch);
        for (row, &byte) in font_data.iter().enumerate() {
            for col in 0..8 {
                if (byte >> (7 - col)) & 1 == 1 {
                    self.set_pixel(x + col, y + row, color);
                }
            }
        }
    }
    
    fn get_font_data(&self, ch: char) -> &[u8] {
        match ch {
            'A' => &[0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
            'B' => &[0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
            'C' => &[0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
            'D' => &[0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
            'E' => &[0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x7E, 0x00],
            'F' => &[0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x00],
            'G' => &[0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
            'H' => &[0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
            'I' => &[0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
            'O' => &[0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
            'R' => &[0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00],
            'S' => &[0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
            'T' => &[0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
            'U' => &[0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
            ' ' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            _ => &[0xFF, 0x81, 0x81, 0x81, 0x81, 0x81, 0xFF, 0x00],
        }
    }
    
    fn rgb_to_vga(&self, color: Color) -> u8 {
        match (color.r, color.g, color.b) {
            (255, 255, 255) => 15,
            (0, 0, 0) => 0,
            (128, 128, 128) => 8,
            (240, 240, 245) => 7,
            (60, 60, 60) => 8,
            (0, 122, 255) => 9,
            (255, 59, 48) => 12,
            (52, 199, 89) => 10,
            (255, 204, 0) => 14,
            _ => 7,
        }
    }
}
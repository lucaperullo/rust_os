// src/desktop.rs
use crate::graphics::{Graphics, Color, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::window_manager::{WindowManager, Window};
use crate::notifications::NotificationCenter;
use crate::spotlight::Spotlight;
use crate::mission_control::MissionControl;
use alloc::string::String;

pub struct Desktop {
    window_manager: WindowManager,
    notification_center: NotificationCenter,
    spotlight: Spotlight,
    mission_control: MissionControl,
    wallpaper_color: Color,
    menu_bar_height: usize,
    dock_height: usize,
    dock_y: usize,
    time_counter: u32,
    mouse_x: usize,
    mouse_y: usize,
    show_about_dialog: bool,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
            notification_center: NotificationCenter::new(),
            spotlight: Spotlight::new(),
            mission_control: MissionControl::new(),
            wallpaper_color: Color::new(30, 130, 180),
            menu_bar_height: 24,
            dock_height: 60,
            dock_y: SCREEN_HEIGHT - 60,
            time_counter: 0,
            mouse_x: 320,
            mouse_y: 240,
            show_about_dialog: false,
        }
    }
    
    pub fn init(&mut self, graphics: &mut Graphics) {
        // Create sample windows
        self.create_sample_windows();
        
        // Show welcome notification
        self.notification_center.show_notification(
            "Welcome to RustOS".to_string(),
            "macOS-inspired operating system".to_string()
        );
        
        // Show system ready notification after a delay
        self.notification_center.show_notification(
            "System Ready".to_string(),
            "All services loaded successfully".to_string()
        );
    }
    
    pub fn draw(&mut self, graphics: &mut Graphics) {
        // Draw wallpaper with subtle gradient effect
        self.draw_wallpaper(graphics);
        
        // Draw Mission Control if visible
        if self.mission_control.is_visible {
            self.mission_control.draw(graphics, &self.window_manager);
            return; // Don't draw desktop when Mission Control is active
        }
        
        // Draw windows
        self.window_manager.draw_all(graphics);
        
        // Draw menu bar
        self.draw_menu_bar(graphics);
        
        // Draw dock with reflection effect
        self.draw_dock(graphics);
        
        // Draw Spotlight if visible
        self.spotlight.draw(graphics);
        
        // Draw notifications
        self.notification_center.draw(graphics);
        
        // Draw about dialog if visible
        if self.show_about_dialog {
            self.draw_about_dialog(graphics);
        }
        
        // Draw cursor
        self.draw_cursor(graphics, self.mouse_x, self.mouse_y);
    }
    
    pub fn update(&mut self, graphics: &mut Graphics) {
        self.time_counter += 1;
        
        // Update animations
        self.mission_control.update();
        self.notification_center.update();
        
        // Simulate some dynamic notifications
        if self.time_counter == 300 { // After 5 seconds
            self.notification_center.show_notification(
                "Memory Update".to_string(),
                "Available: 847MB of 1024MB".to_string()
            );
        }
        
        if self.time_counter == 600 { // After 10 seconds
            self.notification_center.show_notification(
                "Network Status".to_string(),
                "Connected to RustOS Network".to_string()
            );
        }
        
        // Simulate mouse movement
        self.mouse_x = 320 + ((self.time_counter as f32 * 0.1).sin() * 50.0) as usize;
        self.mouse_y = 240 + ((self.time_counter as f32 * 0.08).cos() * 30.0) as usize;
        
        if self.window_manager.needs_redraw() {
            self.draw(graphics);
        }
    }
    
    pub fn handle_events(&mut self) {
        // Simulate keyboard events
        if self.time_counter == 180 { // Show Spotlight after 3 seconds
            self.spotlight.show();
            self.spotlight.add_character('t');
            self.spotlight.add_character('e');
            self.spotlight.add_character('r');
        }
        
        if self.time_counter == 240 { // Hide Spotlight
            self.spotlight.hide();
        }
        
        if self.time_counter == 420 { // Show Mission Control
            self.mission_control.show();
        }
        
        if self.time_counter == 480 { // Hide Mission Control
            self.mission_control.hide();
        }
        
        if self.time_counter == 540 { // Show About dialog
            self.show_about_dialog = true;
        }
        
        if self.time_counter == 660 { // Hide About dialog
            self.show_about_dialog = false;
        }
    }
    
    fn draw_wallpaper(&self, graphics: &mut Graphics) {
        // Create a gradient effect from top to bottom
        for y in 0..SCREEN_HEIGHT {
            let intensity = 1.0 - (y as f32 / SCREEN_HEIGHT as f32) * 0.3;
            let r = (self.wallpaper_color.r as f32 * intensity) as u8;
            let g = (self.wallpaper_color.g as f32 * intensity) as u8;
            let b = (self.wallpaper_color.b as f32 * intensity) as u8;
            
            graphics.draw_rect(0, y, SCREEN_WIDTH, 1, Color::new(r, g, b));
        }
        
        // Add some decorative elements
        self.draw_floating_particles(graphics);
    }
    
    fn draw_floating_particles(&self, graphics: &mut Graphics) {
        // Draw some floating geometric shapes for visual interest
        let particles = [
            (100, 150, 20),
            (500, 200, 15),
            (300, 350, 25),
            (150, 400, 18),
            (450, 100, 22),
        ];
        
        for &(x, y, size) in &particles {
            let offset_x = ((self.time_counter as f32 * 0.02 + x as f32 * 0.01).sin() * 10.0) as i32;
            let offset_y = ((self.time_counter as f32 * 0.015 + y as f32 * 0.008).cos() * 8.0) as i32;
            
            let final_x = (x as i32 + offset_x) as usize;
            let final_y = (y as i32 + offset_y) as usize;
            
            // Draw translucent circles
            graphics.draw_rounded_rect(
                final_x, 
                final_y, 
                size, 
                size, 
                Color::new(255, 255, 255) // Semi-transparent white
            );
        }
    }
    
    fn draw_menu_bar(&self, graphics: &mut Graphics) {
        // Draw menu bar background with transparency
        graphics.draw_rect(0, 0, SCREEN_WIDTH, self.menu_bar_height, Color::new(248, 248, 248));
        
        // Draw subtle shadow
        graphics.draw_rect(0, self.menu_bar_height - 1, SCREEN_WIDTH, 1, Color::new(220, 220, 220));
        
        // Draw Apple logo
        graphics.draw_text("🍎", 10, 8, Color::BLACK);
        
        // Draw application name
        graphics.draw_text("RustOS", 40, 8, Color::BLACK);
        
        // Draw menu items
        let menus = ["File", "Edit", "View", "Window", "Help"];
        let mut x = 100;
        for menu in &menus {
            graphics.draw_text(menu, x, 8, Color::BLACK);
            x += menu.len() * 8 + 20;
        }
        
        // Draw right side status items
        let time_str = self.format_time();
        graphics.draw_text(&time_str, SCREEN_WIDTH - 80, 8, Color::BLACK);
        
        // System status icons
        graphics.draw_text("🔋", SCREEN_WIDTH - 120, 8, Color::GREEN);
        graphics.draw_text("📶", SCREEN_WIDTH - 140, 8, Color::BLACK);
        graphics.draw_text("🔍", SCREEN_WIDTH - 160, 8, Color::BLACK);
    }
    
    fn draw_dock(&self, graphics: &mut Graphics) {
        let dock_width = 480;
        let dock_x = (SCREEN_WIDTH - dock_width) / 2;
        let margin = 10;
        
        // Draw dock reflection/shadow first
        graphics.draw_rect(
            dock_x + 2, 
            self.dock_y + 2, 
            dock_width, 
            self.dock_height + 10, 
            Color::new(0, 0, 0)
        );
        
        // Draw dock background with glass effect
        graphics.draw_rounded_rect(
            dock_x, 
            self.dock_y, 
            dock_width, 
            self.dock_height, 
            Color::new(245, 245, 245)
        );
        
        // Draw dock separator line
        graphics.draw_rect(dock_x + 350, self.dock_y + 10, 2, self.dock_height - 20, Color::GRAY);
        
        // Draw application icons
        let apps = [
            ("📁", "Finder"),
            ("🌐", "Safari"),
            ("📧", "Mail"),
            ("📅", "Calendar"),
            ("🎵", "Music"),
            ("📸", "Photos"),
            ("⚙️", "Preferences"),
        ];
        
        let icon_size = 48;
        let icon_spacing = 58;
        let start_x = dock_x + 20;
        let icon_y = self.dock_y + 6;
        
        for (i, &(icon, name)) in apps.iter().enumerate() {
            let x = start_x + i * icon_spacing;
            
            // Add hover effect (simulate mouse over first icon)
            let size = if i == 0 && self.time_counter % 120 < 60 { 
                icon_size + 8 
            } else { 
                icon_size 
            };
            let// src/main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::vec::Vec;

mod vga_buffer;
mod graphics;
mod desktop;
mod window_manager;
mod mouse;
mod keyboard;
mod allocator;
mod animations;
mod notifications;
mod spotlight;
mod mission_control;

use desktop::Desktop;
use graphics::{Color, Graphics};

static mut DESKTOP: Option<Desktop> = None;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize graphics mode
    let mut graphics = Graphics::new();
    graphics.clear_screen(Color::new(240, 240, 245)); // macOS-like light gray background
    
    // Initialize desktop environment
    unsafe {
        DESKTOP = Some(Desktop::new());
        if let Some(ref mut desktop) = DESKTOP {
            desktop.init(&mut graphics);
            desktop.draw(&mut graphics);
            
            // Main event loop
            loop {
                desktop.handle_events();
                desktop.update(&mut graphics);
                
                // Small delay to prevent 100% CPU usage
                for _ in 0..100000 {
                    core::hint::spin_loop();
                }
            }
        }
    }
    
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

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
    pub const TRANSPARENT: Color = Color::new(0, 0, 1); // Special transparent color
}

pub struct Graphics {
    framebuffer: &'static mut [Volatile<u8>],
}

impl Graphics {
    pub fn new() -> Self {
        // VGA mode 13h framebuffer at 0xA0000
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
            // Convert RGB to VGA palette index (simplified)
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
        // Top and bottom lines
        for dx in 0..width {
            self.set_pixel(x + dx, y, color);
            self.set_pixel(x + dx, y + height - 1, color);
        }
        // Left and right lines
        for dy in 0..height {
            self.set_pixel(x, y + dy, color);
            self.set_pixel(x + width - 1, y + dy, color);
        }
    }
    
    pub fn draw_rounded_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        // Simple rounded rectangle (just draw normal rect for now, can be enhanced)
        self.draw_rect(x, y, width, height, color);
        
        // Add simple corner rounding by drawing smaller rects at corners
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
        // Simple 8x8 bitmap font rendering
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
        // Simple 8x8 font data for basic characters
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
            _ => &[0xFF, 0x81, 0x81, 0x81, 0x81, 0x81, 0xFF, 0x00], // Unknown char
        }
    }
    
    fn rgb_to_vga(&self, color: Color) -> u8 {
        // Convert RGB to closest VGA 256-color palette entry
        match (color.r, color.g, color.b) {
            (255, 255, 255) => 15, // White
            (0, 0, 0) => 0,        // Black
            (128, 128, 128) => 8,  // Gray
            (240, 240, 245) => 7,  // Light gray
            (60, 60, 60) => 8,     // Dark gray
            (0, 122, 255) => 9,    // Blue
            (255, 59, 48) => 12,   // Red
            (52, 199, 89) => 10,   // Green
            (255, 204, 0) => 14,   // Yellow
            _ => 7, // Default to light gray
        }
    }
}

// src/desktop.rs
use crate::graphics::{Graphics, Color, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::window_manager::{WindowManager, Window};

pub struct Desktop {
    window_manager: WindowManager,
    wallpaper_color: Color,
    menu_bar_height: usize,
    dock_height: usize,
    dock_y: usize,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
            wallpaper_color: Color::new(30, 130, 180), // macOS Big Sur blue
            menu_bar_height: 24,
            dock_height: 60,
            dock_y: SCREEN_HEIGHT - 60,
        }
    }
    
    pub fn init(&mut self, graphics: &mut Graphics) {
        // Create some sample windows
        self.create_sample_windows();
    }
    
    pub fn draw(&mut self, graphics: &mut Graphics) {
        // Draw wallpaper
        graphics.clear_screen(self.wallpaper_color);
        
        // Draw menu bar
        self.draw_menu_bar(graphics);
        
        // Draw dock
        self.draw_dock(graphics);
        
        // Draw windows
        self.window_manager.draw_all(graphics);
        
        // Draw cursor
        self.draw_cursor(graphics, 320, 240);
    }
    
    pub fn update(&mut self, graphics: &mut Graphics) {
        // Update window animations, etc.
        if self.window_manager.needs_redraw() {
            self.draw(graphics);
        }
    }
    
    pub fn handle_events(&mut self) {
        // Handle keyboard and mouse events
        // This would integrate with actual hardware in a real OS
    }
    
    fn draw_menu_bar(&self, graphics: &mut Graphics) {
        // Draw menu bar background
        graphics.draw_rect(0, 0, SCREEN_WIDTH, self.menu_bar_height, Color::new(248, 248, 248));
        
        // Draw menu bar shadow
        graphics.draw_rect(0, self.menu_bar_height - 1, SCREEN_WIDTH, 1, Color::new(200, 200, 200));
        
        // Draw Apple logo (simplified)
        graphics.draw_text("🍎", 10, 8, Color::BLACK);
        
        // Draw menu items
        graphics.draw_text("RUSTOS", 40, 8, Color::BLACK);
        graphics.draw_text("FILE", 100, 8, Color::BLACK);
        graphics.draw_text("EDIT", 140, 8, Color::BLACK);
        graphics.draw_text("VIEW", 180, 8, Color::BLACK);
        graphics.draw_text("HELP", 220, 8, Color::BLACK);
        
        // Draw right side items
        graphics.draw_text("12:34", SCREEN_WIDTH - 60, 8, Color::BLACK);
        graphics.draw_text("🔋", SCREEN_WIDTH - 100, 8, Color::BLACK);
        graphics.draw_text("📶", SCREEN_WIDTH - 120, 8, Color::BLACK);
    }
    
    fn draw_dock(&self, graphics: &mut Graphics) {
        let dock_width = 400;
        let dock_x = (SCREEN_WIDTH - dock_width) / 2;
        
        // Draw dock background with rounded corners
        graphics.draw_rounded_rect(
            dock_x, 
            self.dock_y, 
            dock_width, 
            self.dock_height, 
            Color::new(255, 255, 255)
        );
        
        // Draw dock shadow
        graphics.draw_rect(
            dock_x + 2, 
            self.dock_y + 2, 
            dock_width, 
            self.dock_height, 
            Color::new(0, 0, 0)
        );
        
        // Draw dock items
        let icon_size = 40;
        let icon_spacing = 50;
        let start_x = dock_x + 20;
        let icon_y = self.dock_y + 10;
        
        let apps = ["📁", "🌐", "📧", "⚙️", "🎵", "📝", "🗑️"];
        
        for (i, &app) in apps.iter().enumerate() {
            let x = start_x + i * icon_spacing;
            
            // Draw app icon background
            graphics.draw_rounded_rect(x, icon_y, icon_size, icon_size, Color::LIGHT_GRAY);
            
            // Draw app icon
            graphics.draw_text(app, x + 16, icon_y + 16, Color::BLACK);
        }
    }
    
    fn draw_cursor(&self, graphics: &mut Graphics, x: usize, y: usize) {
        // Draw macOS-style cursor
        let cursor_data = [
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 0, 0, 0, 0],
            [1, 2, 2, 1, 0, 0, 0, 0, 0, 0],
            [1, 2, 2, 2, 1, 0, 0, 0, 0, 0],
            [1, 2, 2, 2, 2, 1, 0, 0, 0, 0],
            [1, 2, 2, 2, 2, 2, 1, 0, 0, 0],
            [1, 2, 2, 2, 2, 2, 2, 1, 0, 0],
            [1, 2, 2, 2, 2, 2, 2, 2, 1, 0],
            [1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        ];
        
        for (dy, row) in cursor_data.iter().enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                let color = match pixel {
                    0 => continue, // Transparent
                    1 => Color::BLACK,
                    2 => Color::WHITE,
                    _ => Color::BLACK,
                };
                graphics.set_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    fn create_sample_windows(&mut self) {
        // Create a Finder-style window
        let finder = Window::new(
            "Finder".to_string(),
            50, 50, 300, 200,
            Color::WHITE
        );
        self.window_manager.add_window(finder);
        
        // Create a Terminal window
        let terminal = Window::new(
            "Terminal".to_string(),
            100, 100, 400, 250,
            Color::BLACK
        );
        self.window_manager.add_window(terminal);
        
        // Create a Settings window
        let settings = Window::new(
            "System Preferences".to_string(),
            200, 150, 350, 300,
            Color::LIGHT_GRAY
        );
        self.window_manager.add_window(settings);
    }
}

// src/window_manager.rs
use crate::graphics::{Graphics, Color};
use alloc::vec::Vec;
use alloc::string::String;

extern crate alloc;

pub struct Window {
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub background_color: Color,
    pub is_focused: bool,
    pub is_minimized: bool,
}

impl Window {
    pub fn new(title: String, x: usize, y: usize, width: usize, height: usize, background_color: Color) -> Self {
        Self {
            title,
            x,
            y,
            width,
            height,
            background_color,
            is_focused: false,
            is_minimized: false,
        }
    }
    
    pub fn draw(&self, graphics: &mut Graphics) {
        if self.is_minimized {
            return;
        }
        
        let title_bar_height = 30;
        
        // Draw window shadow
        graphics.draw_rect(
            self.x + 3,
            self.y + 3,
            self.width,
            self.height,
            Color::new(0, 0, 0)
        );
        
        // Draw window background
        graphics.draw_rect(self.x, self.y, self.width, self.height, self.background_color);
        
        // Draw title bar
        let title_bar_color = if self.is_focused {
            Color::new(235, 235, 235)
        } else {
            Color::new(245, 245, 245)
        };
        
        graphics.draw_rect(self.x, self.y, self.width, title_bar_height, title_bar_color);
        
        // Draw title bar buttons (red, yellow, green)
        let button_size = 12;
        let button_y = self.y + 9;
        
        // Close button (red)
        graphics.draw_rounded_rect(self.x + 8, button_y, button_size, button_size, Color::RED);
        
        // Minimize button (yellow)
        graphics.draw_rounded_rect(self.x + 28, button_y, button_size, button_size, Color::YELLOW);
        
        // Maximize button (green)
        graphics.draw_rounded_rect(self.x + 48, button_y, button_size, button_size, Color::GREEN);
        
        // Draw title text
        graphics.draw_text(&self.title, self.x + 70, self.y + 11, Color::BLACK);
        
        // Draw window border
        graphics.draw_rect_outline(self.x, self.y, self.width, self.height, Color::GRAY);
        
        // Draw content area
        self.draw_content(graphics);
    }
    
    fn draw_content(&self, graphics: &mut Graphics) {
        let content_y = self.y + 30;
        let content_height = self.height - 30;
        
        match self.title.as_str() {
            "Finder" => {
                // Draw finder sidebar
                graphics.draw_rect(self.x + 1, content_y, 80, content_height - 1, Color::new(245, 245, 245));
                graphics.draw_text("FAVORITES", self.x + 5, content_y + 10, Color::DARK_GRAY);
                graphics.draw_text("Desktop", self.x + 5, content_y + 30, Color::BLACK);
                graphics.draw_text("Documents", self.x + 5, content_y + 50, Color::BLACK);
                graphics.draw_text("Downloads", self.x + 5, content_y + 70, Color::BLACK);
                
                // Draw main content area
                graphics.draw_text("RustOS Files", self.x + 90, content_y + 20, Color::BLACK);
                graphics.draw_text("📁 System", self.x + 90, content_y + 50, Color::BLACK);
                graphics.draw_text("📁 Users", self.x + 90, content_y + 70, Color::BLACK);
                graphics.draw_text("📄 README.md", self.x + 90, content_y + 90, Color::BLACK);
            },
            "Terminal" => {
                graphics.draw_text("RustOS Terminal v1.0", self.x + 10, content_y + 20, Color::GREEN);
                graphics.draw_text("$ ls -la", self.x + 10, content_y + 40, Color::WHITE);
                graphics.draw_text("total 42", self.x + 10, content_y + 60, Color::WHITE);
                graphics.draw_text("drwxr-xr-x  kernel", self.x + 10, content_y + 80, Color::WHITE);
                graphics.draw_text("drwxr-xr-x  drivers", self.x + 10, content_y + 100, Color::WHITE);
                graphics.draw_text("$ _", self.x + 10, content_y + 120, Color::GREEN);
            },
            "System Preferences" => {
                graphics.draw_text("System Preferences", self.x + 10, content_y + 20, Color::BLACK);
                
                // Draw preference icons
                graphics.draw_rounded_rect(self.x + 20, content_y + 50, 60, 60, Color::BLUE);
                graphics.draw_text("General", self.x + 30, content_y + 75, Color::WHITE);
                
                graphics.draw_rounded_rect(self.x + 100, content_y + 50, 60, 60, Color::GRAY);
                graphics.draw_text("Display", self.x + 110, content_y + 75, Color::WHITE);
                
                graphics.draw_rounded_rect(self.x + 180, content_y + 50, 60, 60, Color::GREEN);
                graphics.draw_text("Sound", self.x + 195, content_y + 75, Color::WHITE);
            },
            _ => {
                graphics.draw_text("Welcome to RustOS!", self.x + 10, content_y + 20, Color::BLACK);
            }
        }
    }
}

pub struct WindowManager {
    windows: Vec<Window>,
    focused_window: Option<usize>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused_window: None,
        }
    }
    
    pub fn add_window(&mut self, mut window: Window) {
        window.is_focused = self.windows.is_empty();
        self.windows.push(window);
        if self.focused_window.is_none() {
            self.focused_window = Some(0);
        }
    }
    
    pub fn draw_all(&mut self, graphics: &mut Graphics) {
        // Draw unfocused windows first
        for (i, window) in self.windows.iter().enumerate() {
            if Some(i) != self.focused_window {
                window.draw(graphics);
            }
        }
        
        // Draw focused window last (on top)
        if let Some(focused_idx) = self.focused_window {
            if let Some(window) = self.windows.get(focused_idx) {
                window.draw(graphics);
            }
        }
    }
    
    pub fn needs_redraw(&self) -> bool {
        // For now, always redraw. In a real OS, this would be optimized.
        false
    }
}

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
            x: 640.0,  // Start off-screen
            y: 50.0,
            width: 300.0,
            height: 80.0,
            animation: Animation::new(640.0, 320.0, 30, EasingType::EaseOut), // Slide in
            lifetime: 300, // Show for 5 seconds at 60fps
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
        
        // Draw notification background with transparency effect
        graphics.draw_rounded_rect(x, y, w, h, Color::new(248, 248, 248));
        graphics.draw_rect_outline(x, y, w, h, Color::new(200, 200, 200));
        
        // Draw content
        graphics.draw_text(&self.title, x + 15, y + 15, Color::BLACK);
        graphics.draw_text(&self.message, x + 15, y + 35, Color::DARK_GRAY);
        
        // Draw app icon placeholder
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
        
        // Stack notifications vertically
        let stack_offset = self.notifications.len() as f32 * 90.0;
        notification.y += stack_offset;
        notification.animation = Animation::new(640.0, 320.0, 30, EasingType::EaseOut);
        
        self.notifications.push(notification);
    }
    
    pub fn update(&mut self) {
        for notification in &mut self.notifications {
            notification.update();
        }
        
        // Remove expired notifications
        self.notifications.retain(|n| !n.is_expired());
    }
    
    pub fn draw(&self, graphics: &mut Graphics) {
        for notification in &self.notifications {
            notification.draw(graphics);
        }
    }
}

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
        
        // Simulate search results based on query
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
            // Filter results based on search query
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
        
        // Draw backdrop blur effect (simplified)
        graphics.draw_rect(0, 0, 640, 480, Color::new(0, 0, 0)); // Semi-transparent overlay
        
        // Draw main spotlight window
        graphics.draw_rounded_rect(self.x, self.y, self.width, self.height, Color::new(245, 245, 245));
        graphics.draw_rect_outline(self.x, self.y, self.width, self.height, Color::new(200, 200, 200));
        
        // Draw search bar
        graphics.draw_rounded_rect(self.x + 20, self.y + 20, self.width - 40, 40, Color::WHITE);
        graphics.draw_rect_outline(self.x + 20, self.y + 20, self.width - 40, 40, Color::new(180, 180, 180));
        
        // Draw search icon
        graphics.draw_text("🔍", self.x + 30, self.y + 35, Color::GRAY);
        
        // Draw search query
        graphics.draw_text(&self.search_query, self.x + 60, self.y + 35, Color::BLACK);
        
        // Draw cursor
        let cursor_x = self.x + 60 + self.search_query.len() * 8;
        graphics.draw_rect(cursor_x, self.y + 32, 2, 16, Color::BLUE);
        
        // Draw results
        let result_start_y = self.y + 80;
        for (i, result) in self.results.iter().enumerate() {
            let result_y = result_start_y + i * 50;
            
            // Highlight selected result
            if i == self.selected_index {
                graphics.draw_rounded_rect(self.x + 10, result_y - 5, self.width - 20, 40, Color::BLUE);
            }
            
            // Draw icon
            graphics.draw_text(&result.icon.to_string(), self.x + 25, result_y + 10, Color::BLACK);
            
            // Draw title and subtitle
            let text_color = if i == self.selected_index { Color::WHITE } else { Color::BLACK };
            let subtitle_color = if i == self.selected_index { Color::new(200, 200, 200) } else { Color::GRAY };
            
            graphics.draw_text(&result.title, self.x + 60, result_y + 5, text_color);
            graphics.draw_text(&result.subtitle, self.x + 60, result_y + 20, subtitle_color);
        }
    }
}

// src/mission_control.rs
use crate::graphics::{Graphics, Color};
use crate::window_manager::{Window, WindowManager};
use alloc::vec::Vec;

pub struct MissionControl {
    pub is_visible: bool,
    pub animation_progress: f32,
    pub desktop_spaces: Vec<DesktopSpace>,
    pub current_space: usize,
}

pub struct DesktopSpace {
    pub id: usize,
    pub windows: Vec<usize>, // Window IDs
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
            self.animation_progress += 0.05; // Smooth animation
            if self.animation_progress > 1.0 {
                self.animation_progress = 1.0;
            }
        }
    }
    
    pub fn draw(&self, graphics: &mut Graphics, window_manager: &WindowManager) {
        if !self.is_visible {
            return;
        }
        
        // Draw dark overlay
        graphics.draw_rect(0, 0, 640, 480, Color::new(20, 20, 20));
        
        // Draw desktop spaces as thumbnails
        let space_width = 200;
        let space_height = 150;
        let space_spacing = 220;
        let start_x = (640 - (self.desktop_spaces.len() * space_spacing - 20)) / 2;
        let start_y = 100;
        
        for (i, space) in self.desktop_spaces.iter().enumerate() {
            let x = start_x + i * space_spacing;
            let y = start_y;
            
            // Draw space background
            let border_color = if i == self.current_space { Color::BLUE } else { Color::GRAY };
            graphics.draw_rect_outline(x - 2, y - 2, space_width + 4, space_height + 4, border_color);
            graphics.draw_rect(x, y, space_width, space_height, space.wallpaper_color);
            
            // Draw miniature windows in this space
            // This would show actual window thumbnails in a real implementation
            graphics.draw_rect(x + 20, y + 20, 60, 40, Color::WHITE);
            graphics.draw_rect(x + 90, y + 30, 80, 50, Color::BLACK);
            
            // Draw space label
            let label = if i == self.current_space { "Current Desktop" } else { "Desktop" };
            graphics.draw_text(label, x + 60, y + space_height + 10, Color::WHITE);
        }
        
        // Draw instructions
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

// Update Cargo.toml
[package]
name = "rust_os"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = "0.9.23"
volatile = "0.2.6"
spin = "0.5.2"
x86_64 = "0.14.2"
uart_16550 = "0.2.0"
pic8259 = "0.10.1"
pc-keyboard = "0.5.0"
linked_list_allocator = "0.9.0"

[dependencies.lazy_static]
version = "1.0"
features = ["spin_no_std"]

[[bin]]
name = "rust_os"
test = false
bench = false

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

# Add heap allocator support
# src/allocator.rs
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

# Configuration files remain the same as before...
# .cargo/config.toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins", "alloc"]

[build]
target = "x86_64-rust_os.json"

[target.'cfg(target_os = "none")']
runner = "bootimage runner"

# Makefile and other build files remain the same...
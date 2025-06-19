// src/main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod graphics;
mod desktop;
mod window_manager;
mod mouse;
mod keyboard;

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

// src/mouse.rs
pub struct Mouse {
    pub x: usize,
    pub y: usize,
    pub left_button: bool,
    pub right_button: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            x: 320,
            y: 240,
            left_button: false,
            right_button: false,
        }
    }
}

// src/keyboard.rs
pub struct Keyboard {
    // Keyboard state would go here
}

impl Keyboard {
    pub fn new() -> Self {
        Self {}
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
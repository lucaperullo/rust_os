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
        
        for (i, &(icon, _name)) in apps.iter().enumerate() {
            let x = start_x + i * icon_spacing;
            
            // Add hover effect (simulate mouse over first icon)
            let size = if i == 0 && self.time_counter % 120 < 60 { 
                icon_size + 8 
            } else { 
                icon_size 
            };
            let y_offset = if i == 0 && self.time_counter % 120 < 60 { -4 } else { 0 };
            
            // Draw app icon background with subtle reflection
            graphics.draw_rounded_rect(
                x, 
                icon_y + y_offset as usize, 
                size, 
                size, 
                Color::new(240, 240, 240)
            );
            
            // Draw app icon
            graphics.draw_text(icon, x + size/4, icon_y + size/4 + y_offset as usize, Color::BLACK);
            
            // Draw running indicator (dot under icon)
            if i < 3 { // First 3 apps are "running"
                graphics.draw_rounded_rect(
                    x + size/2 - 2, 
                    self.dock_y + self.dock_height - 8, 
                    4, 
                    4, 
                    Color::BLACK
                );
            }
        }
        
        // Draw trash icon
        let trash_x = dock_x + 370;
        graphics.draw_rounded_rect(trash_x, icon_y, icon_size, icon_size, Color::new(240, 240, 240));
        graphics.draw_text("🗑️", trash_x + 16, icon_y + 16, Color::BLACK);
    }
    
    fn draw_about_dialog(&self, graphics: &mut Graphics) {
        let dialog_width = 400;
        let dialog_height = 300;
        let dialog_x = (SCREEN_WIDTH - dialog_width) / 2;
        let dialog_y = (SCREEN_HEIGHT - dialog_height) / 2;
        
        // Draw backdrop blur
        graphics.draw_rect(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0));
        
        // Draw dialog background
        graphics.draw_rounded_rect(dialog_x, dialog_y, dialog_width, dialog_height, Color::WHITE);
        graphics.draw_rect_outline(dialog_x, dialog_y, dialog_width, dialog_height, Color::GRAY);
        
        // Draw title bar
        graphics.draw_rounded_rect(dialog_x, dialog_y, dialog_width, 40, Color::new(245, 245, 245));
        graphics.draw_text("About This Mac", dialog_x + 20, dialog_y + 15, Color::BLACK);
        
        // Draw close button
        graphics.draw_rounded_rect(dialog_x + 15, dialog_y + 12, 16, 16, Color::RED);
        
        // Draw content
        let content_y = dialog_y + 60;
        graphics.draw_text("RustOS", dialog_x + 180, content_y, Color::BLACK);
        graphics.draw_text("Version 2.0.0", dialog_x + 160, content_y + 30, Color::GRAY);
        
        // Draw system info
        graphics.draw_text("Processor: Custom Rust CPU", dialog_x + 20, content_y + 80, Color::BLACK);
        graphics.draw_text("Memory: 1024 MB", dialog_x + 20, content_y + 100, Color::BLACK);
        graphics.draw_text("Graphics: VGA Compatible", dialog_x + 20, content_y + 120, Color::BLACK);
        graphics.draw_text("Storage: Virtual Disk", dialog_x + 20, content_y + 140, Color::BLACK);
        
        // Draw system logo
        graphics.draw_rounded_rect(dialog_x + 50, content_y - 30, 80, 80, Color::BLUE);
        graphics.draw_text("🦀", dialog_x + 75, content_y - 5, Color::WHITE);
        
        // Draw buttons
        let button_y = dialog_y + dialog_height - 50;
        graphics.draw_rounded_rect(dialog_x + dialog_width - 120, button_y, 100, 30, Color::BLUE);
        graphics.draw_text("More Info", dialog_x + dialog_width - 100, button_y + 10, Color::WHITE);
    }
    
    fn draw_cursor(&self, graphics: &mut Graphics, x: usize, y: usize) {
        // Enhanced macOS-style cursor with shadow
        let cursor_data = [
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 2, 1, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 2, 2, 1, 0, 0, 0, 0, 0],
            [0, 1, 2, 2, 2, 2, 1, 0, 0, 0, 0],
            [0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0],
            [0, 1, 2, 2, 2, 2, 2, 2, 1, 0, 0],
            [0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 0],
            [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            [0, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1],
            [0, 1, 2, 2, 1, 2, 1, 0, 0, 0, 0],
            [0, 1, 2, 1, 0, 1, 2, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 2, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 1, 2, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0],
        ];
        
        // Draw cursor shadow first
        for (dy, row) in cursor_data.iter().enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                if pixel > 0 {
                    graphics.set_pixel(x + dx + 1, y + dy + 1, Color::new(0, 0, 0));
                }
            }
        }
        
        // Draw cursor
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
    
    fn format_time(&self) -> String {
        // Simulate time display
        let hours = (self.time_counter / 3600) % 24 + 12; // Start at 12:xx
        let minutes = (self.time_counter / 60) % 60;
        let seconds = self.time_counter % 60;
        
        if hours > 12 {
            format!("{}:{:02}:{:02} PM", hours - 12, minutes, seconds)
        } else {
            format!("{}:{:02}:{:02} AM", hours, minutes, seconds)
        }
    }
    
    fn create_sample_windows(&mut self) {
        // Create enhanced Finder window
        let mut finder = Window::new(
            "Finder".to_string(),
            80, 80, 500, 350,
            Color::WHITE
        );
        finder.is_focused = true;
        self.window_manager.add_window(finder);
        
        // Create Terminal window with dark theme
        let terminal = Window::new(
            "Terminal — zsh — 80×24".to_string(),
            200, 120, 450, 300,
            Color::new(40, 44, 52) // Dark theme
        );
        self.window_manager.add_window(terminal);
        
        // Create System Preferences window
        let preferences = Window::new(
            "System Preferences".to_string(),
            150, 200, 400, 350,
            Color::new(248, 248, 248)
        );
        self.window_manager.add_window(preferences);
        
        // Create Safari window
        let safari = Window::new(
            "Safari — RustOS Documentation".to_string(),
            120, 60, 520, 400,
            Color::WHITE
        );
        self.window_manager.add_window(safari);
    }
}
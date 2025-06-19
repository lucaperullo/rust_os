// src/main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

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
    graphics.clear_screen(Color::new(240, 240, 245));
    
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
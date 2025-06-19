# 🍎 RustOS - macOS-Inspired Operating System

A beautiful, modern operating system kernel written in Rust that recreates the elegant macOS user experience.

## ✨ Features

### 🖥️ Desktop Environment
- **Menu Bar** - Complete with Apple logo, app menus, and system status
- **Dock** - App launcher with hover effects and running indicators
- **Wallpaper** - Dynamic gradient backgrounds with floating particles
- **Cursor** - Pixel-perfect macOS-style pointer with shadow

### 🪟 Window Management
- **Traffic Light Buttons** - Red, yellow, green window controls
- **Window Shadows** - Realistic drop shadows with blur effects
- **Focus States** - Visual feedback for active/inactive windows
- **Minimize/Maximize** - Full window state management

### 📱 Applications
1. **🗂️ Finder** - File browser with sidebar navigation
2. **💻 Terminal** - Dark-themed command line interface  
3. **⚙️ System Preferences** - Settings with categorized panels
4. **🌐 Safari** - Web browser with tabs and address bar

### 🔍 System Features
- **Spotlight Search** - Quick app and file search
- **Mission Control** - Desktop space overview
- **Notifications** - Sliding notification center
- **Animations** - Smooth easing transitions

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust and components
rustup component add rust-src llvm-tools-preview
cargo install bootimage

# Install system tools (Ubuntu/Debian)
sudo apt install grub-pc-bin grub-efi-amd64-bin mtools xorriso

# macOS
brew install grub xorriso
.PHONY: all build bootimage iso clean run-qemu run-virtualbox demo

all: iso

build:
	@echo "🦀 Building RustOS kernel..."
	cargo build

bootimage: build
	@echo "📦 Creating bootable image..."
	cargo bootimage

iso: bootimage
	@echo "💿 Creating ISO for VirtualBox..."
	mkdir -p build/isofiles/boot/grub
	cp target/x86_64-rust_os/debug/bootimage-rust_os.bin build/isofiles/boot/kernel.bin
	
	@echo "⚙️  Generating GRUB configuration..."
	echo 'set timeout=5' > build/isofiles/boot/grub/grub.cfg
	echo 'set default=0' >> build/isofiles/boot/grub/grub.cfg
	echo '' >> build/isofiles/boot/grub/grub.cfg
	echo 'menuentry "🍎 RustOS - macOS-inspired OS" {' >> build/isofiles/boot/grub/grub.cfg
	echo '    echo "Loading RustOS kernel..."' >> build/isofiles/boot/grub/grub.cfg
	echo '    multiboot2 /boot/kernel.bin' >> build/isofiles/boot/grub/grub.cfg
	echo '    boot' >> build/isofiles/boot/grub/grub.cfg
	echo '}' >> build/isofiles/boot/grub/grub.cfg
	echo '' >> build/isofiles/boot/grub/grub.cfg
	echo 'menuentry "RustOS - Safe Mode" {' >> build/isofiles/boot/grub/grub.cfg
	echo '    echo "Loading RustOS in safe mode..."' >> build/isofiles/boot/grub/grub.cfg
	echo '    multiboot2 /boot/kernel.bin safe_mode' >> build/isofiles/boot/grub/grub.cfg
	echo '    boot' >> build/isofiles/boot/grub/grub.cfg
	echo '}' >> build/isofiles/boot/grub/grub.cfg
	
	@echo "🔥 Generating ISO with GRUB..."
	grub-mkrescue -o rust_os.iso build/isofiles
	
	@echo "✅ ISO created successfully: rust_os.iso"

demo: iso
	@echo "🚀 Starting RustOS demo in QEMU..."
	qemu-system-x86_64 -cdrom rust_os.iso -m 1024

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf build/
	rm -f rust_os.iso

run-qemu: bootimage
	@echo "🖥️  Running RustOS in QEMU..."
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin -m 1024

run-virtualbox: iso
	@echo "📦 RustOS ISO ready for VirtualBox!"
	@echo "✨ Features you'll see:"
	@echo "   🍎 macOS-style menu bar and dock"
	@echo "   🪟 Multiple windows with traffic light buttons"
	@echo "   🔍 Spotlight search interface"
	@echo "   📱 Mission Control overview"
	@echo "   🔔 Notification system"
	@echo "   🎨 Smooth animations and gradients"
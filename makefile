build:
	cargo bootimage && qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin -device isa-debug-exit,iobase=0xf4,iosize=0x04
all: build
clean:
	rmdir /s /q target
all:
	cargo build --target=arm-unknown-linux-gnueabi
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c boot/boot.S -o target/boot.o -fno-exceptions
	arm-none-eabi-gcc -T boot/linker.ld -o target/rustberry.elf -ffreestanding -O2 -nostdlib target/boot.o -Ltarget/arm-unknown-linux-gnueabi/debug/ -lrustberry -fno-exceptions
	arm-none-eabi-objcopy target/rustberry.elf -O binary target/rustberry.bin

qemu:
	qemu-system-arm -m 256 -M raspi2 -serial stdio -kernel target/rustberry.elf

clean:
	rm -r target

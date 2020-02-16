target = aarch64-unknown-none-softfloat

all: kernel8.img

target/$(target)/release/kernel8: $(wildcard src/**/*.rs)
	cargo xbuild --release

kernel8.img: target/$(target)/release/kernel8
	cargo objcopy -- --strip-all -O binary $^ $@

target/start.o: src/start.s
	clang --target=aarch64-elf -c $^ -o $@

dump:
	cargo objdump --target $(target) -- -disassemble -no-show-raw-insn -print-imm-hex kernel8

clean:
	rm -rf target kernel8 kernel8.img

qemu: kernel8.img
	qemu-system-aarch64 -M raspi3 -kernel $^ -serial stdio

nm:
	cargo nm --target $(target) -- --print-size $^ | sort

.PHONY: all clean target/$(target)/release/kernel8

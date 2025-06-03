.PHONY: run
run: build disk-image
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-smp 1 \
		-m 128M \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-display none \
		-serial stdio \
		-kernel hermit-loader-x86_64 \
		-initrd ../../target/x86_64-unknown-hermit/debug/nvme-test \
		-drive file=nvme.img,format=raw,if=none,id=nvm \
		-device nvme,serial=deadbeef,drive=nvm

.PHONY: build
build: 
	HERMIT_LOG_LEVEL_FILTER=INFO \
	cargo build -Z build-std=std,core,alloc,panic_abort \
	--target x86_64-unknown-hermit

.PHONY: disk-image
disk-image:
	qemu-img create nvme.img 4M
	
.PHONY: clean
clean:
	cargo clean
	rm -f nvme.img

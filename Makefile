TARGET := riscv64imac-unknown-none-elf
MODE := debug
PROJECT_NAME := kernel
KERNEL_FILE := target/$(TARGET)/$(MODE)/$(PROJECT_NAME)
BIN_FILE := target/$(TARGET)/$(MODE)/kernel.bin

ifeq ($(MODE), release)
FLAG = -r 
else 
FLAG = 
endif

OBJDUMP := rust-objdump 
OBJCOPY := rust-objcopy

.PHONY: doc kernel build clean qemu run asm r c cbuild debug

build: $(BIN_FILE)

doc: 
	@cargo doc --document-private-items

kernel: 
	@cargo build $(FLAG)

$(BIN_FILE): kernel 
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

asm: 
	@$(OBJDUMP) -d $(KERNEL_FILE) | less 

clean: 
	@cargo clean 

qemu: build 
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(BIN_FILE),addr=0x80200000

run: build qemu 

r: run 

c: 
	@cargo c

cbuild: 
	@make clean 
	@make build

debug: build
	@qemu-system-riscv64 -machine virt -nographic -bios default -device loader,file=$(BIN_FILE),addr=0x80200000 -s -S & \
		riscv64-unknown-elf-gdb --symbols=target/riscv64imac-unknown-none-elf/debug/$(PROJECT_NAME) --eval-command='target remote localhost:1234'
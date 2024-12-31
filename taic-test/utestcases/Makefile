CUR_DIR := $(shell pwd)
APP_DIR := $(CUR_DIR)/src/bin
APPS := $(patsubst $(APP_DIR)/%.rs, %, $(wildcard $(APP_DIR)/*.rs))
TARGET ?= riscv64gc-unknown-linux-musl
MODE ?= release
TARGET_DIR := $(CUR_DIR)/target/$(TARGET)/$(MODE)
ELFS := $(patsubst %, $(TARGET_DIR)/%, $(APPS))
ARCH := riscv64

build:
	@cargo build --$(MODE) --target $(TARGET)
	@for elf in $(ELFS); do \
		cp $$elf $(CUR_DIR)/../testcases; \
	done

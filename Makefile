TEST_DIR = taic-test

A ?= apps/taic_test
ARCH ?= riscv64

build:
	@make -C $(TEST_DIR) A=$(A) ARCH=$(ARCH)

include scripts/opensbi.mk

build_opensbi: build
	$(call build_payload)

run_on_qemu:
	@make -C $(TEST_DIR) A=$(A) ARCH=$(ARCH) run

clean_test:
	make -C $(TEST_DIR) clean
include scripts/qemu.mk
include scripts/opensbi.mk
include scripts/fpga.mk

TEST_DIR = taic-test
A ?= apps/enq_deq_test
ARCH ?= riscv64

QEMU = taic-qemu/build/qemu-system-$(ARCH)

build:
	@make -C $(TEST_DIR) A=$(A) ARCH=$(ARCH)

build_payload: build
ifeq ($(wildcard $(OPENSBI_DIR)/*),)
	git submodule update --init $(OPENSBI_DIR)
endif
	$(call build_payload)

upload_payload: build_payload
	$(call upload_payload)

run_on_qemu:
ifeq ($(wildcard $(QEMU)),)
	echo "QEMU not found, building QEMU..."
	$(call build_qemu)
endif
	@make -C $(TEST_DIR) A=$(A) ARCH=$(ARCH) run

build_fpga:
ifeq ($(wildcard $(ROCKET_CHIP_TAIC)),)
	git submodule update --init $(FPGA_DIR)
	make -C $(FPGA_DIR) init
endif
ifeq ($(wildcard $(ROCKET_CHIP).v),)
	make -C $(FPGA_DIR) build
endif
	$(call build_fpga)

upload_fpga:
ifeq ($(wildcard $(PLDTBO).dtbo),)
	$(call build_fpga)
endif
	$(call upload_fpga)

clean_test:
	make -C $(TEST_DIR) clean
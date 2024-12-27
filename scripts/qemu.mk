QEMU_DIR = taic-qemu

define build_qemu
	git submodule update --init $(QEMU_DIR)
	cd $(QEMU_DIR) && ./configure --target-list="riscv64-softmmu" --enable-slirp && make -j4
endef

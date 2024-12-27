OPENSBI_DIR = opensbi
PLATFORM ?= axu15eg
OPENSBI_OBJMK = $(OPENSBI_DIR)/platform/$(PLATFORM)/objects.mk
FW_PAYLOAD = $(OPENSBI_DIR)/build/platform/$(PLATFORM)/firmware/fw_payload.bin

LOAD ?=$(wildcard $(TEST_DIR)/$(A)/*.bin)

define build_payload
	echo $(LOAD)
	@sed -i "/FW_PAYLOAD_PATH=/d" $(OPENSBI_OBJMK)
	@echo "FW_PAYLOAD_PATH=../$(LOAD)" >> $(OPENSBI_OBJMK)
	make -C opensbi PLATFORM=$(PLATFORM) CROSS_COMPILE=riscv64-linux-musl-
endef

define upload_payload
	scp $(FW_PAYLOAD) axu15eg:~
endef

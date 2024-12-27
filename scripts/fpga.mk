FPGA_DIR = taic-rocket-chip
ROCKET_CHIP = taic-rocket-chip/vivado_proj/src/hdl/rocketchip
ROCKET_CHIP_TAIC = taic-rocket-chip/rocket-chip/src/main/scala/taic
VIVADO_PRJ_DIR = taic-rocket-chip/vivado_proj/proj
VIVADO_CFG_FILE = taic-rocket-chip/digilent-vivado-scripts/config.ini

VIVADO_VERSION ?= 2022.2
VIVADO_PATH ?= /opt/Xilinx/Vivado

define build_fpga
	$(shell sed -i 's/VivadoVersion = \([0-9]\+\.[0-9]\+\)/VivadoVersion = $(VIVADO_VERSION)/g' $(VIVADO_CFG_FILE))
	$(shell sed -i "s|VivadoInstallPath = [^ ]*|VivadoInstallPath = $(VIVADO_PATH)|" $(VIVADO_CFG_FILE))
	@rm -rf $(VIVADO_PRJ_DIR)
	@make -C $(FPGA_DIR) checkout
	@make -C $(FPGA_DIR) bitbin
endef

BITBIN := taic-rocket-chip/vivado_proj/proj/system_wrapper.bit.bin
PLDTBO := taic-rocket-chip/vivado_proj/proj/pl.dtbo

define upload_fpga
	scp $(BITBIN) $(PLDTBO) axu15eg:~
endef
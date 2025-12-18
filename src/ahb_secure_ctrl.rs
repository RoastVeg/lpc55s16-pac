#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sec_ctrl_flash_rom_slave_rule: SecCtrlFlashRomSlaveRule,
    _reserved1: [u8; 0x0c],
    sec_ctrl_flash_mem_rule0: SecCtrlFlashMemRule0,
    _reserved2: [u8; 0x0c],
    sec_ctrl_rom_mem_rule0: SecCtrlRomMemRule0,
    sec_ctrl_rom_mem_rule1: SecCtrlRomMemRule1,
    sec_ctrl_rom_mem_rule2: SecCtrlRomMemRule2,
    sec_ctrl_rom_mem_rule3: SecCtrlRomMemRule3,
    sec_ctrl_ramx_slave_rule: SecCtrlRamxSlaveRule,
    _reserved7: [u8; 0x0c],
    sec_ctrl_ramx_mem_rule0: SecCtrlRamxMemRule0,
    _reserved8: [u8; 0x0c],
    sec_ctrl_ram0_slave_rule: SecCtrlRam0SlaveRule,
    _reserved9: [u8; 0x0c],
    sec_ctrl_ram0_mem_rule0: SecCtrlRam0MemRule0,
    _reserved10: [u8; 0x0c],
    sec_ctrl_ram1_slave_rule: SecCtrlRam1SlaveRule,
    _reserved11: [u8; 0x0c],
    sec_ctrl_ram1_mem_rule0: SecCtrlRam1MemRule0,
    _reserved12: [u8; 0x0c],
    sec_ctrl_ram2_slave_rule: SecCtrlRam2SlaveRule,
    _reserved13: [u8; 0x0c],
    sec_ctrl_ram2_mem_rule0: SecCtrlRam2MemRule0,
    _reserved14: [u8; 0x0c],
    sec_ctrl_usb_hs_slave_rule: SecCtrlUsbHsSlaveRule,
    _reserved15: [u8; 0x0c],
    sec_ctrl_usb_hs_mem_rule: SecCtrlUsbHsMemRule,
    _reserved16: [u8; 0x0c],
    sec_ctrl_apb_bridge_slave_rule: SecCtrlApbBridgeSlaveRule,
    _reserved17: [u8; 0x0c],
    sec_ctrl_apb_bridge0_mem_ctrl0: SecCtrlApbBridge0MemCtrl0,
    sec_ctrl_apb_bridge0_mem_ctrl1: SecCtrlApbBridge0MemCtrl1,
    sec_ctrl_apb_bridge0_mem_ctrl2: SecCtrlApbBridge0MemCtrl2,
    _reserved20: [u8; 0x04],
    sec_ctrl_apb_bridge1_mem_ctrl0: SecCtrlApbBridge1MemCtrl0,
    sec_ctrl_apb_bridge1_mem_ctrl1: SecCtrlApbBridge1MemCtrl1,
    sec_ctrl_apb_bridge1_mem_ctrl2: SecCtrlApbBridge1MemCtrl2,
    sec_ctrl_apb_bridge1_mem_ctrl3: SecCtrlApbBridge1MemCtrl3,
    sec_ctrl_ahb_port7_slave0_rule: SecCtrlAhbPort7Slave0Rule,
    sec_ctrl_ahb_port7_slave1_rule: SecCtrlAhbPort7Slave1Rule,
    _reserved26: [u8; 0x08],
    sec_ctrl_ahb_port8_slave0_rule: SecCtrlAhbPort8Slave0Rule,
    sec_ctrl_ahb_port8_slave1_rule: SecCtrlAhbPort8Slave1Rule,
    _reserved28: [u8; 0x08],
    sec_ctrl_ahb_port9_slave0_rule: SecCtrlAhbPort9Slave0Rule,
    sec_ctrl_ahb_port9_slave1_rule: SecCtrlAhbPort9Slave1Rule,
    _reserved30: [u8; 0x08],
    sec_ctrl_ahb_sec_ctrl_mem_rule: SecCtrlAhbSecCtrlMemRule,
    _reserved31: [u8; 0x0ccc],
    sec_vio_addr: [SecVioAddr; 10],
    _reserved32: [u8; 0x58],
    sec_vio_misc_info: [SecVioMiscInfo; 10],
    _reserved33: [u8; 0x58],
    sec_vio_info_valid: SecVioInfoValid,
    _reserved34: [u8; 0x7c],
    sec_gpio_mask0: SecGpioMask0,
    sec_gpio_mask1: SecGpioMask1,
    _reserved36: [u8; 0x34],
    sec_mask_lock: SecMaskLock,
    _reserved37: [u8; 0x10],
    master_sec_level: MasterSecLevel,
    master_sec_anti_pol_reg: MasterSecAntiPolReg,
    _reserved39: [u8; 0x14],
    cpu0_lock_reg: Cpu0LockReg,
    _reserved40: [u8; 0x08],
    misc_ctrl_dp_reg: MiscCtrlDpReg,
    misc_ctrl_reg: MiscCtrlReg,
}
impl RegisterBlock {
    #[doc = "0x00 - Security access rules for Flash and ROM slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_flash_rom_slave_rule(&self) -> &SecCtrlFlashRomSlaveRule {
        &self.sec_ctrl_flash_rom_slave_rule
    }
    #[doc = "0x10 - Security access rules for FLASH sector 0 to sector 7. Each Flash sector is 32 Kbytes. There are 8 FLASH sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_flash_mem_rule0(&self) -> &SecCtrlFlashMemRule0 {
        &self.sec_ctrl_flash_mem_rule0
    }
    #[doc = "0x20 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule0(&self) -> &SecCtrlRomMemRule0 {
        &self.sec_ctrl_rom_mem_rule0
    }
    #[doc = "0x24 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule1(&self) -> &SecCtrlRomMemRule1 {
        &self.sec_ctrl_rom_mem_rule1
    }
    #[doc = "0x28 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule2(&self) -> &SecCtrlRomMemRule2 {
        &self.sec_ctrl_rom_mem_rule2
    }
    #[doc = "0x2c - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule3(&self) -> &SecCtrlRomMemRule3 {
        &self.sec_ctrl_rom_mem_rule3
    }
    #[doc = "0x30 - Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ramx_slave_rule(&self) -> &SecCtrlRamxSlaveRule {
        &self.sec_ctrl_ramx_slave_rule
    }
    #[doc = "0x40 - Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ramx_mem_rule0(&self) -> &SecCtrlRamxMemRule0 {
        &self.sec_ctrl_ramx_mem_rule0
    }
    #[doc = "0x50 - Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram0_slave_rule(&self) -> &SecCtrlRam0SlaveRule {
        &self.sec_ctrl_ram0_slave_rule
    }
    #[doc = "0x60 - Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram0_mem_rule0(&self) -> &SecCtrlRam0MemRule0 {
        &self.sec_ctrl_ram0_mem_rule0
    }
    #[doc = "0x70 - Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram1_slave_rule(&self) -> &SecCtrlRam1SlaveRule {
        &self.sec_ctrl_ram1_slave_rule
    }
    #[doc = "0x80 - Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram1_mem_rule0(&self) -> &SecCtrlRam1MemRule0 {
        &self.sec_ctrl_ram1_mem_rule0
    }
    #[doc = "0x90 - Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram2_slave_rule(&self) -> &SecCtrlRam2SlaveRule {
        &self.sec_ctrl_ram2_slave_rule
    }
    #[doc = "0xa0 - Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram2_mem_rule0(&self) -> &SecCtrlRam2MemRule0 {
        &self.sec_ctrl_ram2_mem_rule0
    }
    #[doc = "0xb0 - Security access rules for USB High speed RAM slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_usb_hs_slave_rule(&self) -> &SecCtrlUsbHsSlaveRule {
        &self.sec_ctrl_usb_hs_slave_rule
    }
    #[doc = "0xc0 - Security access rules for RAM_USB_HS."]
    #[inline(always)]
    pub const fn sec_ctrl_usb_hs_mem_rule(&self) -> &SecCtrlUsbHsMemRule {
        &self.sec_ctrl_usb_hs_mem_rule
    }
    #[doc = "0xd0 - Security access rules for both APB Bridges slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge_slave_rule(&self) -> &SecCtrlApbBridgeSlaveRule {
        &self.sec_ctrl_apb_bridge_slave_rule
    }
    #[doc = "0xe0 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl0(&self) -> &SecCtrlApbBridge0MemCtrl0 {
        &self.sec_ctrl_apb_bridge0_mem_ctrl0
    }
    #[doc = "0xe4 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl1(&self) -> &SecCtrlApbBridge0MemCtrl1 {
        &self.sec_ctrl_apb_bridge0_mem_ctrl1
    }
    #[doc = "0xe8 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl2(&self) -> &SecCtrlApbBridge0MemCtrl2 {
        &self.sec_ctrl_apb_bridge0_mem_ctrl2
    }
    #[doc = "0xf0 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl0(&self) -> &SecCtrlApbBridge1MemCtrl0 {
        &self.sec_ctrl_apb_bridge1_mem_ctrl0
    }
    #[doc = "0xf4 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl1(&self) -> &SecCtrlApbBridge1MemCtrl1 {
        &self.sec_ctrl_apb_bridge1_mem_ctrl1
    }
    #[doc = "0xf8 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl2(&self) -> &SecCtrlApbBridge1MemCtrl2 {
        &self.sec_ctrl_apb_bridge1_mem_ctrl2
    }
    #[doc = "0xfc - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl3(&self) -> &SecCtrlApbBridge1MemCtrl3 {
        &self.sec_ctrl_apb_bridge1_mem_ctrl3
    }
    #[doc = "0x100 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port7_slave0_rule(&self) -> &SecCtrlAhbPort7Slave0Rule {
        &self.sec_ctrl_ahb_port7_slave0_rule
    }
    #[doc = "0x104 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port7_slave1_rule(&self) -> &SecCtrlAhbPort7Slave1Rule {
        &self.sec_ctrl_ahb_port7_slave1_rule
    }
    #[doc = "0x110 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port8_slave0_rule(&self) -> &SecCtrlAhbPort8Slave0Rule {
        &self.sec_ctrl_ahb_port8_slave0_rule
    }
    #[doc = "0x114 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port8_slave1_rule(&self) -> &SecCtrlAhbPort8Slave1Rule {
        &self.sec_ctrl_ahb_port8_slave1_rule
    }
    #[doc = "0x120 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port9_slave0_rule(&self) -> &SecCtrlAhbPort9Slave0Rule {
        &self.sec_ctrl_ahb_port9_slave0_rule
    }
    #[doc = "0x124 - Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port9_slave1_rule(&self) -> &SecCtrlAhbPort9Slave1Rule {
        &self.sec_ctrl_ahb_port9_slave1_rule
    }
    #[doc = "0x130 - Security access rules for AHB_SEC_CTRL_AHB."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_sec_ctrl_mem_rule(&self) -> &SecCtrlAhbSecCtrlMemRule {
        &self.sec_ctrl_ahb_sec_ctrl_mem_rule
    }
    #[doc = "0xe00..0xe28 - most recent security violation address for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_addr(&self, n: usize) -> &SecVioAddr {
        &self.sec_vio_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe00..0xe28 - most recent security violation address for AHB layer n"]
    #[inline(always)]
    pub fn sec_vio_addr_iter(&self) -> impl Iterator<Item = &SecVioAddr> {
        self.sec_vio_addr.iter()
    }
    #[doc = "0xe80..0xea8 - most recent security violation miscellaneous information for AHB layer n"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(&self, n: usize) -> &SecVioMiscInfo {
        &self.sec_vio_misc_info[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe80..0xea8 - most recent security violation miscellaneous information for AHB layer n"]
    #[inline(always)]
    pub fn sec_vio_misc_info_iter(&self) -> impl Iterator<Item = &SecVioMiscInfo> {
        self.sec_vio_misc_info.iter()
    }
    #[doc = "0xf00 - security violation address/information registers valid flags"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(&self) -> &SecVioInfoValid {
        &self.sec_vio_info_valid
    }
    #[doc = "0xf80 - Secure GPIO mask for port 0 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask0(&self) -> &SecGpioMask0 {
        &self.sec_gpio_mask0
    }
    #[doc = "0xf84 - Secure GPIO mask for port 1 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask1(&self) -> &SecGpioMask1 {
        &self.sec_gpio_mask1
    }
    #[doc = "0xfbc - Security General Purpose register access control."]
    #[inline(always)]
    pub const fn sec_mask_lock(&self) -> &SecMaskLock {
        &self.sec_mask_lock
    }
    #[doc = "0xfd0 - master secure level register"]
    #[inline(always)]
    pub const fn master_sec_level(&self) -> &MasterSecLevel {
        &self.master_sec_level
    }
    #[doc = "0xfd4 - master secure level anti-pole register"]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(&self) -> &MasterSecAntiPolReg {
        &self.master_sec_anti_pol_reg
    }
    #[doc = "0xfec - Miscalleneous control signals for in Cortex M33 (CPU0)"]
    #[inline(always)]
    pub const fn cpu0_lock_reg(&self) -> &Cpu0LockReg {
        &self.cpu0_lock_reg
    }
    #[doc = "0xff8 - secure control duplicate register"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(&self) -> &MiscCtrlDpReg {
        &self.misc_ctrl_dp_reg
    }
    #[doc = "0xffc - secure control register"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(&self) -> &MiscCtrlReg {
        &self.misc_ctrl_reg
    }
}
#[doc = "SEC_CTRL_FLASH_ROM_SLAVE_RULE (rw) register accessor: Security access rules for Flash and ROM slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_flash_rom_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_flash_rom_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_flash_rom_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_FLASH_ROM_SLAVE_RULE")]
pub type SecCtrlFlashRomSlaveRule =
    crate::Reg<sec_ctrl_flash_rom_slave_rule::SecCtrlFlashRomSlaveRuleSpec>;
#[doc = "Security access rules for Flash and ROM slaves."]
pub mod sec_ctrl_flash_rom_slave_rule;
#[doc = "SEC_CTRL_FLASH_MEM_RULE0 (rw) register accessor: Security access rules for FLASH sector 0 to sector 7. Each Flash sector is 32 Kbytes. There are 8 FLASH sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_flash_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_flash_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_flash_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_FLASH_MEM_RULE0")]
pub type SecCtrlFlashMemRule0 = crate::Reg<sec_ctrl_flash_mem_rule0::SecCtrlFlashMemRule0Spec>;
#[doc = "Security access rules for FLASH sector 0 to sector 7. Each Flash sector is 32 Kbytes. There are 8 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule0;
#[doc = "SEC_CTRL_ROM_MEM_RULE0 (rw) register accessor: Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_rom_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_rom_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_rom_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_ROM_MEM_RULE0")]
pub type SecCtrlRomMemRule0 = crate::Reg<sec_ctrl_rom_mem_rule0::SecCtrlRomMemRule0Spec>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule0;
#[doc = "SEC_CTRL_ROM_MEM_RULE1 (rw) register accessor: Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_rom_mem_rule1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_rom_mem_rule1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_rom_mem_rule1`] module"]
#[doc(alias = "SEC_CTRL_ROM_MEM_RULE1")]
pub type SecCtrlRomMemRule1 = crate::Reg<sec_ctrl_rom_mem_rule1::SecCtrlRomMemRule1Spec>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule1;
#[doc = "SEC_CTRL_ROM_MEM_RULE2 (rw) register accessor: Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_rom_mem_rule2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_rom_mem_rule2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_rom_mem_rule2`] module"]
#[doc(alias = "SEC_CTRL_ROM_MEM_RULE2")]
pub type SecCtrlRomMemRule2 = crate::Reg<sec_ctrl_rom_mem_rule2::SecCtrlRomMemRule2Spec>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule2;
#[doc = "SEC_CTRL_ROM_MEM_RULE3 (rw) register accessor: Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_rom_mem_rule3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_rom_mem_rule3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_rom_mem_rule3`] module"]
#[doc(alias = "SEC_CTRL_ROM_MEM_RULE3")]
pub type SecCtrlRomMemRule3 = crate::Reg<sec_ctrl_rom_mem_rule3::SecCtrlRomMemRule3Spec>;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule3;
#[doc = "SEC_CTRL_RAMX_SLAVE_RULE (rw) register accessor: Security access rules for RAMX slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ramx_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ramx_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ramx_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_RAMX_SLAVE_RULE")]
pub type SecCtrlRamxSlaveRule = crate::Reg<sec_ctrl_ramx_slave_rule::SecCtrlRamxSlaveRuleSpec>;
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_slave_rule;
#[doc = "SEC_CTRL_RAMX_MEM_RULE0 (rw) register accessor: Security access rules for RAMX slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ramx_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ramx_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ramx_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_RAMX_MEM_RULE0")]
pub type SecCtrlRamxMemRule0 = crate::Reg<sec_ctrl_ramx_mem_rule0::SecCtrlRamxMemRule0Spec>;
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_mem_rule0;
#[doc = "SEC_CTRL_RAM0_SLAVE_RULE (rw) register accessor: Security access rules for RAM0 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram0_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram0_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram0_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_RAM0_SLAVE_RULE")]
pub type SecCtrlRam0SlaveRule = crate::Reg<sec_ctrl_ram0_slave_rule::SecCtrlRam0SlaveRuleSpec>;
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_slave_rule;
#[doc = "SEC_CTRL_RAM0_MEM_RULE0 (rw) register accessor: Security access rules for RAM0 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram0_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram0_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram0_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_RAM0_MEM_RULE0")]
pub type SecCtrlRam0MemRule0 = crate::Reg<sec_ctrl_ram0_mem_rule0::SecCtrlRam0MemRule0Spec>;
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_mem_rule0;
#[doc = "SEC_CTRL_RAM1_SLAVE_RULE (rw) register accessor: Security access rules for RAM1 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram1_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram1_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram1_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_RAM1_SLAVE_RULE")]
pub type SecCtrlRam1SlaveRule = crate::Reg<sec_ctrl_ram1_slave_rule::SecCtrlRam1SlaveRuleSpec>;
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_slave_rule;
#[doc = "SEC_CTRL_RAM1_MEM_RULE0 (rw) register accessor: Security access rules for RAM1 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram1_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram1_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram1_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_RAM1_MEM_RULE0")]
pub type SecCtrlRam1MemRule0 = crate::Reg<sec_ctrl_ram1_mem_rule0::SecCtrlRam1MemRule0Spec>;
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_mem_rule0;
#[doc = "SEC_CTRL_RAM2_SLAVE_RULE (rw) register accessor: Security access rules for RAM2 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram2_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram2_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram2_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_RAM2_SLAVE_RULE")]
pub type SecCtrlRam2SlaveRule = crate::Reg<sec_ctrl_ram2_slave_rule::SecCtrlRam2SlaveRuleSpec>;
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_slave_rule;
#[doc = "SEC_CTRL_RAM2_MEM_RULE0 (rw) register accessor: Security access rules for RAM2 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram2_mem_rule0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram2_mem_rule0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ram2_mem_rule0`] module"]
#[doc(alias = "SEC_CTRL_RAM2_MEM_RULE0")]
pub type SecCtrlRam2MemRule0 = crate::Reg<sec_ctrl_ram2_mem_rule0::SecCtrlRam2MemRule0Spec>;
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_mem_rule0;
#[doc = "SEC_CTRL_USB_HS_SLAVE_RULE (rw) register accessor: Security access rules for USB High speed RAM slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_usb_hs_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_usb_hs_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_usb_hs_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_USB_HS_SLAVE_RULE")]
pub type SecCtrlUsbHsSlaveRule = crate::Reg<sec_ctrl_usb_hs_slave_rule::SecCtrlUsbHsSlaveRuleSpec>;
#[doc = "Security access rules for USB High speed RAM slaves."]
pub mod sec_ctrl_usb_hs_slave_rule;
#[doc = "SEC_CTRL_USB_HS_MEM_RULE (rw) register accessor: Security access rules for RAM_USB_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_usb_hs_mem_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_usb_hs_mem_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_usb_hs_mem_rule`] module"]
#[doc(alias = "SEC_CTRL_USB_HS_MEM_RULE")]
pub type SecCtrlUsbHsMemRule = crate::Reg<sec_ctrl_usb_hs_mem_rule::SecCtrlUsbHsMemRuleSpec>;
#[doc = "Security access rules for RAM_USB_HS."]
pub mod sec_ctrl_usb_hs_mem_rule;
#[doc = "SEC_CTRL_APB_BRIDGE_SLAVE_RULE (rw) register accessor: Security access rules for both APB Bridges slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge_slave_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge_slave_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge_slave_rule`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE_SLAVE_RULE")]
pub type SecCtrlApbBridgeSlaveRule =
    crate::Reg<sec_ctrl_apb_bridge_slave_rule::SecCtrlApbBridgeSlaveRuleSpec>;
#[doc = "Security access rules for both APB Bridges slaves."]
pub mod sec_ctrl_apb_bridge_slave_rule;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 (rw) register accessor: Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge0_mem_ctrl0`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL0")]
pub type SecCtrlApbBridge0MemCtrl0 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl0::SecCtrlApbBridge0MemCtrl0Spec>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl0;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 (rw) register accessor: Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge0_mem_ctrl1`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL1")]
pub type SecCtrlApbBridge0MemCtrl1 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl1::SecCtrlApbBridge0MemCtrl1Spec>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl1;
#[doc = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 (rw) register accessor: Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge0_mem_ctrl2`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE0_MEM_CTRL2")]
pub type SecCtrlApbBridge0MemCtrl2 =
    crate::Reg<sec_ctrl_apb_bridge0_mem_ctrl2::SecCtrlApbBridge0MemCtrl2Spec>;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl2;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge1_mem_ctrl0`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL0")]
pub type SecCtrlApbBridge1MemCtrl0 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl0::SecCtrlApbBridge1MemCtrl0Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl0;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge1_mem_ctrl1`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL1")]
pub type SecCtrlApbBridge1MemCtrl1 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl1::SecCtrlApbBridge1MemCtrl1Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl1;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge1_mem_ctrl2`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL2")]
pub type SecCtrlApbBridge1MemCtrl2 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl2::SecCtrlApbBridge1MemCtrl2Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl2;
#[doc = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 (rw) register accessor: Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_apb_bridge1_mem_ctrl3`] module"]
#[doc(alias = "SEC_CTRL_APB_BRIDGE1_MEM_CTRL3")]
pub type SecCtrlApbBridge1MemCtrl3 =
    crate::Reg<sec_ctrl_apb_bridge1_mem_ctrl3::SecCtrlApbBridge1MemCtrl3Spec>;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl3;
#[doc = "SEC_CTRL_AHB_PORT7_SLAVE0_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port7_slave0_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port7_slave0_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port7_slave0_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT7_SLAVE0_RULE")]
pub type SecCtrlAhbPort7Slave0Rule =
    crate::Reg<sec_ctrl_ahb_port7_slave0_rule::SecCtrlAhbPort7Slave0RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port7_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT7_SLAVE1_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port7_slave1_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port7_slave1_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port7_slave1_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT7_SLAVE1_RULE")]
pub type SecCtrlAhbPort7Slave1Rule =
    crate::Reg<sec_ctrl_ahb_port7_slave1_rule::SecCtrlAhbPort7Slave1RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port7_slave1_rule;
#[doc = "SEC_CTRL_AHB_PORT8_SLAVE0_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port8_slave0_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port8_slave0_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port8_slave0_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT8_SLAVE0_RULE")]
pub type SecCtrlAhbPort8Slave0Rule =
    crate::Reg<sec_ctrl_ahb_port8_slave0_rule::SecCtrlAhbPort8Slave0RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT8_SLAVE1_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port8_slave1_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port8_slave1_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port8_slave1_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT8_SLAVE1_RULE")]
pub type SecCtrlAhbPort8Slave1Rule =
    crate::Reg<sec_ctrl_ahb_port8_slave1_rule::SecCtrlAhbPort8Slave1RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port8_slave1_rule;
#[doc = "SEC_CTRL_AHB_PORT9_SLAVE0_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port9_slave0_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port9_slave0_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port9_slave0_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT9_SLAVE0_RULE")]
pub type SecCtrlAhbPort9Slave0Rule =
    crate::Reg<sec_ctrl_ahb_port9_slave0_rule::SecCtrlAhbPort9Slave0RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave0_rule;
#[doc = "SEC_CTRL_AHB_PORT9_SLAVE1_RULE (rw) register accessor: Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port9_slave1_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port9_slave1_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_port9_slave1_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_PORT9_SLAVE1_RULE")]
pub type SecCtrlAhbPort9Slave1Rule =
    crate::Reg<sec_ctrl_ahb_port9_slave1_rule::SecCtrlAhbPort9Slave1RuleSpec>;
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb_port9_slave1_rule;
#[doc = "SEC_CTRL_AHB_SEC_CTRL_MEM_RULE (rw) register accessor: Security access rules for AHB_SEC_CTRL_AHB.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_sec_ctrl_mem_rule::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_sec_ctrl_mem_rule::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl_ahb_sec_ctrl_mem_rule`] module"]
#[doc(alias = "SEC_CTRL_AHB_SEC_CTRL_MEM_RULE")]
pub type SecCtrlAhbSecCtrlMemRule =
    crate::Reg<sec_ctrl_ahb_sec_ctrl_mem_rule::SecCtrlAhbSecCtrlMemRuleSpec>;
#[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
pub mod sec_ctrl_ahb_sec_ctrl_mem_rule;
#[doc = "sec_vio_addr (r) register accessor: most recent security violation address for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_addr`] module"]
#[doc(alias = "sec_vio_addr")]
pub type SecVioAddr = crate::Reg<sec_vio_addr::SecVioAddrSpec>;
#[doc = "most recent security violation address for AHB layer n"]
pub mod sec_vio_addr;
#[doc = "sec_vio_misc_info (r) register accessor: most recent security violation miscellaneous information for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_misc_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_misc_info`] module"]
#[doc(alias = "sec_vio_misc_info")]
pub type SecVioMiscInfo = crate::Reg<sec_vio_misc_info::SecVioMiscInfoSpec>;
#[doc = "most recent security violation miscellaneous information for AHB layer n"]
pub mod sec_vio_misc_info;
#[doc = "SEC_VIO_INFO_VALID (rw) register accessor: security violation address/information registers valid flags\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_info_valid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_vio_info_valid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_vio_info_valid`] module"]
#[doc(alias = "SEC_VIO_INFO_VALID")]
pub type SecVioInfoValid = crate::Reg<sec_vio_info_valid::SecVioInfoValidSpec>;
#[doc = "security violation address/information registers valid flags"]
pub mod sec_vio_info_valid;
#[doc = "SEC_GPIO_MASK0 (rw) register accessor: Secure GPIO mask for port 0 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask0`] module"]
#[doc(alias = "SEC_GPIO_MASK0")]
pub type SecGpioMask0 = crate::Reg<sec_gpio_mask0::SecGpioMask0Spec>;
#[doc = "Secure GPIO mask for port 0 pins."]
pub mod sec_gpio_mask0;
#[doc = "SEC_GPIO_MASK1 (rw) register accessor: Secure GPIO mask for port 1 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_gpio_mask1`] module"]
#[doc(alias = "SEC_GPIO_MASK1")]
pub type SecGpioMask1 = crate::Reg<sec_gpio_mask1::SecGpioMask1Spec>;
#[doc = "Secure GPIO mask for port 1 pins."]
pub mod sec_gpio_mask1;
#[doc = "SEC_MASK_LOCK (rw) register accessor: Security General Purpose register access control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_mask_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_mask_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_mask_lock`] module"]
#[doc(alias = "SEC_MASK_LOCK")]
pub type SecMaskLock = crate::Reg<sec_mask_lock::SecMaskLockSpec>;
#[doc = "Security General Purpose register access control."]
pub mod sec_mask_lock;
#[doc = "MASTER_SEC_LEVEL (rw) register accessor: master secure level register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_sec_level`] module"]
#[doc(alias = "MASTER_SEC_LEVEL")]
pub type MasterSecLevel = crate::Reg<master_sec_level::MasterSecLevelSpec>;
#[doc = "master secure level register"]
pub mod master_sec_level;
#[doc = "MASTER_SEC_ANTI_POL_REG (rw) register accessor: master secure level anti-pole register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_anti_pol_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_anti_pol_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_sec_anti_pol_reg`] module"]
#[doc(alias = "MASTER_SEC_ANTI_POL_REG")]
pub type MasterSecAntiPolReg = crate::Reg<master_sec_anti_pol_reg::MasterSecAntiPolRegSpec>;
#[doc = "master secure level anti-pole register"]
pub mod master_sec_anti_pol_reg;
#[doc = "CPU0_LOCK_REG (rw) register accessor: Miscalleneous control signals for in Cortex M33 (CPU0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_lock_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_lock_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_lock_reg`] module"]
#[doc(alias = "CPU0_LOCK_REG")]
pub type Cpu0LockReg = crate::Reg<cpu0_lock_reg::Cpu0LockRegSpec>;
#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)"]
pub mod cpu0_lock_reg;
#[doc = "MISC_CTRL_DP_REG (rw) register accessor: secure control duplicate register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl_dp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl_dp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl_dp_reg`] module"]
#[doc(alias = "MISC_CTRL_DP_REG")]
pub type MiscCtrlDpReg = crate::Reg<misc_ctrl_dp_reg::MiscCtrlDpRegSpec>;
#[doc = "secure control duplicate register"]
pub mod misc_ctrl_dp_reg;
#[doc = "MISC_CTRL_REG (rw) register accessor: secure control register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl_reg`] module"]
#[doc(alias = "MISC_CTRL_REG")]
pub type MiscCtrlReg = crate::Reg<misc_ctrl_reg::MiscCtrlRegSpec>;
#[doc = "secure control register"]
pub mod misc_ctrl_reg;

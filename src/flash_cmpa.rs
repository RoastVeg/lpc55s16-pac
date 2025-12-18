#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    boot_cfg: BootCfg,
    spi_flash_cfg: SpiFlashCfg,
    usb_id: UsbId,
    sdio_cfg: SdioCfg,
    cc_socu_pin: CcSocuPin,
    cc_socu_dflt: CcSocuDflt,
    vendor_usage: VendorUsage,
    secure_boot_cfg: SecureBootCfg,
    prince_base_addr: PrinceBaseAddr,
    prince_sr_0: PrinceSr0,
    prince_sr_1: PrinceSr1,
    prince_sr_2: PrinceSr2,
    xtal_32khz_capabank_trim: Xtal32khzCapabankTrim,
    xtal_16mhz_capabank_trim: Xtal16mhzCapabankTrim,
    flash_remap_size: FlashRemapSize,
    flash_remap_offset: FlashRemapOffset,
    _reserved16: [u8; 0x10],
    rotkh: [Rotkh; 8],
    _reserved17: [u8; 0x90],
    customer_defined: [CustomerDefined; 56],
    sha256_digest: [Sha256Digest; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn boot_cfg(&self) -> &BootCfg {
        &self.boot_cfg
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn spi_flash_cfg(&self) -> &SpiFlashCfg {
        &self.spi_flash_cfg
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn usb_id(&self) -> &UsbId {
        &self.usb_id
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn sdio_cfg(&self) -> &SdioCfg {
        &self.sdio_cfg
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn cc_socu_pin(&self) -> &CcSocuPin {
        &self.cc_socu_pin
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn cc_socu_dflt(&self) -> &CcSocuDflt {
        &self.cc_socu_dflt
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn vendor_usage(&self) -> &VendorUsage {
        &self.vendor_usage
    }
    #[doc = "0x1c - Secure boot configuration flags."]
    #[inline(always)]
    pub const fn secure_boot_cfg(&self) -> &SecureBootCfg {
        &self.secure_boot_cfg
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn prince_base_addr(&self) -> &PrinceBaseAddr {
        &self.prince_base_addr
    }
    #[doc = "0x24 - Region 0, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_0(&self) -> &PrinceSr0 {
        &self.prince_sr_0
    }
    #[doc = "0x28 - Region 1, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_1(&self) -> &PrinceSr1 {
        &self.prince_sr_1
    }
    #[doc = "0x2c - Region 2, sub-region enable"]
    #[inline(always)]
    pub const fn prince_sr_2(&self) -> &PrinceSr2 {
        &self.prince_sr_2
    }
    #[doc = "0x30 - Xtal 32kHz capabank triming."]
    #[inline(always)]
    pub const fn xtal_32khz_capabank_trim(&self) -> &Xtal32khzCapabankTrim {
        &self.xtal_32khz_capabank_trim
    }
    #[doc = "0x34 - Xtal 16MHz capabank triming."]
    #[inline(always)]
    pub const fn xtal_16mhz_capabank_trim(&self) -> &Xtal16mhzCapabankTrim {
        &self.xtal_16mhz_capabank_trim
    }
    #[doc = "0x38 - This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
    #[inline(always)]
    pub const fn flash_remap_size(&self) -> &FlashRemapSize {
        &self.flash_remap_size
    }
    #[doc = "0x3c - This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
    #[inline(always)]
    pub const fn flash_remap_offset(&self) -> &FlashRemapOffset {
        &self.flash_remap_offset
    }
    #[doc = "0x50..0x70 - ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
    #[inline(always)]
    pub const fn rotkh(&self, n: usize) -> &Rotkh {
        &self.rotkh[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x70 - ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
    #[inline(always)]
    pub fn rotkh_iter(&self) -> impl Iterator<Item = &Rotkh> {
        self.rotkh.iter()
    }
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub const fn customer_defined(&self, n: usize) -> &CustomerDefined {
        &self.customer_defined[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub fn customer_defined_iter(&self) -> impl Iterator<Item = &CustomerDefined> {
        self.customer_defined.iter()
    }
    #[doc = "0x1e0..0x200 - SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(&self, n: usize) -> &Sha256Digest {
        &self.sha256_digest[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1e0..0x200 - SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub fn sha256_digest_iter(&self) -> impl Iterator<Item = &Sha256Digest> {
        self.sha256_digest.iter()
    }
}
#[doc = "BOOT_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_cfg`] module"]
#[doc(alias = "BOOT_CFG")]
pub type BootCfg = crate::Reg<boot_cfg::BootCfgSpec>;
#[doc = "no description available"]
pub mod boot_cfg;
#[doc = "SPI_FLASH_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_flash_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_flash_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_flash_cfg`] module"]
#[doc(alias = "SPI_FLASH_CFG")]
pub type SpiFlashCfg = crate::Reg<spi_flash_cfg::SpiFlashCfgSpec>;
#[doc = "no description available"]
pub mod spi_flash_cfg;
#[doc = "USB_ID (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_id`] module"]
#[doc(alias = "USB_ID")]
pub type UsbId = crate::Reg<usb_id::UsbIdSpec>;
#[doc = "no description available"]
pub mod usb_id;
#[doc = "SDIO_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_cfg`] module"]
#[doc(alias = "SDIO_CFG")]
pub type SdioCfg = crate::Reg<sdio_cfg::SdioCfgSpec>;
#[doc = "no description available"]
pub mod sdio_cfg;
#[doc = "CC_SOCU_PIN (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_socu_pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_socu_pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_socu_pin`] module"]
#[doc(alias = "CC_SOCU_PIN")]
pub type CcSocuPin = crate::Reg<cc_socu_pin::CcSocuPinSpec>;
#[doc = "no description available"]
pub mod cc_socu_pin;
#[doc = "CC_SOCU_DFLT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_socu_dflt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_socu_dflt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_socu_dflt`] module"]
#[doc(alias = "CC_SOCU_DFLT")]
pub type CcSocuDflt = crate::Reg<cc_socu_dflt::CcSocuDfltSpec>;
#[doc = "no description available"]
pub mod cc_socu_dflt;
#[doc = "VENDOR_USAGE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`vendor_usage::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendor_usage::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_usage`] module"]
#[doc(alias = "VENDOR_USAGE")]
pub type VendorUsage = crate::Reg<vendor_usage::VendorUsageSpec>;
#[doc = "no description available"]
pub mod vendor_usage;
#[doc = "SECURE_BOOT_CFG (rw) register accessor: Secure boot configuration flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_boot_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_boot_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_boot_cfg`] module"]
#[doc(alias = "SECURE_BOOT_CFG")]
pub type SecureBootCfg = crate::Reg<secure_boot_cfg::SecureBootCfgSpec>;
#[doc = "Secure boot configuration flags."]
pub mod secure_boot_cfg;
#[doc = "PRINCE_BASE_ADDR (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_base_addr`] module"]
#[doc(alias = "PRINCE_BASE_ADDR")]
pub type PrinceBaseAddr = crate::Reg<prince_base_addr::PrinceBaseAddrSpec>;
#[doc = "no description available"]
pub mod prince_base_addr;
#[doc = "PRINCE_SR_0 (rw) register accessor: Region 0, sub-region enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_sr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_sr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_sr_0`] module"]
#[doc(alias = "PRINCE_SR_0")]
pub type PrinceSr0 = crate::Reg<prince_sr_0::PrinceSr0Spec>;
#[doc = "Region 0, sub-region enable"]
pub mod prince_sr_0;
#[doc = "PRINCE_SR_1 (rw) register accessor: Region 1, sub-region enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_sr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_sr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_sr_1`] module"]
#[doc(alias = "PRINCE_SR_1")]
pub type PrinceSr1 = crate::Reg<prince_sr_1::PrinceSr1Spec>;
#[doc = "Region 1, sub-region enable"]
pub mod prince_sr_1;
#[doc = "PRINCE_SR_2 (rw) register accessor: Region 2, sub-region enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_sr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_sr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_sr_2`] module"]
#[doc(alias = "PRINCE_SR_2")]
pub type PrinceSr2 = crate::Reg<prince_sr_2::PrinceSr2Spec>;
#[doc = "Region 2, sub-region enable"]
pub mod prince_sr_2;
#[doc = "XTAL_32KHZ_CAPABANK_TRIM (rw) register accessor: Xtal 32kHz capabank triming.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_32khz_capabank_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_32khz_capabank_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_32khz_capabank_trim`] module"]
#[doc(alias = "XTAL_32KHZ_CAPABANK_TRIM")]
pub type Xtal32khzCapabankTrim = crate::Reg<xtal_32khz_capabank_trim::Xtal32khzCapabankTrimSpec>;
#[doc = "Xtal 32kHz capabank triming."]
pub mod xtal_32khz_capabank_trim;
#[doc = "XTAL_16MHZ_CAPABANK_TRIM (rw) register accessor: Xtal 16MHz capabank triming.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_16mhz_capabank_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_16mhz_capabank_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_16mhz_capabank_trim`] module"]
#[doc(alias = "XTAL_16MHZ_CAPABANK_TRIM")]
pub type Xtal16mhzCapabankTrim = crate::Reg<xtal_16mhz_capabank_trim::Xtal16mhzCapabankTrimSpec>;
#[doc = "Xtal 16MHz capabank triming."]
pub mod xtal_16mhz_capabank_trim;
#[doc = "FLASH_REMAP_SIZE (rw) register accessor: This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB.\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_remap_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_remap_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_remap_size`] module"]
#[doc(alias = "FLASH_REMAP_SIZE")]
pub type FlashRemapSize = crate::Reg<flash_remap_size::FlashRemapSizeSpec>;
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
pub mod flash_remap_size;
#[doc = "FLASH_REMAP_OFFSET (rw) register accessor: This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB.\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_remap_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_remap_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_remap_offset`] module"]
#[doc(alias = "FLASH_REMAP_OFFSET")]
pub type FlashRemapOffset = crate::Reg<flash_remap_offset::FlashRemapOffsetSpec>;
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
pub mod flash_remap_offset;
#[doc = "ROTKH (rw) register accessor: ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`rotkh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotkh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rotkh`] module"]
#[doc(alias = "ROTKH")]
pub type Rotkh = crate::Reg<rotkh::RotkhSpec>;
#[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
pub mod rotkh;
#[doc = "CUSTOMER_DEFINED (rw) register accessor: Customer Defined (Programable through ROM API)\n\nYou can [`read`](crate::Reg::read) this register and get [`customer_defined::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer_defined::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer_defined`] module"]
#[doc(alias = "CUSTOMER_DEFINED")]
pub type CustomerDefined = crate::Reg<customer_defined::CustomerDefinedSpec>;
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST (rw) register accessor: SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha256_digest`] module"]
#[doc(alias = "SHA256_DIGEST")]
pub type Sha256Digest = crate::Reg<sha256_digest::Sha256DigestSpec>;
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
pub mod sha256_digest;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    enc_enable: EncEnable,
    mask_lsb: MaskLsb,
    mask_msb: MaskMsb,
    lock: Lock,
    iv_lsb0: IvLsb0,
    iv_msb0: IvMsb0,
    base_addr0: BaseAddr0,
    sr_enable0: SrEnable0,
    iv_lsb1: IvLsb1,
    iv_msb1: IvMsb1,
    base_addr1: BaseAddr1,
    sr_enable1: SrEnable1,
    iv_lsb2: IvLsb2,
    iv_msb2: IvMsb2,
    base_addr2: BaseAddr2,
    sr_enable2: SrEnable2,
    _reserved16: [u8; 0x50],
    err: Err,
}
impl RegisterBlock {
    #[doc = "0x00 - Encryption Enable register"]
    #[inline(always)]
    pub const fn enc_enable(&self) -> &EncEnable {
        &self.enc_enable
    }
    #[doc = "0x04 - Data Mask register, 32 Least Significant Bits"]
    #[inline(always)]
    pub const fn mask_lsb(&self) -> &MaskLsb {
        &self.mask_lsb
    }
    #[doc = "0x08 - Data Mask register, 32 Most Significant Bits"]
    #[inline(always)]
    pub const fn mask_msb(&self) -> &MaskMsb {
        &self.mask_msb
    }
    #[doc = "0x0c - Lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x10 - Initial Vector register for region 0, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb0(&self) -> &IvLsb0 {
        &self.iv_lsb0
    }
    #[doc = "0x14 - Initial Vector register for region 0, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb0(&self) -> &IvMsb0 {
        &self.iv_msb0
    }
    #[doc = "0x18 - Base Address for region 0 register"]
    #[inline(always)]
    pub const fn base_addr0(&self) -> &BaseAddr0 {
        &self.base_addr0
    }
    #[doc = "0x1c - Sub-Region Enable register for region 0"]
    #[inline(always)]
    pub const fn sr_enable0(&self) -> &SrEnable0 {
        &self.sr_enable0
    }
    #[doc = "0x20 - Initial Vector register for region 1, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb1(&self) -> &IvLsb1 {
        &self.iv_lsb1
    }
    #[doc = "0x24 - Initial Vector register for region 1, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb1(&self) -> &IvMsb1 {
        &self.iv_msb1
    }
    #[doc = "0x28 - Base Address for region 1 register"]
    #[inline(always)]
    pub const fn base_addr1(&self) -> &BaseAddr1 {
        &self.base_addr1
    }
    #[doc = "0x2c - Sub-Region Enable register for region 1"]
    #[inline(always)]
    pub const fn sr_enable1(&self) -> &SrEnable1 {
        &self.sr_enable1
    }
    #[doc = "0x30 - Initial Vector register for region 2, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb2(&self) -> &IvLsb2 {
        &self.iv_lsb2
    }
    #[doc = "0x34 - Initial Vector register for region 2, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb2(&self) -> &IvMsb2 {
        &self.iv_msb2
    }
    #[doc = "0x38 - Base Address for region 2 register"]
    #[inline(always)]
    pub const fn base_addr2(&self) -> &BaseAddr2 {
        &self.base_addr2
    }
    #[doc = "0x3c - Sub-Region Enable register for region 2"]
    #[inline(always)]
    pub const fn sr_enable2(&self) -> &SrEnable2 {
        &self.sr_enable2
    }
    #[doc = "0x90 - Error status register"]
    #[inline(always)]
    pub const fn err(&self) -> &Err {
        &self.err
    }
}
#[doc = "ENC_ENABLE (rw) register accessor: Encryption Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enc_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc_enable`] module"]
#[doc(alias = "ENC_ENABLE")]
pub type EncEnable = crate::Reg<enc_enable::EncEnableSpec>;
#[doc = "Encryption Enable register"]
pub mod enc_enable;
#[doc = "MASK_LSB (w) register accessor: Data Mask register, 32 Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_lsb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_lsb`] module"]
#[doc(alias = "MASK_LSB")]
pub type MaskLsb = crate::Reg<mask_lsb::MaskLsbSpec>;
#[doc = "Data Mask register, 32 Least Significant Bits"]
pub mod mask_lsb;
#[doc = "MASK_MSB (w) register accessor: Data Mask register, 32 Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_msb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_msb`] module"]
#[doc(alias = "MASK_MSB")]
pub type MaskMsb = crate::Reg<mask_msb::MaskMsbSpec>;
#[doc = "Data Mask register, 32 Most Significant Bits"]
pub mod mask_msb;
#[doc = "LOCK (rw) register accessor: Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Lock register"]
pub mod lock;
#[doc = "IV_LSB0 (w) register accessor: Initial Vector register for region 0, Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_lsb0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_lsb0`] module"]
#[doc(alias = "IV_LSB0")]
pub type IvLsb0 = crate::Reg<iv_lsb0::IvLsb0Spec>;
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
pub mod iv_lsb0;
#[doc = "IV_MSB0 (w) register accessor: Initial Vector register for region 0, Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_msb0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_msb0`] module"]
#[doc(alias = "IV_MSB0")]
pub type IvMsb0 = crate::Reg<iv_msb0::IvMsb0Spec>;
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
pub mod iv_msb0;
#[doc = "BASE_ADDR0 (rw) register accessor: Base Address for region 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`base_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_addr0`] module"]
#[doc(alias = "BASE_ADDR0")]
pub type BaseAddr0 = crate::Reg<base_addr0::BaseAddr0Spec>;
#[doc = "Base Address for region 0 register"]
pub mod base_addr0;
#[doc = "SR_ENABLE0 (rw) register accessor: Sub-Region Enable register for region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_enable0`] module"]
#[doc(alias = "SR_ENABLE0")]
pub type SrEnable0 = crate::Reg<sr_enable0::SrEnable0Spec>;
#[doc = "Sub-Region Enable register for region 0"]
pub mod sr_enable0;
#[doc = "IV_LSB1 (w) register accessor: Initial Vector register for region 1, Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_lsb1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_lsb1`] module"]
#[doc(alias = "IV_LSB1")]
pub type IvLsb1 = crate::Reg<iv_lsb1::IvLsb1Spec>;
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
pub mod iv_lsb1;
#[doc = "IV_MSB1 (w) register accessor: Initial Vector register for region 1, Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_msb1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_msb1`] module"]
#[doc(alias = "IV_MSB1")]
pub type IvMsb1 = crate::Reg<iv_msb1::IvMsb1Spec>;
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
pub mod iv_msb1;
#[doc = "BASE_ADDR1 (rw) register accessor: Base Address for region 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`base_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_addr1`] module"]
#[doc(alias = "BASE_ADDR1")]
pub type BaseAddr1 = crate::Reg<base_addr1::BaseAddr1Spec>;
#[doc = "Base Address for region 1 register"]
pub mod base_addr1;
#[doc = "SR_ENABLE1 (rw) register accessor: Sub-Region Enable register for region 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_enable1`] module"]
#[doc(alias = "SR_ENABLE1")]
pub type SrEnable1 = crate::Reg<sr_enable1::SrEnable1Spec>;
#[doc = "Sub-Region Enable register for region 1"]
pub mod sr_enable1;
#[doc = "IV_LSB2 (w) register accessor: Initial Vector register for region 2, Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_lsb2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_lsb2`] module"]
#[doc(alias = "IV_LSB2")]
pub type IvLsb2 = crate::Reg<iv_lsb2::IvLsb2Spec>;
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
pub mod iv_lsb2;
#[doc = "IV_MSB2 (w) register accessor: Initial Vector register for region 2, Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_msb2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_msb2`] module"]
#[doc(alias = "IV_MSB2")]
pub type IvMsb2 = crate::Reg<iv_msb2::IvMsb2Spec>;
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
pub mod iv_msb2;
#[doc = "BASE_ADDR2 (rw) register accessor: Base Address for region 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`base_addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_addr2`] module"]
#[doc(alias = "BASE_ADDR2")]
pub type BaseAddr2 = crate::Reg<base_addr2::BaseAddr2Spec>;
#[doc = "Base Address for region 2 register"]
pub mod base_addr2;
#[doc = "SR_ENABLE2 (rw) register accessor: Sub-Region Enable register for region 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_enable2`] module"]
#[doc(alias = "SR_ENABLE2")]
pub type SrEnable2 = crate::Reg<sr_enable2::SrEnable2Spec>;
#[doc = "Sub-Region Enable register for region 2"]
pub mod sr_enable2;
#[doc = "ERR (rw) register accessor: Error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`] module"]
#[doc(alias = "ERR")]
pub type Err = crate::Reg<err::ErrSpec>;
#[doc = "Error status register"]
pub mod err;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    cfg: Cfg,
    status: Status,
    _reserved2: [u8; 0xd0],
    int_clr_enable: IntClrEnable,
    int_set_enable: IntSetEnable,
    int_status: IntStatus,
    int_enable: IntEnable,
    int_clr_status: IntClrStatus,
    int_set_status: IntSetStatus,
}
impl RegisterBlock {
    #[doc = "0x300 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x304 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x3d8 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn int_clr_enable(&self) -> &IntClrEnable {
        &self.int_clr_enable
    }
    #[doc = "0x3dc - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn int_set_enable(&self) -> &IntSetEnable {
        &self.int_set_enable
    }
    #[doc = "0x3e0 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x3e4 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_enable(&self) -> &IntEnable {
        &self.int_enable
    }
    #[doc = "0x3e8 - Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn int_clr_status(&self) -> &IntClrStatus {
        &self.int_clr_status
    }
    #[doc = "0x3ec - Interrupt Status set"]
    #[inline(always)]
    pub const fn int_set_status(&self) -> &IntSetStatus {
        &self.int_set_status
    }
}
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INT_CLR_ENABLE (w) register accessor: Interrupt Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_enable`] module"]
#[doc(alias = "INT_CLR_ENABLE")]
pub type IntClrEnable = crate::Reg<int_clr_enable::IntClrEnableSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod int_clr_enable;
#[doc = "INT_SET_ENABLE (w) register accessor: Interrupt Enable Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set_enable`] module"]
#[doc(alias = "INT_SET_ENABLE")]
pub type IntSetEnable = crate::Reg<int_set_enable::IntSetEnableSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod int_set_enable;
#[doc = "INT_STATUS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`] module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "INT_ENABLE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enable`] module"]
#[doc(alias = "INT_ENABLE")]
pub type IntEnable = crate::Reg<int_enable::IntEnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod int_enable;
#[doc = "INT_CLR_STATUS (w) register accessor: Interrupt Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_status`] module"]
#[doc(alias = "INT_CLR_STATUS")]
pub type IntClrStatus = crate::Reg<int_clr_status::IntClrStatusSpec>;
#[doc = "Interrupt Status Clear Register"]
pub mod int_clr_status;
#[doc = "INT_SET_STATUS (w) register accessor: Interrupt Status set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set_status`] module"]
#[doc(alias = "INT_SET_STATUS")]
pub type IntSetStatus = crate::Reg<int_set_status::IntSetStatusSpec>;
#[doc = "Interrupt Status set"]
pub mod int_set_status;

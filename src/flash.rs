#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: Cmd,
    event: Event,
    _reserved2: [u8; 0x08],
    starta: Starta,
    stopa: Stopa,
    _reserved4: [u8; 0x68],
    dataw: [Dataw; 4],
    _reserved5: [u8; 0x0f48],
    int_clr_enable: IntClrEnable,
    int_set_enable: IntSetEnable,
    int_status: IntStatus,
    int_enable: IntEnable,
    int_clr_status: IntClrStatus,
    int_set_status: IntSetStatus,
    _reserved11: [u8; 0x0c],
    module_id: ModuleId,
}
impl RegisterBlock {
    #[doc = "0x00 - command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x04 - event register"]
    #[inline(always)]
    pub const fn event(&self) -> &Event {
        &self.event
    }
    #[doc = "0x10 - start (or only) address for next flash command"]
    #[inline(always)]
    pub const fn starta(&self) -> &Starta {
        &self.starta
    }
    #[doc = "0x14 - end address for next flash command, if command operates on address ranges"]
    #[inline(always)]
    pub const fn stopa(&self) -> &Stopa {
        &self.stopa
    }
    #[doc = "0x80..0x90 - data register, word 0-7; Memory data, or command parameter, or command result."]
    #[inline(always)]
    pub const fn dataw(&self, n: usize) -> &Dataw {
        &self.dataw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - data register, word 0-7; Memory data, or command parameter, or command result."]
    #[inline(always)]
    pub fn dataw_iter(&self) -> impl Iterator<Item = &Dataw> {
        self.dataw.iter()
    }
    #[doc = "0xfd8 - Clear interrupt enable bits"]
    #[inline(always)]
    pub const fn int_clr_enable(&self) -> &IntClrEnable {
        &self.int_clr_enable
    }
    #[doc = "0xfdc - Set interrupt enable bits"]
    #[inline(always)]
    pub const fn int_set_enable(&self) -> &IntSetEnable {
        &self.int_set_enable
    }
    #[doc = "0xfe0 - Interrupt status bits"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0xfe4 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_enable(&self) -> &IntEnable {
        &self.int_enable
    }
    #[doc = "0xfe8 - Clear interrupt status bits"]
    #[inline(always)]
    pub const fn int_clr_status(&self) -> &IntClrStatus {
        &self.int_clr_status
    }
    #[doc = "0xfec - Set interrupt status bits"]
    #[inline(always)]
    pub const fn int_set_status(&self) -> &IntSetStatus {
        &self.int_set_status
    }
    #[doc = "0xffc - Controller+Memory module identification"]
    #[inline(always)]
    pub const fn module_id(&self) -> &ModuleId {
        &self.module_id
    }
}
#[doc = "CMD (w) register accessor: command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "command register"]
pub mod cmd;
#[doc = "EVENT (w) register accessor: event register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event`] module"]
#[doc(alias = "EVENT")]
pub type Event = crate::Reg<event::EventSpec>;
#[doc = "event register"]
pub mod event;
#[doc = "STARTA (rw) register accessor: start (or only) address for next flash command\n\nYou can [`read`](crate::Reg::read) this register and get [`starta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starta`] module"]
#[doc(alias = "STARTA")]
pub type Starta = crate::Reg<starta::StartaSpec>;
#[doc = "start (or only) address for next flash command"]
pub mod starta;
#[doc = "STOPA (rw) register accessor: end address for next flash command, if command operates on address ranges\n\nYou can [`read`](crate::Reg::read) this register and get [`stopa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stopa`] module"]
#[doc(alias = "STOPA")]
pub type Stopa = crate::Reg<stopa::StopaSpec>;
#[doc = "end address for next flash command, if command operates on address ranges"]
pub mod stopa;
#[doc = "DATAW (rw) register accessor: data register, word 0-7; Memory data, or command parameter, or command result.\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataw`] module"]
#[doc(alias = "DATAW")]
pub type Dataw = crate::Reg<dataw::DatawSpec>;
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
pub mod dataw;
#[doc = "INT_CLR_ENABLE (w) register accessor: Clear interrupt enable bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_enable`] module"]
#[doc(alias = "INT_CLR_ENABLE")]
pub type IntClrEnable = crate::Reg<int_clr_enable::IntClrEnableSpec>;
#[doc = "Clear interrupt enable bits"]
pub mod int_clr_enable;
#[doc = "INT_SET_ENABLE (w) register accessor: Set interrupt enable bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set_enable`] module"]
#[doc(alias = "INT_SET_ENABLE")]
pub type IntSetEnable = crate::Reg<int_set_enable::IntSetEnableSpec>;
#[doc = "Set interrupt enable bits"]
pub mod int_set_enable;
#[doc = "INT_STATUS (rw) register accessor: Interrupt status bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`] module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt status bits"]
pub mod int_status;
#[doc = "INT_ENABLE (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enable`] module"]
#[doc(alias = "INT_ENABLE")]
pub type IntEnable = crate::Reg<int_enable::IntEnableSpec>;
#[doc = "Interrupt enable bits"]
pub mod int_enable;
#[doc = "INT_CLR_STATUS (w) register accessor: Clear interrupt status bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_status`] module"]
#[doc(alias = "INT_CLR_STATUS")]
pub type IntClrStatus = crate::Reg<int_clr_status::IntClrStatusSpec>;
#[doc = "Clear interrupt status bits"]
pub mod int_clr_status;
#[doc = "INT_SET_STATUS (w) register accessor: Set interrupt status bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set_status`] module"]
#[doc(alias = "INT_SET_STATUS")]
pub type IntSetStatus = crate::Reg<int_set_status::IntSetStatusSpec>;
#[doc = "Set interrupt status bits"]
pub mod int_set_status;
#[doc = "MODULE_ID (r) register accessor: Controller+Memory module identification\n\nYou can [`read`](crate::Reg::read) this register and get [`module_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@module_id`] module"]
#[doc(alias = "MODULE_ID")]
pub type ModuleId = crate::Reg<module_id::ModuleIdSpec>;
#[doc = "Controller+Memory module identification"]
pub mod module_id;

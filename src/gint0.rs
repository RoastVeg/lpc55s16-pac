#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x1c],
    port_pol: [PortPol; 2],
    _reserved2: [u8; 0x18],
    port_ena: [PortEna; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO grouped interrupt control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x20..0x28 - GPIO grouped interrupt port 0 polarity register"]
    #[inline(always)]
    pub const fn port_pol(&self, n: usize) -> &PortPol {
        &self.port_pol[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - GPIO grouped interrupt port 0 polarity register"]
    #[inline(always)]
    pub fn port_pol_iter(&self) -> impl Iterator<Item = &PortPol> {
        self.port_pol.iter()
    }
    #[doc = "0x40..0x48 - GPIO grouped interrupt port 0 enable register"]
    #[inline(always)]
    pub const fn port_ena(&self, n: usize) -> &PortEna {
        &self.port_ena[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - GPIO grouped interrupt port 0 enable register"]
    #[inline(always)]
    pub fn port_ena_iter(&self) -> impl Iterator<Item = &PortEna> {
        self.port_ena.iter()
    }
}
#[doc = "CTRL (rw) register accessor: GPIO grouped interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "GPIO grouped interrupt control register"]
pub mod ctrl;
#[doc = "PORT_POL (rw) register accessor: GPIO grouped interrupt port 0 polarity register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_pol`] module"]
#[doc(alias = "PORT_POL")]
pub type PortPol = crate::Reg<port_pol::PortPolSpec>;
#[doc = "GPIO grouped interrupt port 0 polarity register"]
pub mod port_pol;
#[doc = "PORT_ENA (rw) register accessor: GPIO grouped interrupt port 0 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_ena`] module"]
#[doc(alias = "PORT_ENA")]
pub type PortEna = crate::Reg<port_ena::PortEnaSpec>;
#[doc = "GPIO grouped interrupt port 0 enable register"]
pub mod port_ena;

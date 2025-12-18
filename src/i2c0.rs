#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    cfg: Cfg,
    stat: Stat,
    intenset: Intenset,
    intenclr: Intenclr,
    timeout: Timeout,
    clkdiv: Clkdiv,
    intstat: Intstat,
    _reserved7: [u8; 0x04],
    mstctl: Mstctl,
    msttime: Msttime,
    mstdat: Mstdat,
    _reserved10: [u8; 0x14],
    slvctl: Slvctl,
    slvdat: Slvdat,
    slvadr0: Slvadr0,
    slvadr1: Slvadr1,
    slvadr2: Slvadr2,
    slvadr3: Slvadr3,
    slvqual0: Slvqual0,
    _reserved17: [u8; 0x24],
    monrxdat: Monrxdat,
    _reserved18: [u8; 0x0778],
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x800 - Configuration for shared functions."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x804 - Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x808 - Interrupt Enable Set and read register."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x80c - Interrupt Enable Clear register."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x810 - Time-out value register."]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x814 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x818 - Interrupt Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x820 - Master control register."]
    #[inline(always)]
    pub const fn mstctl(&self) -> &Mstctl {
        &self.mstctl
    }
    #[doc = "0x824 - Master timing configuration."]
    #[inline(always)]
    pub const fn msttime(&self) -> &Msttime {
        &self.msttime
    }
    #[doc = "0x828 - Combined Master receiver and transmitter data register."]
    #[inline(always)]
    pub const fn mstdat(&self) -> &Mstdat {
        &self.mstdat
    }
    #[doc = "0x840 - Slave control register."]
    #[inline(always)]
    pub const fn slvctl(&self) -> &Slvctl {
        &self.slvctl
    }
    #[doc = "0x844 - Combined Slave receiver and transmitter data register."]
    #[inline(always)]
    pub const fn slvdat(&self) -> &Slvdat {
        &self.slvdat
    }
    #[doc = "0x848 - Slave address register."]
    #[inline(always)]
    pub const fn slvadr0(&self) -> &Slvadr0 {
        &self.slvadr0
    }
    #[doc = "0x84c - Slave address register."]
    #[inline(always)]
    pub const fn slvadr1(&self) -> &Slvadr1 {
        &self.slvadr1
    }
    #[doc = "0x850 - Slave address register."]
    #[inline(always)]
    pub const fn slvadr2(&self) -> &Slvadr2 {
        &self.slvadr2
    }
    #[doc = "0x854 - Slave address register."]
    #[inline(always)]
    pub const fn slvadr3(&self) -> &Slvadr3 {
        &self.slvadr3
    }
    #[doc = "0x858 - Slave Qualification for address 0."]
    #[inline(always)]
    pub const fn slvqual0(&self) -> &Slvqual0 {
        &self.slvqual0
    }
    #[doc = "0x880 - Monitor receiver data register."]
    #[inline(always)]
    pub const fn monrxdat(&self) -> &Monrxdat {
        &self.monrxdat
    }
    #[doc = "0xffc - Peripheral identification register."]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CFG (rw) register accessor: Configuration for shared functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "STAT (rw) register accessor: Status register for Master, Slave, and Monitor functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: Interrupt Enable Clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "TIMEOUT (rw) register accessor: Time-out value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`] module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "CLKDIV (rw) register accessor: Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub mod clkdiv;
#[doc = "INTSTAT (r) register accessor: Interrupt Status register for Master, Slave, and Monitor functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "MSTCTL (rw) register accessor: Master control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstctl`] module"]
#[doc(alias = "MSTCTL")]
pub type Mstctl = crate::Reg<mstctl::MstctlSpec>;
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "MSTTIME (rw) register accessor: Master timing configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`msttime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msttime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msttime`] module"]
#[doc(alias = "MSTTIME")]
pub type Msttime = crate::Reg<msttime::MsttimeSpec>;
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "MSTDAT (rw) register accessor: Combined Master receiver and transmitter data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstdat`] module"]
#[doc(alias = "MSTDAT")]
pub type Mstdat = crate::Reg<mstdat::MstdatSpec>;
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "SLVCTL (rw) register accessor: Slave control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvctl`] module"]
#[doc(alias = "SLVCTL")]
pub type Slvctl = crate::Reg<slvctl::SlvctlSpec>;
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "SLVDAT (rw) register accessor: Combined Slave receiver and transmitter data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvdat`] module"]
#[doc(alias = "SLVDAT")]
pub type Slvdat = crate::Reg<slvdat::SlvdatSpec>;
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "SLVADR0 (rw) register accessor: Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvadr0`] module"]
#[doc(alias = "SLVADR0")]
pub type Slvadr0 = crate::Reg<slvadr0::Slvadr0Spec>;
#[doc = "Slave address register."]
pub mod slvadr0;
#[doc = "SLVADR1 (rw) register accessor: Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvadr1`] module"]
#[doc(alias = "SLVADR1")]
pub type Slvadr1 = crate::Reg<slvadr1::Slvadr1Spec>;
#[doc = "Slave address register."]
pub mod slvadr1;
#[doc = "SLVADR2 (rw) register accessor: Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvadr2`] module"]
#[doc(alias = "SLVADR2")]
pub type Slvadr2 = crate::Reg<slvadr2::Slvadr2Spec>;
#[doc = "Slave address register."]
pub mod slvadr2;
#[doc = "SLVADR3 (rw) register accessor: Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvadr3`] module"]
#[doc(alias = "SLVADR3")]
pub type Slvadr3 = crate::Reg<slvadr3::Slvadr3Spec>;
#[doc = "Slave address register."]
pub mod slvadr3;
#[doc = "SLVQUAL0 (rw) register accessor: Slave Qualification for address 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvqual0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvqual0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvqual0`] module"]
#[doc(alias = "SLVQUAL0")]
pub type Slvqual0 = crate::Reg<slvqual0::Slvqual0Spec>;
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "MONRXDAT (r) register accessor: Monitor receiver data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`monrxdat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monrxdat`] module"]
#[doc(alias = "MONRXDAT")]
pub type Monrxdat = crate::Reg<monrxdat::MonrxdatSpec>;
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
#[doc = "ID (r) register accessor: Peripheral identification register.\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Peripheral identification register."]
pub mod id;

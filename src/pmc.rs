#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    status: Status,
    resetctrl: Resetctrl,
    _reserved2: [u8; 0x04],
    dcdc0: Dcdc0,
    dcdc1: Dcdc1,
    _reserved4: [u8; 0x04],
    ldopmu: Ldopmu,
    _reserved5: [u8; 0x10],
    bodvbat: Bodvbat,
    _reserved6: [u8; 0x0c],
    reffastwkup: Reffastwkup,
    _reserved7: [u8; 0x08],
    xtal32k: Xtal32k,
    comp: Comp,
    _reserved9: [u8; 0x10],
    wakeupioctrl: Wakeupioctrl,
    wakeiocause: Wakeiocause,
    _reserved11: [u8; 0x08],
    statusclk: Statusclk,
    _reserved12: [u8; 0x0c],
    aoreg1: Aoreg1,
    _reserved13: [u8; 0x08],
    miscctrl: Miscctrl,
    _reserved14: [u8; 0x04],
    rtcosc32k: Rtcosc32k,
    ostimer: Ostimer,
    _reserved16: [u8; 0x18],
    pdruncfg0: Pdruncfg0,
    _reserved17: [u8; 0x04],
    pdruncfgset0: Pdruncfgset0,
    _reserved18: [u8; 0x04],
    pdruncfgclr0: Pdruncfgclr0,
    _reserved19: [u8; 0x08],
    sramctrl: Sramctrl,
}
impl RegisterBlock {
    #[doc = "0x04 - Power Management Controller FSM (Finite State Machines) status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn resetctrl(&self) -> &Resetctrl {
        &self.resetctrl
    }
    #[doc = "0x10 - DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn dcdc0(&self) -> &Dcdc0 {
        &self.dcdc0
    }
    #[doc = "0x14 - DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn dcdc1(&self) -> &Dcdc1 {
        &self.dcdc1
    }
    #[doc = "0x1c - Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn ldopmu(&self) -> &Ldopmu {
        &self.ldopmu
    }
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn bodvbat(&self) -> &Bodvbat {
        &self.bodvbat
    }
    #[doc = "0x40 - Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
    #[inline(always)]
    pub const fn reffastwkup(&self) -> &Reffastwkup {
        &self.reffastwkup
    }
    #[doc = "0x4c - 32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn xtal32k(&self) -> &Xtal32k {
        &self.xtal32k
    }
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn comp(&self) -> &Comp {
        &self.comp
    }
    #[doc = "0x64 - Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn wakeupioctrl(&self) -> &Wakeupioctrl {
        &self.wakeupioctrl
    }
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    #[inline(always)]
    pub const fn wakeiocause(&self) -> &Wakeiocause {
        &self.wakeiocause
    }
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn statusclk(&self) -> &Statusclk {
        &self.statusclk
    }
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn aoreg1(&self) -> &Aoreg1 {
        &self.aoreg1
    }
    #[doc = "0x90 - Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn miscctrl(&self) -> &Miscctrl {
        &self.miscctrl
    }
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn rtcosc32k(&self) -> &Rtcosc32k {
        &self.rtcosc32k
    }
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    #[inline(always)]
    pub const fn ostimer(&self) -> &Ostimer {
        &self.ostimer
    }
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfg0(&self) -> &Pdruncfg0 {
        &self.pdruncfg0
    }
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfgset0(&self) -> &Pdruncfgset0 {
        &self.pdruncfgset0
    }
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn pdruncfgclr0(&self) -> &Pdruncfgclr0 {
        &self.pdruncfgclr0
    }
    #[doc = "0xd4 - All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
    #[inline(always)]
    pub const fn sramctrl(&self) -> &Sramctrl {
        &self.sramctrl
    }
}
#[doc = "STATUS (rw) register accessor: Power Management Controller FSM (Finite State Machines) status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Power Management Controller FSM (Finite State Machines) status"]
pub mod status;
#[doc = "RESETCTRL (rw) register accessor: Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`resetctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetctrl`] module"]
#[doc(alias = "RESETCTRL")]
pub type Resetctrl = crate::Reg<resetctrl::ResetctrlSpec>;
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod resetctrl;
#[doc = "DCDC0 (rw) register accessor: DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc0`] module"]
#[doc(alias = "DCDC0")]
pub type Dcdc0 = crate::Reg<dcdc0::Dcdc0Spec>;
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod dcdc0;
#[doc = "DCDC1 (rw) register accessor: DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc1`] module"]
#[doc(alias = "DCDC1")]
pub type Dcdc1 = crate::Reg<dcdc1::Dcdc1Spec>;
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod dcdc1;
#[doc = "LDOPMU (rw) register accessor: Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ldopmu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldopmu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldopmu`] module"]
#[doc(alias = "LDOPMU")]
pub type Ldopmu = crate::Reg<ldopmu::LdopmuSpec>;
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod ldopmu;
#[doc = "BODVBAT (rw) register accessor: VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bodvbat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodvbat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodvbat`] module"]
#[doc(alias = "BODVBAT")]
pub type Bodvbat = crate::Reg<bodvbat::BodvbatSpec>;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod bodvbat;
#[doc = "REFFASTWKUP (rw) register accessor: Analog References fast wake-up Control register \\[Reset by: PoR\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`reffastwkup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reffastwkup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reffastwkup`] module"]
#[doc(alias = "REFFASTWKUP")]
pub type Reffastwkup = crate::Reg<reffastwkup::ReffastwkupSpec>;
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
pub mod reffastwkup;
#[doc = "XTAL32K (rw) register accessor: 32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k`] module"]
#[doc(alias = "XTAL32K")]
pub type Xtal32k = crate::Reg<xtal32k::Xtal32kSpec>;
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod xtal32k;
#[doc = "COMP (rw) register accessor: Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp`] module"]
#[doc(alias = "COMP")]
pub type Comp = crate::Reg<comp::CompSpec>;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod comp;
#[doc = "WAKEUPIOCTRL (rw) register accessor: Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeupioctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeupioctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeupioctrl`] module"]
#[doc(alias = "WAKEUPIOCTRL")]
pub type Wakeupioctrl = crate::Reg<wakeupioctrl::WakeupioctrlSpec>;
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod wakeupioctrl;
#[doc = "WAKEIOCAUSE (rw) register accessor: Allows to identify the Wake-up I/O source from Deep Power Down mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeiocause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeiocause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeiocause`] module"]
#[doc(alias = "WAKEIOCAUSE")]
pub type Wakeiocause = crate::Reg<wakeiocause::WakeiocauseSpec>;
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub mod wakeiocause;
#[doc = "STATUSCLK (rw) register accessor: FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`statusclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusclk`] module"]
#[doc(alias = "STATUSCLK")]
pub type Statusclk = crate::Reg<statusclk::StatusclkSpec>;
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod statusclk;
#[doc = "AOREG1 (rw) register accessor: General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`aoreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoreg1`] module"]
#[doc(alias = "AOREG1")]
pub type Aoreg1 = crate::Reg<aoreg1::Aoreg1Spec>;
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod aoreg1;
#[doc = "MISCCTRL (rw) register accessor: Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`miscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miscctrl`] module"]
#[doc(alias = "MISCCTRL")]
pub type Miscctrl = crate::Reg<miscctrl::MiscctrlSpec>;
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod miscctrl;
#[doc = "RTCOSC32K (rw) register accessor: RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcosc32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcosc32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcosc32k`] module"]
#[doc(alias = "RTCOSC32K")]
pub type Rtcosc32k = crate::Reg<rtcosc32k::Rtcosc32kSpec>;
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod rtcosc32k;
#[doc = "OSTIMER (rw) register accessor: OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ostimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostimer`] module"]
#[doc(alias = "OSTIMER")]
pub type Ostimer = crate::Reg<ostimer::OstimerSpec>;
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod ostimer;
#[doc = "PDRUNCFG0 (rw) register accessor: Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg0`] module"]
#[doc(alias = "PDRUNCFG0")]
pub type Pdruncfg0 = crate::Reg<pdruncfg0::Pdruncfg0Spec>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfg0;
#[doc = "PDRUNCFGSET0 (w) register accessor: Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfgset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfgset0`] module"]
#[doc(alias = "PDRUNCFGSET0")]
pub type Pdruncfgset0 = crate::Reg<pdruncfgset0::Pdruncfgset0Spec>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "PDRUNCFGCLR0 (w) register accessor: Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfgclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfgclr0`] module"]
#[doc(alias = "PDRUNCFGCLR0")]
pub type Pdruncfgclr0 = crate::Reg<pdruncfgclr0::Pdruncfgclr0Spec>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
#[doc = "SRAMCTRL (rw) register accessor: All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramctrl`] module"]
#[doc(alias = "SRAMCTRL")]
pub type Sramctrl = crate::Reg<sramctrl::SramctrlSpec>;
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
pub mod sramctrl;

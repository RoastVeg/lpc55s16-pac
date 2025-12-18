#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    analog_ctrl_cfg: AnalogCtrlCfg,
    analog_ctrl_status: AnalogCtrlStatus,
    _reserved2: [u8; 0x04],
    freq_me_ctrl: FreqMeCtrl,
    fro192m_ctrl: Fro192mCtrl,
    fro192m_status: Fro192mStatus,
    adc_ctrl: AdcCtrl,
    _reserved6: [u8; 0x04],
    xo32m_ctrl: Xo32mCtrl,
    xo32m_status: Xo32mStatus,
    _reserved8: [u8; 0x08],
    bod_dcdc_int_ctrl: BodDcdcIntCtrl,
    bod_dcdc_int_status: BodDcdcIntStatus,
    _reserved10: [u8; 0x08],
    ringo0_ctrl: Ringo0Ctrl,
    ringo1_ctrl: Ringo1Ctrl,
    ringo2_ctrl: Ringo2Ctrl,
    _reserved13: [u8; 0x64],
    ldo_xo32m: LdoXo32m,
    aux_bias: AuxBias,
    _reserved15: [u8; 0x40],
    dummy_ctrl: DummyCtrl,
    _reserved16: [u8; 0x04],
    usbhs_phy_ctrl: UsbhsPhyCtrl,
    usbhs_phy_trim: UsbhsPhyTrim,
}
impl RegisterBlock {
    #[doc = "0x00 - Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
    #[inline(always)]
    pub const fn analog_ctrl_cfg(&self) -> &AnalogCtrlCfg {
        &self.analog_ctrl_cfg
    }
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers"]
    #[inline(always)]
    pub const fn analog_ctrl_status(&self) -> &AnalogCtrlStatus {
        &self.analog_ctrl_status
    }
    #[doc = "0x0c - Frequency Measure function control register"]
    #[inline(always)]
    pub const fn freq_me_ctrl(&self) -> &FreqMeCtrl {
        &self.freq_me_ctrl
    }
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register"]
    #[inline(always)]
    pub const fn fro192m_ctrl(&self) -> &Fro192mCtrl {
        &self.fro192m_ctrl
    }
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register"]
    #[inline(always)]
    pub const fn fro192m_status(&self) -> &Fro192mStatus {
        &self.fro192m_status
    }
    #[doc = "0x18 - General Purpose ADC VBAT Divider branch control"]
    #[inline(always)]
    pub const fn adc_ctrl(&self) -> &AdcCtrl {
        &self.adc_ctrl
    }
    #[doc = "0x20 - High speed Crystal Oscillator Control register"]
    #[inline(always)]
    pub const fn xo32m_ctrl(&self) -> &Xo32mCtrl {
        &self.xo32m_ctrl
    }
    #[doc = "0x24 - High speed Crystal Oscillator Status register"]
    #[inline(always)]
    pub const fn xo32m_status(&self) -> &Xo32mStatus {
        &self.xo32m_status
    }
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    #[inline(always)]
    pub const fn bod_dcdc_int_ctrl(&self) -> &BodDcdcIntCtrl {
        &self.bod_dcdc_int_ctrl
    }
    #[doc = "0x34 - BoDs & DCDC interrupts status register"]
    #[inline(always)]
    pub const fn bod_dcdc_int_status(&self) -> &BodDcdcIntStatus {
        &self.bod_dcdc_int_status
    }
    #[doc = "0x40 - First Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo0_ctrl(&self) -> &Ringo0Ctrl {
        &self.ringo0_ctrl
    }
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo1_ctrl(&self) -> &Ringo1Ctrl {
        &self.ringo1_ctrl
    }
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo2_ctrl(&self) -> &Ringo2Ctrl {
        &self.ringo2_ctrl
    }
    #[doc = "0xb0 - High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
    #[inline(always)]
    pub const fn ldo_xo32m(&self) -> &LdoXo32m {
        &self.ldo_xo32m
    }
    #[doc = "0xb4 - AUX_BIAS"]
    #[inline(always)]
    pub const fn aux_bias(&self) -> &AuxBias {
        &self.aux_bias
    }
    #[doc = "0xf8 - Dummy Control bus to analog modules"]
    #[inline(always)]
    pub const fn dummy_ctrl(&self) -> &DummyCtrl {
        &self.dummy_ctrl
    }
    #[doc = "0x100 - USB High Speed Phy Control"]
    #[inline(always)]
    pub const fn usbhs_phy_ctrl(&self) -> &UsbhsPhyCtrl {
        &self.usbhs_phy_ctrl
    }
    #[doc = "0x104 - USB High Speed Phy Trim values"]
    #[inline(always)]
    pub const fn usbhs_phy_trim(&self) -> &UsbhsPhyTrim {
        &self.usbhs_phy_trim
    }
}
#[doc = "ANALOG_CTRL_CFG (rw) register accessor: Various Analog blocks configuration (like FRO 192MHz trimmings source ...)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctrl_cfg`] module"]
#[doc(alias = "ANALOG_CTRL_CFG")]
pub type AnalogCtrlCfg = crate::Reg<analog_ctrl_cfg::AnalogCtrlCfgSpec>;
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
pub mod analog_ctrl_cfg;
#[doc = "ANALOG_CTRL_STATUS (r) register accessor: Analog Macroblock Identity registers, Flash Status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_ctrl_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_ctrl_status`] module"]
#[doc(alias = "ANALOG_CTRL_STATUS")]
pub type AnalogCtrlStatus = crate::Reg<analog_ctrl_status::AnalogCtrlStatusSpec>;
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
pub mod analog_ctrl_status;
#[doc = "FREQ_ME_CTRL (rw) register accessor: Frequency Measure function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_me_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_me_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_me_ctrl`] module"]
#[doc(alias = "FREQ_ME_CTRL")]
pub type FreqMeCtrl = crate::Reg<freq_me_ctrl::FreqMeCtrlSpec>;
#[doc = "Frequency Measure function control register"]
pub mod freq_me_ctrl;
#[doc = "FRO192M_CTRL (rw) register accessor: 192MHz Free Running OScillator (FRO) Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fro192m_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro192m_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro192m_ctrl`] module"]
#[doc(alias = "FRO192M_CTRL")]
pub type Fro192mCtrl = crate::Reg<fro192m_ctrl::Fro192mCtrlSpec>;
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
pub mod fro192m_ctrl;
#[doc = "FRO192M_STATUS (rw) register accessor: 192MHz Free Running OScillator (FRO) Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fro192m_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro192m_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro192m_status`] module"]
#[doc(alias = "FRO192M_STATUS")]
pub type Fro192mStatus = crate::Reg<fro192m_status::Fro192mStatusSpec>;
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
pub mod fro192m_status;
#[doc = "ADC_CTRL (rw) register accessor: General Purpose ADC VBAT Divider branch control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctrl`] module"]
#[doc(alias = "ADC_CTRL")]
pub type AdcCtrl = crate::Reg<adc_ctrl::AdcCtrlSpec>;
#[doc = "General Purpose ADC VBAT Divider branch control"]
pub mod adc_ctrl;
#[doc = "XO32M_CTRL (rw) register accessor: High speed Crystal Oscillator Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32m_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo32m_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xo32m_ctrl`] module"]
#[doc(alias = "XO32M_CTRL")]
pub type Xo32mCtrl = crate::Reg<xo32m_ctrl::Xo32mCtrlSpec>;
#[doc = "High speed Crystal Oscillator Control register"]
pub mod xo32m_ctrl;
#[doc = "XO32M_STATUS (r) register accessor: High speed Crystal Oscillator Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32m_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xo32m_status`] module"]
#[doc(alias = "XO32M_STATUS")]
pub type Xo32mStatus = crate::Reg<xo32m_status::Xo32mStatusSpec>;
#[doc = "High speed Crystal Oscillator Status register"]
pub mod xo32m_status;
#[doc = "BOD_DCDC_INT_CTRL (rw) register accessor: Brown Out Detectors (BoDs) & DCDC interrupts generation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bod_dcdc_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_dcdc_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod_dcdc_int_ctrl`] module"]
#[doc(alias = "BOD_DCDC_INT_CTRL")]
pub type BodDcdcIntCtrl = crate::Reg<bod_dcdc_int_ctrl::BodDcdcIntCtrlSpec>;
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
pub mod bod_dcdc_int_ctrl;
#[doc = "BOD_DCDC_INT_STATUS (r) register accessor: BoDs & DCDC interrupts status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bod_dcdc_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod_dcdc_int_status`] module"]
#[doc(alias = "BOD_DCDC_INT_STATUS")]
pub type BodDcdcIntStatus = crate::Reg<bod_dcdc_int_status::BodDcdcIntStatusSpec>;
#[doc = "BoDs & DCDC interrupts status register"]
pub mod bod_dcdc_int_status;
#[doc = "RINGO0_CTRL (rw) register accessor: First Ring Oscillator module control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo0_ctrl`] module"]
#[doc(alias = "RINGO0_CTRL")]
pub type Ringo0Ctrl = crate::Reg<ringo0_ctrl::Ringo0CtrlSpec>;
#[doc = "First Ring Oscillator module control register."]
pub mod ringo0_ctrl;
#[doc = "RINGO1_CTRL (rw) register accessor: Second Ring Oscillator module control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo1_ctrl`] module"]
#[doc(alias = "RINGO1_CTRL")]
pub type Ringo1Ctrl = crate::Reg<ringo1_ctrl::Ringo1CtrlSpec>;
#[doc = "Second Ring Oscillator module control register."]
pub mod ringo1_ctrl;
#[doc = "RINGO2_CTRL (rw) register accessor: Third Ring Oscillator module control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo2_ctrl`] module"]
#[doc(alias = "RINGO2_CTRL")]
pub type Ringo2Ctrl = crate::Reg<ringo2_ctrl::Ringo2CtrlSpec>;
#[doc = "Third Ring Oscillator module control register."]
pub mod ringo2_ctrl;
#[doc = "LDO_XO32M (rw) register accessor: High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo_xo32m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_xo32m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo_xo32m`] module"]
#[doc(alias = "LDO_XO32M")]
pub type LdoXo32m = crate::Reg<ldo_xo32m::LdoXo32mSpec>;
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
pub mod ldo_xo32m;
#[doc = "AUX_BIAS (rw) register accessor: AUX_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias`] module"]
#[doc(alias = "AUX_BIAS")]
pub type AuxBias = crate::Reg<aux_bias::AuxBiasSpec>;
#[doc = "AUX_BIAS"]
pub mod aux_bias;
#[doc = "DUMMY_CTRL (rw) register accessor: Dummy Control bus to analog modules\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy_ctrl`] module"]
#[doc(alias = "DUMMY_CTRL")]
pub type DummyCtrl = crate::Reg<dummy_ctrl::DummyCtrlSpec>;
#[doc = "Dummy Control bus to analog modules"]
pub mod dummy_ctrl;
#[doc = "USBHS_PHY_CTRL (rw) register accessor: USB High Speed Phy Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhs_phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhs_phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhs_phy_ctrl`] module"]
#[doc(alias = "USBHS_PHY_CTRL")]
pub type UsbhsPhyCtrl = crate::Reg<usbhs_phy_ctrl::UsbhsPhyCtrlSpec>;
#[doc = "USB High Speed Phy Control"]
pub mod usbhs_phy_ctrl;
#[doc = "USBHS_PHY_TRIM (rw) register accessor: USB High Speed Phy Trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhs_phy_trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhs_phy_trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhs_phy_trim`] module"]
#[doc(alias = "USBHS_PHY_TRIM")]
pub type UsbhsPhyTrim = crate::Reg<usbhs_phy_trim::UsbhsPhyTrimSpec>;
#[doc = "USB High Speed Phy Trim values"]
pub mod usbhs_phy_trim;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lut: [Lut; 26],
    _reserved1: [u8; 0x04c0],
    lut_truth: [LutTruth; 26],
    _reserved2: [u8; 0x98],
    outputs: Outputs,
    wakeint_ctrl: WakeintCtrl,
    _reserved4: [u8; 0x02f8],
    output_mux: [OutputMux; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x340 - no description available"]
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &Lut {
        &self.lut[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x340 - no description available"]
    #[inline(always)]
    pub fn lut_iter(&self) -> impl Iterator<Item = &Lut> {
        self.lut.iter()
    }
    #[doc = "0x800..0x868 - Specifies the Truth Table contents for LUTLUTn"]
    #[inline(always)]
    pub const fn lut_truth(&self, n: usize) -> &LutTruth {
        &self.lut_truth[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x868 - Specifies the Truth Table contents for LUTLUTn"]
    #[inline(always)]
    pub fn lut_truth_iter(&self) -> impl Iterator<Item = &LutTruth> {
        self.lut_truth.iter()
    }
    #[doc = "0x900 - Provides the current state of the 8 designated PLU Outputs."]
    #[inline(always)]
    pub const fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    #[doc = "0x904 - Wakeup interrupt control for PLU"]
    #[inline(always)]
    pub const fn wakeint_ctrl(&self) -> &WakeintCtrl {
        &self.wakeint_ctrl
    }
    #[doc = "0xc00..0xc20 - Selects the source to be connected to PLU Output OUTPUT_n"]
    #[inline(always)]
    pub const fn output_mux(&self, n: usize) -> &OutputMux {
        &self.output_mux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc00..0xc20 - Selects the source to be connected to PLU Output OUTPUT_n"]
    #[inline(always)]
    pub fn output_mux_iter(&self) -> impl Iterator<Item = &OutputMux> {
        self.output_mux.iter()
    }
}
#[doc = "no description available"]
pub use self::lut::Lut;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod lut;
#[doc = "LUT_TRUTH (rw) register accessor: Specifies the Truth Table contents for LUTLUTn\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_truth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_truth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_truth`] module"]
#[doc(alias = "LUT_TRUTH")]
pub type LutTruth = crate::Reg<lut_truth::LutTruthSpec>;
#[doc = "Specifies the Truth Table contents for LUTLUTn"]
pub mod lut_truth;
#[doc = "OUTPUTS (rw) register accessor: Provides the current state of the 8 designated PLU Outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`outputs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputs`] module"]
#[doc(alias = "OUTPUTS")]
pub type Outputs = crate::Reg<outputs::OutputsSpec>;
#[doc = "Provides the current state of the 8 designated PLU Outputs."]
pub mod outputs;
#[doc = "WAKEINT_CTRL (rw) register accessor: Wakeup interrupt control for PLU\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeint_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeint_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeint_ctrl`] module"]
#[doc(alias = "WAKEINT_CTRL")]
pub type WakeintCtrl = crate::Reg<wakeint_ctrl::WakeintCtrlSpec>;
#[doc = "Wakeup interrupt control for PLU"]
pub mod wakeint_ctrl;
#[doc = "OUTPUT_MUX (rw) register accessor: Selects the source to be connected to PLU Output OUTPUT_n\n\nYou can [`read`](crate::Reg::read) this register and get [`output_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_mux`] module"]
#[doc(alias = "OUTPUT_MUX")]
pub type OutputMux = crate::Reg<output_mux::OutputMuxSpec>;
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n"]
pub mod output_mux;

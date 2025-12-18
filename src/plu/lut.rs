#[repr(C)]
#[doc = "no description available"]
#[doc(alias = "LUT")]
pub struct Lut {
    lut_inp_mux: [LutInpMux; 5],
    _reserved_end: [u8; 0x0c],
}
impl Lut {
    #[doc = "0x00..0x14 - LUTn input x MUX"]
    #[inline(always)]
    pub const fn lut_inp_mux(&self, n: usize) -> &LutInpMux {
        &self.lut_inp_mux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - LUTn input x MUX"]
    #[inline(always)]
    pub fn lut_inp_mux_iter(&self) -> impl Iterator<Item = &LutInpMux> {
        self.lut_inp_mux.iter()
    }
}
#[doc = "LUT_INP_MUX (rw) register accessor: LUTn input x MUX\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_inp_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_inp_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_inp_mux`] module"]
#[doc(alias = "LUT_INP_MUX")]
pub type LutInpMux = crate::Reg<lut_inp_mux::LutInpMuxSpec>;
#[doc = "LUTn input x MUX"]
pub mod lut_inp_mux;

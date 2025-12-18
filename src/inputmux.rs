#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sct0_inmux: [Sct0Inmux; 7],
    _reserved1: [u8; 0x04],
    timer0captsel: [Timer0captsel; 4],
    _reserved2: [u8; 0x10],
    timer1captsel: [Timer1captsel; 4],
    _reserved3: [u8; 0x10],
    timer2captsel: [Timer2captsel; 4],
    _reserved4: [u8; 0x50],
    pintsel: [Pintsel; 8],
    dma0_itrig_inmux: [Dma0ItrigInmux; 23],
    _reserved6: [u8; 0x24],
    dma0_otrig_inmux: [Dma0OtrigInmux; 4],
    _reserved7: [u8; 0x10],
    freqmeas_ref: FreqmeasRef,
    freqmeas_target: FreqmeasTarget,
    _reserved9: [u8; 0x18],
    timer3captsel: [Timer3captsel; 4],
    _reserved10: [u8; 0x10],
    timer4captsel: [Timer4captsel; 4],
    _reserved11: [u8; 0x10],
    pintsecsel: [Pintsecsel; 2],
    _reserved12: [u8; 0x18],
    dma1_itrig_inmux: [Dma1ItrigInmux; 10],
    _reserved13: [u8; 0x18],
    dma1_otrig_inmux: [Dma1OtrigInmux; 4],
    _reserved14: [u8; 0x04f0],
    dma0_req_ena: Dma0ReqEna,
    _reserved15: [u8; 0x04],
    dma0_req_ena_set: Dma0ReqEnaSet,
    _reserved16: [u8; 0x04],
    dma0_req_ena_clr: Dma0ReqEnaClr,
    _reserved17: [u8; 0x0c],
    dma1_req_ena: Dma1ReqEna,
    _reserved18: [u8; 0x04],
    dma1_req_ena_set: Dma1ReqEnaSet,
    _reserved19: [u8; 0x04],
    dma1_req_ena_clr: Dma1ReqEnaClr,
    _reserved20: [u8; 0x0c],
    dma0_itrig_ena: Dma0ItrigEna,
    _reserved21: [u8; 0x04],
    dma0_itrig_ena_set: Dma0ItrigEnaSet,
    _reserved22: [u8; 0x04],
    dma0_itrig_ena_clr: Dma0ItrigEnaClr,
    _reserved23: [u8; 0x0c],
    dma1_itrig_ena: Dma1ItrigEna,
    _reserved24: [u8; 0x04],
    dma1_itrig_ena_set: Dma1ItrigEnaSet,
    _reserved25: [u8; 0x04],
    dma1_itrig_ena_clr: Dma1ItrigEnaClr,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - Input mux register for SCT0 input"]
    #[inline(always)]
    pub const fn sct0_inmux(&self, n: usize) -> &Sct0Inmux {
        &self.sct0_inmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - Input mux register for SCT0 input"]
    #[inline(always)]
    pub fn sct0_inmux_iter(&self) -> impl Iterator<Item = &Sct0Inmux> {
        self.sct0_inmux.iter()
    }
    #[doc = "0x20..0x30 - Capture select registers for TIMER0 inputs"]
    #[inline(always)]
    pub const fn timer0captsel(&self, n: usize) -> &Timer0captsel {
        &self.timer0captsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Capture select registers for TIMER0 inputs"]
    #[inline(always)]
    pub fn timer0captsel_iter(&self) -> impl Iterator<Item = &Timer0captsel> {
        self.timer0captsel.iter()
    }
    #[doc = "0x40..0x50 - Capture select registers for TIMER1 inputs"]
    #[inline(always)]
    pub const fn timer1captsel(&self, n: usize) -> &Timer1captsel {
        &self.timer1captsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Capture select registers for TIMER1 inputs"]
    #[inline(always)]
    pub fn timer1captsel_iter(&self) -> impl Iterator<Item = &Timer1captsel> {
        self.timer1captsel.iter()
    }
    #[doc = "0x60..0x70 - Capture select registers for TIMER2 inputs"]
    #[inline(always)]
    pub const fn timer2captsel(&self, n: usize) -> &Timer2captsel {
        &self.timer2captsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Capture select registers for TIMER2 inputs"]
    #[inline(always)]
    pub fn timer2captsel_iter(&self) -> impl Iterator<Item = &Timer2captsel> {
        self.timer2captsel.iter()
    }
    #[doc = "0xc0..0xe0 - Pin interrupt select register"]
    #[inline(always)]
    pub const fn pintsel(&self, n: usize) -> &Pintsel {
        &self.pintsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - Pin interrupt select register"]
    #[inline(always)]
    pub fn pintsel_iter(&self) -> impl Iterator<Item = &Pintsel> {
        self.pintsel.iter()
    }
    #[doc = "0xe0..0x13c - Trigger select register for DMA0 channel"]
    #[inline(always)]
    pub const fn dma0_itrig_inmux(&self, n: usize) -> &Dma0ItrigInmux {
        &self.dma0_itrig_inmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0x13c - Trigger select register for DMA0 channel"]
    #[inline(always)]
    pub fn dma0_itrig_inmux_iter(&self) -> impl Iterator<Item = &Dma0ItrigInmux> {
        self.dma0_itrig_inmux.iter()
    }
    #[doc = "0x160..0x170 - DMA0 output trigger selection to become DMA0 trigger"]
    #[inline(always)]
    pub const fn dma0_otrig_inmux(&self, n: usize) -> &Dma0OtrigInmux {
        &self.dma0_otrig_inmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - DMA0 output trigger selection to become DMA0 trigger"]
    #[inline(always)]
    pub fn dma0_otrig_inmux_iter(&self) -> impl Iterator<Item = &Dma0OtrigInmux> {
        self.dma0_otrig_inmux.iter()
    }
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_ref(&self) -> &FreqmeasRef {
        &self.freqmeas_ref
    }
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    #[inline(always)]
    pub const fn freqmeas_target(&self) -> &FreqmeasTarget {
        &self.freqmeas_target
    }
    #[doc = "0x1a0..0x1b0 - Capture select registers for TIMER3 inputs"]
    #[inline(always)]
    pub const fn timer3captsel(&self, n: usize) -> &Timer3captsel {
        &self.timer3captsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1b0 - Capture select registers for TIMER3 inputs"]
    #[inline(always)]
    pub fn timer3captsel_iter(&self) -> impl Iterator<Item = &Timer3captsel> {
        self.timer3captsel.iter()
    }
    #[doc = "0x1c0..0x1d0 - Capture select registers for TIMER4 inputs"]
    #[inline(always)]
    pub const fn timer4captsel(&self, n: usize) -> &Timer4captsel {
        &self.timer4captsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1d0 - Capture select registers for TIMER4 inputs"]
    #[inline(always)]
    pub fn timer4captsel_iter(&self) -> impl Iterator<Item = &Timer4captsel> {
        self.timer4captsel.iter()
    }
    #[doc = "0x1e0..0x1e8 - Pin interrupt secure select register"]
    #[inline(always)]
    pub const fn pintsecsel(&self, n: usize) -> &Pintsecsel {
        &self.pintsecsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1e0..0x1e8 - Pin interrupt secure select register"]
    #[inline(always)]
    pub fn pintsecsel_iter(&self) -> impl Iterator<Item = &Pintsecsel> {
        self.pintsecsel.iter()
    }
    #[doc = "0x200..0x228 - Trigger select register for DMA1 channel"]
    #[inline(always)]
    pub const fn dma1_itrig_inmux(&self, n: usize) -> &Dma1ItrigInmux {
        &self.dma1_itrig_inmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x228 - Trigger select register for DMA1 channel"]
    #[inline(always)]
    pub fn dma1_itrig_inmux_iter(&self) -> impl Iterator<Item = &Dma1ItrigInmux> {
        self.dma1_itrig_inmux.iter()
    }
    #[doc = "0x240..0x250 - DMA1 output trigger selection to become DMA1 trigger"]
    #[inline(always)]
    pub const fn dma1_otrig_inmux(&self, n: usize) -> &Dma1OtrigInmux {
        &self.dma1_otrig_inmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x250 - DMA1 output trigger selection to become DMA1 trigger"]
    #[inline(always)]
    pub fn dma1_otrig_inmux_iter(&self) -> impl Iterator<Item = &Dma1OtrigInmux> {
        self.dma1_otrig_inmux.iter()
    }
    #[doc = "0x740 - Enable DMA0 requests"]
    #[inline(always)]
    pub const fn dma0_req_ena(&self) -> &Dma0ReqEna {
        &self.dma0_req_ena
    }
    #[doc = "0x748 - Set one or several bits in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma0_req_ena_set(&self) -> &Dma0ReqEnaSet {
        &self.dma0_req_ena_set
    }
    #[doc = "0x750 - Clear one or several bits in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma0_req_ena_clr(&self) -> &Dma0ReqEnaClr {
        &self.dma0_req_ena_clr
    }
    #[doc = "0x760 - Enable DMA1 requests"]
    #[inline(always)]
    pub const fn dma1_req_ena(&self) -> &Dma1ReqEna {
        &self.dma1_req_ena
    }
    #[doc = "0x768 - Set one or several bits in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma1_req_ena_set(&self) -> &Dma1ReqEnaSet {
        &self.dma1_req_ena_set
    }
    #[doc = "0x770 - Clear one or several bits in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma1_req_ena_clr(&self) -> &Dma1ReqEnaClr {
        &self.dma1_req_ena_clr
    }
    #[doc = "0x780 - Enable DMA0 triggers"]
    #[inline(always)]
    pub const fn dma0_itrig_ena(&self) -> &Dma0ItrigEna {
        &self.dma0_itrig_ena
    }
    #[doc = "0x788 - Set one or several bits in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma0_itrig_ena_set(&self) -> &Dma0ItrigEnaSet {
        &self.dma0_itrig_ena_set
    }
    #[doc = "0x790 - Clear one or several bits in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma0_itrig_ena_clr(&self) -> &Dma0ItrigEnaClr {
        &self.dma0_itrig_ena_clr
    }
    #[doc = "0x7a0 - Enable DMA1 triggers"]
    #[inline(always)]
    pub const fn dma1_itrig_ena(&self) -> &Dma1ItrigEna {
        &self.dma1_itrig_ena
    }
    #[doc = "0x7a8 - Set one or several bits in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma1_itrig_ena_set(&self) -> &Dma1ItrigEnaSet {
        &self.dma1_itrig_ena_set
    }
    #[doc = "0x7b0 - Clear one or several bits in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma1_itrig_ena_clr(&self) -> &Dma1ItrigEnaClr {
        &self.dma1_itrig_ena_clr
    }
}
#[doc = "SCT0_INMUX (rw) register accessor: Input mux register for SCT0 input\n\nYou can [`read`](crate::Reg::read) this register and get [`sct0_inmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sct0_inmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sct0_inmux`] module"]
#[doc(alias = "SCT0_INMUX")]
pub type Sct0Inmux = crate::Reg<sct0_inmux::Sct0InmuxSpec>;
#[doc = "Input mux register for SCT0 input"]
pub mod sct0_inmux;
#[doc = "TIMER0CAPTSEL (rw) register accessor: Capture select registers for TIMER0 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0captsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0captsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0captsel`] module"]
#[doc(alias = "TIMER0CAPTSEL")]
pub type Timer0captsel = crate::Reg<timer0captsel::Timer0captselSpec>;
#[doc = "Capture select registers for TIMER0 inputs"]
pub mod timer0captsel;
#[doc = "TIMER1CAPTSEL (rw) register accessor: Capture select registers for TIMER1 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1captsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1captsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1captsel`] module"]
#[doc(alias = "TIMER1CAPTSEL")]
pub type Timer1captsel = crate::Reg<timer1captsel::Timer1captselSpec>;
#[doc = "Capture select registers for TIMER1 inputs"]
pub mod timer1captsel;
#[doc = "TIMER2CAPTSEL (rw) register accessor: Capture select registers for TIMER2 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2captsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2captsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2captsel`] module"]
#[doc(alias = "TIMER2CAPTSEL")]
pub type Timer2captsel = crate::Reg<timer2captsel::Timer2captselSpec>;
#[doc = "Capture select registers for TIMER2 inputs"]
pub mod timer2captsel;
#[doc = "PINTSEL (rw) register accessor: Pin interrupt select register\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintsel`] module"]
#[doc(alias = "PINTSEL")]
pub type Pintsel = crate::Reg<pintsel::PintselSpec>;
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "DMA0_ITRIG_INMUX (rw) register accessor: Trigger select register for DMA0 channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_itrig_inmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_inmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_itrig_inmux`] module"]
#[doc(alias = "DMA0_ITRIG_INMUX")]
pub type Dma0ItrigInmux = crate::Reg<dma0_itrig_inmux::Dma0ItrigInmuxSpec>;
#[doc = "Trigger select register for DMA0 channel"]
pub mod dma0_itrig_inmux;
#[doc = "DMA0_OTRIG_INMUX (rw) register accessor: DMA0 output trigger selection to become DMA0 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_otrig_inmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_otrig_inmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_otrig_inmux`] module"]
#[doc(alias = "DMA0_OTRIG_INMUX")]
pub type Dma0OtrigInmux = crate::Reg<dma0_otrig_inmux::Dma0OtrigInmuxSpec>;
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
pub mod dma0_otrig_inmux;
#[doc = "FREQMEAS_REF (rw) register accessor: Selection for frequency measurement reference clock\n\nYou can [`read`](crate::Reg::read) this register and get [`freqmeas_ref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqmeas_ref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqmeas_ref`] module"]
#[doc(alias = "FREQMEAS_REF")]
pub type FreqmeasRef = crate::Reg<freqmeas_ref::FreqmeasRefSpec>;
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "FREQMEAS_TARGET (rw) register accessor: Selection for frequency measurement target clock\n\nYou can [`read`](crate::Reg::read) this register and get [`freqmeas_target::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqmeas_target::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqmeas_target`] module"]
#[doc(alias = "FREQMEAS_TARGET")]
pub type FreqmeasTarget = crate::Reg<freqmeas_target::FreqmeasTargetSpec>;
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
#[doc = "TIMER3CAPTSEL (rw) register accessor: Capture select registers for TIMER3 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3captsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3captsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3captsel`] module"]
#[doc(alias = "TIMER3CAPTSEL")]
pub type Timer3captsel = crate::Reg<timer3captsel::Timer3captselSpec>;
#[doc = "Capture select registers for TIMER3 inputs"]
pub mod timer3captsel;
#[doc = "TIMER4CAPTSEL (rw) register accessor: Capture select registers for TIMER4 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4captsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4captsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4captsel`] module"]
#[doc(alias = "TIMER4CAPTSEL")]
pub type Timer4captsel = crate::Reg<timer4captsel::Timer4captselSpec>;
#[doc = "Capture select registers for TIMER4 inputs"]
pub mod timer4captsel;
#[doc = "PINTSECSEL (rw) register accessor: Pin interrupt secure select register\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsecsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintsecsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintsecsel`] module"]
#[doc(alias = "PINTSECSEL")]
pub type Pintsecsel = crate::Reg<pintsecsel::PintsecselSpec>;
#[doc = "Pin interrupt secure select register"]
pub mod pintsecsel;
#[doc = "DMA1_ITRIG_INMUX (rw) register accessor: Trigger select register for DMA1 channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_itrig_inmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_inmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_itrig_inmux`] module"]
#[doc(alias = "DMA1_ITRIG_INMUX")]
pub type Dma1ItrigInmux = crate::Reg<dma1_itrig_inmux::Dma1ItrigInmuxSpec>;
#[doc = "Trigger select register for DMA1 channel"]
pub mod dma1_itrig_inmux;
#[doc = "DMA1_OTRIG_INMUX (rw) register accessor: DMA1 output trigger selection to become DMA1 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_otrig_inmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_otrig_inmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_otrig_inmux`] module"]
#[doc(alias = "DMA1_OTRIG_INMUX")]
pub type Dma1OtrigInmux = crate::Reg<dma1_otrig_inmux::Dma1OtrigInmuxSpec>;
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
pub mod dma1_otrig_inmux;
#[doc = "DMA0_REQ_ENA (rw) register accessor: Enable DMA0 requests\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_req_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_req_ena`] module"]
#[doc(alias = "DMA0_REQ_ENA")]
pub type Dma0ReqEna = crate::Reg<dma0_req_ena::Dma0ReqEnaSpec>;
#[doc = "Enable DMA0 requests"]
pub mod dma0_req_ena;
#[doc = "DMA0_REQ_ENA_SET (w) register accessor: Set one or several bits in DMA0_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_req_ena_set`] module"]
#[doc(alias = "DMA0_REQ_ENA_SET")]
pub type Dma0ReqEnaSet = crate::Reg<dma0_req_ena_set::Dma0ReqEnaSetSpec>;
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_set;
#[doc = "DMA0_REQ_ENA_CLR (w) register accessor: Clear one or several bits in DMA0_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_req_ena_clr`] module"]
#[doc(alias = "DMA0_REQ_ENA_CLR")]
pub type Dma0ReqEnaClr = crate::Reg<dma0_req_ena_clr::Dma0ReqEnaClrSpec>;
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_clr;
#[doc = "DMA1_REQ_ENA (rw) register accessor: Enable DMA1 requests\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_req_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_req_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_req_ena`] module"]
#[doc(alias = "DMA1_REQ_ENA")]
pub type Dma1ReqEna = crate::Reg<dma1_req_ena::Dma1ReqEnaSpec>;
#[doc = "Enable DMA1 requests"]
pub mod dma1_req_ena;
#[doc = "DMA1_REQ_ENA_SET (w) register accessor: Set one or several bits in DMA1_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_req_ena_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_req_ena_set`] module"]
#[doc(alias = "DMA1_REQ_ENA_SET")]
pub type Dma1ReqEnaSet = crate::Reg<dma1_req_ena_set::Dma1ReqEnaSetSpec>;
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_set;
#[doc = "DMA1_REQ_ENA_CLR (w) register accessor: Clear one or several bits in DMA1_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_req_ena_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_req_ena_clr`] module"]
#[doc(alias = "DMA1_REQ_ENA_CLR")]
pub type Dma1ReqEnaClr = crate::Reg<dma1_req_ena_clr::Dma1ReqEnaClrSpec>;
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_clr;
#[doc = "DMA0_ITRIG_ENA (rw) register accessor: Enable DMA0 triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_itrig_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_itrig_ena`] module"]
#[doc(alias = "DMA0_ITRIG_ENA")]
pub type Dma0ItrigEna = crate::Reg<dma0_itrig_ena::Dma0ItrigEnaSpec>;
#[doc = "Enable DMA0 triggers"]
pub mod dma0_itrig_ena;
#[doc = "DMA0_ITRIG_ENA_SET (w) register accessor: Set one or several bits in DMA0_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_ena_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_itrig_ena_set`] module"]
#[doc(alias = "DMA0_ITRIG_ENA_SET")]
pub type Dma0ItrigEnaSet = crate::Reg<dma0_itrig_ena_set::Dma0ItrigEnaSetSpec>;
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_set;
#[doc = "DMA0_ITRIG_ENA_CLR (w) register accessor: Clear one or several bits in DMA0_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_ena_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0_itrig_ena_clr`] module"]
#[doc(alias = "DMA0_ITRIG_ENA_CLR")]
pub type Dma0ItrigEnaClr = crate::Reg<dma0_itrig_ena_clr::Dma0ItrigEnaClrSpec>;
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_clr;
#[doc = "DMA1_ITRIG_ENA (rw) register accessor: Enable DMA1 triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_itrig_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_itrig_ena`] module"]
#[doc(alias = "DMA1_ITRIG_ENA")]
pub type Dma1ItrigEna = crate::Reg<dma1_itrig_ena::Dma1ItrigEnaSpec>;
#[doc = "Enable DMA1 triggers"]
pub mod dma1_itrig_ena;
#[doc = "DMA1_ITRIG_ENA_SET (w) register accessor: Set one or several bits in DMA1_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_ena_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_itrig_ena_set`] module"]
#[doc(alias = "DMA1_ITRIG_ENA_SET")]
pub type Dma1ItrigEnaSet = crate::Reg<dma1_itrig_ena_set::Dma1ItrigEnaSetSpec>;
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_set;
#[doc = "DMA1_ITRIG_ENA_CLR (w) register accessor: Clear one or several bits in DMA1_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_ena_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_itrig_ena_clr`] module"]
#[doc(alias = "DMA1_ITRIG_ENA_CLR")]
pub type Dma1ItrigEnaClr = crate::Reg<dma1_itrig_ena_clr::Dma1ItrigEnaClrSpec>;
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_clr;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    b0_0: B0_0,
    b0_1: B0_1,
    b0_2: B0_2,
    b0_3: B0_3,
    b0_4: B0_4,
    b0_5: B0_5,
    b0_6: B0_6,
    b0_7: B0_7,
    b0_8: B0_8,
    b0_9: B0_9,
    b0_10: B0_10,
    b0_11: B0_11,
    b0_12: B0_12,
    b0_13: B0_13,
    b0_14: B0_14,
    b0_15: B0_15,
    b0_16: B0_16,
    b0_17: B0_17,
    b0_18: B0_18,
    b0_19: B0_19,
    b0_20: B0_20,
    b0_21: B0_21,
    b0_22: B0_22,
    b0_23: B0_23,
    b0_24: B0_24,
    b0_25: B0_25,
    b0_26: B0_26,
    b0_27: B0_27,
    b0_28: B0_28,
    b0_29: B0_29,
    b0_30: B0_30,
    b0_31: B0_31,
    _reserved32: [u8; 0x0fe0],
    w0_0: W0_0,
    w0_1: W0_1,
    w0_2: W0_2,
    w0_3: W0_3,
    w0_4: W0_4,
    w0_5: W0_5,
    w0_6: W0_6,
    w0_7: W0_7,
    w0_8: W0_8,
    w0_9: W0_9,
    w0_10: W0_10,
    w0_11: W0_11,
    w0_12: W0_12,
    w0_13: W0_13,
    w0_14: W0_14,
    w0_15: W0_15,
    w0_16: W0_16,
    w0_17: W0_17,
    w0_18: W0_18,
    w0_19: W0_19,
    w0_20: W0_20,
    w0_21: W0_21,
    w0_22: W0_22,
    w0_23: W0_23,
    w0_24: W0_24,
    w0_25: W0_25,
    w0_26: W0_26,
    w0_27: W0_27,
    w0_28: W0_28,
    w0_29: W0_29,
    w0_30: W0_30,
    w0_31: W0_31,
    _reserved64: [u8; 0x0f80],
    dir0: Dir0,
    _reserved65: [u8; 0x7c],
    mask0: Mask0,
    _reserved66: [u8; 0x7c],
    pin0: Pin0,
    _reserved67: [u8; 0x7c],
    mpin0: Mpin0,
    _reserved68: [u8; 0x7c],
    set0: Set0,
    _reserved69: [u8; 0x7c],
    clr0: Clr0,
    _reserved70: [u8; 0x7c],
    not0: Not0,
    _reserved71: [u8; 0x7c],
    dirset0: Dirset0,
    _reserved72: [u8; 0x7c],
    dirclr0: Dirclr0,
    _reserved73: [u8; 0x7c],
    dirnot0: Dirnot0,
}
impl RegisterBlock {
    #[doc = "0x00 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_0(&self) -> &B0_0 {
        &self.b0_0
    }
    #[doc = "0x01 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_1(&self) -> &B0_1 {
        &self.b0_1
    }
    #[doc = "0x02 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_2(&self) -> &B0_2 {
        &self.b0_2
    }
    #[doc = "0x03 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_3(&self) -> &B0_3 {
        &self.b0_3
    }
    #[doc = "0x04 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_4(&self) -> &B0_4 {
        &self.b0_4
    }
    #[doc = "0x05 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_5(&self) -> &B0_5 {
        &self.b0_5
    }
    #[doc = "0x06 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_6(&self) -> &B0_6 {
        &self.b0_6
    }
    #[doc = "0x07 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_7(&self) -> &B0_7 {
        &self.b0_7
    }
    #[doc = "0x08 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_8(&self) -> &B0_8 {
        &self.b0_8
    }
    #[doc = "0x09 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_9(&self) -> &B0_9 {
        &self.b0_9
    }
    #[doc = "0x0a - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_10(&self) -> &B0_10 {
        &self.b0_10
    }
    #[doc = "0x0b - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_11(&self) -> &B0_11 {
        &self.b0_11
    }
    #[doc = "0x0c - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_12(&self) -> &B0_12 {
        &self.b0_12
    }
    #[doc = "0x0d - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_13(&self) -> &B0_13 {
        &self.b0_13
    }
    #[doc = "0x0e - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_14(&self) -> &B0_14 {
        &self.b0_14
    }
    #[doc = "0x0f - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_15(&self) -> &B0_15 {
        &self.b0_15
    }
    #[doc = "0x10 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_16(&self) -> &B0_16 {
        &self.b0_16
    }
    #[doc = "0x11 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_17(&self) -> &B0_17 {
        &self.b0_17
    }
    #[doc = "0x12 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_18(&self) -> &B0_18 {
        &self.b0_18
    }
    #[doc = "0x13 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_19(&self) -> &B0_19 {
        &self.b0_19
    }
    #[doc = "0x14 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_20(&self) -> &B0_20 {
        &self.b0_20
    }
    #[doc = "0x15 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_21(&self) -> &B0_21 {
        &self.b0_21
    }
    #[doc = "0x16 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_22(&self) -> &B0_22 {
        &self.b0_22
    }
    #[doc = "0x17 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_23(&self) -> &B0_23 {
        &self.b0_23
    }
    #[doc = "0x18 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_24(&self) -> &B0_24 {
        &self.b0_24
    }
    #[doc = "0x19 - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_25(&self) -> &B0_25 {
        &self.b0_25
    }
    #[doc = "0x1a - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_26(&self) -> &B0_26 {
        &self.b0_26
    }
    #[doc = "0x1b - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_27(&self) -> &B0_27 {
        &self.b0_27
    }
    #[doc = "0x1c - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_28(&self) -> &B0_28 {
        &self.b0_28
    }
    #[doc = "0x1d - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_29(&self) -> &B0_29 {
        &self.b0_29
    }
    #[doc = "0x1e - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_30(&self) -> &B0_30 {
        &self.b0_30
    }
    #[doc = "0x1f - Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b0_31(&self) -> &B0_31 {
        &self.b0_31
    }
    #[doc = "0x1000 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_0(&self) -> &W0_0 {
        &self.w0_0
    }
    #[doc = "0x1004 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_1(&self) -> &W0_1 {
        &self.w0_1
    }
    #[doc = "0x1008 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_2(&self) -> &W0_2 {
        &self.w0_2
    }
    #[doc = "0x100c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_3(&self) -> &W0_3 {
        &self.w0_3
    }
    #[doc = "0x1010 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_4(&self) -> &W0_4 {
        &self.w0_4
    }
    #[doc = "0x1014 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_5(&self) -> &W0_5 {
        &self.w0_5
    }
    #[doc = "0x1018 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_6(&self) -> &W0_6 {
        &self.w0_6
    }
    #[doc = "0x101c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_7(&self) -> &W0_7 {
        &self.w0_7
    }
    #[doc = "0x1020 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_8(&self) -> &W0_8 {
        &self.w0_8
    }
    #[doc = "0x1024 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_9(&self) -> &W0_9 {
        &self.w0_9
    }
    #[doc = "0x1028 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_10(&self) -> &W0_10 {
        &self.w0_10
    }
    #[doc = "0x102c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_11(&self) -> &W0_11 {
        &self.w0_11
    }
    #[doc = "0x1030 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_12(&self) -> &W0_12 {
        &self.w0_12
    }
    #[doc = "0x1034 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_13(&self) -> &W0_13 {
        &self.w0_13
    }
    #[doc = "0x1038 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_14(&self) -> &W0_14 {
        &self.w0_14
    }
    #[doc = "0x103c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_15(&self) -> &W0_15 {
        &self.w0_15
    }
    #[doc = "0x1040 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_16(&self) -> &W0_16 {
        &self.w0_16
    }
    #[doc = "0x1044 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_17(&self) -> &W0_17 {
        &self.w0_17
    }
    #[doc = "0x1048 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_18(&self) -> &W0_18 {
        &self.w0_18
    }
    #[doc = "0x104c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_19(&self) -> &W0_19 {
        &self.w0_19
    }
    #[doc = "0x1050 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_20(&self) -> &W0_20 {
        &self.w0_20
    }
    #[doc = "0x1054 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_21(&self) -> &W0_21 {
        &self.w0_21
    }
    #[doc = "0x1058 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_22(&self) -> &W0_22 {
        &self.w0_22
    }
    #[doc = "0x105c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_23(&self) -> &W0_23 {
        &self.w0_23
    }
    #[doc = "0x1060 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_24(&self) -> &W0_24 {
        &self.w0_24
    }
    #[doc = "0x1064 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_25(&self) -> &W0_25 {
        &self.w0_25
    }
    #[doc = "0x1068 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_26(&self) -> &W0_26 {
        &self.w0_26
    }
    #[doc = "0x106c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_27(&self) -> &W0_27 {
        &self.w0_27
    }
    #[doc = "0x1070 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_28(&self) -> &W0_28 {
        &self.w0_28
    }
    #[doc = "0x1074 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_29(&self) -> &W0_29 {
        &self.w0_29
    }
    #[doc = "0x1078 - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_30(&self) -> &W0_30 {
        &self.w0_30
    }
    #[doc = "0x107c - Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w0_31(&self) -> &W0_31 {
        &self.w0_31
    }
    #[doc = "0x2000 - Direction registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn dir0(&self) -> &Dir0 {
        &self.dir0
    }
    #[doc = "0x2080 - Mask register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
    #[doc = "0x2100 - Port pin register for all port GPIO pins"]
    #[inline(always)]
    pub const fn pin0(&self) -> &Pin0 {
        &self.pin0
    }
    #[doc = "0x2180 - Masked port register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mpin0(&self) -> &Mpin0 {
        &self.mpin0
    }
    #[doc = "0x2200 - Write: Set register for port. Read: output bits for port"]
    #[inline(always)]
    pub const fn set0(&self) -> &Set0 {
        &self.set0
    }
    #[doc = "0x2280 - Clear port for all port GPIO pins"]
    #[inline(always)]
    pub const fn clr0(&self) -> &Clr0 {
        &self.clr0
    }
    #[doc = "0x2300 - Toggle port for all port GPIO pins"]
    #[inline(always)]
    pub const fn not0(&self) -> &Not0 {
        &self.not0
    }
    #[doc = "0x2380 - Set pin direction bits for port"]
    #[inline(always)]
    pub const fn dirset0(&self) -> &Dirset0 {
        &self.dirset0
    }
    #[doc = "0x2400 - Clear pin direction bits for port"]
    #[inline(always)]
    pub const fn dirclr0(&self) -> &Dirclr0 {
        &self.dirclr0
    }
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    #[inline(always)]
    pub const fn dirnot0(&self) -> &Dirnot0 {
        &self.dirnot0
    }
}
#[doc = "B0_0 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_0`] module"]
pub type B0_0 = crate::Reg<b0_0::B0_0Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_0;
#[doc = "B0_1 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_1`] module"]
pub type B0_1 = crate::Reg<b0_1::B0_1Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_1;
#[doc = "B0_2 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_2`] module"]
pub type B0_2 = crate::Reg<b0_2::B0_2Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_2;
#[doc = "B0_3 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_3`] module"]
pub type B0_3 = crate::Reg<b0_3::B0_3Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_3;
#[doc = "B0_4 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_4`] module"]
pub type B0_4 = crate::Reg<b0_4::B0_4Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_4;
#[doc = "B0_5 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_5`] module"]
pub type B0_5 = crate::Reg<b0_5::B0_5Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_5;
#[doc = "B0_6 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_6`] module"]
pub type B0_6 = crate::Reg<b0_6::B0_6Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_6;
#[doc = "B0_7 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_7`] module"]
pub type B0_7 = crate::Reg<b0_7::B0_7Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_7;
#[doc = "B0_8 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_8`] module"]
pub type B0_8 = crate::Reg<b0_8::B0_8Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_8;
#[doc = "B0_9 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_9`] module"]
pub type B0_9 = crate::Reg<b0_9::B0_9Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_9;
#[doc = "B0_10 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_10`] module"]
pub type B0_10 = crate::Reg<b0_10::B0_10Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_10;
#[doc = "B0_11 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_11`] module"]
pub type B0_11 = crate::Reg<b0_11::B0_11Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_11;
#[doc = "B0_12 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_12`] module"]
pub type B0_12 = crate::Reg<b0_12::B0_12Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_12;
#[doc = "B0_13 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_13`] module"]
pub type B0_13 = crate::Reg<b0_13::B0_13Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_13;
#[doc = "B0_14 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_14`] module"]
pub type B0_14 = crate::Reg<b0_14::B0_14Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_14;
#[doc = "B0_15 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_15`] module"]
pub type B0_15 = crate::Reg<b0_15::B0_15Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_15;
#[doc = "B0_16 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_16`] module"]
pub type B0_16 = crate::Reg<b0_16::B0_16Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_16;
#[doc = "B0_17 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_17`] module"]
pub type B0_17 = crate::Reg<b0_17::B0_17Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_17;
#[doc = "B0_18 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_18`] module"]
pub type B0_18 = crate::Reg<b0_18::B0_18Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_18;
#[doc = "B0_19 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_19`] module"]
pub type B0_19 = crate::Reg<b0_19::B0_19Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_19;
#[doc = "B0_20 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_20`] module"]
pub type B0_20 = crate::Reg<b0_20::B0_20Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_20;
#[doc = "B0_21 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_21`] module"]
pub type B0_21 = crate::Reg<b0_21::B0_21Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_21;
#[doc = "B0_22 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_22`] module"]
pub type B0_22 = crate::Reg<b0_22::B0_22Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_22;
#[doc = "B0_23 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_23`] module"]
pub type B0_23 = crate::Reg<b0_23::B0_23Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_23;
#[doc = "B0_24 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_24`] module"]
pub type B0_24 = crate::Reg<b0_24::B0_24Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_24;
#[doc = "B0_25 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_25`] module"]
pub type B0_25 = crate::Reg<b0_25::B0_25Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_25;
#[doc = "B0_26 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_26`] module"]
pub type B0_26 = crate::Reg<b0_26::B0_26Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_26;
#[doc = "B0_27 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_27`] module"]
pub type B0_27 = crate::Reg<b0_27::B0_27Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_27;
#[doc = "B0_28 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_28`] module"]
pub type B0_28 = crate::Reg<b0_28::B0_28Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_28;
#[doc = "B0_29 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_29`] module"]
pub type B0_29 = crate::Reg<b0_29::B0_29Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_29;
#[doc = "B0_30 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_30`] module"]
pub type B0_30 = crate::Reg<b0_30::B0_30Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_30;
#[doc = "B0_31 (rw) register accessor: Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0_31`] module"]
pub type B0_31 = crate::Reg<b0_31::B0_31Spec>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b0_31;
#[doc = "W0_0 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_0`] module"]
pub type W0_0 = crate::Reg<w0_0::W0_0Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_0;
#[doc = "W0_1 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_1`] module"]
pub type W0_1 = crate::Reg<w0_1::W0_1Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_1;
#[doc = "W0_2 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_2`] module"]
pub type W0_2 = crate::Reg<w0_2::W0_2Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_2;
#[doc = "W0_3 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_3`] module"]
pub type W0_3 = crate::Reg<w0_3::W0_3Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_3;
#[doc = "W0_4 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_4`] module"]
pub type W0_4 = crate::Reg<w0_4::W0_4Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_4;
#[doc = "W0_5 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_5`] module"]
pub type W0_5 = crate::Reg<w0_5::W0_5Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_5;
#[doc = "W0_6 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_6`] module"]
pub type W0_6 = crate::Reg<w0_6::W0_6Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_6;
#[doc = "W0_7 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_7`] module"]
pub type W0_7 = crate::Reg<w0_7::W0_7Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_7;
#[doc = "W0_8 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_8`] module"]
pub type W0_8 = crate::Reg<w0_8::W0_8Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_8;
#[doc = "W0_9 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_9`] module"]
pub type W0_9 = crate::Reg<w0_9::W0_9Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_9;
#[doc = "W0_10 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_10`] module"]
pub type W0_10 = crate::Reg<w0_10::W0_10Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_10;
#[doc = "W0_11 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_11`] module"]
pub type W0_11 = crate::Reg<w0_11::W0_11Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_11;
#[doc = "W0_12 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_12`] module"]
pub type W0_12 = crate::Reg<w0_12::W0_12Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_12;
#[doc = "W0_13 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_13`] module"]
pub type W0_13 = crate::Reg<w0_13::W0_13Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_13;
#[doc = "W0_14 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_14`] module"]
pub type W0_14 = crate::Reg<w0_14::W0_14Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_14;
#[doc = "W0_15 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_15`] module"]
pub type W0_15 = crate::Reg<w0_15::W0_15Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_15;
#[doc = "W0_16 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_16`] module"]
pub type W0_16 = crate::Reg<w0_16::W0_16Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_16;
#[doc = "W0_17 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_17`] module"]
pub type W0_17 = crate::Reg<w0_17::W0_17Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_17;
#[doc = "W0_18 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_18`] module"]
pub type W0_18 = crate::Reg<w0_18::W0_18Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_18;
#[doc = "W0_19 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_19`] module"]
pub type W0_19 = crate::Reg<w0_19::W0_19Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_19;
#[doc = "W0_20 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_20`] module"]
pub type W0_20 = crate::Reg<w0_20::W0_20Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_20;
#[doc = "W0_21 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_21`] module"]
pub type W0_21 = crate::Reg<w0_21::W0_21Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_21;
#[doc = "W0_22 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_22`] module"]
pub type W0_22 = crate::Reg<w0_22::W0_22Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_22;
#[doc = "W0_23 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_23`] module"]
pub type W0_23 = crate::Reg<w0_23::W0_23Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_23;
#[doc = "W0_24 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_24`] module"]
pub type W0_24 = crate::Reg<w0_24::W0_24Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_24;
#[doc = "W0_25 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_25`] module"]
pub type W0_25 = crate::Reg<w0_25::W0_25Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_25;
#[doc = "W0_26 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_26`] module"]
pub type W0_26 = crate::Reg<w0_26::W0_26Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_26;
#[doc = "W0_27 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_27`] module"]
pub type W0_27 = crate::Reg<w0_27::W0_27Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_27;
#[doc = "W0_28 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_28`] module"]
pub type W0_28 = crate::Reg<w0_28::W0_28Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_28;
#[doc = "W0_29 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_29`] module"]
pub type W0_29 = crate::Reg<w0_29::W0_29Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_29;
#[doc = "W0_30 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_30`] module"]
pub type W0_30 = crate::Reg<w0_30::W0_30Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_30;
#[doc = "W0_31 (rw) register accessor: Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w0_31`] module"]
pub type W0_31 = crate::Reg<w0_31::W0_31Spec>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w0_31;
#[doc = "DIR0 (rw) register accessor: Direction registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`dir0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir0`] module"]
#[doc(alias = "DIR0")]
pub type Dir0 = crate::Reg<dir0::Dir0Spec>;
#[doc = "Direction registers for all port GPIO pins"]
pub mod dir0;
#[doc = "MASK0 (rw) register accessor: Mask register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`] module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "Mask register for all port GPIO pins"]
pub mod mask0;
#[doc = "PIN0 (rw) register accessor: Port pin register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`pin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin0`] module"]
#[doc(alias = "PIN0")]
pub type Pin0 = crate::Reg<pin0::Pin0Spec>;
#[doc = "Port pin register for all port GPIO pins"]
pub mod pin0;
#[doc = "MPIN0 (rw) register accessor: Masked port register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mpin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpin0`] module"]
#[doc(alias = "MPIN0")]
pub type Mpin0 = crate::Reg<mpin0::Mpin0Spec>;
#[doc = "Masked port register for all port GPIO pins"]
pub mod mpin0;
#[doc = "SET0 (rw) register accessor: Write: Set register for port. Read: output bits for port\n\nYou can [`read`](crate::Reg::read) this register and get [`set0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set0`] module"]
#[doc(alias = "SET0")]
pub type Set0 = crate::Reg<set0::Set0Spec>;
#[doc = "Write: Set register for port. Read: output bits for port"]
pub mod set0;
#[doc = "CLR0 (w) register accessor: Clear port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr0`] module"]
#[doc(alias = "CLR0")]
pub type Clr0 = crate::Reg<clr0::Clr0Spec>;
#[doc = "Clear port for all port GPIO pins"]
pub mod clr0;
#[doc = "NOT0 (w) register accessor: Toggle port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`not0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@not0`] module"]
#[doc(alias = "NOT0")]
pub type Not0 = crate::Reg<not0::Not0Spec>;
#[doc = "Toggle port for all port GPIO pins"]
pub mod not0;
#[doc = "DIRSET0 (w) register accessor: Set pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset0`] module"]
#[doc(alias = "DIRSET0")]
pub type Dirset0 = crate::Reg<dirset0::Dirset0Spec>;
#[doc = "Set pin direction bits for port"]
pub mod dirset0;
#[doc = "DIRCLR0 (w) register accessor: Clear pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr0`] module"]
#[doc(alias = "DIRCLR0")]
pub type Dirclr0 = crate::Reg<dirclr0::Dirclr0Spec>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr0;
#[doc = "DIRNOT0 (w) register accessor: Toggle pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirnot0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirnot0`] module"]
#[doc(alias = "DIRNOT0")]
pub type Dirnot0 = crate::Reg<dirnot0::Dirnot0Spec>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot0;

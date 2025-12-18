#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pio0_0: Pio0_0,
    pio0_1: Pio0_1,
    pio0_2: Pio0_2,
    pio0_3: Pio0_3,
    pio0_4: Pio0_4,
    pio0_5: Pio0_5,
    pio0_6: Pio0_6,
    pio0_7: Pio0_7,
    pio0_8: Pio0_8,
    pio0_9: Pio0_9,
    pio0_10: Pio0_10,
    pio0_11: Pio0_11,
    pio0_12: Pio0_12,
    pio0_13: Pio0_13,
    pio0_14: Pio0_14,
    pio0_15: Pio0_15,
    pio0_16: Pio0_16,
    pio0_17: Pio0_17,
    pio0_18: Pio0_18,
    pio0_19: Pio0_19,
    pio0_20: Pio0_20,
    pio0_21: Pio0_21,
    pio0_22: Pio0_22,
    pio0_23: Pio0_23,
    pio0_24: Pio0_24,
    pio0_25: Pio0_25,
    pio0_26: Pio0_26,
    pio0_27: Pio0_27,
    pio0_28: Pio0_28,
    pio0_29: Pio0_29,
    pio0_30: Pio0_30,
    pio0_31: Pio0_31,
    pio1_0: Pio1_0,
    pio1_1: Pio1_1,
    pio1_2: Pio1_2,
    pio1_3: Pio1_3,
    pio1_4: Pio1_4,
    pio1_5: Pio1_5,
    pio1_6: Pio1_6,
    pio1_7: Pio1_7,
    pio1_8: Pio1_8,
    pio1_9: Pio1_9,
    pio1_10: Pio1_10,
    pio1_11: Pio1_11,
    pio1_12: Pio1_12,
    pio1_13: Pio1_13,
    pio1_14: Pio1_14,
    pio1_15: Pio1_15,
    pio1_16: Pio1_16,
    pio1_17: Pio1_17,
    pio1_18: Pio1_18,
    pio1_19: Pio1_19,
    pio1_20: Pio1_20,
    pio1_21: Pio1_21,
    pio1_22: Pio1_22,
    pio1_23: Pio1_23,
    pio1_24: Pio1_24,
    pio1_25: Pio1_25,
    pio1_26: Pio1_26,
    pio1_27: Pio1_27,
    pio1_28: Pio1_28,
    pio1_29: Pio1_29,
    pio1_30: Pio1_30,
    pio1_31: Pio1_31,
}
impl RegisterBlock {
    #[doc = "0x00 - Digital I/O control for port 0 pins PIO0_0"]
    #[inline(always)]
    pub const fn pio0_0(&self) -> &Pio0_0 {
        &self.pio0_0
    }
    #[doc = "0x04 - Digital I/O control for port 0 pins PIO0_1"]
    #[inline(always)]
    pub const fn pio0_1(&self) -> &Pio0_1 {
        &self.pio0_1
    }
    #[doc = "0x08 - Digital I/O control for port 0 pins PIO0_2"]
    #[inline(always)]
    pub const fn pio0_2(&self) -> &Pio0_2 {
        &self.pio0_2
    }
    #[doc = "0x0c - Digital I/O control for port 0 pins PIO0_3"]
    #[inline(always)]
    pub const fn pio0_3(&self) -> &Pio0_3 {
        &self.pio0_3
    }
    #[doc = "0x10 - Digital I/O control for port 0 pins PIO0_4"]
    #[inline(always)]
    pub const fn pio0_4(&self) -> &Pio0_4 {
        &self.pio0_4
    }
    #[doc = "0x14 - Digital I/O control for port 0 pins PIO0_5"]
    #[inline(always)]
    pub const fn pio0_5(&self) -> &Pio0_5 {
        &self.pio0_5
    }
    #[doc = "0x18 - Digital I/O control for port 0 pins PIO0_6"]
    #[inline(always)]
    pub const fn pio0_6(&self) -> &Pio0_6 {
        &self.pio0_6
    }
    #[doc = "0x1c - Digital I/O control for port 0 pins PIO0_7"]
    #[inline(always)]
    pub const fn pio0_7(&self) -> &Pio0_7 {
        &self.pio0_7
    }
    #[doc = "0x20 - Digital I/O control for port 0 pins PIO0_8"]
    #[inline(always)]
    pub const fn pio0_8(&self) -> &Pio0_8 {
        &self.pio0_8
    }
    #[doc = "0x24 - Digital I/O control for port 0 pins PIO0_9"]
    #[inline(always)]
    pub const fn pio0_9(&self) -> &Pio0_9 {
        &self.pio0_9
    }
    #[doc = "0x28 - Digital I/O control for port 0 pins PIO0_10"]
    #[inline(always)]
    pub const fn pio0_10(&self) -> &Pio0_10 {
        &self.pio0_10
    }
    #[doc = "0x2c - Digital I/O control for port 0 pins PIO0_11"]
    #[inline(always)]
    pub const fn pio0_11(&self) -> &Pio0_11 {
        &self.pio0_11
    }
    #[doc = "0x30 - Digital I/O control for port 0 pins PIO0_12"]
    #[inline(always)]
    pub const fn pio0_12(&self) -> &Pio0_12 {
        &self.pio0_12
    }
    #[doc = "0x34 - Digital I/O control for port 0 pins PIO0_13"]
    #[inline(always)]
    pub const fn pio0_13(&self) -> &Pio0_13 {
        &self.pio0_13
    }
    #[doc = "0x38 - Digital I/O control for port 0 pins PIO0_14"]
    #[inline(always)]
    pub const fn pio0_14(&self) -> &Pio0_14 {
        &self.pio0_14
    }
    #[doc = "0x3c - Digital I/O control for port 0 pins PIO0_15"]
    #[inline(always)]
    pub const fn pio0_15(&self) -> &Pio0_15 {
        &self.pio0_15
    }
    #[doc = "0x40 - Digital I/O control for port 0 pins PIO0_16"]
    #[inline(always)]
    pub const fn pio0_16(&self) -> &Pio0_16 {
        &self.pio0_16
    }
    #[doc = "0x44 - Digital I/O control for port 0 pins PIO0_17"]
    #[inline(always)]
    pub const fn pio0_17(&self) -> &Pio0_17 {
        &self.pio0_17
    }
    #[doc = "0x48 - Digital I/O control for port 0 pins PIO0_18"]
    #[inline(always)]
    pub const fn pio0_18(&self) -> &Pio0_18 {
        &self.pio0_18
    }
    #[doc = "0x4c - Digital I/O control for port 0 pins PIO0_19"]
    #[inline(always)]
    pub const fn pio0_19(&self) -> &Pio0_19 {
        &self.pio0_19
    }
    #[doc = "0x50 - Digital I/O control for port 0 pins PIO0_20"]
    #[inline(always)]
    pub const fn pio0_20(&self) -> &Pio0_20 {
        &self.pio0_20
    }
    #[doc = "0x54 - Digital I/O control for port 0 pins PIO0_21"]
    #[inline(always)]
    pub const fn pio0_21(&self) -> &Pio0_21 {
        &self.pio0_21
    }
    #[doc = "0x58 - Digital I/O control for port 0 pins PIO0_22"]
    #[inline(always)]
    pub const fn pio0_22(&self) -> &Pio0_22 {
        &self.pio0_22
    }
    #[doc = "0x5c - Digital I/O control for port 0 pins PIO0_23"]
    #[inline(always)]
    pub const fn pio0_23(&self) -> &Pio0_23 {
        &self.pio0_23
    }
    #[doc = "0x60 - Digital I/O control for port 0 pins PIO0_24"]
    #[inline(always)]
    pub const fn pio0_24(&self) -> &Pio0_24 {
        &self.pio0_24
    }
    #[doc = "0x64 - Digital I/O control for port 0 pins PIO0_25"]
    #[inline(always)]
    pub const fn pio0_25(&self) -> &Pio0_25 {
        &self.pio0_25
    }
    #[doc = "0x68 - Digital I/O control for port 0 pins PIO0_26"]
    #[inline(always)]
    pub const fn pio0_26(&self) -> &Pio0_26 {
        &self.pio0_26
    }
    #[doc = "0x6c - Digital I/O control for port 0 pins PIO0_27"]
    #[inline(always)]
    pub const fn pio0_27(&self) -> &Pio0_27 {
        &self.pio0_27
    }
    #[doc = "0x70 - Digital I/O control for port 0 pins PIO0_28"]
    #[inline(always)]
    pub const fn pio0_28(&self) -> &Pio0_28 {
        &self.pio0_28
    }
    #[doc = "0x74 - Digital I/O control for port 0 pins PIO0_29"]
    #[inline(always)]
    pub const fn pio0_29(&self) -> &Pio0_29 {
        &self.pio0_29
    }
    #[doc = "0x78 - Digital I/O control for port 0 pins PIO0_30"]
    #[inline(always)]
    pub const fn pio0_30(&self) -> &Pio0_30 {
        &self.pio0_30
    }
    #[doc = "0x7c - Digital I/O control for port 0 pins PIO0_31"]
    #[inline(always)]
    pub const fn pio0_31(&self) -> &Pio0_31 {
        &self.pio0_31
    }
    #[doc = "0x80 - Digital I/O control for port 1 pins PIO1_0"]
    #[inline(always)]
    pub const fn pio1_0(&self) -> &Pio1_0 {
        &self.pio1_0
    }
    #[doc = "0x84 - Digital I/O control for port 1 pins PIO1_1"]
    #[inline(always)]
    pub const fn pio1_1(&self) -> &Pio1_1 {
        &self.pio1_1
    }
    #[doc = "0x88 - Digital I/O control for port 1 pins PIO1_2"]
    #[inline(always)]
    pub const fn pio1_2(&self) -> &Pio1_2 {
        &self.pio1_2
    }
    #[doc = "0x8c - Digital I/O control for port 1 pins PIO1_3"]
    #[inline(always)]
    pub const fn pio1_3(&self) -> &Pio1_3 {
        &self.pio1_3
    }
    #[doc = "0x90 - Digital I/O control for port 1 pins PIO1_4"]
    #[inline(always)]
    pub const fn pio1_4(&self) -> &Pio1_4 {
        &self.pio1_4
    }
    #[doc = "0x94 - Digital I/O control for port 1 pins PIO1_5"]
    #[inline(always)]
    pub const fn pio1_5(&self) -> &Pio1_5 {
        &self.pio1_5
    }
    #[doc = "0x98 - Digital I/O control for port 1 pins PIO1_6"]
    #[inline(always)]
    pub const fn pio1_6(&self) -> &Pio1_6 {
        &self.pio1_6
    }
    #[doc = "0x9c - Digital I/O control for port 1 pins PIO1_7"]
    #[inline(always)]
    pub const fn pio1_7(&self) -> &Pio1_7 {
        &self.pio1_7
    }
    #[doc = "0xa0 - Digital I/O control for port 1 pins PIO1_8"]
    #[inline(always)]
    pub const fn pio1_8(&self) -> &Pio1_8 {
        &self.pio1_8
    }
    #[doc = "0xa4 - Digital I/O control for port 1 pins PIO1_9"]
    #[inline(always)]
    pub const fn pio1_9(&self) -> &Pio1_9 {
        &self.pio1_9
    }
    #[doc = "0xa8 - Digital I/O control for port 1 pins PIO1_10"]
    #[inline(always)]
    pub const fn pio1_10(&self) -> &Pio1_10 {
        &self.pio1_10
    }
    #[doc = "0xac - Digital I/O control for port 1 pins PIO1_11"]
    #[inline(always)]
    pub const fn pio1_11(&self) -> &Pio1_11 {
        &self.pio1_11
    }
    #[doc = "0xb0 - Digital I/O control for port 1 pins PIO1_12"]
    #[inline(always)]
    pub const fn pio1_12(&self) -> &Pio1_12 {
        &self.pio1_12
    }
    #[doc = "0xb4 - Digital I/O control for port 1 pins PIO1_13"]
    #[inline(always)]
    pub const fn pio1_13(&self) -> &Pio1_13 {
        &self.pio1_13
    }
    #[doc = "0xb8 - Digital I/O control for port 1 pins PIO1_14"]
    #[inline(always)]
    pub const fn pio1_14(&self) -> &Pio1_14 {
        &self.pio1_14
    }
    #[doc = "0xbc - Digital I/O control for port 1 pins PIO1_15"]
    #[inline(always)]
    pub const fn pio1_15(&self) -> &Pio1_15 {
        &self.pio1_15
    }
    #[doc = "0xc0 - Digital I/O control for port 1 pins PIO1_16"]
    #[inline(always)]
    pub const fn pio1_16(&self) -> &Pio1_16 {
        &self.pio1_16
    }
    #[doc = "0xc4 - Digital I/O control for port 1 pins PIO1_17"]
    #[inline(always)]
    pub const fn pio1_17(&self) -> &Pio1_17 {
        &self.pio1_17
    }
    #[doc = "0xc8 - Digital I/O control for port 1 pins PIO1_18"]
    #[inline(always)]
    pub const fn pio1_18(&self) -> &Pio1_18 {
        &self.pio1_18
    }
    #[doc = "0xcc - Digital I/O control for port 1 pins PIO1_19"]
    #[inline(always)]
    pub const fn pio1_19(&self) -> &Pio1_19 {
        &self.pio1_19
    }
    #[doc = "0xd0 - Digital I/O control for port 1 pins PIO1_20"]
    #[inline(always)]
    pub const fn pio1_20(&self) -> &Pio1_20 {
        &self.pio1_20
    }
    #[doc = "0xd4 - Digital I/O control for port 1 pins PIO1_21"]
    #[inline(always)]
    pub const fn pio1_21(&self) -> &Pio1_21 {
        &self.pio1_21
    }
    #[doc = "0xd8 - Digital I/O control for port 1 pins PIO1_22"]
    #[inline(always)]
    pub const fn pio1_22(&self) -> &Pio1_22 {
        &self.pio1_22
    }
    #[doc = "0xdc - Digital I/O control for port 1 pins PIO1_23"]
    #[inline(always)]
    pub const fn pio1_23(&self) -> &Pio1_23 {
        &self.pio1_23
    }
    #[doc = "0xe0 - Digital I/O control for port 1 pins PIO1_24"]
    #[inline(always)]
    pub const fn pio1_24(&self) -> &Pio1_24 {
        &self.pio1_24
    }
    #[doc = "0xe4 - Digital I/O control for port 1 pins PIO1_25"]
    #[inline(always)]
    pub const fn pio1_25(&self) -> &Pio1_25 {
        &self.pio1_25
    }
    #[doc = "0xe8 - Digital I/O control for port 1 pins PIO1_26"]
    #[inline(always)]
    pub const fn pio1_26(&self) -> &Pio1_26 {
        &self.pio1_26
    }
    #[doc = "0xec - Digital I/O control for port 1 pins PIO1_27"]
    #[inline(always)]
    pub const fn pio1_27(&self) -> &Pio1_27 {
        &self.pio1_27
    }
    #[doc = "0xf0 - Digital I/O control for port 1 pins PIO1_28"]
    #[inline(always)]
    pub const fn pio1_28(&self) -> &Pio1_28 {
        &self.pio1_28
    }
    #[doc = "0xf4 - Digital I/O control for port 1 pins PIO1_29"]
    #[inline(always)]
    pub const fn pio1_29(&self) -> &Pio1_29 {
        &self.pio1_29
    }
    #[doc = "0xf8 - Digital I/O control for port 1 pins PIO1_30"]
    #[inline(always)]
    pub const fn pio1_30(&self) -> &Pio1_30 {
        &self.pio1_30
    }
    #[doc = "0xfc - Digital I/O control for port 1 pins PIO1_31"]
    #[inline(always)]
    pub const fn pio1_31(&self) -> &Pio1_31 {
        &self.pio1_31
    }
}
#[doc = "PIO0_0 (rw) register accessor: Digital I/O control for port 0 pins PIO0_0\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_0`] module"]
#[doc(alias = "PIO0_0")]
pub type Pio0_0 = crate::Reg<pio0_0::Pio0_0Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_0"]
pub mod pio0_0;
#[doc = "PIO0_1 (rw) register accessor: Digital I/O control for port 0 pins PIO0_1\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_1`] module"]
#[doc(alias = "PIO0_1")]
pub type Pio0_1 = crate::Reg<pio0_1::Pio0_1Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
pub mod pio0_1;
#[doc = "PIO0_2 (rw) register accessor: Digital I/O control for port 0 pins PIO0_2\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_2`] module"]
#[doc(alias = "PIO0_2")]
pub type Pio0_2 = crate::Reg<pio0_2::Pio0_2Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
pub mod pio0_2;
#[doc = "PIO0_3 (rw) register accessor: Digital I/O control for port 0 pins PIO0_3\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_3`] module"]
#[doc(alias = "PIO0_3")]
pub type Pio0_3 = crate::Reg<pio0_3::Pio0_3Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
pub mod pio0_3;
#[doc = "PIO0_4 (rw) register accessor: Digital I/O control for port 0 pins PIO0_4\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_4`] module"]
#[doc(alias = "PIO0_4")]
pub type Pio0_4 = crate::Reg<pio0_4::Pio0_4Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
pub mod pio0_4;
#[doc = "PIO0_5 (rw) register accessor: Digital I/O control for port 0 pins PIO0_5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_5`] module"]
#[doc(alias = "PIO0_5")]
pub type Pio0_5 = crate::Reg<pio0_5::Pio0_5Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
pub mod pio0_5;
#[doc = "PIO0_6 (rw) register accessor: Digital I/O control for port 0 pins PIO0_6\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_6`] module"]
#[doc(alias = "PIO0_6")]
pub type Pio0_6 = crate::Reg<pio0_6::Pio0_6Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
pub mod pio0_6;
#[doc = "PIO0_7 (rw) register accessor: Digital I/O control for port 0 pins PIO0_7\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_7`] module"]
#[doc(alias = "PIO0_7")]
pub type Pio0_7 = crate::Reg<pio0_7::Pio0_7Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
pub mod pio0_7;
#[doc = "PIO0_8 (rw) register accessor: Digital I/O control for port 0 pins PIO0_8\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_8`] module"]
#[doc(alias = "PIO0_8")]
pub type Pio0_8 = crate::Reg<pio0_8::Pio0_8Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
pub mod pio0_8;
#[doc = "PIO0_9 (rw) register accessor: Digital I/O control for port 0 pins PIO0_9\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_9`] module"]
#[doc(alias = "PIO0_9")]
pub type Pio0_9 = crate::Reg<pio0_9::Pio0_9Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
pub mod pio0_9;
#[doc = "PIO0_10 (rw) register accessor: Digital I/O control for port 0 pins PIO0_10\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_10`] module"]
#[doc(alias = "PIO0_10")]
pub type Pio0_10 = crate::Reg<pio0_10::Pio0_10Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
pub mod pio0_10;
#[doc = "PIO0_11 (rw) register accessor: Digital I/O control for port 0 pins PIO0_11\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_11`] module"]
#[doc(alias = "PIO0_11")]
pub type Pio0_11 = crate::Reg<pio0_11::Pio0_11Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
pub mod pio0_11;
#[doc = "PIO0_12 (rw) register accessor: Digital I/O control for port 0 pins PIO0_12\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_12`] module"]
#[doc(alias = "PIO0_12")]
pub type Pio0_12 = crate::Reg<pio0_12::Pio0_12Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
pub mod pio0_12;
#[doc = "PIO0_13 (rw) register accessor: Digital I/O control for port 0 pins PIO0_13\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_13`] module"]
#[doc(alias = "PIO0_13")]
pub type Pio0_13 = crate::Reg<pio0_13::Pio0_13Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
pub mod pio0_13;
#[doc = "PIO0_14 (rw) register accessor: Digital I/O control for port 0 pins PIO0_14\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_14`] module"]
#[doc(alias = "PIO0_14")]
pub type Pio0_14 = crate::Reg<pio0_14::Pio0_14Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
pub mod pio0_14;
#[doc = "PIO0_15 (rw) register accessor: Digital I/O control for port 0 pins PIO0_15\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_15`] module"]
#[doc(alias = "PIO0_15")]
pub type Pio0_15 = crate::Reg<pio0_15::Pio0_15Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
pub mod pio0_15;
#[doc = "PIO0_16 (rw) register accessor: Digital I/O control for port 0 pins PIO0_16\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_16`] module"]
#[doc(alias = "PIO0_16")]
pub type Pio0_16 = crate::Reg<pio0_16::Pio0_16Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
pub mod pio0_16;
#[doc = "PIO0_17 (rw) register accessor: Digital I/O control for port 0 pins PIO0_17\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_17`] module"]
#[doc(alias = "PIO0_17")]
pub type Pio0_17 = crate::Reg<pio0_17::Pio0_17Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
pub mod pio0_17;
#[doc = "PIO0_18 (rw) register accessor: Digital I/O control for port 0 pins PIO0_18\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_18`] module"]
#[doc(alias = "PIO0_18")]
pub type Pio0_18 = crate::Reg<pio0_18::Pio0_18Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
pub mod pio0_18;
#[doc = "PIO0_19 (rw) register accessor: Digital I/O control for port 0 pins PIO0_19\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_19`] module"]
#[doc(alias = "PIO0_19")]
pub type Pio0_19 = crate::Reg<pio0_19::Pio0_19Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
pub mod pio0_19;
#[doc = "PIO0_20 (rw) register accessor: Digital I/O control for port 0 pins PIO0_20\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_20`] module"]
#[doc(alias = "PIO0_20")]
pub type Pio0_20 = crate::Reg<pio0_20::Pio0_20Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
pub mod pio0_20;
#[doc = "PIO0_21 (rw) register accessor: Digital I/O control for port 0 pins PIO0_21\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_21`] module"]
#[doc(alias = "PIO0_21")]
pub type Pio0_21 = crate::Reg<pio0_21::Pio0_21Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
pub mod pio0_21;
#[doc = "PIO0_22 (rw) register accessor: Digital I/O control for port 0 pins PIO0_22\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_22`] module"]
#[doc(alias = "PIO0_22")]
pub type Pio0_22 = crate::Reg<pio0_22::Pio0_22Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
pub mod pio0_22;
#[doc = "PIO0_23 (rw) register accessor: Digital I/O control for port 0 pins PIO0_23\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_23`] module"]
#[doc(alias = "PIO0_23")]
pub type Pio0_23 = crate::Reg<pio0_23::Pio0_23Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
pub mod pio0_23;
#[doc = "PIO0_24 (rw) register accessor: Digital I/O control for port 0 pins PIO0_24\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_24`] module"]
#[doc(alias = "PIO0_24")]
pub type Pio0_24 = crate::Reg<pio0_24::Pio0_24Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
pub mod pio0_24;
#[doc = "PIO0_25 (rw) register accessor: Digital I/O control for port 0 pins PIO0_25\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_25`] module"]
#[doc(alias = "PIO0_25")]
pub type Pio0_25 = crate::Reg<pio0_25::Pio0_25Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
pub mod pio0_25;
#[doc = "PIO0_26 (rw) register accessor: Digital I/O control for port 0 pins PIO0_26\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_26`] module"]
#[doc(alias = "PIO0_26")]
pub type Pio0_26 = crate::Reg<pio0_26::Pio0_26Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
pub mod pio0_26;
#[doc = "PIO0_27 (rw) register accessor: Digital I/O control for port 0 pins PIO0_27\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_27`] module"]
#[doc(alias = "PIO0_27")]
pub type Pio0_27 = crate::Reg<pio0_27::Pio0_27Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
pub mod pio0_27;
#[doc = "PIO0_28 (rw) register accessor: Digital I/O control for port 0 pins PIO0_28\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_28`] module"]
#[doc(alias = "PIO0_28")]
pub type Pio0_28 = crate::Reg<pio0_28::Pio0_28Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
pub mod pio0_28;
#[doc = "PIO0_29 (rw) register accessor: Digital I/O control for port 0 pins PIO0_29\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_29`] module"]
#[doc(alias = "PIO0_29")]
pub type Pio0_29 = crate::Reg<pio0_29::Pio0_29Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
pub mod pio0_29;
#[doc = "PIO0_30 (rw) register accessor: Digital I/O control for port 0 pins PIO0_30\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_30`] module"]
#[doc(alias = "PIO0_30")]
pub type Pio0_30 = crate::Reg<pio0_30::Pio0_30Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
pub mod pio0_30;
#[doc = "PIO0_31 (rw) register accessor: Digital I/O control for port 0 pins PIO0_31\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio0_31`] module"]
#[doc(alias = "PIO0_31")]
pub type Pio0_31 = crate::Reg<pio0_31::Pio0_31Spec>;
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
pub mod pio0_31;
#[doc = "PIO1_0 (rw) register accessor: Digital I/O control for port 1 pins PIO1_0\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_0`] module"]
#[doc(alias = "PIO1_0")]
pub type Pio1_0 = crate::Reg<pio1_0::Pio1_0Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
pub mod pio1_0;
#[doc = "PIO1_1 (rw) register accessor: Digital I/O control for port 1 pins PIO1_1\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_1`] module"]
#[doc(alias = "PIO1_1")]
pub type Pio1_1 = crate::Reg<pio1_1::Pio1_1Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
pub mod pio1_1;
#[doc = "PIO1_2 (rw) register accessor: Digital I/O control for port 1 pins PIO1_2\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_2`] module"]
#[doc(alias = "PIO1_2")]
pub type Pio1_2 = crate::Reg<pio1_2::Pio1_2Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
pub mod pio1_2;
#[doc = "PIO1_3 (rw) register accessor: Digital I/O control for port 1 pins PIO1_3\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_3`] module"]
#[doc(alias = "PIO1_3")]
pub type Pio1_3 = crate::Reg<pio1_3::Pio1_3Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
pub mod pio1_3;
#[doc = "PIO1_4 (rw) register accessor: Digital I/O control for port 1 pins PIO1_4\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_4`] module"]
#[doc(alias = "PIO1_4")]
pub type Pio1_4 = crate::Reg<pio1_4::Pio1_4Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
pub mod pio1_4;
#[doc = "PIO1_5 (rw) register accessor: Digital I/O control for port 1 pins PIO1_5\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_5`] module"]
#[doc(alias = "PIO1_5")]
pub type Pio1_5 = crate::Reg<pio1_5::Pio1_5Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
pub mod pio1_5;
#[doc = "PIO1_6 (rw) register accessor: Digital I/O control for port 1 pins PIO1_6\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_6`] module"]
#[doc(alias = "PIO1_6")]
pub type Pio1_6 = crate::Reg<pio1_6::Pio1_6Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
pub mod pio1_6;
#[doc = "PIO1_7 (rw) register accessor: Digital I/O control for port 1 pins PIO1_7\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_7`] module"]
#[doc(alias = "PIO1_7")]
pub type Pio1_7 = crate::Reg<pio1_7::Pio1_7Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
pub mod pio1_7;
#[doc = "PIO1_8 (rw) register accessor: Digital I/O control for port 1 pins PIO1_8\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_8`] module"]
#[doc(alias = "PIO1_8")]
pub type Pio1_8 = crate::Reg<pio1_8::Pio1_8Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
pub mod pio1_8;
#[doc = "PIO1_9 (rw) register accessor: Digital I/O control for port 1 pins PIO1_9\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_9`] module"]
#[doc(alias = "PIO1_9")]
pub type Pio1_9 = crate::Reg<pio1_9::Pio1_9Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
pub mod pio1_9;
#[doc = "PIO1_10 (rw) register accessor: Digital I/O control for port 1 pins PIO1_10\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_10`] module"]
#[doc(alias = "PIO1_10")]
pub type Pio1_10 = crate::Reg<pio1_10::Pio1_10Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
pub mod pio1_10;
#[doc = "PIO1_11 (rw) register accessor: Digital I/O control for port 1 pins PIO1_11\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_11`] module"]
#[doc(alias = "PIO1_11")]
pub type Pio1_11 = crate::Reg<pio1_11::Pio1_11Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
pub mod pio1_11;
#[doc = "PIO1_12 (rw) register accessor: Digital I/O control for port 1 pins PIO1_12\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_12`] module"]
#[doc(alias = "PIO1_12")]
pub type Pio1_12 = crate::Reg<pio1_12::Pio1_12Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
pub mod pio1_12;
#[doc = "PIO1_13 (rw) register accessor: Digital I/O control for port 1 pins PIO1_13\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_13`] module"]
#[doc(alias = "PIO1_13")]
pub type Pio1_13 = crate::Reg<pio1_13::Pio1_13Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
pub mod pio1_13;
#[doc = "PIO1_14 (rw) register accessor: Digital I/O control for port 1 pins PIO1_14\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_14`] module"]
#[doc(alias = "PIO1_14")]
pub type Pio1_14 = crate::Reg<pio1_14::Pio1_14Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
pub mod pio1_14;
#[doc = "PIO1_15 (rw) register accessor: Digital I/O control for port 1 pins PIO1_15\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_15`] module"]
#[doc(alias = "PIO1_15")]
pub type Pio1_15 = crate::Reg<pio1_15::Pio1_15Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
pub mod pio1_15;
#[doc = "PIO1_16 (rw) register accessor: Digital I/O control for port 1 pins PIO1_16\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_16`] module"]
#[doc(alias = "PIO1_16")]
pub type Pio1_16 = crate::Reg<pio1_16::Pio1_16Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
pub mod pio1_16;
#[doc = "PIO1_17 (rw) register accessor: Digital I/O control for port 1 pins PIO1_17\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_17`] module"]
#[doc(alias = "PIO1_17")]
pub type Pio1_17 = crate::Reg<pio1_17::Pio1_17Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
pub mod pio1_17;
#[doc = "PIO1_18 (rw) register accessor: Digital I/O control for port 1 pins PIO1_18\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_18`] module"]
#[doc(alias = "PIO1_18")]
pub type Pio1_18 = crate::Reg<pio1_18::Pio1_18Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
pub mod pio1_18;
#[doc = "PIO1_19 (rw) register accessor: Digital I/O control for port 1 pins PIO1_19\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_19`] module"]
#[doc(alias = "PIO1_19")]
pub type Pio1_19 = crate::Reg<pio1_19::Pio1_19Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
pub mod pio1_19;
#[doc = "PIO1_20 (rw) register accessor: Digital I/O control for port 1 pins PIO1_20\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_20`] module"]
#[doc(alias = "PIO1_20")]
pub type Pio1_20 = crate::Reg<pio1_20::Pio1_20Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
pub mod pio1_20;
#[doc = "PIO1_21 (rw) register accessor: Digital I/O control for port 1 pins PIO1_21\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_21`] module"]
#[doc(alias = "PIO1_21")]
pub type Pio1_21 = crate::Reg<pio1_21::Pio1_21Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
pub mod pio1_21;
#[doc = "PIO1_22 (rw) register accessor: Digital I/O control for port 1 pins PIO1_22\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_22`] module"]
#[doc(alias = "PIO1_22")]
pub type Pio1_22 = crate::Reg<pio1_22::Pio1_22Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
pub mod pio1_22;
#[doc = "PIO1_23 (rw) register accessor: Digital I/O control for port 1 pins PIO1_23\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_23`] module"]
#[doc(alias = "PIO1_23")]
pub type Pio1_23 = crate::Reg<pio1_23::Pio1_23Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
pub mod pio1_23;
#[doc = "PIO1_24 (rw) register accessor: Digital I/O control for port 1 pins PIO1_24\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_24`] module"]
#[doc(alias = "PIO1_24")]
pub type Pio1_24 = crate::Reg<pio1_24::Pio1_24Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
pub mod pio1_24;
#[doc = "PIO1_25 (rw) register accessor: Digital I/O control for port 1 pins PIO1_25\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_25`] module"]
#[doc(alias = "PIO1_25")]
pub type Pio1_25 = crate::Reg<pio1_25::Pio1_25Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
pub mod pio1_25;
#[doc = "PIO1_26 (rw) register accessor: Digital I/O control for port 1 pins PIO1_26\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_26`] module"]
#[doc(alias = "PIO1_26")]
pub type Pio1_26 = crate::Reg<pio1_26::Pio1_26Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
pub mod pio1_26;
#[doc = "PIO1_27 (rw) register accessor: Digital I/O control for port 1 pins PIO1_27\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_27`] module"]
#[doc(alias = "PIO1_27")]
pub type Pio1_27 = crate::Reg<pio1_27::Pio1_27Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
pub mod pio1_27;
#[doc = "PIO1_28 (rw) register accessor: Digital I/O control for port 1 pins PIO1_28\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_28`] module"]
#[doc(alias = "PIO1_28")]
pub type Pio1_28 = crate::Reg<pio1_28::Pio1_28Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
pub mod pio1_28;
#[doc = "PIO1_29 (rw) register accessor: Digital I/O control for port 1 pins PIO1_29\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_29`] module"]
#[doc(alias = "PIO1_29")]
pub type Pio1_29 = crate::Reg<pio1_29::Pio1_29Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
pub mod pio1_29;
#[doc = "PIO1_30 (rw) register accessor: Digital I/O control for port 1 pins PIO1_30\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_30`] module"]
#[doc(alias = "PIO1_30")]
pub type Pio1_30 = crate::Reg<pio1_30::Pio1_30Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
pub mod pio1_30;
#[doc = "PIO1_31 (rw) register accessor: Digital I/O control for port 1 pins PIO1_31\n\nYou can [`read`](crate::Reg::read) this register and get [`pio1_31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1_31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio1_31`] module"]
#[doc(alias = "PIO1_31")]
pub type Pio1_31 = crate::Reg<pio1_31::Pio1_31Spec>;
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
pub mod pio1_31;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    b: [B; 2],
    _reserved1: [u8; 0x0fc0],
    w: [W; 2],
    _reserved2: [u8; 0x0f00],
    dir: [Dir; 2],
    _reserved3: [u8; 0x78],
    mask: [Mask; 2],
    _reserved4: [u8; 0x78],
    pin: [Pin; 2],
    _reserved5: [u8; 0x78],
    mpin: [Mpin; 2],
    _reserved6: [u8; 0x78],
    set: [Set; 2],
    _reserved7: [u8; 0x78],
    clr: [Clr; 2],
    _reserved8: [u8; 0x78],
    not: [Not; 2],
    _reserved9: [u8; 0x78],
    dirset: [Dirset; 2],
    _reserved10: [u8; 0x78],
    dirclr: [Dirclr; 2],
    _reserved11: [u8; 0x78],
    dirnot: [Dirnot; 2],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub const fn b(&self, n: usize) -> &B {
        &self.b[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub fn b_iter(&self) -> impl Iterator<Item = &B> {
        self.b.iter()
    }
    #[doc = "0x1000..0x1100 - no description available"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1100 - no description available"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0x2000..0x2008 - Direction registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn dir(&self, n: usize) -> &Dir {
        &self.dir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2008 - Direction registers for all port GPIO pins"]
    #[inline(always)]
    pub fn dir_iter(&self) -> impl Iterator<Item = &Dir> {
        self.dir.iter()
    }
    #[doc = "0x2080..0x2088 - Mask register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        &self.mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2080..0x2088 - Mask register for all port GPIO pins"]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        self.mask.iter()
    }
    #[doc = "0x2100..0x2108 - Port pin register for all port GPIO pins"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &Pin {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2100..0x2108 - Port pin register for all port GPIO pins"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &Pin> {
        self.pin.iter()
    }
    #[doc = "0x2180..0x2188 - Masked port register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mpin(&self, n: usize) -> &Mpin {
        &self.mpin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2180..0x2188 - Masked port register for all port GPIO pins"]
    #[inline(always)]
    pub fn mpin_iter(&self) -> impl Iterator<Item = &Mpin> {
        self.mpin.iter()
    }
    #[doc = "0x2200..0x2208 - Write: Set register for port. Read: output bits for port"]
    #[inline(always)]
    pub const fn set(&self, n: usize) -> &Set {
        &self.set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2200..0x2208 - Write: Set register for port. Read: output bits for port"]
    #[inline(always)]
    pub fn set_iter(&self) -> impl Iterator<Item = &Set> {
        self.set.iter()
    }
    #[doc = "0x2280..0x2288 - Clear port for all port GPIO pins"]
    #[inline(always)]
    pub const fn clr(&self, n: usize) -> &Clr {
        &self.clr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2280..0x2288 - Clear port for all port GPIO pins"]
    #[inline(always)]
    pub fn clr_iter(&self) -> impl Iterator<Item = &Clr> {
        self.clr.iter()
    }
    #[doc = "0x2300..0x2308 - Toggle port for all port GPIO pins"]
    #[inline(always)]
    pub const fn not(&self, n: usize) -> &Not {
        &self.not[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2300..0x2308 - Toggle port for all port GPIO pins"]
    #[inline(always)]
    pub fn not_iter(&self) -> impl Iterator<Item = &Not> {
        self.not.iter()
    }
    #[doc = "0x2380..0x2388 - Set pin direction bits for port"]
    #[inline(always)]
    pub const fn dirset(&self, n: usize) -> &Dirset {
        &self.dirset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2380..0x2388 - Set pin direction bits for port"]
    #[inline(always)]
    pub fn dirset_iter(&self) -> impl Iterator<Item = &Dirset> {
        self.dirset.iter()
    }
    #[doc = "0x2400..0x2408 - Clear pin direction bits for port"]
    #[inline(always)]
    pub const fn dirclr(&self, n: usize) -> &Dirclr {
        &self.dirclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2400..0x2408 - Clear pin direction bits for port"]
    #[inline(always)]
    pub fn dirclr_iter(&self) -> impl Iterator<Item = &Dirclr> {
        self.dirclr.iter()
    }
    #[doc = "0x2480..0x2488 - Toggle pin direction bits for port"]
    #[inline(always)]
    pub const fn dirnot(&self, n: usize) -> &Dirnot {
        &self.dirnot[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2480..0x2488 - Toggle pin direction bits for port"]
    #[inline(always)]
    pub fn dirnot_iter(&self) -> impl Iterator<Item = &Dirnot> {
        self.dirnot.iter()
    }
}
#[doc = "no description available"]
pub use self::b::B;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod b;
#[doc = "no description available"]
pub use self::w::W;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod w;
#[doc = "DIR (rw) register accessor: Direction registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Direction registers for all port GPIO pins"]
pub mod dir;
#[doc = "MASK (rw) register accessor: Mask register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`] module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Mask register for all port GPIO pins"]
pub mod mask;
#[doc = "PIN (rw) register accessor: Port pin register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
#[doc(alias = "PIN")]
pub type Pin = crate::Reg<pin::PinSpec>;
#[doc = "Port pin register for all port GPIO pins"]
pub mod pin;
#[doc = "MPIN (rw) register accessor: Masked port register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mpin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpin`] module"]
#[doc(alias = "MPIN")]
pub type Mpin = crate::Reg<mpin::MpinSpec>;
#[doc = "Masked port register for all port GPIO pins"]
pub mod mpin;
#[doc = "SET (rw) register accessor: Write: Set register for port. Read: output bits for port\n\nYou can [`read`](crate::Reg::read) this register and get [`set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`] module"]
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = "Write: Set register for port. Read: output bits for port"]
pub mod set;
#[doc = "CLR (w) register accessor: Clear port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Clear port for all port GPIO pins"]
pub mod clr;
#[doc = "NOT (w) register accessor: Toggle port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`not::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@not`] module"]
#[doc(alias = "NOT")]
pub type Not = crate::Reg<not::NotSpec>;
#[doc = "Toggle port for all port GPIO pins"]
pub mod not;
#[doc = "DIRSET (w) register accessor: Set pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`] module"]
#[doc(alias = "DIRSET")]
pub type Dirset = crate::Reg<dirset::DirsetSpec>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR (w) register accessor: Clear pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`] module"]
#[doc(alias = "DIRCLR")]
pub type Dirclr = crate::Reg<dirclr::DirclrSpec>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT (w) register accessor: Toggle pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirnot::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirnot`] module"]
#[doc(alias = "DIRNOT")]
pub type Dirnot = crate::Reg<dirnot::DirnotSpec>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;

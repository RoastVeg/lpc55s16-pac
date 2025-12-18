#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    keyindex: Keyindex,
    keysize: Keysize,
    _reserved3: [u8; 0x14],
    stat: Stat,
    _reserved4: [u8; 0x04],
    allow: Allow,
    _reserved5: [u8; 0x14],
    keyinput: Keyinput,
    codeinput: Codeinput,
    codeoutput: Codeoutput,
    _reserved8: [u8; 0x14],
    keyoutindex: Keyoutindex,
    keyoutput: Keyoutput,
    _reserved10: [u8; 0x74],
    ifstat: Ifstat,
    _reserved11: [u8; 0x20],
    inten: Inten,
    intstat: Intstat,
    _reserved13: [u8; 0x04],
    cfg: Cfg,
    _reserved14: [u8; 0xf0],
    keylock: Keylock,
    keyenable: Keyenable,
    keyreset: Keyreset,
    idxblk: Idxblk,
    idxblk_dp: IdxblkDp,
    keymask: [Keymask; 4],
    _reserved20: [u8; 0x30],
    idxblk_status: IdxblkStatus,
    shift_status: ShiftStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - PUF Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - PUF Key Index register"]
    #[inline(always)]
    pub const fn keyindex(&self) -> &Keyindex {
        &self.keyindex
    }
    #[doc = "0x08 - PUF Key Size register"]
    #[inline(always)]
    pub const fn keysize(&self) -> &Keysize {
        &self.keysize
    }
    #[doc = "0x20 - PUF Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x28 - PUF Allow register"]
    #[inline(always)]
    pub const fn allow(&self) -> &Allow {
        &self.allow
    }
    #[doc = "0x40 - PUF Key Input register"]
    #[inline(always)]
    pub const fn keyinput(&self) -> &Keyinput {
        &self.keyinput
    }
    #[doc = "0x44 - PUF Code Input register"]
    #[inline(always)]
    pub const fn codeinput(&self) -> &Codeinput {
        &self.codeinput
    }
    #[doc = "0x48 - PUF Code Output register"]
    #[inline(always)]
    pub const fn codeoutput(&self) -> &Codeoutput {
        &self.codeoutput
    }
    #[doc = "0x60 - PUF Key Output Index register"]
    #[inline(always)]
    pub const fn keyoutindex(&self) -> &Keyoutindex {
        &self.keyoutindex
    }
    #[doc = "0x64 - PUF Key Output register"]
    #[inline(always)]
    pub const fn keyoutput(&self) -> &Keyoutput {
        &self.keyoutput
    }
    #[doc = "0xdc - PUF Interface Status and clear register"]
    #[inline(always)]
    pub const fn ifstat(&self) -> &Ifstat {
        &self.ifstat
    }
    #[doc = "0x100 - PUF Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x104 - PUF interrupt status"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x10c - PUF config register for block bits"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x200 - Only reset in case of full IC reset"]
    #[inline(always)]
    pub const fn keylock(&self) -> &Keylock {
        &self.keylock
    }
    #[doc = "0x204 - no description available"]
    #[inline(always)]
    pub const fn keyenable(&self) -> &Keyenable {
        &self.keyenable
    }
    #[doc = "0x208 - Reinitialize Keys shift registers counters"]
    #[inline(always)]
    pub const fn keyreset(&self) -> &Keyreset {
        &self.keyreset
    }
    #[doc = "0x20c - no description available"]
    #[inline(always)]
    pub const fn idxblk(&self) -> &Idxblk {
        &self.idxblk
    }
    #[doc = "0x210 - no description available"]
    #[inline(always)]
    pub const fn idxblk_dp(&self) -> &IdxblkDp {
        &self.idxblk_dp
    }
    #[doc = "0x214..0x224 - Only reset in case of full IC reset"]
    #[inline(always)]
    pub const fn keymask(&self, n: usize) -> &Keymask {
        &self.keymask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x214..0x224 - Only reset in case of full IC reset"]
    #[inline(always)]
    pub fn keymask_iter(&self) -> impl Iterator<Item = &Keymask> {
        self.keymask.iter()
    }
    #[doc = "0x254 - Index block status"]
    #[inline(always)]
    pub const fn idxblk_status(&self) -> &IdxblkStatus {
        &self.idxblk_status
    }
    #[doc = "0x258 - no description available"]
    #[inline(always)]
    pub const fn shift_status(&self) -> &ShiftStatus {
        &self.shift_status
    }
}
#[doc = "CTRL (rw) register accessor: PUF Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "PUF Control register"]
pub mod ctrl;
#[doc = "KEYINDEX (rw) register accessor: PUF Key Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyindex`] module"]
#[doc(alias = "KEYINDEX")]
pub type Keyindex = crate::Reg<keyindex::KeyindexSpec>;
#[doc = "PUF Key Index register"]
pub mod keyindex;
#[doc = "KEYSIZE (rw) register accessor: PUF Key Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`keysize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keysize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keysize`] module"]
#[doc(alias = "KEYSIZE")]
pub type Keysize = crate::Reg<keysize::KeysizeSpec>;
#[doc = "PUF Key Size register"]
pub mod keysize;
#[doc = "STAT (r) register accessor: PUF Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "PUF Status register"]
pub mod stat;
#[doc = "ALLOW (rw) register accessor: PUF Allow register\n\nYou can [`read`](crate::Reg::read) this register and get [`allow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`allow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@allow`] module"]
#[doc(alias = "ALLOW")]
pub type Allow = crate::Reg<allow::AllowSpec>;
#[doc = "PUF Allow register"]
pub mod allow;
#[doc = "KEYINPUT (w) register accessor: PUF Key Input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyinput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyinput`] module"]
#[doc(alias = "KEYINPUT")]
pub type Keyinput = crate::Reg<keyinput::KeyinputSpec>;
#[doc = "PUF Key Input register"]
pub mod keyinput;
#[doc = "CODEINPUT (w) register accessor: PUF Code Input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codeinput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codeinput`] module"]
#[doc(alias = "CODEINPUT")]
pub type Codeinput = crate::Reg<codeinput::CodeinputSpec>;
#[doc = "PUF Code Input register"]
pub mod codeinput;
#[doc = "CODEOUTPUT (r) register accessor: PUF Code Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`codeoutput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codeoutput`] module"]
#[doc(alias = "CODEOUTPUT")]
pub type Codeoutput = crate::Reg<codeoutput::CodeoutputSpec>;
#[doc = "PUF Code Output register"]
pub mod codeoutput;
#[doc = "KEYOUTINDEX (r) register accessor: PUF Key Output Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutindex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyoutindex`] module"]
#[doc(alias = "KEYOUTINDEX")]
pub type Keyoutindex = crate::Reg<keyoutindex::KeyoutindexSpec>;
#[doc = "PUF Key Output Index register"]
pub mod keyoutindex;
#[doc = "KEYOUTPUT (r) register accessor: PUF Key Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyoutput`] module"]
#[doc(alias = "KEYOUTPUT")]
pub type Keyoutput = crate::Reg<keyoutput::KeyoutputSpec>;
#[doc = "PUF Key Output register"]
pub mod keyoutput;
#[doc = "IFSTAT (rw) register accessor: PUF Interface Status and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifstat`] module"]
#[doc(alias = "IFSTAT")]
pub type Ifstat = crate::Reg<ifstat::IfstatSpec>;
#[doc = "PUF Interface Status and clear register"]
pub mod ifstat;
#[doc = "INTEN (rw) register accessor: PUF Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: PUF interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "PUF interrupt status"]
pub mod intstat;
#[doc = "CFG (rw) register accessor: PUF config register for block bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "PUF config register for block bits"]
pub mod cfg;
#[doc = "KEYLOCK (rw) register accessor: Only reset in case of full IC reset\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keylock`] module"]
#[doc(alias = "KEYLOCK")]
pub type Keylock = crate::Reg<keylock::KeylockSpec>;
#[doc = "Only reset in case of full IC reset"]
pub mod keylock;
#[doc = "KEYENABLE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`keyenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyenable`] module"]
#[doc(alias = "KEYENABLE")]
pub type Keyenable = crate::Reg<keyenable::KeyenableSpec>;
#[doc = "no description available"]
pub mod keyenable;
#[doc = "KEYRESET (w) register accessor: Reinitialize Keys shift registers counters\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyreset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyreset`] module"]
#[doc(alias = "KEYRESET")]
pub type Keyreset = crate::Reg<keyreset::KeyresetSpec>;
#[doc = "Reinitialize Keys shift registers counters"]
pub mod keyreset;
#[doc = "IDXBLK (w) register accessor: no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk`] module"]
#[doc(alias = "IDXBLK")]
pub type Idxblk = crate::Reg<idxblk::IdxblkSpec>;
#[doc = "no description available"]
pub mod idxblk;
#[doc = "IDXBLK_DP (w) register accessor: no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_dp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_dp`] module"]
#[doc(alias = "IDXBLK_DP")]
pub type IdxblkDp = crate::Reg<idxblk_dp::IdxblkDpSpec>;
#[doc = "no description available"]
pub mod idxblk_dp;
#[doc = "KEYMASK (w) register accessor: Only reset in case of full IC reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keymask::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keymask`] module"]
#[doc(alias = "KEYMASK")]
pub type Keymask = crate::Reg<keymask::KeymaskSpec>;
#[doc = "Only reset in case of full IC reset"]
pub mod keymask;
#[doc = "IDXBLK_STATUS (r) register accessor: Index block status\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_status`] module"]
#[doc(alias = "IDXBLK_STATUS")]
pub type IdxblkStatus = crate::Reg<idxblk_status::IdxblkStatusSpec>;
#[doc = "Index block status"]
pub mod idxblk_status;
#[doc = "SHIFT_STATUS (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`shift_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shift_status`] module"]
#[doc(alias = "SHIFT_STATUS")]
pub type ShiftStatus = crate::Reg<shift_status::ShiftStatusSpec>;
#[doc = "no description available"]
pub mod shift_status;

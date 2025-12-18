#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    header: Header,
    patch: [Patch; 255],
}
impl RegisterBlock {
    #[doc = "0x00 - ."]
    #[inline(always)]
    pub const fn header(&self) -> &Header {
        &self.header
    }
    #[doc = "0x04..0x400 - ."]
    #[inline(always)]
    pub const fn patch(&self, n: usize) -> &Patch {
        &self.patch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x400 - ."]
    #[inline(always)]
    pub fn patch_iter(&self) -> impl Iterator<Item = &Patch> {
        self.patch.iter()
    }
}
#[doc = "HEADER (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header`] module"]
#[doc(alias = "HEADER")]
pub type Header = crate::Reg<header::HeaderSpec>;
#[doc = "."]
pub mod header;
#[doc = "PATCH (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`patch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patch`] module"]
#[doc(alias = "PATCH")]
pub type Patch = crate::Reg<patch::PatchSpec>;
#[doc = "."]
pub mod patch;

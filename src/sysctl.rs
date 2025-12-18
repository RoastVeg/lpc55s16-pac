#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    updatelckout: Updatelckout,
    _reserved1: [u8; 0x3c],
    fcctrlsel: [Fcctrlsel; 8],
    _reserved2: [u8; 0x20],
    sharedctrlset: [Sharedctrlset; 2],
    _reserved3: [u8; 0x78],
    usb_hs_status: UsbHsStatus,
    _reserved4: [u8; 0x7c],
    code_gray_lsb: CodeGrayLsb,
    code_gray_msb: CodeGrayMsb,
    code_bin_lsb: CodeBinLsb,
    code_bin_msb: CodeBinMsb,
}
impl RegisterBlock {
    #[doc = "0x00 - update lock out control"]
    #[inline(always)]
    pub const fn updatelckout(&self) -> &Updatelckout {
        &self.updatelckout
    }
    #[doc = "0x40..0x60 - Selects the source for SCK going into Flexcomm index"]
    #[inline(always)]
    pub const fn fcctrlsel(&self, n: usize) -> &Fcctrlsel {
        &self.fcctrlsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Selects the source for SCK going into Flexcomm index"]
    #[inline(always)]
    pub fn fcctrlsel_iter(&self) -> impl Iterator<Item = &Fcctrlsel> {
        self.fcctrlsel.iter()
    }
    #[doc = "0x80..0x88 - Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub const fn sharedctrlset(&self, n: usize) -> &Sharedctrlset {
        &self.sharedctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub fn sharedctrlset_iter(&self) -> impl Iterator<Item = &Sharedctrlset> {
        self.sharedctrlset.iter()
    }
    #[doc = "0x100 - Status register for USB HS"]
    #[inline(always)]
    pub const fn usb_hs_status(&self) -> &UsbHsStatus {
        &self.usb_hs_status
    }
    #[doc = "0x180 - CODE_GRAY LSB input Register"]
    #[inline(always)]
    pub const fn code_gray_lsb(&self) -> &CodeGrayLsb {
        &self.code_gray_lsb
    }
    #[doc = "0x184 - CODE_GRAY MSB input Register"]
    #[inline(always)]
    pub const fn code_gray_msb(&self) -> &CodeGrayMsb {
        &self.code_gray_msb
    }
    #[doc = "0x188 - CODE_BIN LSB output Register"]
    #[inline(always)]
    pub const fn code_bin_lsb(&self) -> &CodeBinLsb {
        &self.code_bin_lsb
    }
    #[doc = "0x18c - CODE_BIN MSB output Register"]
    #[inline(always)]
    pub const fn code_bin_msb(&self) -> &CodeBinMsb {
        &self.code_bin_msb
    }
}
#[doc = "UPDATELCKOUT (rw) register accessor: update lock out control\n\nYou can [`read`](crate::Reg::read) this register and get [`updatelckout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updatelckout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@updatelckout`] module"]
#[doc(alias = "UPDATELCKOUT")]
pub type Updatelckout = crate::Reg<updatelckout::UpdatelckoutSpec>;
#[doc = "update lock out control"]
pub mod updatelckout;
#[doc = "FCCTRLSEL (rw) register accessor: Selects the source for SCK going into Flexcomm index\n\nYou can [`read`](crate::Reg::read) this register and get [`fcctrlsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcctrlsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcctrlsel`] module"]
#[doc(alias = "FCCTRLSEL")]
pub type Fcctrlsel = crate::Reg<fcctrlsel::FcctrlselSpec>;
#[doc = "Selects the source for SCK going into Flexcomm index"]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET (rw) register accessor: Selects sources and data combinations for shared signal set index.\n\nYou can [`read`](crate::Reg::read) this register and get [`sharedctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharedctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharedctrlset`] module"]
#[doc(alias = "SHAREDCTRLSET")]
pub type Sharedctrlset = crate::Reg<sharedctrlset::SharedctrlsetSpec>;
#[doc = "Selects sources and data combinations for shared signal set index."]
pub mod sharedctrlset;
#[doc = "USB_HS_STATUS (r) register accessor: Status register for USB HS\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_hs_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_hs_status`] module"]
#[doc(alias = "USB_HS_STATUS")]
pub type UsbHsStatus = crate::Reg<usb_hs_status::UsbHsStatusSpec>;
#[doc = "Status register for USB HS"]
pub mod usb_hs_status;
#[doc = "CODE_GRAY_LSB (rw) register accessor: CODE_GRAY LSB input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_gray_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`code_gray_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@code_gray_lsb`] module"]
#[doc(alias = "CODE_GRAY_LSB")]
pub type CodeGrayLsb = crate::Reg<code_gray_lsb::CodeGrayLsbSpec>;
#[doc = "CODE_GRAY LSB input Register"]
pub mod code_gray_lsb;
#[doc = "CODE_GRAY_MSB (rw) register accessor: CODE_GRAY MSB input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_gray_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`code_gray_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@code_gray_msb`] module"]
#[doc(alias = "CODE_GRAY_MSB")]
pub type CodeGrayMsb = crate::Reg<code_gray_msb::CodeGrayMsbSpec>;
#[doc = "CODE_GRAY MSB input Register"]
pub mod code_gray_msb;
#[doc = "CODE_BIN_LSB (r) register accessor: CODE_BIN LSB output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_bin_lsb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@code_bin_lsb`] module"]
#[doc(alias = "CODE_BIN_LSB")]
pub type CodeBinLsb = crate::Reg<code_bin_lsb::CodeBinLsbSpec>;
#[doc = "CODE_BIN LSB output Register"]
pub mod code_bin_lsb;
#[doc = "CODE_BIN_MSB (r) register accessor: CODE_BIN MSB output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_bin_msb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@code_bin_msb`] module"]
#[doc(alias = "CODE_BIN_MSB")]
pub type CodeBinMsb = crate::Reg<code_bin_msb::CodeBinMsbSpec>;
#[doc = "CODE_BIN MSB output Register"]
pub mod code_bin_msb;

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csw: Csw,
    request: Request,
    return_: Return,
    _reserved3: [u8; 0xf0],
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    #[inline(always)]
    pub const fn csw(&self) -> &Csw {
        &self.csw
    }
    #[doc = "0x04 - CRC seed register"]
    #[inline(always)]
    pub const fn request(&self) -> &Request {
        &self.request
    }
    #[doc = "0x08 - Return value from ROM."]
    #[inline(always)]
    pub const fn return_(&self) -> &Return {
        &self.return_
    }
    #[doc = "0xfc - Identification register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CSW (rw) register accessor: CRC mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`csw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csw`] module"]
#[doc(alias = "CSW")]
pub type Csw = crate::Reg<csw::CswSpec>;
#[doc = "CRC mode register"]
pub mod csw;
#[doc = "REQUEST (rw) register accessor: CRC seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`request::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`request::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@request`] module"]
#[doc(alias = "REQUEST")]
pub type Request = crate::Reg<request::RequestSpec>;
#[doc = "CRC seed register"]
pub mod request;
#[doc = "RETURN (rw) register accessor: Return value from ROM.\n\nYou can [`read`](crate::Reg::read) this register and get [`return_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`return_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@return_`] module"]
#[doc(alias = "RETURN")]
pub type Return = crate::Reg<return_::ReturnSpec>;
#[doc = "Return value from ROM."]
pub mod return_;
#[doc = "ID (r) register accessor: Identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Identification register"]
pub mod id;

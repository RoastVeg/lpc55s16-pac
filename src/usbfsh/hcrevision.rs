#[doc = "Register `HCREVISION` reader"]
pub type R = crate::R<HcrevisionSpec>;
#[doc = "Field `REV` reader - Revision."]
pub type RevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Revision."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCREVISION")
            .field("rev", &self.rev())
            .finish()
    }
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrevision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrevisionSpec;
impl crate::RegisterSpec for HcrevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrevision::R`](R) reader structure"]
impl crate::Readable for HcrevisionSpec {}
#[doc = "`reset()` method sets HCREVISION to value 0x10"]
impl crate::Resettable for HcrevisionSpec {
    const RESET_VALUE: u32 = 0x10;
}

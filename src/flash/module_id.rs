#[doc = "Register `MODULE_ID` reader"]
pub type R = crate::R<ModuleIdSpec>;
#[doc = "Field `APERTURE` reader - Aperture i."]
pub type ApertureR = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor revision i."]
pub type MinorRevR = crate::FieldReader;
#[doc = "Field `MAJOR_REV` reader - Major revision i."]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `ID` reader - Identifier."]
pub type IdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Aperture i."]
    #[inline(always)]
    pub fn aperture(&self) -> ApertureR {
        ApertureR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision i."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision i."]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Identifier."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Controller+Memory module identification\n\nYou can [`read`](crate::Reg::read) this register and get [`module_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModuleIdSpec;
impl crate::RegisterSpec for ModuleIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`module_id::R`](R) reader structure"]
impl crate::Readable for ModuleIdSpec {}
#[doc = "`reset()` method sets MODULE_ID to value 0xc40a_0b00"]
impl crate::Resettable for ModuleIdSpec {
    const RESET_VALUE: u32 = 0xc40a_0b00;
}

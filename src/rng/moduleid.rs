#[doc = "Register `MODULEID` reader"]
pub type R = crate::R<ModuleidSpec>;
#[doc = "Field `APERTURE` reader - Aperture i."]
pub type ApertureR = crate::FieldReader;
#[doc = "Field `MIN_REV` reader - Minor revision i."]
pub type MinRevR = crate::FieldReader;
#[doc = "Field `MAJ_REV` reader - Major revision i."]
pub type MajRevR = crate::FieldReader;
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
    pub fn min_rev(&self) -> MinRevR {
        MinRevR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision i."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MajRevR {
        MajRevR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Identifier."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODULEID")
            .field("aperture", &self.aperture())
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[doc = "IP identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`moduleid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModuleidSpec;
impl crate::RegisterSpec for ModuleidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moduleid::R`](R) reader structure"]
impl crate::Readable for ModuleidSpec {}
#[doc = "`reset()` method sets MODULEID to value 0xa0b8_4000"]
impl crate::Resettable for ModuleidSpec {
    const RESET_VALUE: u32 = 0xa0b8_4000;
}

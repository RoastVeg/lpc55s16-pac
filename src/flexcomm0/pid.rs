#[doc = "Register `PID` reader"]
pub type R = crate::R<PidSpec>;
#[doc = "Field `APERTURE` reader - size aperture for the register port on the bus (APB or AHB)."]
pub type ApertureR = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor revision of module implementation."]
pub type MinorRevR = crate::FieldReader;
#[doc = "Field `MAJOR_REV` reader - Major revision of module implementation."]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `ID` reader - Module identifier for the selected function."]
pub type IdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - size aperture for the register port on the bus (APB or AHB)."]
    #[inline(always)]
    pub fn aperture(&self) -> ApertureR {
        ApertureR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation."]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID")
            .field("aperture", &self.aperture())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[doc = "Peripheral identification register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidSpec;
impl crate::RegisterSpec for PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PidSpec {}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PidSpec {}

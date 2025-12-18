#[doc = "Register `CMPA_PROG_IN_PROGRESS` reader"]
pub type R = crate::R<CmpaProgInProgressSpec>;
#[doc = "Register `CMPA_PROG_IN_PROGRESS` writer"]
pub type W = crate::W<CmpaProgInProgressSpec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, CmpaProgInProgressSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpa_prog_in_progress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpa_prog_in_progress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpaProgInProgressSpec;
impl crate::RegisterSpec for CmpaProgInProgressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpa_prog_in_progress::R`](R) reader structure"]
impl crate::Readable for CmpaProgInProgressSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpa_prog_in_progress::W`](W) writer structure"]
impl crate::Writable for CmpaProgInProgressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPA_PROG_IN_PROGRESS to value 0"]
impl crate::Resettable for CmpaProgInProgressSpec {}

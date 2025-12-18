#[doc = "Register `SUBSEC` reader"]
pub type R = crate::R<SubsecSpec>;
#[doc = "Register `SUBSEC` writer"]
pub type W = crate::W<SubsecSpec>;
#[doc = "Field `SUBSEC` reader - A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
pub type SubsecR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub fn subsec(&self) -> SubsecR {
        SubsecR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "Sub-second counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`subsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsecSpec;
impl crate::RegisterSpec for SubsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsec::R`](R) reader structure"]
impl crate::Readable for SubsecSpec {}
#[doc = "`write(|w| ..)` method takes [`subsec::W`](W) writer structure"]
impl crate::Writable for SubsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SubsecSpec {}

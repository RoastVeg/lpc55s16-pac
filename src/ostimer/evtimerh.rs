#[doc = "Register `EVTIMERH` reader"]
pub type R = crate::R<EvtimerhSpec>;
#[doc = "Register `EVTIMERH` writer"]
pub type W = crate::W<EvtimerhSpec>;
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - A read reflects the current value of the upper 10 bits of the 42-bits EVTIMER. Note there is only one EVTIMER, readable from all domains."]
pub type EvtimerCountValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - A read reflects the current value of the upper 10 bits of the 42-bits EVTIMER. Note there is only one EVTIMER, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EvtimerCountValueR {
        EvtimerCountValueR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTIMERH")
            .field("evtimer_count_value", &self.evtimer_count_value())
            .finish()
    }
}
impl W {}
#[doc = "EVTIMER High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evtimerh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtimerh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtimerhSpec;
impl crate::RegisterSpec for EvtimerhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtimerh::R`](R) reader structure"]
impl crate::Readable for EvtimerhSpec {}
#[doc = "`write(|w| ..)` method takes [`evtimerh::W`](W) writer structure"]
impl crate::Writable for EvtimerhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVTIMERH to value 0"]
impl crate::Resettable for EvtimerhSpec {}

#[doc = "Register `CLR[%s]` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `CLRP` writer - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
pub type ClrpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp(&mut self) -> ClrpW<'_, ClrSpec> {
        ClrpW::new(self, 0)
    }
}
#[doc = "Clear port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLR[%s] to value 0"]
impl crate::Resettable for ClrSpec {}

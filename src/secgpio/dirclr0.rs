#[doc = "Register `DIRCLR0` writer"]
pub type W = crate::W<Dirclr0Spec>;
#[doc = "Field `DIRCLRP` writer - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
pub type DirclrpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub fn dirclrp(&mut self) -> DirclrpW<'_, Dirclr0Spec> {
        DirclrpW::new(self, 0)
    }
}
#[doc = "Clear pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dirclr0Spec;
impl crate::RegisterSpec for Dirclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirclr0::W`](W) writer structure"]
impl crate::Writable for Dirclr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIRCLR0 to value 0"]
impl crate::Resettable for Dirclr0Spec {}

#[doc = "Register `SETVALID0` writer"]
pub type W = crate::W<Setvalid0Spec>;
#[doc = "Field `SV` writer - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
pub type SvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Setvalid0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
    #[inline(always)]
    pub fn sv(&mut self) -> SvW<'_, Setvalid0Spec> {
        SvW::new(self, 0)
    }
}
#[doc = "Set ValidPending control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setvalid0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setvalid0Spec;
impl crate::RegisterSpec for Setvalid0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`setvalid0::W`](W) writer structure"]
impl crate::Writable for Setvalid0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SETVALID0 to value 0"]
impl crate::Resettable for Setvalid0Spec {}

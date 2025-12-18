#[doc = "Register `DMA0_ITRIG_ENA_SET` writer"]
pub type W = crate::W<Dma0ItrigEnaSetSpec>;
#[doc = "Field `SET` writer - Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl core::fmt::Debug for crate::generic::Reg<Dma0ItrigEnaSetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<'_, Dma0ItrigEnaSetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_ena_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ItrigEnaSetSpec;
impl crate::RegisterSpec for Dma0ItrigEnaSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma0_itrig_ena_set::W`](W) writer structure"]
impl crate::Writable for Dma0ItrigEnaSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_ITRIG_ENA_SET to value 0"]
impl crate::Resettable for Dma0ItrigEnaSetSpec {}

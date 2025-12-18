#[doc = "Register `DMA1_ITRIG_ENA_SET` writer"]
pub type W = crate::W<Dma1ItrigEnaSetSpec>;
#[doc = "Field `SET` writer - Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl W {
    #[doc = "Bits 0:14 - Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<'_, Dma1ItrigEnaSetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_ena_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ItrigEnaSetSpec;
impl crate::RegisterSpec for Dma1ItrigEnaSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma1_itrig_ena_set::W`](W) writer structure"]
impl crate::Writable for Dma1ItrigEnaSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_ITRIG_ENA_SET to value 0"]
impl crate::Resettable for Dma1ItrigEnaSetSpec {}

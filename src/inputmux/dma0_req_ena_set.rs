#[doc = "Register `DMA0_REQ_ENA_SET` writer"]
pub type W = crate::W<Dma0ReqEnaSetSpec>;
#[doc = "Field `SET` writer - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl W {
    #[doc = "Bits 0:22 - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<'_, Dma0ReqEnaSetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Set one or several bits in DMA0_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ReqEnaSetSpec;
impl crate::RegisterSpec for Dma0ReqEnaSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma0_req_ena_set::W`](W) writer structure"]
impl crate::Writable for Dma0ReqEnaSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_REQ_ENA_SET to value 0"]
impl crate::Resettable for Dma0ReqEnaSetSpec {}

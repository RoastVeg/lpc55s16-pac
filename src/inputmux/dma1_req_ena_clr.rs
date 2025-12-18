#[doc = "Register `DMA1_REQ_ENA_CLR` writer"]
pub type W = crate::W<Dma1ReqEnaClrSpec>;
#[doc = "Field `CLR` writer - Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl W {
    #[doc = "Bits 0:9 - Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, Dma1ReqEnaClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Clear one or several bits in DMA1_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_req_ena_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ReqEnaClrSpec;
impl crate::RegisterSpec for Dma1ReqEnaClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma1_req_ena_clr::W`](W) writer structure"]
impl crate::Writable for Dma1ReqEnaClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_REQ_ENA_CLR to value 0"]
impl crate::Resettable for Dma1ReqEnaClrSpec {}

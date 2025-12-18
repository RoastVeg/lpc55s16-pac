#[doc = "Register `DMA0_REQ_ENA_CLR` writer"]
pub type W = crate::W<Dma0ReqEnaClrSpec>;
#[doc = "Field `CLR` writer - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl core::fmt::Debug for crate::generic::Reg<Dma0ReqEnaClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:22 - Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, Dma0ReqEnaClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ReqEnaClrSpec;
impl crate::RegisterSpec for Dma0ReqEnaClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma0_req_ena_clr::W`](W) writer structure"]
impl crate::Writable for Dma0ReqEnaClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_REQ_ENA_CLR to value 0"]
impl crate::Resettable for Dma0ReqEnaClrSpec {}

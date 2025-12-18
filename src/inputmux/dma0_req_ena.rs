#[doc = "Register `DMA0_REQ_ENA` reader"]
pub type R = crate::R<Dma0ReqEnaSpec>;
#[doc = "Register `DMA0_REQ_ENA` writer"]
pub type W = crate::W<Dma0ReqEnaSpec>;
#[doc = "Field `REQ_ENA` reader - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
pub type ReqEnaR = crate::FieldReader<u32>;
#[doc = "Field `REQ_ENA` writer - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
pub type ReqEnaW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> ReqEnaR {
        ReqEnaR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&mut self) -> ReqEnaW<'_, Dma0ReqEnaSpec> {
        ReqEnaW::new(self, 0)
    }
}
#[doc = "Enable DMA0 requests\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_req_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_req_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ReqEnaSpec;
impl crate::RegisterSpec for Dma0ReqEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma0_req_ena::R`](R) reader structure"]
impl crate::Readable for Dma0ReqEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0_req_ena::W`](W) writer structure"]
impl crate::Writable for Dma0ReqEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_REQ_ENA to value 0x007f_ffff"]
impl crate::Resettable for Dma0ReqEnaSpec {
    const RESET_VALUE: u32 = 0x007f_ffff;
}

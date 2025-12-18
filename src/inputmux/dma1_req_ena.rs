#[doc = "Register `DMA1_REQ_ENA` reader"]
pub type R = crate::R<Dma1ReqEnaSpec>;
#[doc = "Register `DMA1_REQ_ENA` writer"]
pub type W = crate::W<Dma1ReqEnaSpec>;
#[doc = "Field `REQ_ENA` reader - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type ReqEnaR = crate::FieldReader<u16>;
#[doc = "Field `REQ_ENA` writer - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type ReqEnaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> ReqEnaR {
        ReqEnaR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_REQ_ENA")
            .field("req_ena", &self.req_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&mut self) -> ReqEnaW<'_, Dma1ReqEnaSpec> {
        ReqEnaW::new(self, 0)
    }
}
#[doc = "Enable DMA1 requests\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_req_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_req_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ReqEnaSpec;
impl crate::RegisterSpec for Dma1ReqEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1_req_ena::R`](R) reader structure"]
impl crate::Readable for Dma1ReqEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1_req_ena::W`](W) writer structure"]
impl crate::Writable for Dma1ReqEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_REQ_ENA to value 0x03ff"]
impl crate::Resettable for Dma1ReqEnaSpec {
    const RESET_VALUE: u32 = 0x03ff;
}

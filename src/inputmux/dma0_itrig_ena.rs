#[doc = "Register `DMA0_ITRIG_ENA` reader"]
pub type R = crate::R<Dma0ItrigEnaSpec>;
#[doc = "Register `DMA0_ITRIG_ENA` writer"]
pub type W = crate::W<Dma0ItrigEnaSpec>;
#[doc = "Field `ITRIG_ENA` reader - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ItrigEnaR = crate::FieldReader<u32>;
#[doc = "Field `ITRIG_ENA` writer - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ItrigEnaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ItrigEnaR {
        ItrigEnaR::new(self.bits & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_ENA")
            .field("itrig_ena", &self.itrig_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&mut self) -> ItrigEnaW<'_, Dma0ItrigEnaSpec> {
        ItrigEnaW::new(self, 0)
    }
}
#[doc = "Enable DMA0 triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_itrig_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ItrigEnaSpec;
impl crate::RegisterSpec for Dma0ItrigEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma0_itrig_ena::R`](R) reader structure"]
impl crate::Readable for Dma0ItrigEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0_itrig_ena::W`](W) writer structure"]
impl crate::Writable for Dma0ItrigEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_ITRIG_ENA to value 0x003f_ffff"]
impl crate::Resettable for Dma0ItrigEnaSpec {
    const RESET_VALUE: u32 = 0x003f_ffff;
}

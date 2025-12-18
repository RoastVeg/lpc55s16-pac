#[doc = "Register `DMA1_ITRIG_ENA` reader"]
pub type R = crate::R<Dma1ItrigEnaSpec>;
#[doc = "Register `DMA1_ITRIG_ENA` writer"]
pub type W = crate::W<Dma1ItrigEnaSpec>;
#[doc = "Field `ITRIG_ENA` reader - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ItrigEnaR = crate::FieldReader<u16>;
#[doc = "Field `ITRIG_ENA` writer - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
pub type ItrigEnaW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ItrigEnaR {
        ItrigEnaR::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_ENA")
            .field("itrig_ena", &self.itrig_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&mut self) -> ItrigEnaW<'_, Dma1ItrigEnaSpec> {
        ItrigEnaW::new(self, 0)
    }
}
#[doc = "Enable DMA1 triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_itrig_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ItrigEnaSpec;
impl crate::RegisterSpec for Dma1ItrigEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1_itrig_ena::R`](R) reader structure"]
impl crate::Readable for Dma1ItrigEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1_itrig_ena::W`](W) writer structure"]
impl crate::Writable for Dma1ItrigEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_ITRIG_ENA to value 0x7fff"]
impl crate::Resettable for Dma1ItrigEnaSpec {
    const RESET_VALUE: u32 = 0x7fff;
}

#[doc = "Register `DMA1_OTRIG_INMUX[%s]` reader"]
pub type R = crate::R<Dma1OtrigInmuxSpec>;
#[doc = "Register `DMA1_OTRIG_INMUX[%s]` writer"]
pub type W = crate::W<Dma1OtrigInmuxSpec>;
#[doc = "Field `INP` reader - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
pub type InpR = crate::FieldReader;
#[doc = "Field `INP` writer - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Dma1OtrigInmuxSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "DMA1 output trigger selection to become DMA1 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_otrig_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_otrig_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1OtrigInmuxSpec;
impl crate::RegisterSpec for Dma1OtrigInmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1_otrig_inmux::R`](R) reader structure"]
impl crate::Readable for Dma1OtrigInmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1_otrig_inmux::W`](W) writer structure"]
impl crate::Writable for Dma1OtrigInmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_OTRIG_INMUX[%s] to value 0x0f"]
impl crate::Resettable for Dma1OtrigInmuxSpec {
    const RESET_VALUE: u32 = 0x0f;
}

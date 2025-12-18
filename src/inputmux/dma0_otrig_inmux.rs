#[doc = "Register `DMA0_OTRIG_INMUX[%s]` reader"]
pub type R = crate::R<Dma0OtrigInmuxSpec>;
#[doc = "Register `DMA0_OTRIG_INMUX[%s]` writer"]
pub type W = crate::W<Dma0OtrigInmuxSpec>;
#[doc = "Field `INP` reader - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpR = crate::FieldReader;
#[doc = "Field `INP` writer - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Dma0OtrigInmuxSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "DMA0 output trigger selection to become DMA0 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_otrig_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_otrig_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0OtrigInmuxSpec;
impl crate::RegisterSpec for Dma0OtrigInmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma0_otrig_inmux::R`](R) reader structure"]
impl crate::Readable for Dma0OtrigInmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0_otrig_inmux::W`](W) writer structure"]
impl crate::Writable for Dma0OtrigInmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_OTRIG_INMUX[%s] to value 0x1f"]
impl crate::Resettable for Dma0OtrigInmuxSpec {
    const RESET_VALUE: u32 = 0x1f;
}

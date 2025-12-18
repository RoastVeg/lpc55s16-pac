#[doc = "Register `ERRINT0` reader"]
pub type R = crate::R<Errint0Spec>;
#[doc = "Register `ERRINT0` writer"]
pub type W = crate::W<Errint0Spec>;
#[doc = "Field `ERR` reader - Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
pub type ErrR = crate::FieldReader<u32>;
#[doc = "Field `ERR` writer - Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
pub type ErrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRINT0").field("err", &self.err()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Error Interrupt flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = error interrupt is not active. 1 = error interrupt is active."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, Errint0Spec> {
        ErrW::new(self, 0)
    }
}
#[doc = "Error Interrupt status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`errint0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Errint0Spec;
impl crate::RegisterSpec for Errint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errint0::R`](R) reader structure"]
impl crate::Readable for Errint0Spec {}
#[doc = "`write(|w| ..)` method takes [`errint0::W`](W) writer structure"]
impl crate::Writable for Errint0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRINT0 to value 0"]
impl crate::Resettable for Errint0Spec {}

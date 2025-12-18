#[doc = "Register `INTB0` reader"]
pub type R = crate::R<Intb0Spec>;
#[doc = "Register `INTB0` writer"]
pub type W = crate::W<Intb0Spec>;
#[doc = "Field `IB` reader - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
pub type IbR = crate::FieldReader<u32>;
#[doc = "Field `IB` writer - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
pub type IbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn ib(&self) -> IbR {
        IbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt B is not active. 1 = the DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn ib(&mut self) -> IbW<'_, Intb0Spec> {
        IbW::new(self, 0)
    }
}
#[doc = "Interrupt B status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intb0Spec;
impl crate::RegisterSpec for Intb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intb0::R`](R) reader structure"]
impl crate::Readable for Intb0Spec {}
#[doc = "`write(|w| ..)` method takes [`intb0::W`](W) writer structure"]
impl crate::Writable for Intb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTB0 to value 0"]
impl crate::Resettable for Intb0Spec {}

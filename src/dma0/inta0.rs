#[doc = "Register `INTA0` reader"]
pub type R = crate::R<Inta0Spec>;
#[doc = "Register `INTA0` writer"]
pub type W = crate::W<Inta0Spec>;
#[doc = "Field `IA` reader - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub type IaR = crate::FieldReader<u32>;
#[doc = "Field `IA` writer - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub type IaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn ia(&self) -> IaR {
        IaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTA0").field("ia", &self.ia()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn ia(&mut self) -> IaW<'_, Inta0Spec> {
        IaW::new(self, 0)
    }
}
#[doc = "Interrupt A status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`inta0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inta0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inta0Spec;
impl crate::RegisterSpec for Inta0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inta0::R`](R) reader structure"]
impl crate::Readable for Inta0Spec {}
#[doc = "`write(|w| ..)` method takes [`inta0::W`](W) writer structure"]
impl crate::Writable for Inta0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTA0 to value 0"]
impl crate::Resettable for Inta0Spec {}

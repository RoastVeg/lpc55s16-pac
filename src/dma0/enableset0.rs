#[doc = "Register `ENABLESET0` reader"]
pub type R = crate::R<Enableset0Spec>;
#[doc = "Register `ENABLESET0` writer"]
pub type W = crate::W<Enableset0Spec>;
#[doc = "Field `ENA` reader - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub type EnaR = crate::FieldReader<u32>;
#[doc = "Field `ENA` writer - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub type EnaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&mut self) -> EnaW<'_, Enableset0Spec> {
        EnaW::new(self, 0)
    }
}
#[doc = "Channel Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`enableset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enableset0Spec;
impl crate::RegisterSpec for Enableset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enableset0::R`](R) reader structure"]
impl crate::Readable for Enableset0Spec {}
#[doc = "`write(|w| ..)` method takes [`enableset0::W`](W) writer structure"]
impl crate::Writable for Enableset0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLESET0 to value 0"]
impl crate::Resettable for Enableset0Spec {}

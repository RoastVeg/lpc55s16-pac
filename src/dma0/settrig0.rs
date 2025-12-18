#[doc = "Register `SETTRIG0` writer"]
pub type W = crate::W<Settrig0Spec>;
#[doc = "Field `TRIG` writer - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
pub type TrigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, Settrig0Spec> {
        TrigW::new(self, 0)
    }
}
#[doc = "Set Trigger control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`settrig0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Settrig0Spec;
impl crate::RegisterSpec for Settrig0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`settrig0::W`](W) writer structure"]
impl crate::Writable for Settrig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SETTRIG0 to value 0"]
impl crate::Resettable for Settrig0Spec {}

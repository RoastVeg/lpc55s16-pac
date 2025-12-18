#[doc = "Register `ACTIVE0` reader"]
pub type R = crate::R<Active0Spec>;
#[doc = "Field `ACT` reader - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
pub type ActR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVE0").field("act", &self.act()).finish()
    }
}
#[doc = "Channel Active status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`active0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Active0Spec;
impl crate::RegisterSpec for Active0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active0::R`](R) reader structure"]
impl crate::Readable for Active0Spec {}
#[doc = "`reset()` method sets ACTIVE0 to value 0"]
impl crate::Resettable for Active0Spec {}

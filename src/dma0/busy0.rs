#[doc = "Register `BUSY0` reader"]
pub type R = crate::R<Busy0Spec>;
#[doc = "Field `BSY` reader - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
pub type BsyR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSY0").field("bsy", &self.bsy()).finish()
    }
}
#[doc = "Channel Busy status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busy0Spec;
impl crate::RegisterSpec for Busy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy0::R`](R) reader structure"]
impl crate::Readable for Busy0Spec {}
#[doc = "`reset()` method sets BUSY0 to value 0"]
impl crate::Resettable for Busy0Spec {}

#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Field `TO` reader - Cancellation finished."]
pub type ToR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation finished."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCF").field("to", &self.to()).finish()
    }
}
#[doc = "Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {}

#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TxbtoSpec>;
#[doc = "Field `TO` reader - Transmission occurred."]
pub type ToR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission occurred."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("to", &self.to()).finish()
    }
}
#[doc = "Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtoSpec;
impl crate::RegisterSpec for TxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TxbtoSpec {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TxbtoSpec {}

#[doc = "Register `STATUS2` reader"]
pub type R = crate::R<Status2Spec>;
#[doc = "Field `NUMCNTF` reader - Number (of) control faults"]
pub type NumcntfR = crate::FieldReader;
#[doc = "Field `NUMILLSTF` reader - Number (of) state faults"]
pub type NumillstfR = crate::FieldReader;
#[doc = "Field `NUMILLA` reader - Number of (illegal) address faults"]
pub type NumillaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number (of) control faults"]
    #[inline(always)]
    pub fn numcntf(&self) -> NumcntfR {
        NumcntfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number (of) state faults"]
    #[inline(always)]
    pub fn numillstf(&self) -> NumillstfR {
        NumillstfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of (illegal) address faults"]
    #[inline(always)]
    pub fn numilla(&self) -> NumillaR {
        NumillaR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS2")
            .field("numcntf", &self.numcntf())
            .field("numillstf", &self.numillstf())
            .field("numilla", &self.numilla())
            .finish()
    }
}
#[doc = "STATUS register (2 of 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status2Spec;
impl crate::RegisterSpec for Status2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status2::R`](R) reader structure"]
impl crate::Readable for Status2Spec {}
#[doc = "`reset()` method sets STATUS2 to value 0"]
impl crate::Resettable for Status2Spec {}

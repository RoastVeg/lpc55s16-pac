#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `NUMTOF` reader - Number of Timeout Faults"]
pub type NumtofR = crate::FieldReader;
#[doc = "Field `NUMMISCOMPF` reader - Number of Miscompare Faults"]
pub type NummiscompfR = crate::FieldReader;
#[doc = "Field `NUMILSEQF` reader - Number of illegal sequence faults"]
pub type NumilseqfR = crate::FieldReader;
#[doc = "Field `CURST` reader - Current State"]
pub type CurstR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of Timeout Faults"]
    #[inline(always)]
    pub fn numtof(&self) -> NumtofR {
        NumtofR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Miscompare Faults"]
    #[inline(always)]
    pub fn nummiscompf(&self) -> NummiscompfR {
        NummiscompfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of illegal sequence faults"]
    #[inline(always)]
    pub fn numilseqf(&self) -> NumilseqfR {
        NumilseqfR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Current State"]
    #[inline(always)]
    pub fn curst(&self) -> CurstR {
        CurstR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("numtof", &self.numtof())
            .field("nummiscompf", &self.nummiscompf())
            .field("numilseqf", &self.numilseqf())
            .field("curst", &self.curst())
            .finish()
    }
}
#[doc = "Status register (1 of 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

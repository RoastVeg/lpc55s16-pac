#[doc = "Register `RANDOM_NUMBER` reader"]
pub type R = crate::R<RandomNumberSpec>;
#[doc = "Field `RANDOM_NUMBER` reader - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
pub type RandomNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub fn random_number(&self) -> RandomNumberR {
        RandomNumberR::new(self.bits)
    }
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read\n\nYou can [`read`](crate::Reg::read) this register and get [`random_number::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandomNumberSpec;
impl crate::RegisterSpec for RandomNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random_number::R`](R) reader structure"]
impl crate::Readable for RandomNumberSpec {}
#[doc = "`reset()` method sets RANDOM_NUMBER to value 0"]
impl crate::Resettable for RandomNumberSpec {}

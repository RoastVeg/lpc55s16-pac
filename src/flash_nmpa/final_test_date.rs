#[doc = "Register `FINAL_TEST_DATE` reader"]
pub type R = crate::R<FinalTestDateSpec>;
#[doc = "Register `FINAL_TEST_DATE` writer"]
pub type W = crate::W<FinalTestDateSpec>;
#[doc = "Field `DATE` reader - DATE \\[stored as : year*10000+month*100+day\\]"]
pub type DateR = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - DATE \\[stored as : year*10000+month*100+day\\]"]
pub type DateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_DATE")
            .field("date", &self.date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn date(&mut self) -> DateW<'_, FinalTestDateSpec> {
        DateW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinalTestDateSpec;
impl crate::RegisterSpec for FinalTestDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`final_test_date::R`](R) reader structure"]
impl crate::Readable for FinalTestDateSpec {}
#[doc = "`write(|w| ..)` method takes [`final_test_date::W`](W) writer structure"]
impl crate::Writable for FinalTestDateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FINAL_TEST_DATE to value 0"]
impl crate::Resettable for FinalTestDateSpec {}

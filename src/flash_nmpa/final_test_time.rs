#[doc = "Register `FINAL_TEST_TIME` reader"]
pub type R = crate::R<FinalTestTimeSpec>;
#[doc = "Register `FINAL_TEST_TIME` writer"]
pub type W = crate::W<FinalTestTimeSpec>;
#[doc = "Field `TIME` reader - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type TimeR = crate::FieldReader<u32>;
#[doc = "Field `TIME` writer - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_TIME")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn time(&mut self) -> TimeW<'_, FinalTestTimeSpec> {
        TimeW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinalTestTimeSpec;
impl crate::RegisterSpec for FinalTestTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`final_test_time::R`](R) reader structure"]
impl crate::Readable for FinalTestTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`final_test_time::W`](W) writer structure"]
impl crate::Writable for FinalTestTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FINAL_TEST_TIME to value 0"]
impl crate::Resettable for FinalTestTimeSpec {}

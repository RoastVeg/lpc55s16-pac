#[doc = "Register `WAFER_TEST2_TIME` reader"]
pub type R = crate::R<WaferTest2TimeSpec>;
#[doc = "Register `WAFER_TEST2_TIME` writer"]
pub type W = crate::W<WaferTest2TimeSpec>;
#[doc = "Field `WT2_TIME` reader - WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type Wt2TimeR = crate::FieldReader<u32>;
#[doc = "Field `WT2_TIME` writer - WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type Wt2TimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn wt2_time(&self) -> Wt2TimeR {
        Wt2TimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn wt2_time(&mut self) -> Wt2TimeW<'_, WaferTest2TimeSpec> {
        Wt2TimeW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest2TimeSpec;
impl crate::RegisterSpec for WaferTest2TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test2_time::R`](R) reader structure"]
impl crate::Readable for WaferTest2TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test2_time::W`](W) writer structure"]
impl crate::Writable for WaferTest2TimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST2_TIME to value 0"]
impl crate::Resettable for WaferTest2TimeSpec {}

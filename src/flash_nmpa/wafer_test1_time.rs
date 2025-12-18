#[doc = "Register `WAFER_TEST1_TIME` reader"]
pub type R = crate::R<WaferTest1TimeSpec>;
#[doc = "Register `WAFER_TEST1_TIME` writer"]
pub type W = crate::W<WaferTest1TimeSpec>;
#[doc = "Field `WT1_TIME` reader - WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type Wt1TimeR = crate::FieldReader<u32>;
#[doc = "Field `WT1_TIME` writer - WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type Wt1TimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn wt1_time(&self) -> Wt1TimeR {
        Wt1TimeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST1_TIME")
            .field("wt1_time", &self.wt1_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn wt1_time(&mut self) -> Wt1TimeW<'_, WaferTest1TimeSpec> {
        Wt1TimeW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest1TimeSpec;
impl crate::RegisterSpec for WaferTest1TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test1_time::R`](R) reader structure"]
impl crate::Readable for WaferTest1TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test1_time::W`](W) writer structure"]
impl crate::Writable for WaferTest1TimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST1_TIME to value 0"]
impl crate::Resettable for WaferTest1TimeSpec {}

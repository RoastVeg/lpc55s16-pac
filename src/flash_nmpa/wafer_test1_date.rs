#[doc = "Register `WAFER_TEST1_DATE` reader"]
pub type R = crate::R<WaferTest1DateSpec>;
#[doc = "Register `WAFER_TEST1_DATE` writer"]
pub type W = crate::W<WaferTest1DateSpec>;
#[doc = "Field `WT1_DATE` reader - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type Wt1DateR = crate::FieldReader<u32>;
#[doc = "Field `WT1_DATE` writer - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type Wt1DateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt1_date(&self) -> Wt1DateR {
        Wt1DateR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST1_DATE")
            .field("wt1_date", &self.wt1_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt1_date(&mut self) -> Wt1DateW<'_, WaferTest1DateSpec> {
        Wt1DateW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest1DateSpec;
impl crate::RegisterSpec for WaferTest1DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test1_date::R`](R) reader structure"]
impl crate::Readable for WaferTest1DateSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test1_date::W`](W) writer structure"]
impl crate::Writable for WaferTest1DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST1_DATE to value 0"]
impl crate::Resettable for WaferTest1DateSpec {}

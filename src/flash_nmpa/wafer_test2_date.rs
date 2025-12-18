#[doc = "Register `WAFER_TEST2_DATE` reader"]
pub type R = crate::R<WaferTest2DateSpec>;
#[doc = "Register `WAFER_TEST2_DATE` writer"]
pub type W = crate::W<WaferTest2DateSpec>;
#[doc = "Field `WT2_DATE` reader - WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type Wt2DateR = crate::FieldReader<u32>;
#[doc = "Field `WT2_DATE` writer - WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type Wt2DateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt2_date(&self) -> Wt2DateR {
        Wt2DateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt2_date(&mut self) -> Wt2DateW<'_, WaferTest2DateSpec> {
        Wt2DateW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest2DateSpec;
impl crate::RegisterSpec for WaferTest2DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test2_date::R`](R) reader structure"]
impl crate::Readable for WaferTest2DateSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test2_date::W`](W) writer structure"]
impl crate::Writable for WaferTest2DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST2_DATE to value 0"]
impl crate::Resettable for WaferTest2DateSpec {}

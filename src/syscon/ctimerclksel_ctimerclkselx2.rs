#[doc = "Register `CTIMERCLKSELX2` reader"]
pub type R = crate::R<CtimerclkselCtimerclkselx2Spec>;
#[doc = "Register `CTIMERCLKSELX2` writer"]
pub type W = crate::W<CtimerclkselCtimerclkselx2Spec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, CtimerclkselCtimerclkselx2Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtimerclkselCtimerclkselx2Spec;
impl crate::RegisterSpec for CtimerclkselCtimerclkselx2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctimerclksel_ctimerclkselx2::R`](R) reader structure"]
impl crate::Readable for CtimerclkselCtimerclkselx2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctimerclksel_ctimerclkselx2::W`](W) writer structure"]
impl crate::Writable for CtimerclkselCtimerclkselx2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIMERCLKSELX2 to value 0"]
impl crate::Resettable for CtimerclkselCtimerclkselx2Spec {}

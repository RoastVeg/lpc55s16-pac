#[doc = "Register `FCCLKSELX6` reader"]
pub type R = crate::R<FcclkselFcclkselx6Spec>;
#[doc = "Register `FCCLKSELX6` writer"]
pub type W = crate::W<FcclkselFcclkselx6Spec>;
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
    pub fn data(&mut self) -> DataW<'_, FcclkselFcclkselx6Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcclkselFcclkselx6Spec;
impl crate::RegisterSpec for FcclkselFcclkselx6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcclksel_fcclkselx6::R`](R) reader structure"]
impl crate::Readable for FcclkselFcclkselx6Spec {}
#[doc = "`write(|w| ..)` method takes [`fcclksel_fcclkselx6::W`](W) writer structure"]
impl crate::Writable for FcclkselFcclkselx6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCLKSELX6 to value 0"]
impl crate::Resettable for FcclkselFcclkselx6Spec {}

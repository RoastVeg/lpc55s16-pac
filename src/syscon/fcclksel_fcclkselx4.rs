#[doc = "Register `FCCLKSELX4` reader"]
pub type R = crate::R<FcclkselFcclkselx4Spec>;
#[doc = "Register `FCCLKSELX4` writer"]
pub type W = crate::W<FcclkselFcclkselx4Spec>;
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
    pub fn data(&mut self) -> DataW<'_, FcclkselFcclkselx4Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcclkselFcclkselx4Spec;
impl crate::RegisterSpec for FcclkselFcclkselx4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcclksel_fcclkselx4::R`](R) reader structure"]
impl crate::Readable for FcclkselFcclkselx4Spec {}
#[doc = "`write(|w| ..)` method takes [`fcclksel_fcclkselx4::W`](W) writer structure"]
impl crate::Writable for FcclkselFcclkselx4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCLKSELX4 to value 0"]
impl crate::Resettable for FcclkselFcclkselx4Spec {}

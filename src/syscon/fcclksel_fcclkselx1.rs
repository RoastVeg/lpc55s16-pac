#[doc = "Register `FCCLKSELX1` reader"]
pub type R = crate::R<FcclkselFcclkselx1Spec>;
#[doc = "Register `FCCLKSELX1` writer"]
pub type W = crate::W<FcclkselFcclkselx1Spec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL_FCCLKSELX1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, FcclkselFcclkselx1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcclkselFcclkselx1Spec;
impl crate::RegisterSpec for FcclkselFcclkselx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcclksel_fcclkselx1::R`](R) reader structure"]
impl crate::Readable for FcclkselFcclkselx1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcclksel_fcclkselx1::W`](W) writer structure"]
impl crate::Writable for FcclkselFcclkselx1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCLKSELX1 to value 0"]
impl crate::Resettable for FcclkselFcclkselx1Spec {}

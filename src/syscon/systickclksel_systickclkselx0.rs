#[doc = "Register `SYSTICKCLKSELX0` reader"]
pub type R = crate::R<SystickclkselSystickclkselx0Spec>;
#[doc = "Register `SYSTICKCLKSELX0` writer"]
pub type W = crate::W<SystickclkselSystickclkselx0Spec>;
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
    pub fn data(&mut self) -> DataW<'_, SystickclkselSystickclkselx0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel_systickclkselx0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel_systickclkselx0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystickclkselSystickclkselx0Spec;
impl crate::RegisterSpec for SystickclkselSystickclkselx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systickclksel_systickclkselx0::R`](R) reader structure"]
impl crate::Readable for SystickclkselSystickclkselx0Spec {}
#[doc = "`write(|w| ..)` method takes [`systickclksel_systickclkselx0::W`](W) writer structure"]
impl crate::Writable for SystickclkselSystickclkselx0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTICKCLKSELX0 to value 0"]
impl crate::Resettable for SystickclkselSystickclkselx0Spec {}

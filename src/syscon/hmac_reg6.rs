#[doc = "Register `HMAC_REG6` reader"]
pub type R = crate::R<HmacReg6Spec>;
#[doc = "Register `HMAC_REG6` writer"]
pub type W = crate::W<HmacReg6Spec>;
#[doc = "Field `HMAC_REG6` reader - no description available"]
pub type HmacReg6R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG6` writer - no description available"]
pub type HmacReg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg6(&self) -> HmacReg6R {
        HmacReg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg6(&mut self) -> HmacReg6W<'_, HmacReg6Spec> {
        HmacReg6W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg6Spec;
impl crate::RegisterSpec for HmacReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg6::R`](R) reader structure"]
impl crate::Readable for HmacReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg6::W`](W) writer structure"]
impl crate::Writable for HmacReg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG6 to value 0"]
impl crate::Resettable for HmacReg6Spec {}

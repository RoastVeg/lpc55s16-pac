#[doc = "Register `HMAC_REG0` reader"]
pub type R = crate::R<HmacReg0Spec>;
#[doc = "Register `HMAC_REG0` writer"]
pub type W = crate::W<HmacReg0Spec>;
#[doc = "Field `HMAC_REG0` reader - no description available"]
pub type HmacReg0R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG0` writer - no description available"]
pub type HmacReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg0(&self) -> HmacReg0R {
        HmacReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg0(&mut self) -> HmacReg0W<'_, HmacReg0Spec> {
        HmacReg0W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg0Spec;
impl crate::RegisterSpec for HmacReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg0::R`](R) reader structure"]
impl crate::Readable for HmacReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg0::W`](W) writer structure"]
impl crate::Writable for HmacReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG0 to value 0"]
impl crate::Resettable for HmacReg0Spec {}

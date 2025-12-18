#[doc = "Register `HMAC_REG3` reader"]
pub type R = crate::R<HmacReg3Spec>;
#[doc = "Register `HMAC_REG3` writer"]
pub type W = crate::W<HmacReg3Spec>;
#[doc = "Field `HMAC_REG3` reader - no description available"]
pub type HmacReg3R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG3` writer - no description available"]
pub type HmacReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg3(&self) -> HmacReg3R {
        HmacReg3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG3")
            .field("hmac_reg3", &self.hmac_reg3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg3(&mut self) -> HmacReg3W<'_, HmacReg3Spec> {
        HmacReg3W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg3Spec;
impl crate::RegisterSpec for HmacReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg3::R`](R) reader structure"]
impl crate::Readable for HmacReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg3::W`](W) writer structure"]
impl crate::Writable for HmacReg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG3 to value 0"]
impl crate::Resettable for HmacReg3Spec {}

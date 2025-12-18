#[doc = "Register `HMAC_REG2` reader"]
pub type R = crate::R<HmacReg2Spec>;
#[doc = "Register `HMAC_REG2` writer"]
pub type W = crate::W<HmacReg2Spec>;
#[doc = "Field `HMAC_REG2` reader - no description available"]
pub type HmacReg2R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG2` writer - no description available"]
pub type HmacReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg2(&self) -> HmacReg2R {
        HmacReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg2(&mut self) -> HmacReg2W<'_, HmacReg2Spec> {
        HmacReg2W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg2Spec;
impl crate::RegisterSpec for HmacReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg2::R`](R) reader structure"]
impl crate::Readable for HmacReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg2::W`](W) writer structure"]
impl crate::Writable for HmacReg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG2 to value 0"]
impl crate::Resettable for HmacReg2Spec {}

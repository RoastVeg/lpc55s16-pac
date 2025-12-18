#[doc = "Register `HMAC_REG1` reader"]
pub type R = crate::R<HmacReg1Spec>;
#[doc = "Register `HMAC_REG1` writer"]
pub type W = crate::W<HmacReg1Spec>;
#[doc = "Field `HMAC_REG1` reader - no description available"]
pub type HmacReg1R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG1` writer - no description available"]
pub type HmacReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg1(&self) -> HmacReg1R {
        HmacReg1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG1")
            .field("hmac_reg1", &self.hmac_reg1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg1(&mut self) -> HmacReg1W<'_, HmacReg1Spec> {
        HmacReg1W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg1Spec;
impl crate::RegisterSpec for HmacReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg1::R`](R) reader structure"]
impl crate::Readable for HmacReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg1::W`](W) writer structure"]
impl crate::Writable for HmacReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG1 to value 0"]
impl crate::Resettable for HmacReg1Spec {}

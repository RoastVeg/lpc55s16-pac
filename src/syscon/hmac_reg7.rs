#[doc = "Register `HMAC_REG7` reader"]
pub type R = crate::R<HmacReg7Spec>;
#[doc = "Register `HMAC_REG7` writer"]
pub type W = crate::W<HmacReg7Spec>;
#[doc = "Field `HMAC_REG7` reader - no description available"]
pub type HmacReg7R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG7` writer - no description available"]
pub type HmacReg7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg7(&self) -> HmacReg7R {
        HmacReg7R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG7")
            .field("hmac_reg7", &self.hmac_reg7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg7(&mut self) -> HmacReg7W<'_, HmacReg7Spec> {
        HmacReg7W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg7Spec;
impl crate::RegisterSpec for HmacReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg7::R`](R) reader structure"]
impl crate::Readable for HmacReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg7::W`](W) writer structure"]
impl crate::Writable for HmacReg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG7 to value 0"]
impl crate::Resettable for HmacReg7Spec {}

#[doc = "Register `HMAC_REG4` reader"]
pub type R = crate::R<HmacReg4Spec>;
#[doc = "Register `HMAC_REG4` writer"]
pub type W = crate::W<HmacReg4Spec>;
#[doc = "Field `HMAC_REG4` reader - no description available"]
pub type HmacReg4R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG4` writer - no description available"]
pub type HmacReg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg4(&self) -> HmacReg4R {
        HmacReg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg4(&mut self) -> HmacReg4W<'_, HmacReg4Spec> {
        HmacReg4W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg4Spec;
impl crate::RegisterSpec for HmacReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg4::R`](R) reader structure"]
impl crate::Readable for HmacReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg4::W`](W) writer structure"]
impl crate::Writable for HmacReg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG4 to value 0"]
impl crate::Resettable for HmacReg4Spec {}

#[doc = "Register `HMAC_REG5` reader"]
pub type R = crate::R<HmacReg5Spec>;
#[doc = "Register `HMAC_REG5` writer"]
pub type W = crate::W<HmacReg5Spec>;
#[doc = "Field `HMAC_REG5` reader - no description available"]
pub type HmacReg5R = crate::FieldReader<u32>;
#[doc = "Field `HMAC_REG5` writer - no description available"]
pub type HmacReg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg5(&self) -> HmacReg5R {
        HmacReg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg5(&mut self) -> HmacReg5W<'_, HmacReg5Spec> {
        HmacReg5W::new(self, 0)
    }
}
#[doc = "HMAC\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacReg5Spec;
impl crate::RegisterSpec for HmacReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_reg5::R`](R) reader structure"]
impl crate::Readable for HmacReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`hmac_reg5::W`](W) writer structure"]
impl crate::Writable for HmacReg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_REG5 to value 0"]
impl crate::Resettable for HmacReg5Spec {}

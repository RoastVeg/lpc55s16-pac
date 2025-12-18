#[doc = "Register `SECURE_COUNTER` reader"]
pub type R = crate::R<SecureCounterSpec>;
#[doc = "Register `SECURE_COUNTER` writer"]
pub type W = crate::W<SecureCounterSpec>;
#[doc = "Field `SECCNT` reader - Secure Counter"]
pub type SeccntR = crate::FieldReader<u32>;
#[doc = "Field `SECCNT` writer - Secure Counter"]
pub type SeccntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Counter"]
    #[inline(always)]
    pub fn seccnt(&self) -> SeccntR {
        SeccntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Counter"]
    #[inline(always)]
    pub fn seccnt(&mut self) -> SeccntW<'_, SecureCounterSpec> {
        SeccntW::new(self, 0)
    }
}
#[doc = "Also known as SEC_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureCounterSpec;
impl crate::RegisterSpec for SecureCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_counter::R`](R) reader structure"]
impl crate::Readable for SecureCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`secure_counter::W`](W) writer structure"]
impl crate::Writable for SecureCounterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE_COUNTER to value 0"]
impl crate::Resettable for SecureCounterSpec {}

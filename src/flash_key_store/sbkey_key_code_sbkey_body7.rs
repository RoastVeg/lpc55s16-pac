#[doc = "Register `SBKEY_BODY7` reader"]
pub type R = crate::R<SbkeyKeyCodeSbkeyBody7Spec>;
#[doc = "Register `SBKEY_BODY7` writer"]
pub type W = crate::W<SbkeyKeyCodeSbkeyBody7Spec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE_SBKEY_BODY7")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, SbkeyKeyCodeSbkeyBody7Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbkeyKeyCodeSbkeyBody7Spec;
impl crate::RegisterSpec for SbkeyKeyCodeSbkeyBody7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbkey_key_code_sbkey_body7::R`](R) reader structure"]
impl crate::Readable for SbkeyKeyCodeSbkeyBody7Spec {}
#[doc = "`write(|w| ..)` method takes [`sbkey_key_code_sbkey_body7::W`](W) writer structure"]
impl crate::Writable for SbkeyKeyCodeSbkeyBody7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SBKEY_BODY7 to value 0"]
impl crate::Resettable for SbkeyKeyCodeSbkeyBody7Spec {}

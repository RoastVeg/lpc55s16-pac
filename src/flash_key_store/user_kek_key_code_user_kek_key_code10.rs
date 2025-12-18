#[doc = "Register `USER_KEK_KEY_CODE10` reader"]
pub type R = crate::R<UserKekKeyCodeUserKekKeyCode10Spec>;
#[doc = "Register `USER_KEK_KEY_CODE10` writer"]
pub type W = crate::W<UserKekKeyCodeUserKekKeyCode10Spec>;
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
        f.debug_struct("USER_KEK_KEY_CODE_USER_KEK_KEY_CODE10")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, UserKekKeyCodeUserKekKeyCode10Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserKekKeyCodeUserKekKeyCode10Spec;
impl crate::RegisterSpec for UserKekKeyCodeUserKekKeyCode10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_kek_key_code_user_kek_key_code10::R`](R) reader structure"]
impl crate::Readable for UserKekKeyCodeUserKekKeyCode10Spec {}
#[doc = "`write(|w| ..)` method takes [`user_kek_key_code_user_kek_key_code10::W`](W) writer structure"]
impl crate::Writable for UserKekKeyCodeUserKekKeyCode10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER_KEK_KEY_CODE10 to value 0"]
impl crate::Resettable for UserKekKeyCodeUserKekKeyCode10Spec {}

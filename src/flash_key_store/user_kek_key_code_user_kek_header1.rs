#[doc = "Register `USER_KEK_HEADER1` reader"]
pub type R = crate::R<UserKekKeyCodeUserKekHeader1Spec>;
#[doc = "Register `USER_KEK_HEADER1` writer"]
pub type W = crate::W<UserKekKeyCodeUserKekHeader1Spec>;
#[doc = "Field `TYPE` reader - ."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - ."]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INDEX` reader - ."]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - ."]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - ."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - ."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE_USER_KEK_HEADER1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, UserKekKeyCodeUserKekHeader1Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, UserKekKeyCodeUserKekHeader1Spec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, UserKekKeyCodeUserKekHeader1Spec> {
        SizeW::new(self, 24)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_header1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_header1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserKekKeyCodeUserKekHeader1Spec;
impl crate::RegisterSpec for UserKekKeyCodeUserKekHeader1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_kek_key_code_user_kek_header1::R`](R) reader structure"]
impl crate::Readable for UserKekKeyCodeUserKekHeader1Spec {}
#[doc = "`write(|w| ..)` method takes [`user_kek_key_code_user_kek_header1::W`](W) writer structure"]
impl crate::Writable for UserKekKeyCodeUserKekHeader1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER_KEK_HEADER1 to value 0"]
impl crate::Resettable for UserKekKeyCodeUserKekHeader1Spec {}

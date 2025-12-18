#[doc = "Register `UDS_HEADER1` reader"]
pub type R = crate::R<UdsKeyCodeUdsHeader1Spec>;
#[doc = "Register `UDS_HEADER1` writer"]
pub type W = crate::W<UdsKeyCodeUdsHeader1Spec>;
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
        f.debug_struct("UDS_KEY_CODE_UDS_HEADER1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, UdsKeyCodeUdsHeader1Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, UdsKeyCodeUdsHeader1Spec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, UdsKeyCodeUdsHeader1Spec> {
        SizeW::new(self, 24)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_header1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_header1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdsKeyCodeUdsHeader1Spec;
impl crate::RegisterSpec for UdsKeyCodeUdsHeader1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uds_key_code_uds_header1::R`](R) reader structure"]
impl crate::Readable for UdsKeyCodeUdsHeader1Spec {}
#[doc = "`write(|w| ..)` method takes [`uds_key_code_uds_header1::W`](W) writer structure"]
impl crate::Writable for UdsKeyCodeUdsHeader1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDS_HEADER1 to value 0"]
impl crate::Resettable for UdsKeyCodeUdsHeader1Spec {}

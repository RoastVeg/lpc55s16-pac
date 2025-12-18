#[doc = "Register `PRINCE_REGION0_BODY5` reader"]
pub type R = crate::R<PrinceRegion0KeyCodePrinceRegion0Body5Spec>;
#[doc = "Register `PRINCE_REGION0_BODY5` writer"]
pub type W = crate::W<PrinceRegion0KeyCodePrinceRegion0Body5Spec>;
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
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, PrinceRegion0KeyCodePrinceRegion0Body5Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceRegion0KeyCodePrinceRegion0Body5Spec;
impl crate::RegisterSpec for PrinceRegion0KeyCodePrinceRegion0Body5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_region0_key_code_prince_region0_body5::R`](R) reader structure"]
impl crate::Readable for PrinceRegion0KeyCodePrinceRegion0Body5Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_region0_key_code_prince_region0_body5::W`](W) writer structure"]
impl crate::Writable for PrinceRegion0KeyCodePrinceRegion0Body5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRINCE_REGION0_BODY5 to value 0"]
impl crate::Resettable for PrinceRegion0KeyCodePrinceRegion0Body5Spec {}

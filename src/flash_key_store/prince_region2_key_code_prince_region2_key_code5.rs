#[doc = "Register `PRINCE_REGION2_KEY_CODE5` reader"]
pub type R = crate::R<PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec>;
#[doc = "Register `PRINCE_REGION2_KEY_CODE5` writer"]
pub type W = crate::W<PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec>;
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
    pub fn field(&mut self) -> FieldW<'_, PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec;
impl crate::RegisterSpec for PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_region2_key_code_prince_region2_key_code5::R`](R) reader structure"]
impl crate::Readable for PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_region2_key_code_prince_region2_key_code5::W`](W) writer structure"]
impl crate::Writable for PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRINCE_REGION2_KEY_CODE5 to value 0"]
impl crate::Resettable for PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec {}

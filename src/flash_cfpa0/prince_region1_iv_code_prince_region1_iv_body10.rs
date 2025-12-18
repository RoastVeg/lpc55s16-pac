#[doc = "Register `PRINCE_REGION1_IV_BODY10` reader"]
pub type R = crate::R<PrinceRegion1IvCodePrinceRegion1IvBody10Spec>;
#[doc = "Register `PRINCE_REGION1_IV_BODY10` writer"]
pub type W = crate::W<PrinceRegion1IvCodePrinceRegion1IvBody10Spec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, PrinceRegion1IvCodePrinceRegion1IvBody10Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceRegion1IvCodePrinceRegion1IvBody10Spec;
impl crate::RegisterSpec for PrinceRegion1IvCodePrinceRegion1IvBody10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_region1_iv_code_prince_region1_iv_body10::R`](R) reader structure"]
impl crate::Readable for PrinceRegion1IvCodePrinceRegion1IvBody10Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_region1_iv_code_prince_region1_iv_body10::W`](W) writer structure"]
impl crate::Writable for PrinceRegion1IvCodePrinceRegion1IvBody10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRINCE_REGION1_IV_BODY10 to value 0"]
impl crate::Resettable for PrinceRegion1IvCodePrinceRegion1IvBody10Spec {}

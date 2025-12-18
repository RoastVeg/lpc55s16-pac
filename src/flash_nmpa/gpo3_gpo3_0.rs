#[doc = "Register `GPO3_0` reader"]
pub type R = crate::R<Gpo3Gpo3_0Spec>;
#[doc = "Register `GPO3_0` writer"]
pub type W = crate::W<Gpo3Gpo3_0Spec>;
#[doc = "Field `AUX_BIAS_TRIM_VALID` reader - no description available"]
pub type AuxBiasTrimValidR = crate::BitReader;
#[doc = "Field `AUX_BIAS_TRIM_VALID` writer - no description available"]
pub type AuxBiasTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_BIAS_ITRIM` reader - no description available"]
pub type AuxBiasItrimR = crate::FieldReader;
#[doc = "Field `AUX_BIAS_ITRIM` writer - no description available"]
pub type AuxBiasItrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AUX_BIAS_PTAT_ITRIM` reader - no description available"]
pub type AuxBiasPtatItrimR = crate::FieldReader;
#[doc = "Field `AUX_BIAS_PTAT_ITRIM` writer - no description available"]
pub type AuxBiasPtatItrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AUX_BIAS_VREF1_VTRIM` reader - no description available"]
pub type AuxBiasVref1VtrimR = crate::FieldReader;
#[doc = "Field `AUX_BIAS_VREF1_VTRIM` writer - no description available"]
pub type AuxBiasVref1VtrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AUX_BIAS_VREF1_VCURVE_TRIM` reader - no description available"]
pub type AuxBiasVref1VcurveTrimR = crate::FieldReader;
#[doc = "Field `AUX_BIAS_VREF1_VCURVE_TRIM` writer - no description available"]
pub type AuxBiasVref1VcurveTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MODELNUM_EXTENSION` reader - ModelNumber extension\\[2:0\\]"]
pub type ModelnumExtensionR = crate::FieldReader;
#[doc = "Field `MODELNUM_EXTENSION` writer - ModelNumber extension\\[2:0\\]"]
pub type ModelnumExtensionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FINAL_TEST_NOT_DONE` reader - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FinalTestNotDoneR = crate::FieldReader;
#[doc = "Field `FINAL_TEST_NOT_DONE` writer - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FinalTestNotDoneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn aux_bias_trim_valid(&self) -> AuxBiasTrimValidR {
        AuxBiasTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn aux_bias_itrim(&self) -> AuxBiasItrimR {
        AuxBiasItrimR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - no description available"]
    #[inline(always)]
    pub fn aux_bias_ptat_itrim(&self) -> AuxBiasPtatItrimR {
        AuxBiasPtatItrimR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - no description available"]
    #[inline(always)]
    pub fn aux_bias_vref1_vtrim(&self) -> AuxBiasVref1VtrimR {
        AuxBiasVref1VtrimR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - no description available"]
    #[inline(always)]
    pub fn aux_bias_vref1_vcurve_trim(&self) -> AuxBiasVref1VcurveTrimR {
        AuxBiasVref1VcurveTrimR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:24 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bits 25:27 - ModelNumber extension\\[2:0\\]"]
    #[inline(always)]
    pub fn modelnum_extension(&self) -> ModelnumExtensionR {
        ModelnumExtensionR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&self) -> FinalTestNotDoneR {
        FinalTestNotDoneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn aux_bias_trim_valid(&mut self) -> AuxBiasTrimValidW<'_, Gpo3Gpo3_0Spec> {
        AuxBiasTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn aux_bias_itrim(&mut self) -> AuxBiasItrimW<'_, Gpo3Gpo3_0Spec> {
        AuxBiasItrimW::new(self, 1)
    }
    #[doc = "Bits 6:10 - no description available"]
    #[inline(always)]
    pub fn aux_bias_ptat_itrim(&mut self) -> AuxBiasPtatItrimW<'_, Gpo3Gpo3_0Spec> {
        AuxBiasPtatItrimW::new(self, 6)
    }
    #[doc = "Bits 11:15 - no description available"]
    #[inline(always)]
    pub fn aux_bias_vref1_vtrim(&mut self) -> AuxBiasVref1VtrimW<'_, Gpo3Gpo3_0Spec> {
        AuxBiasVref1VtrimW::new(self, 11)
    }
    #[doc = "Bits 16:18 - no description available"]
    #[inline(always)]
    pub fn aux_bias_vref1_vcurve_trim(&mut self) -> AuxBiasVref1VcurveTrimW<'_, Gpo3Gpo3_0Spec> {
        AuxBiasVref1VcurveTrimW::new(self, 16)
    }
    #[doc = "Bits 19:24 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo3Gpo3_0Spec> {
        FieldW::new(self, 19)
    }
    #[doc = "Bits 25:27 - ModelNumber extension\\[2:0\\]"]
    #[inline(always)]
    pub fn modelnum_extension(&mut self) -> ModelnumExtensionW<'_, Gpo3Gpo3_0Spec> {
        ModelnumExtensionW::new(self, 25)
    }
    #[doc = "Bits 28:31 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&mut self) -> FinalTestNotDoneW<'_, Gpo3Gpo3_0Spec> {
        FinalTestNotDoneW::new(self, 28)
    }
}
#[doc = "GPO3 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo3Gpo3_0Spec;
impl crate::RegisterSpec for Gpo3Gpo3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo3_gpo3_0::R`](R) reader structure"]
impl crate::Readable for Gpo3Gpo3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo3_gpo3_0::W`](W) writer structure"]
impl crate::Writable for Gpo3Gpo3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO3_0 to value 0"]
impl crate::Resettable for Gpo3Gpo3_0Spec {}

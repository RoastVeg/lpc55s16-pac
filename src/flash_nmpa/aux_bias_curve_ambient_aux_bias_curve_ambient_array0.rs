#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_ARRAY0` reader"]
pub type R = crate::R<AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec>;
#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_ARRAY0` writer"]
pub type W = crate::W<AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec>;
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
    pub fn field(&mut self) -> FieldW<'_, AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec;
impl crate::RegisterSpec for AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::R`](R) reader structure"]
impl crate::Readable for AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::W`](W) writer structure"]
impl crate::Writable for AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUX_BIAS_CURVE_AMBIENT_ARRAY0 to value 0"]
impl crate::Resettable for AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec {}

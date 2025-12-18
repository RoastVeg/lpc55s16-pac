#[doc = "Register `AUX_BIAS_CURVE_TEMP_2` reader"]
pub type R = crate::R<AuxBiasCurveTempAuxBiasCurveTemp2Spec>;
#[doc = "Register `AUX_BIAS_CURVE_TEMP_2` writer"]
pub type W = crate::W<AuxBiasCurveTempAuxBiasCurveTemp2Spec>;
#[doc = "Field `VREF1VCURVETRIM_4` reader - VREF1VCURVETRIM_4 (unit: 100uV)"]
pub type Vref1vcurvetrim4R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_4` writer - VREF1VCURVETRIM_4 (unit: 100uV)"]
pub type Vref1vcurvetrim4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VREF1VCURVETRIM_5` reader - VREF1VCURVETRIM_5 (unit: 100uV)"]
pub type Vref1vcurvetrim5R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_5` writer - VREF1VCURVETRIM_5 (unit: 100uV)"]
pub type Vref1vcurvetrim5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_4(&self) -> Vref1vcurvetrim4R {
        Vref1vcurvetrim4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_5(&self) -> Vref1vcurvetrim5R {
        Vref1vcurvetrim5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2")
            .field("vref1vcurvetrim_4", &self.vref1vcurvetrim_4())
            .field("vref1vcurvetrim_5", &self.vref1vcurvetrim_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_4(
        &mut self,
    ) -> Vref1vcurvetrim4W<'_, AuxBiasCurveTempAuxBiasCurveTemp2Spec> {
        Vref1vcurvetrim4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_5(
        &mut self,
    ) -> Vref1vcurvetrim5W<'_, AuxBiasCurveTempAuxBiasCurveTemp2Spec> {
        Vref1vcurvetrim5W::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxBiasCurveTempAuxBiasCurveTemp2Spec;
impl crate::RegisterSpec for AuxBiasCurveTempAuxBiasCurveTemp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_bias_curve_temp_aux_bias_curve_temp_2::R`](R) reader structure"]
impl crate::Readable for AuxBiasCurveTempAuxBiasCurveTemp2Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_bias_curve_temp_aux_bias_curve_temp_2::W`](W) writer structure"]
impl crate::Writable for AuxBiasCurveTempAuxBiasCurveTemp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUX_BIAS_CURVE_TEMP_2 to value 0"]
impl crate::Resettable for AuxBiasCurveTempAuxBiasCurveTemp2Spec {}

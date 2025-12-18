#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_0` reader"]
pub type R = crate::R<AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec>;
#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_0` writer"]
pub type W = crate::W<AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec>;
#[doc = "Field `VREF1VCURVETRIM_0` reader - VREF1VCURVETRIM_0 (unit: 100uV)"]
pub type Vref1vcurvetrim0R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_0` writer - VREF1VCURVETRIM_0 (unit: 100uV)"]
pub type Vref1vcurvetrim0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VREF1VCURVETRIM_1` reader - VREF1VCURVETRIM_1 (unit: 100uV)"]
pub type Vref1vcurvetrim1R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_1` writer - VREF1VCURVETRIM_1 (unit: 100uV)"]
pub type Vref1vcurvetrim1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_0(&self) -> Vref1vcurvetrim0R {
        Vref1vcurvetrim0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_1(&self) -> Vref1vcurvetrim1R {
        Vref1vcurvetrim1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_0(
        &mut self,
    ) -> Vref1vcurvetrim0W<'_, AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec> {
        Vref1vcurvetrim0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_1(
        &mut self,
    ) -> Vref1vcurvetrim1W<'_, AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec> {
        Vref1vcurvetrim1W::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec;
impl crate::RegisterSpec for AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::R`](R) reader structure"]
impl crate::Readable for AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::W`](W) writer structure"]
impl crate::Writable for AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUX_BIAS_CURVE_AMBIENT_0 to value 0"]
impl crate::Resettable for AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec {}

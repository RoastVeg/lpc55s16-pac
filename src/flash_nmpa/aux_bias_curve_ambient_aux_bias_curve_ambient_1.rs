#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_1` reader"]
pub type R = crate::R<AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec>;
#[doc = "Register `AUX_BIAS_CURVE_AMBIENT_1` writer"]
pub type W = crate::W<AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec>;
#[doc = "Field `VREF1VCURVETRIM_2` reader - VREF1VCURVETRIM_2 (unit: 100uV)"]
pub type Vref1vcurvetrim2R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_2` writer - VREF1VCURVETRIM_2 (unit: 100uV)"]
pub type Vref1vcurvetrim2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VREF1VCURVETRIM_3` reader - VREF1VCURVETRIM_3 (unit: 100uV)"]
pub type Vref1vcurvetrim3R = crate::FieldReader<u16>;
#[doc = "Field `VREF1VCURVETRIM_3` writer - VREF1VCURVETRIM_3 (unit: 100uV)"]
pub type Vref1vcurvetrim3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_2(&self) -> Vref1vcurvetrim2R {
        Vref1vcurvetrim2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_3(&self) -> Vref1vcurvetrim3R {
        Vref1vcurvetrim3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_2(
        &mut self,
    ) -> Vref1vcurvetrim2W<'_, AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec> {
        Vref1vcurvetrim2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_3(
        &mut self,
    ) -> Vref1vcurvetrim3W<'_, AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec> {
        Vref1vcurvetrim3W::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec;
impl crate::RegisterSpec for AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::R`](R) reader structure"]
impl crate::Readable for AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::W`](W) writer structure"]
impl crate::Writable for AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUX_BIAS_CURVE_AMBIENT_1 to value 0"]
impl crate::Resettable for AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec {}

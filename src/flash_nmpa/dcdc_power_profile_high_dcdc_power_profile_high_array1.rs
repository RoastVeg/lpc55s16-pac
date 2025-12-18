#[doc = "Register `DCDC_POWER_PROFILE_HIGH_ARRAY1` reader"]
pub type R = crate::R<DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec>;
#[doc = "Register `DCDC_POWER_PROFILE_HIGH_ARRAY1` writer"]
pub type W = crate::W<DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_high_dcdc_power_profile_high_array1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_high_dcdc_power_profile_high_array1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec;
impl crate::RegisterSpec for DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_power_profile_high_dcdc_power_profile_high_array1::R`](R) reader structure"]
impl crate::Readable for DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc_power_profile_high_dcdc_power_profile_high_array1::W`](W) writer structure"]
impl crate::Writable for DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_POWER_PROFILE_HIGH_ARRAY1 to value 0"]
impl crate::Resettable for DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec {}

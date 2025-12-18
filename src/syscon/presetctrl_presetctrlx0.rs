#[doc = "Register `PRESETCTRLX0` reader"]
pub type R = crate::R<PresetctrlPresetctrlx0Spec>;
#[doc = "Register `PRESETCTRLX0` writer"]
pub type W = crate::W<PresetctrlPresetctrlx0Spec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, PresetctrlPresetctrlx0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrlx0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrlx0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlPresetctrlx0Spec;
impl crate::RegisterSpec for PresetctrlPresetctrlx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrl_presetctrlx0::R`](R) reader structure"]
impl crate::Readable for PresetctrlPresetctrlx0Spec {}
#[doc = "`write(|w| ..)` method takes [`presetctrl_presetctrlx0::W`](W) writer structure"]
impl crate::Writable for PresetctrlPresetctrlx0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRLX0 to value 0"]
impl crate::Resettable for PresetctrlPresetctrlx0Spec {}

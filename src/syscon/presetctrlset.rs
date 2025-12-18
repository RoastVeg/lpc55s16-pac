#[doc = "Register `PRESETCTRLSET[%s]` reader"]
pub type R = crate::R<PresetctrlsetSpec>;
#[doc = "Register `PRESETCTRLSET[%s]` writer"]
pub type W = crate::W<PresetctrlsetSpec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLSET")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, PresetctrlsetSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control set register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlsetSpec;
impl crate::RegisterSpec for PresetctrlsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrlset::R`](R) reader structure"]
impl crate::Readable for PresetctrlsetSpec {}
#[doc = "`write(|w| ..)` method takes [`presetctrlset::W`](W) writer structure"]
impl crate::Writable for PresetctrlsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRLSET[%s] to value 0"]
impl crate::Resettable for PresetctrlsetSpec {}

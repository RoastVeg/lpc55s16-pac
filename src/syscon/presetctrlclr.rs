#[doc = "Register `PRESETCTRLCLR[%s]` reader"]
pub type R = crate::R<PresetctrlclrSpec>;
#[doc = "Register `PRESETCTRLCLR[%s]` writer"]
pub type W = crate::W<PresetctrlclrSpec>;
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
    pub fn data(&mut self) -> DataW<'_, PresetctrlclrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlclrSpec;
impl crate::RegisterSpec for PresetctrlclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrlclr::R`](R) reader structure"]
impl crate::Readable for PresetctrlclrSpec {}
#[doc = "`write(|w| ..)` method takes [`presetctrlclr::W`](W) writer structure"]
impl crate::Writable for PresetctrlclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRLCLR[%s] to value 0"]
impl crate::Resettable for PresetctrlclrSpec {}

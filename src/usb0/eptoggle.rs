#[doc = "Register `EPTOGGLE` reader"]
pub type R = crate::R<EptoggleSpec>;
#[doc = "Register `EPTOGGLE` writer"]
pub type W = crate::W<EptoggleSpec>;
#[doc = "Field `TOGGLE` reader - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub type ToggleR = crate::FieldReader<u16>;
#[doc = "Field `TOGGLE` writer - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub type ToggleW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> ToggleR {
        ToggleR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPTOGGLE")
            .field("toggle", &self.toggle())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&mut self) -> ToggleW<'_, EptoggleSpec> {
        ToggleW::new(self, 0)
    }
}
#[doc = "USB Endpoint toggle register\n\nYou can [`read`](crate::Reg::read) this register and get [`eptoggle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptoggle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EptoggleSpec;
impl crate::RegisterSpec for EptoggleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptoggle::R`](R) reader structure"]
impl crate::Readable for EptoggleSpec {}
#[doc = "`write(|w| ..)` method takes [`eptoggle::W`](W) writer structure"]
impl crate::Writable for EptoggleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPTOGGLE to value 0"]
impl crate::Resettable for EptoggleSpec {}

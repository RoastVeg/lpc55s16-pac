#[doc = "Register `HCRHDESCRIPTORB` reader"]
pub type R = crate::R<HcrhdescriptorbSpec>;
#[doc = "Register `HCRHDESCRIPTORB` writer"]
pub type W = crate::W<HcrhdescriptorbSpec>;
#[doc = "Field `DR` reader - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
pub type DrR = crate::FieldReader<u16>;
#[doc = "Field `DR` writer - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PPCM` reader - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
pub type PpcmR = crate::FieldReader<u16>;
#[doc = "Field `PPCM` writer - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
pub type PpcmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub fn ppcm(&self) -> PpcmR {
        PpcmR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<'_, HcrhdescriptorbSpec> {
        DrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub fn ppcm(&mut self) -> PpcmW<'_, HcrhdescriptorbSpec> {
        PpcmW::new(self, 16)
    }
}
#[doc = "Second of the two registers which describes the characteristics of the Root Hub\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptorb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptorb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhdescriptorbSpec;
impl crate::RegisterSpec for HcrhdescriptorbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhdescriptorb::R`](R) reader structure"]
impl crate::Readable for HcrhdescriptorbSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhdescriptorb::W`](W) writer structure"]
impl crate::Writable for HcrhdescriptorbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORB to value 0"]
impl crate::Resettable for HcrhdescriptorbSpec {}

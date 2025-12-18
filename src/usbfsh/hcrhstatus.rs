#[doc = "Register `HCRHSTATUS` reader"]
pub type R = crate::R<HcrhstatusSpec>;
#[doc = "Register `HCRHSTATUS` writer"]
pub type W = crate::W<HcrhstatusSpec>;
#[doc = "Field `LPS` reader - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub type LpsR = crate::BitReader;
#[doc = "Field `LPS` writer - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub type LpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCI` reader - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub type OciR = crate::BitReader;
#[doc = "Field `OCI` writer - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub type OciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRWE` reader - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub type DrweR = crate::BitReader;
#[doc = "Field `DRWE` writer - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub type DrweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPSC` reader - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub type LpscR = crate::BitReader;
#[doc = "Field `LPSC` writer - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub type LpscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIC` reader - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub type OcicR = crate::BitReader;
#[doc = "Field `OCIC` writer - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub type OcicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRWE` reader - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub type CrweR = crate::BitReader;
#[doc = "Field `CRWE` writer - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub type CrweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&self) -> LpsR {
        LpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&self) -> OciR {
        OciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&self) -> DrweR {
        DrweR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&self) -> LpscR {
        LpscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&self) -> OcicR {
        OcicR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&self) -> CrweR {
        CrweR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&mut self) -> LpsW<'_, HcrhstatusSpec> {
        LpsW::new(self, 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&mut self) -> OciW<'_, HcrhstatusSpec> {
        OciW::new(self, 1)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&mut self) -> DrweW<'_, HcrhstatusSpec> {
        DrweW::new(self, 15)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&mut self) -> LpscW<'_, HcrhstatusSpec> {
        LpscW::new(self, 16)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OcicW<'_, HcrhstatusSpec> {
        OcicW::new(self, 17)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&mut self) -> CrweW<'_, HcrhstatusSpec> {
        CrweW::new(self, 31)
    }
}
#[doc = "This register is divided into two parts\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhstatusSpec;
impl crate::RegisterSpec for HcrhstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhstatus::R`](R) reader structure"]
impl crate::Readable for HcrhstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhstatus::W`](W) writer structure"]
impl crate::Writable for HcrhstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCRHSTATUS to value 0"]
impl crate::Resettable for HcrhstatusSpec {}

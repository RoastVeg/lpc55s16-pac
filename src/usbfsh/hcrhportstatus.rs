#[doc = "Register `HCRHPORTSTATUS` reader"]
pub type R = crate::R<HcrhportstatusSpec>;
#[doc = "Register `HCRHPORTSTATUS` writer"]
pub type W = crate::W<HcrhportstatusSpec>;
#[doc = "Field `CCS` reader - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub type CcsR = crate::BitReader;
#[doc = "Field `CCS` writer - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES` reader - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub type PesR = crate::BitReader;
#[doc = "Field `PES` writer - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub type PesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSS` reader - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub type PssR = crate::BitReader;
#[doc = "Field `PSS` writer - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub type PssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCI` reader - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub type PociR = crate::BitReader;
#[doc = "Field `POCI` writer - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub type PociW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPS` reader - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub type PpsR = crate::BitReader;
#[doc = "Field `PPS` writer - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub type PpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDA` reader - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub type LsdaR = crate::BitReader;
#[doc = "Field `LSDA` writer - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub type LsdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub type CscR = crate::BitReader;
#[doc = "Field `CSC` writer - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESC` reader - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub type PescR = crate::BitReader;
#[doc = "Field `PESC` writer - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub type PescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSSC` reader - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub type PsscR = crate::BitReader;
#[doc = "Field `PSSC` writer - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub type PsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIC` reader - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
pub type OcicR = crate::BitReader;
#[doc = "Field `OCIC` writer - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
pub type OcicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSC` reader - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub type PrscR = crate::BitReader;
#[doc = "Field `PRSC` writer - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub type PrscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&self) -> PociR {
        PociR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&self) -> PpsR {
        PpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&self) -> LsdaR {
        LsdaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&self) -> PescR {
        PescR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&self) -> PsscR {
        PsscR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&self) -> OcicR {
        OcicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&self) -> PrscR {
        PrscR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> CcsW<'_, HcrhportstatusSpec> {
        CcsW::new(self, 0)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&mut self) -> PesW<'_, HcrhportstatusSpec> {
        PesW::new(self, 1)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&mut self) -> PssW<'_, HcrhportstatusSpec> {
        PssW::new(self, 2)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&mut self) -> PociW<'_, HcrhportstatusSpec> {
        PociW::new(self, 3)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, HcrhportstatusSpec> {
        PrsW::new(self, 4)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&mut self) -> PpsW<'_, HcrhportstatusSpec> {
        PpsW::new(self, 8)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&mut self) -> LsdaW<'_, HcrhportstatusSpec> {
        LsdaW::new(self, 9)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<'_, HcrhportstatusSpec> {
        CscW::new(self, 16)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&mut self) -> PescW<'_, HcrhportstatusSpec> {
        PescW::new(self, 17)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&mut self) -> PsscW<'_, HcrhportstatusSpec> {
        PsscW::new(self, 18)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OcicW<'_, HcrhportstatusSpec> {
        OcicW::new(self, 19)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&mut self) -> PrscW<'_, HcrhportstatusSpec> {
        PrscW::new(self, 20)
    }
}
#[doc = "Controls and reports the port events on a per-port basis\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhportstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhportstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrhportstatusSpec;
impl crate::RegisterSpec for HcrhportstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcrhportstatus::R`](R) reader structure"]
impl crate::Readable for HcrhportstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcrhportstatus::W`](W) writer structure"]
impl crate::Writable for HcrhportstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCRHPORTSTATUS to value 0"]
impl crate::Resettable for HcrhportstatusSpec {}

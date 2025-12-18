#[doc = "Register `HCINTERRUPTSTATUS` reader"]
pub type R = crate::R<HcinterruptstatusSpec>;
#[doc = "Register `HCINTERRUPTSTATUS` writer"]
pub type W = crate::W<HcinterruptstatusSpec>;
#[doc = "Field `SO` reader - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
pub type SoR = crate::BitReader;
#[doc = "Field `SO` writer - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
pub type SoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDH` reader - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
pub type WdhR = crate::BitReader;
#[doc = "Field `WDH` writer - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
pub type WdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF` reader - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
pub type SfR = crate::BitReader;
#[doc = "Field `SF` writer - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
pub type SfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNO` reader - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
pub type FnoR = crate::BitReader;
#[doc = "Field `FNO` writer - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
pub type FnoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHSC` reader - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
pub type RhscR = crate::BitReader;
#[doc = "Field `RHSC` writer - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
pub type RhscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC` reader - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
pub type OcR = crate::FieldReader<u32>;
#[doc = "Field `OC` writer - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
pub type OcW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&self) -> SoR {
        SoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&self) -> WdhR {
        WdhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&self) -> SfR {
        SfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&self) -> FnoR {
        FnoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub fn rhsc(&self) -> RhscR {
        RhscR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTERRUPTSTATUS")
            .field("so", &self.so())
            .field("wdh", &self.wdh())
            .field("sf", &self.sf())
            .field("rd", &self.rd())
            .field("ue", &self.ue())
            .field("fno", &self.fno())
            .field("rhsc", &self.rhsc())
            .field("oc", &self.oc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&mut self) -> SoW<'_, HcinterruptstatusSpec> {
        SoW::new(self, 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&mut self) -> WdhW<'_, HcinterruptstatusSpec> {
        WdhW::new(self, 1)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&mut self) -> SfW<'_, HcinterruptstatusSpec> {
        SfW::new(self, 2)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, HcinterruptstatusSpec> {
        RdW::new(self, 3)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<'_, HcinterruptstatusSpec> {
        UeW::new(self, 4)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&mut self) -> FnoW<'_, HcinterruptstatusSpec> {
        FnoW::new(self, 5)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub fn rhsc(&mut self) -> RhscW<'_, HcinterruptstatusSpec> {
        RhscW::new(self, 6)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<'_, HcinterruptstatusSpec> {
        OcW::new(self, 10)
    }
}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcinterruptstatusSpec;
impl crate::RegisterSpec for HcinterruptstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcinterruptstatus::R`](R) reader structure"]
impl crate::Readable for HcinterruptstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hcinterruptstatus::W`](W) writer structure"]
impl crate::Writable for HcinterruptstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCINTERRUPTSTATUS to value 0"]
impl crate::Resettable for HcinterruptstatusSpec {}

#[doc = "Register `AOREG1` reader"]
pub type R = crate::R<Aoreg1Spec>;
#[doc = "Register `AOREG1` writer"]
pub type W = crate::W<Aoreg1Spec>;
#[doc = "Field `POR` reader - The last chip reset was caused by a Power On Reset."]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - The last chip reset was caused by a Power On Reset."]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADRESET` reader - The last chip reset was caused by a Pin Reset."]
pub type PadresetR = crate::BitReader;
#[doc = "Field `PADRESET` writer - The last chip reset was caused by a Pin Reset."]
pub type PadresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODRESET` reader - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub type BodresetR = crate::BitReader;
#[doc = "Field `BODRESET` writer - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub type BodresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTEMRESET` reader - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub type SystemresetR = crate::BitReader;
#[doc = "Field `SYSTEMRESET` writer - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub type SystemresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTRESET` reader - The last chip reset was caused by the Watchdog Timer."]
pub type WdtresetR = crate::BitReader;
#[doc = "Field `WDTRESET` writer - The last chip reset was caused by the Watchdog Timer."]
pub type WdtresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRRESET` reader - The last chip reset was caused by a Software event."]
pub type SwrresetR = crate::BitReader;
#[doc = "Field `SWRRESET` writer - The last chip reset was caused by a Software event."]
pub type SwrresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDRESET_WAKEUPIO` reader - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub type DpdresetWakeupioR = crate::BitReader;
#[doc = "Field `DPDRESET_WAKEUPIO` writer - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub type DpdresetWakeupioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDRESET_RTC` reader - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub type DpdresetRtcR = crate::BitReader;
#[doc = "Field `DPDRESET_RTC` writer - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub type DpdresetRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDRESET_OSTIMER` reader - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub type DpdresetOstimerR = crate::BitReader;
#[doc = "Field `DPDRESET_OSTIMER` writer - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub type DpdresetOstimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDOGRESET` reader - The last chip reset was caused by the code Watchdog."]
pub type CdogresetR = crate::BitReader;
#[doc = "Field `CDOGRESET` writer - The last chip reset was caused by the code Watchdog."]
pub type CdogresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTERRORCOUNTER` reader - ROM Boot Fatal Error Counter."]
pub type BooterrorcounterR = crate::FieldReader;
#[doc = "Field `BOOTERRORCOUNTER` writer - ROM Boot Fatal Error Counter."]
pub type BooterrorcounterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub fn padreset(&self) -> PadresetR {
        PadresetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub fn bodreset(&self) -> BodresetR {
        BodresetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub fn systemreset(&self) -> SystemresetR {
        SystemresetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub fn wdtreset(&self) -> WdtresetR {
        WdtresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub fn swrreset(&self) -> SwrresetR {
        SwrresetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_wakeupio(&self) -> DpdresetWakeupioR {
        DpdresetWakeupioR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_rtc(&self) -> DpdresetRtcR {
        DpdresetRtcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_ostimer(&self) -> DpdresetOstimerR {
        DpdresetOstimerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The last chip reset was caused by the code Watchdog."]
    #[inline(always)]
    pub fn cdogreset(&self) -> CdogresetR {
        CdogresetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub fn booterrorcounter(&self) -> BooterrorcounterR {
        BooterrorcounterR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AOREG1")
            .field("por", &self.por())
            .field("padreset", &self.padreset())
            .field("bodreset", &self.bodreset())
            .field("systemreset", &self.systemreset())
            .field("wdtreset", &self.wdtreset())
            .field("swrreset", &self.swrreset())
            .field("dpdreset_wakeupio", &self.dpdreset_wakeupio())
            .field("dpdreset_rtc", &self.dpdreset_rtc())
            .field("dpdreset_ostimer", &self.dpdreset_ostimer())
            .field("cdogreset", &self.cdogreset())
            .field("booterrorcounter", &self.booterrorcounter())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, Aoreg1Spec> {
        PorW::new(self, 4)
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub fn padreset(&mut self) -> PadresetW<'_, Aoreg1Spec> {
        PadresetW::new(self, 5)
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub fn bodreset(&mut self) -> BodresetW<'_, Aoreg1Spec> {
        BodresetW::new(self, 6)
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub fn systemreset(&mut self) -> SystemresetW<'_, Aoreg1Spec> {
        SystemresetW::new(self, 7)
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub fn wdtreset(&mut self) -> WdtresetW<'_, Aoreg1Spec> {
        WdtresetW::new(self, 8)
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub fn swrreset(&mut self) -> SwrresetW<'_, Aoreg1Spec> {
        SwrresetW::new(self, 9)
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_wakeupio(&mut self) -> DpdresetWakeupioW<'_, Aoreg1Spec> {
        DpdresetWakeupioW::new(self, 10)
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_rtc(&mut self) -> DpdresetRtcW<'_, Aoreg1Spec> {
        DpdresetRtcW::new(self, 11)
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_ostimer(&mut self) -> DpdresetOstimerW<'_, Aoreg1Spec> {
        DpdresetOstimerW::new(self, 12)
    }
    #[doc = "Bit 13 - The last chip reset was caused by the code Watchdog."]
    #[inline(always)]
    pub fn cdogreset(&mut self) -> CdogresetW<'_, Aoreg1Spec> {
        CdogresetW::new(self, 13)
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub fn booterrorcounter(&mut self) -> BooterrorcounterW<'_, Aoreg1Spec> {
        BooterrorcounterW::new(self, 16)
    }
}
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`aoreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aoreg1Spec;
impl crate::RegisterSpec for Aoreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoreg1::R`](R) reader structure"]
impl crate::Readable for Aoreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`aoreg1::W`](W) writer structure"]
impl crate::Writable for Aoreg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AOREG1 to value 0"]
impl crate::Resettable for Aoreg1Spec {}

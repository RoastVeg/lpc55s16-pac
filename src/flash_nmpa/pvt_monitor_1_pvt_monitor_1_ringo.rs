#[doc = "Register `PVT_MONITOR_1_RINGO` reader"]
pub type R = crate::R<PvtMonitor1PvtMonitor1RingoSpec>;
#[doc = "Register `PVT_MONITOR_1_RINGO` writer"]
pub type W = crate::W<PvtMonitor1PvtMonitor1RingoSpec>;
#[doc = "Field `RINGO_VALID` reader - no description available"]
pub type RingoValidR = crate::BitReader;
#[doc = "Field `RINGO_VALID` writer - no description available"]
pub type RingoValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINGO_FREQ_HZ` reader - no description available"]
pub type RingoFreqHzR = crate::FieldReader<u32>;
#[doc = "Field `RINGO_FREQ_HZ` writer - no description available"]
pub type RingoFreqHzW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ringo_valid(&self) -> RingoValidR {
        RingoValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn ringo_freq_hz(&self) -> RingoFreqHzR {
        RingoFreqHzR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ringo_valid(&mut self) -> RingoValidW<'_, PvtMonitor1PvtMonitor1RingoSpec> {
        RingoValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn ringo_freq_hz(&mut self) -> RingoFreqHzW<'_, PvtMonitor1PvtMonitor1RingoSpec> {
        RingoFreqHzW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_ringo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_ringo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvtMonitor1PvtMonitor1RingoSpec;
impl crate::RegisterSpec for PvtMonitor1PvtMonitor1RingoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_monitor_1_pvt_monitor_1_ringo::R`](R) reader structure"]
impl crate::Readable for PvtMonitor1PvtMonitor1RingoSpec {}
#[doc = "`write(|w| ..)` method takes [`pvt_monitor_1_pvt_monitor_1_ringo::W`](W) writer structure"]
impl crate::Writable for PvtMonitor1PvtMonitor1RingoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT_MONITOR_1_RINGO to value 0"]
impl crate::Resettable for PvtMonitor1PvtMonitor1RingoSpec {}

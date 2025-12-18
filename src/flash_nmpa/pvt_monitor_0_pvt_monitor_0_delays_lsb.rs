#[doc = "Register `PVT_MONITOR_0_DELAYS_LSB` reader"]
pub type R = crate::R<PvtMonitor0PvtMonitor0DelaysLsbSpec>;
#[doc = "Register `PVT_MONITOR_0_DELAYS_LSB` writer"]
pub type W = crate::W<PvtMonitor0PvtMonitor0DelaysLsbSpec>;
#[doc = "Field `DELAY_VALID` reader - no description available"]
pub type DelayValidR = crate::BitReader;
#[doc = "Field `DELAY_VALID` writer - no description available"]
pub type DelayValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_0` reader - Delay in us."]
pub type Delay0R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_0` writer - Delay in us."]
pub type Delay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DELAY_1` reader - Delay in us."]
pub type Delay1R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_1` writer - Delay in us."]
pub type Delay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DELAY_2` reader - Delay in us."]
pub type Delay2R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_2` writer - Delay in us."]
pub type Delay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn delay_valid(&self) -> DelayValidR {
        DelayValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Delay in us."]
    #[inline(always)]
    pub fn delay_0(&self) -> Delay0R {
        Delay0R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - Delay in us."]
    #[inline(always)]
    pub fn delay_1(&self) -> Delay1R {
        Delay1R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - Delay in us."]
    #[inline(always)]
    pub fn delay_2(&self) -> Delay2R {
        Delay2R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB")
            .field("delay_valid", &self.delay_valid())
            .field("delay_0", &self.delay_0())
            .field("delay_1", &self.delay_1())
            .field("delay_2", &self.delay_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn delay_valid(&mut self) -> DelayValidW<'_, PvtMonitor0PvtMonitor0DelaysLsbSpec> {
        DelayValidW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Delay in us."]
    #[inline(always)]
    pub fn delay_0(&mut self) -> Delay0W<'_, PvtMonitor0PvtMonitor0DelaysLsbSpec> {
        Delay0W::new(self, 1)
    }
    #[doc = "Bits 11:20 - Delay in us."]
    #[inline(always)]
    pub fn delay_1(&mut self) -> Delay1W<'_, PvtMonitor0PvtMonitor0DelaysLsbSpec> {
        Delay1W::new(self, 11)
    }
    #[doc = "Bits 21:30 - Delay in us."]
    #[inline(always)]
    pub fn delay_2(&mut self) -> Delay2W<'_, PvtMonitor0PvtMonitor0DelaysLsbSpec> {
        Delay2W::new(self, 21)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_delays_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_delays_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvtMonitor0PvtMonitor0DelaysLsbSpec;
impl crate::RegisterSpec for PvtMonitor0PvtMonitor0DelaysLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_monitor_0_pvt_monitor_0_delays_lsb::R`](R) reader structure"]
impl crate::Readable for PvtMonitor0PvtMonitor0DelaysLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pvt_monitor_0_pvt_monitor_0_delays_lsb::W`](W) writer structure"]
impl crate::Writable for PvtMonitor0PvtMonitor0DelaysLsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT_MONITOR_0_DELAYS_LSB to value 0"]
impl crate::Resettable for PvtMonitor0PvtMonitor0DelaysLsbSpec {}

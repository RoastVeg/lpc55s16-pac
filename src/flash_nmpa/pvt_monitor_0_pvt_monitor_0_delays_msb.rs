#[doc = "Register `PVT_MONITOR_0_DELAYS_MSB` reader"]
pub type R = crate::R<PvtMonitor0PvtMonitor0DelaysMsbSpec>;
#[doc = "Register `PVT_MONITOR_0_DELAYS_MSB` writer"]
pub type W = crate::W<PvtMonitor0PvtMonitor0DelaysMsbSpec>;
#[doc = "Field `DELAY_3` reader - Delay in us."]
pub type Delay3R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_3` writer - Delay in us."]
pub type Delay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DELAY_4` reader - Delay in us."]
pub type Delay4R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_4` writer - Delay in us."]
pub type Delay4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DELAY_5` reader - Delay in us."]
pub type Delay5R = crate::FieldReader<u16>;
#[doc = "Field `DELAY_5` writer - Delay in us."]
pub type Delay5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Delay in us."]
    #[inline(always)]
    pub fn delay_3(&self) -> Delay3R {
        Delay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Delay in us."]
    #[inline(always)]
    pub fn delay_4(&self) -> Delay4R {
        Delay4R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Delay in us."]
    #[inline(always)]
    pub fn delay_5(&self) -> Delay5R {
        Delay5R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Delay in us."]
    #[inline(always)]
    pub fn delay_3(&mut self) -> Delay3W<'_, PvtMonitor0PvtMonitor0DelaysMsbSpec> {
        Delay3W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Delay in us."]
    #[inline(always)]
    pub fn delay_4(&mut self) -> Delay4W<'_, PvtMonitor0PvtMonitor0DelaysMsbSpec> {
        Delay4W::new(self, 10)
    }
    #[doc = "Bits 20:29 - Delay in us."]
    #[inline(always)]
    pub fn delay_5(&mut self) -> Delay5W<'_, PvtMonitor0PvtMonitor0DelaysMsbSpec> {
        Delay5W::new(self, 20)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_delays_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_delays_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvtMonitor0PvtMonitor0DelaysMsbSpec;
impl crate::RegisterSpec for PvtMonitor0PvtMonitor0DelaysMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_monitor_0_pvt_monitor_0_delays_msb::R`](R) reader structure"]
impl crate::Readable for PvtMonitor0PvtMonitor0DelaysMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pvt_monitor_0_pvt_monitor_0_delays_msb::W`](W) writer structure"]
impl crate::Writable for PvtMonitor0PvtMonitor0DelaysMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT_MONITOR_0_DELAYS_MSB to value 0"]
impl crate::Resettable for PvtMonitor0PvtMonitor0DelaysMsbSpec {}

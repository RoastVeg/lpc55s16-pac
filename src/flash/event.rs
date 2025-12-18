#[doc = "Register `EVENT` writer"]
pub type W = crate::W<EventSpec>;
#[doc = "Field `RST` writer - When bit is set, the controller and flash are reset."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` writer - When bit is set, a running program/erase command is aborted."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EventSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - When bit is set, the controller and flash are reset."]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, EventSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 1 - When bit is set, the controller wakes up from whatever low power or powerdown mode was active."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, EventSpec> {
        WakeupW::new(self, 1)
    }
    #[doc = "Bit 2 - When bit is set, a running program/erase command is aborted."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<'_, EventSpec> {
        AbortW::new(self, 2)
    }
}
#[doc = "event register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventSpec;
impl crate::RegisterSpec for EventSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`event::W`](W) writer structure"]
impl crate::Writable for EventSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EventSpec {}

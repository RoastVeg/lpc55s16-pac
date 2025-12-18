#[doc = "Register `OSEVENT_CTRL` reader"]
pub type R = crate::R<OseventCtrlSpec>;
#[doc = "Register `OSEVENT_CTRL` writer"]
pub type W = crate::W<OseventCtrlSpec>;
#[doc = "Field `OSTIMER_INTRFLAG` reader - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
pub type OstimerIntrflagR = crate::BitReader;
#[doc = "Field `OSTIMER_INTRFLAG` writer - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
pub type OstimerIntrflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSTIMER_INTENA` reader - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
pub type OstimerIntenaR = crate::BitReader;
#[doc = "Field `OSTIMER_INTENA` writer - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
pub type OstimerIntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_WR_RDY` reader - This bit will be low when it is safe to write to reload the Match Registers. In typical applications it should not be necessary to test this bit. \\[1\\]"]
pub type MatchWrRdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
    #[inline(always)]
    pub fn ostimer_intrflag(&self) -> OstimerIntrflagR {
        OstimerIntrflagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
    #[inline(always)]
    pub fn ostimer_intena(&self) -> OstimerIntenaR {
        OstimerIntenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit will be low when it is safe to write to reload the Match Registers. In typical applications it should not be necessary to test this bit. \\[1\\]"]
    #[inline(always)]
    pub fn match_wr_rdy(&self) -> MatchWrRdyR {
        MatchWrRdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSEVENT_CTRL")
            .field("ostimer_intrflag", &self.ostimer_intrflag())
            .field("ostimer_intena", &self.ostimer_intena())
            .field("match_wr_rdy", &self.match_wr_rdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 42-bits EVTIMER and the value programmed in the match-register pair. This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. It should be done before a new match value is written into the MATCH_L/H registers."]
    #[inline(always)]
    pub fn ostimer_intrflag(&mut self) -> OstimerIntrflagW<'_, OseventCtrlSpec> {
        OstimerIntrflagW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the domain processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked."]
    #[inline(always)]
    pub fn ostimer_intena(&mut self) -> OstimerIntenaW<'_, OseventCtrlSpec> {
        OstimerIntenaW::new(self, 1)
    }
}
#[doc = "OS_EVENT TIMER Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osevent_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osevent_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OseventCtrlSpec;
impl crate::RegisterSpec for OseventCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osevent_ctrl::R`](R) reader structure"]
impl crate::Readable for OseventCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`osevent_ctrl::W`](W) writer structure"]
impl crate::Writable for OseventCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSEVENT_CTRL to value 0"]
impl crate::Resettable for OseventCtrlSpec {}

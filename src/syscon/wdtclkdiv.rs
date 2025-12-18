#[doc = "Register `WDTCLKDIV` reader"]
pub type R = crate::R<WdtclkdivSpec>;
#[doc = "Register `WDTCLKDIV` writer"]
pub type W = crate::W<WdtclkdivSpec>;
#[doc = "Field `DIV` reader - Clock divider value."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock divider value."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Resets the divider counter.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Divider is not reset."]
    Released = 0,
    #[doc = "1: Divider is reset."]
    Asserted = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Resets the divider counter."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divider is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Released)
    }
    #[doc = "Divider is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Asserted)
    }
}
#[doc = "Halts the divider counter.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: Divider clock is running."]
    Run = 0,
    #[doc = "1: Divider clock is stoped."]
    Halt = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halts the divider counter."]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::Run,
            true => Halt::Halt,
        }
    }
    #[doc = "Divider clock is running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Halt::Run
    }
    #[doc = "Divider clock is stoped."]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Halt::Halt
    }
}
#[doc = "Field `HALT` writer - Halts the divider counter."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divider clock is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Run)
    }
    #[doc = "Divider clock is stoped."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Halt)
    }
}
#[doc = "Divider status flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reqflag {
    #[doc = "0: Divider clock is stable."]
    Stable = 0,
    #[doc = "1: Clock frequency is not stable."]
    Ongoing = 1,
}
impl From<Reqflag> for bool {
    #[inline(always)]
    fn from(variant: Reqflag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub type ReqflagR = crate::BitReader<Reqflag>;
impl ReqflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reqflag {
        match self.bits {
            false => Reqflag::Stable,
            true => Reqflag::Ongoing,
        }
    }
    #[doc = "Divider clock is stable."]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == Reqflag::Stable
    }
    #[doc = "Clock frequency is not stable."]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == Reqflag::Ongoing
    }
}
impl R {
    #[doc = "Bits 0:5 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> ReqflagR {
        ReqflagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCLKDIV")
            .field("div", &self.div())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock divider value."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, WdtclkdivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, WdtclkdivSpec> {
        ResetW::new(self, 29)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, WdtclkdivSpec> {
        HaltW::new(self, 30)
    }
}
#[doc = "WDT clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtclkdivSpec;
impl crate::RegisterSpec for WdtclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkdiv::R`](R) reader structure"]
impl crate::Readable for WdtclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtclkdiv::W`](W) writer structure"]
impl crate::Writable for WdtclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCLKDIV to value 0x4000_0000"]
impl crate::Resettable for WdtclkdivSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}

#[doc = "Register `WAKEIOCAUSE` reader"]
pub type R = crate::R<WakeiocauseSpec>;
#[doc = "Register `WAKEIOCAUSE` writer"]
pub type W = crate::W<WakeiocauseSpec>;
#[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup0 {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    Noevent = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    Event = 1,
}
impl From<Wakeup0> for bool {
    #[inline(always)]
    fn from(variant: Wakeup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP0` reader - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
pub type Wakeup0R = crate::BitReader<Wakeup0>;
impl Wakeup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup0 {
        match self.bits {
            false => Wakeup0::Noevent,
            true => Wakeup0::Event,
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == Wakeup0::Noevent
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Wakeup0::Event
    }
}
#[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup1 {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    Noevent = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    Event = 1,
}
impl From<Wakeup1> for bool {
    #[inline(always)]
    fn from(variant: Wakeup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP1` reader - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
pub type Wakeup1R = crate::BitReader<Wakeup1>;
impl Wakeup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup1 {
        match self.bits {
            false => Wakeup1::Noevent,
            true => Wakeup1::Event,
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == Wakeup1::Noevent
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Wakeup1::Event
    }
}
#[doc = "Field `WAKEUP1` writer - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
pub type Wakeup1W<'a, REG> = crate::BitWriter<'a, REG, Wakeup1>;
impl<'a, REG> Wakeup1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup1::Noevent)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup1::Event)
    }
}
#[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup2 {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    Noevent = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    Event = 1,
}
impl From<Wakeup2> for bool {
    #[inline(always)]
    fn from(variant: Wakeup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP2` reader - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
pub type Wakeup2R = crate::BitReader<Wakeup2>;
impl Wakeup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup2 {
        match self.bits {
            false => Wakeup2::Noevent,
            true => Wakeup2::Event,
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == Wakeup2::Noevent
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Wakeup2::Event
    }
}
#[doc = "Field `WAKEUP2` writer - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
pub type Wakeup2W<'a, REG> = crate::BitWriter<'a, REG, Wakeup2>;
impl<'a, REG> Wakeup2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup2::Noevent)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup2::Event)
    }
}
#[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup3 {
    #[doc = "0: Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    Noevent = 0,
    #[doc = "1: Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    Event = 1,
}
impl From<Wakeup3> for bool {
    #[inline(always)]
    fn from(variant: Wakeup3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP3` reader - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
pub type Wakeup3R = crate::BitReader<Wakeup3>;
impl Wakeup3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup3 {
        match self.bits {
            false => Wakeup3::Noevent,
            true => Wakeup3::Event,
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == Wakeup3::Noevent
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Wakeup3::Event
    }
}
#[doc = "Field `WAKEUP3` writer - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
pub type Wakeup3W<'a, REG> = crate::BitWriter<'a, REG, Wakeup3>;
impl<'a, REG> Wakeup3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup3::Noevent)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup3::Event)
    }
}
impl R {
    #[doc = "Bit 0 - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup0(&self) -> Wakeup0R {
        Wakeup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup1(&self) -> Wakeup1R {
        Wakeup1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup2(&self) -> Wakeup2R {
        Wakeup2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup3(&self) -> Wakeup3R {
        Wakeup3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEIOCAUSE")
            .field("wakeup0", &self.wakeup0())
            .field("wakeup1", &self.wakeup1())
            .field("wakeup2", &self.wakeup2())
            .field("wakeup3", &self.wakeup3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup1(&mut self) -> Wakeup1W<'_, WakeiocauseSpec> {
        Wakeup1W::new(self, 1)
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup2(&mut self) -> Wakeup2W<'_, WakeiocauseSpec> {
        Wakeup2W::new(self, 2)
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup3(&mut self) -> Wakeup3W<'_, WakeiocauseSpec> {
        Wakeup3W::new(self, 3)
    }
}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeiocause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeiocause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeiocauseSpec;
impl crate::RegisterSpec for WakeiocauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeiocause::R`](R) reader structure"]
impl crate::Readable for WakeiocauseSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeiocause::W`](W) writer structure"]
impl crate::Writable for WakeiocauseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEIOCAUSE to value 0"]
impl crate::Resettable for WakeiocauseSpec {}

#[doc = "Register `CLOCKGENUPDATELOCKOUT` reader"]
pub type R = crate::R<ClockgenupdatelockoutSpec>;
#[doc = "Register `CLOCKGENUPDATELOCKOUT` writer"]
pub type W = crate::W<ClockgenupdatelockoutSpec>;
#[doc = "Control clock configuration registers access (for example, xxxDIV, xxxSEL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Clockgenupdatelockout {
    #[doc = "0: all hardware clock configruration are freeze."]
    Freeze = 0,
    #[doc = "1: update all clock configuration."]
    Enable = 1,
}
impl From<Clockgenupdatelockout> for u32 {
    #[inline(always)]
    fn from(variant: Clockgenupdatelockout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clockgenupdatelockout {
    type Ux = u32;
}
impl crate::IsEnum for Clockgenupdatelockout {}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` reader - Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
pub type ClockgenupdatelockoutR = crate::FieldReader<Clockgenupdatelockout>;
impl ClockgenupdatelockoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clockgenupdatelockout> {
        match self.bits {
            0 => Some(Clockgenupdatelockout::Freeze),
            1 => Some(Clockgenupdatelockout::Enable),
            _ => None,
        }
    }
    #[doc = "all hardware clock configruration are freeze."]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == Clockgenupdatelockout::Freeze
    }
    #[doc = "update all clock configuration."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clockgenupdatelockout::Enable
    }
}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` writer - Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
pub type ClockgenupdatelockoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, Clockgenupdatelockout>;
impl<'a, REG> ClockgenupdatelockoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "all hardware clock configruration are freeze."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut crate::W<REG> {
        self.variant(Clockgenupdatelockout::Freeze)
    }
    #[doc = "update all clock configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clockgenupdatelockout::Enable)
    }
}
impl R {
    #[doc = "Bits 0:31 - Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&self) -> ClockgenupdatelockoutR {
        ClockgenupdatelockoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(
        &mut self,
    ) -> ClockgenupdatelockoutW<'_, ClockgenupdatelockoutSpec> {
        ClockgenupdatelockoutW::new(self, 0)
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`clockgenupdatelockout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockgenupdatelockout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockgenupdatelockoutSpec;
impl crate::RegisterSpec for ClockgenupdatelockoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockgenupdatelockout::R`](R) reader structure"]
impl crate::Readable for ClockgenupdatelockoutSpec {}
#[doc = "`write(|w| ..)` method takes [`clockgenupdatelockout::W`](W) writer structure"]
impl crate::Writable for ClockgenupdatelockoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKGENUPDATELOCKOUT to value 0"]
impl crate::Resettable for ClockgenupdatelockoutSpec {}

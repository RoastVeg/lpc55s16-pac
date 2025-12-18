#[doc = "Register `REFFASTWKUP` reader"]
pub type R = crate::R<ReffastwkupSpec>;
#[doc = "Register `REFFASTWKUP` writer"]
pub type W = crate::W<ReffastwkupSpec>;
#[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): .\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpwkup {
    #[doc = "0: Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    Disable = 0,
    #[doc = "1: Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    Enable = 1,
}
impl From<Lpwkup> for bool {
    #[inline(always)]
    fn from(variant: Lpwkup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWKUP` reader - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
pub type LpwkupR = crate::BitReader<Lpwkup>;
impl LpwkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpwkup {
        match self.bits {
            false => Lpwkup::Disable,
            true => Lpwkup::Enable,
        }
    }
    #[doc = "Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lpwkup::Disable
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpwkup::Enable
    }
}
#[doc = "Field `LPWKUP` writer - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
pub type LpwkupW<'a, REG> = crate::BitWriter<'a, REG, Lpwkup>;
impl<'a, REG> LpwkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpwkup::Disable)
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpwkup::Enable)
    }
}
#[doc = "Analog References fast wake-up in case of Hardware Pin reset: .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwwkup {
    #[doc = "0: Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    Disable = 0,
    #[doc = "1: Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    Enable = 1,
}
impl From<Hwwkup> for bool {
    #[inline(always)]
    fn from(variant: Hwwkup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWWKUP` reader - Analog References fast wake-up in case of Hardware Pin reset: ."]
pub type HwwkupR = crate::BitReader<Hwwkup>;
impl HwwkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwwkup {
        match self.bits {
            false => Hwwkup::Disable,
            true => Hwwkup::Enable,
        }
    }
    #[doc = "Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hwwkup::Disable
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hwwkup::Enable
    }
}
#[doc = "Field `HWWKUP` writer - Analog References fast wake-up in case of Hardware Pin reset: ."]
pub type HwwkupW<'a, REG> = crate::BitWriter<'a, REG, Hwwkup>;
impl<'a, REG> HwwkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hwwkup::Disable)
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hwwkup::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[inline(always)]
    pub fn lpwkup(&self) -> LpwkupR {
        LpwkupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[inline(always)]
    pub fn hwwkup(&self) -> HwwkupR {
        HwwkupR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[inline(always)]
    pub fn lpwkup(&mut self) -> LpwkupW<'_, ReffastwkupSpec> {
        LpwkupW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[inline(always)]
    pub fn hwwkup(&mut self) -> HwwkupW<'_, ReffastwkupSpec> {
        HwwkupW::new(self, 1)
    }
}
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`reffastwkup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reffastwkup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReffastwkupSpec;
impl crate::RegisterSpec for ReffastwkupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reffastwkup::R`](R) reader structure"]
impl crate::Readable for ReffastwkupSpec {}
#[doc = "`write(|w| ..)` method takes [`reffastwkup::W`](W) writer structure"]
impl crate::Writable for ReffastwkupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFFASTWKUP to value 0x01"]
impl crate::Resettable for ReffastwkupSpec {
    const RESET_VALUE: u32 = 0x01;
}

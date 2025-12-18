#[doc = "Register `STATUSCLK` reader"]
pub type R = crate::R<StatusclkSpec>;
#[doc = "Register `STATUSCLK` writer"]
pub type W = crate::W<StatusclkSpec>;
#[doc = "Field `XTAL32KOK` reader - XTAL oscillator 32 K OK signal."]
pub type Xtal32kokR = crate::BitReader;
#[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtal32koscfailure {
    #[doc = "0: No oscillation failure has been detetced since the last time this bit has been cleared."]
    Nofail = 0,
    #[doc = "1: At least one oscillation failure has been detetced since the last time this bit has been cleared."]
    Failure = 1,
}
impl From<Xtal32koscfailure> for bool {
    #[inline(always)]
    fn from(variant: Xtal32koscfailure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTAL32KOSCFAILURE` reader - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub type Xtal32koscfailureR = crate::BitReader<Xtal32koscfailure>;
impl Xtal32koscfailureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtal32koscfailure {
        match self.bits {
            false => Xtal32koscfailure::Nofail,
            true => Xtal32koscfailure::Failure,
        }
    }
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared."]
    #[inline(always)]
    pub fn is_nofail(&self) -> bool {
        *self == Xtal32koscfailure::Nofail
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared."]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == Xtal32koscfailure::Failure
    }
}
#[doc = "Field `XTAL32KOSCFAILURE` writer - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub type Xtal32koscfailureW<'a, REG> = crate::BitWriter<'a, REG, Xtal32koscfailure>;
impl<'a, REG> Xtal32koscfailureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared."]
    #[inline(always)]
    pub fn nofail(self) -> &'a mut crate::W<REG> {
        self.variant(Xtal32koscfailure::Nofail)
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared."]
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(Xtal32koscfailure::Failure)
    }
}
impl R {
    #[doc = "Bit 0 - XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub fn xtal32kok(&self) -> Xtal32kokR {
        Xtal32kokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&self) -> Xtal32koscfailureR {
        Xtal32koscfailureR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&mut self) -> Xtal32koscfailureW<'_, StatusclkSpec> {
        Xtal32koscfailureW::new(self, 2)
    }
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`statusclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusclkSpec;
impl crate::RegisterSpec for StatusclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusclk::R`](R) reader structure"]
impl crate::Readable for StatusclkSpec {}
#[doc = "`write(|w| ..)` method takes [`statusclk::W`](W) writer structure"]
impl crate::Writable for StatusclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSCLK to value 0x06"]
impl crate::Resettable for StatusclkSpec {
    const RESET_VALUE: u32 = 0x06;
}

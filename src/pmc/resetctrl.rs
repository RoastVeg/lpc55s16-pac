#[doc = "Register `RESETCTRL` reader"]
pub type R = crate::R<ResetctrlSpec>;
#[doc = "Register `RESETCTRL` writer"]
pub type W = crate::W<ResetctrlSpec>;
#[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpdwakeupresetenable {
    #[doc = "0: Reset event from DEEP POWER DOWN mode is disable."]
    Disable = 0,
    #[doc = "1: Reset event from DEEP POWER DOWN mode is enable."]
    Enable = 1,
}
impl From<Dpdwakeupresetenable> for bool {
    #[inline(always)]
    fn from(variant: Dpdwakeupresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` reader - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DpdwakeupresetenableR = crate::BitReader<Dpdwakeupresetenable>;
impl DpdwakeupresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpdwakeupresetenable {
        match self.bits {
            false => Dpdwakeupresetenable::Disable,
            true => Dpdwakeupresetenable::Enable,
        }
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dpdwakeupresetenable::Disable
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dpdwakeupresetenable::Enable
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` writer - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DpdwakeupresetenableW<'a, REG> = crate::BitWriter<'a, REG, Dpdwakeupresetenable>;
impl<'a, REG> DpdwakeupresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdwakeupresetenable::Disable)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdwakeupresetenable::Enable)
    }
}
#[doc = "Software reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrresetenable {
    #[doc = "0: Software reset is disable."]
    Disable = 0,
    #[doc = "1: Software reset is enable."]
    Enable = 1,
}
impl From<Swrresetenable> for bool {
    #[inline(always)]
    fn from(variant: Swrresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRRESETENABLE` reader - Software reset enable."]
pub type SwrresetenableR = crate::BitReader<Swrresetenable>;
impl SwrresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrresetenable {
        match self.bits {
            false => Swrresetenable::Disable,
            true => Swrresetenable::Enable,
        }
    }
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Swrresetenable::Disable
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Swrresetenable::Enable
    }
}
#[doc = "Field `SWRRESETENABLE` writer - Software reset enable."]
pub type SwrresetenableW<'a, REG> = crate::BitWriter<'a, REG, Swrresetenable>;
impl<'a, REG> SwrresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrresetenable::Disable)
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrresetenable::Enable)
    }
}
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BodvbatresetenaSecure {
    #[doc = "1: Any other value than b10, BOD VBAT reset is enable."]
    Enable = 1,
    #[doc = "2: BOD VBAT reset is disable."]
    Disable = 2,
}
impl From<BodvbatresetenaSecure> for u8 {
    #[inline(always)]
    fn from(variant: BodvbatresetenaSecure) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BodvbatresetenaSecure {
    type Ux = u8;
}
impl crate::IsEnum for BodvbatresetenaSecure {}
#[doc = "Field `BODVBATRESETENA_SECURE` reader - BOD VBAT reset enable."]
pub type BodvbatresetenaSecureR = crate::FieldReader<BodvbatresetenaSecure>;
impl BodvbatresetenaSecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BodvbatresetenaSecure> {
        match self.bits {
            1 => Some(BodvbatresetenaSecure::Enable),
            2 => Some(BodvbatresetenaSecure::Disable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodvbatresetenaSecure::Enable
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodvbatresetenaSecure::Disable
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE` writer - BOD VBAT reset enable."]
pub type BodvbatresetenaSecureW<'a, REG> = crate::FieldWriter<'a, REG, 2, BodvbatresetenaSecure>;
impl<'a, REG> BodvbatresetenaSecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatresetenaSecure::Enable)
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatresetenaSecure::Disable)
    }
}
#[doc = "BOD Core reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BodcoreresetenaSecure {
    #[doc = "1: Any other value than b10, BOD Core reset is enable."]
    Enable = 1,
    #[doc = "2: BOD Core reset is disable."]
    Disable = 2,
}
impl From<BodcoreresetenaSecure> for u8 {
    #[inline(always)]
    fn from(variant: BodcoreresetenaSecure) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BodcoreresetenaSecure {
    type Ux = u8;
}
impl crate::IsEnum for BodcoreresetenaSecure {}
#[doc = "Field `BODCORERESETENA_SECURE` reader - BOD Core reset enable."]
pub type BodcoreresetenaSecureR = crate::FieldReader<BodcoreresetenaSecure>;
impl BodcoreresetenaSecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BodcoreresetenaSecure> {
        match self.bits {
            1 => Some(BodcoreresetenaSecure::Enable),
            2 => Some(BodcoreresetenaSecure::Disable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodcoreresetenaSecure::Enable
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodcoreresetenaSecure::Disable
    }
}
#[doc = "Field `BODCORERESETENA_SECURE` writer - BOD Core reset enable."]
pub type BodcoreresetenaSecureW<'a, REG> = crate::FieldWriter<'a, REG, 2, BodcoreresetenaSecure>;
impl<'a, REG> BodcoreresetenaSecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreresetenaSecure::Enable)
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreresetenaSecure::Disable)
    }
}
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BodvbatresetenaSecureDp {
    #[doc = "1: Any other value than b10, BOD VBAT reset is enable."]
    Enable = 1,
    #[doc = "2: BOD VBAT reset is disable."]
    Disable = 2,
}
impl From<BodvbatresetenaSecureDp> for u8 {
    #[inline(always)]
    fn from(variant: BodvbatresetenaSecureDp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BodvbatresetenaSecureDp {
    type Ux = u8;
}
impl crate::IsEnum for BodvbatresetenaSecureDp {}
#[doc = "Field `BODVBATRESETENA_SECURE_DP` reader - BOD VBAT reset enable."]
pub type BodvbatresetenaSecureDpR = crate::FieldReader<BodvbatresetenaSecureDp>;
impl BodvbatresetenaSecureDpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BodvbatresetenaSecureDp> {
        match self.bits {
            1 => Some(BodvbatresetenaSecureDp::Enable),
            2 => Some(BodvbatresetenaSecureDp::Disable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodvbatresetenaSecureDp::Enable
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodvbatresetenaSecureDp::Disable
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE_DP` writer - BOD VBAT reset enable."]
pub type BodvbatresetenaSecureDpW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, BodvbatresetenaSecureDp>;
impl<'a, REG> BodvbatresetenaSecureDpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatresetenaSecureDp::Enable)
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatresetenaSecureDp::Disable)
    }
}
#[doc = "BOD Core reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BodcoreresetenaSecureDp {
    #[doc = "1: Any other value than b10, BOD Core reset is enable."]
    Enable = 1,
    #[doc = "2: BOD Core reset is disable."]
    Disable = 2,
}
impl From<BodcoreresetenaSecureDp> for u8 {
    #[inline(always)]
    fn from(variant: BodcoreresetenaSecureDp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BodcoreresetenaSecureDp {
    type Ux = u8;
}
impl crate::IsEnum for BodcoreresetenaSecureDp {}
#[doc = "Field `BODCORERESETENA_SECURE_DP` reader - BOD Core reset enable."]
pub type BodcoreresetenaSecureDpR = crate::FieldReader<BodcoreresetenaSecureDp>;
impl BodcoreresetenaSecureDpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BodcoreresetenaSecureDp> {
        match self.bits {
            1 => Some(BodcoreresetenaSecureDp::Enable),
            2 => Some(BodcoreresetenaSecureDp::Disable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodcoreresetenaSecureDp::Enable
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodcoreresetenaSecureDp::Disable
    }
}
#[doc = "Field `BODCORERESETENA_SECURE_DP` writer - BOD Core reset enable."]
pub type BodcoreresetenaSecureDpW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, BodcoreresetenaSecureDp>;
impl<'a, REG> BodcoreresetenaSecureDpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreresetenaSecureDp::Enable)
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreresetenaSecureDp::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&self) -> DpdwakeupresetenableR {
        DpdwakeupresetenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&self) -> SwrresetenableR {
        SwrresetenableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure(&self) -> BodvbatresetenaSecureR {
        BodvbatresetenaSecureR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure(&self) -> BodcoreresetenaSecureR {
        BodcoreresetenaSecureR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure_dp(&self) -> BodvbatresetenaSecureDpR {
        BodvbatresetenaSecureDpR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure_dp(&self) -> BodcoreresetenaSecureDpR {
        BodcoreresetenaSecureDpR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&mut self) -> DpdwakeupresetenableW<'_, ResetctrlSpec> {
        DpdwakeupresetenableW::new(self, 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&mut self) -> SwrresetenableW<'_, ResetctrlSpec> {
        SwrresetenableW::new(self, 3)
    }
    #[doc = "Bits 4:5 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure(&mut self) -> BodvbatresetenaSecureW<'_, ResetctrlSpec> {
        BodvbatresetenaSecureW::new(self, 4)
    }
    #[doc = "Bits 6:7 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure(&mut self) -> BodcoreresetenaSecureW<'_, ResetctrlSpec> {
        BodcoreresetenaSecureW::new(self, 6)
    }
    #[doc = "Bits 28:29 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure_dp(&mut self) -> BodvbatresetenaSecureDpW<'_, ResetctrlSpec> {
        BodvbatresetenaSecureDpW::new(self, 28)
    }
    #[doc = "Bits 30:31 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure_dp(&mut self) -> BodcoreresetenaSecureDpW<'_, ResetctrlSpec> {
        BodcoreresetenaSecureDpW::new(self, 30)
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`resetctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetctrlSpec;
impl crate::RegisterSpec for ResetctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetctrl::R`](R) reader structure"]
impl crate::Readable for ResetctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`resetctrl::W`](W) writer structure"]
impl crate::Writable for ResetctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESETCTRL to value 0x5000_0050"]
impl crate::Resettable for ResetctrlSpec {
    const RESET_VALUE: u32 = 0x5000_0050;
}

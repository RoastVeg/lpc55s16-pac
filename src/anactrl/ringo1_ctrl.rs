#[doc = "Register `RINGO1_CTRL` reader"]
pub type R = crate::R<Ringo1CtrlSpec>;
#[doc = "Register `RINGO1_CTRL` writer"]
pub type W = crate::W<Ringo1CtrlSpec>;
#[doc = "Select short or long ringo (for all ringos types).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S {
    #[doc = "0: Select short ringo (few elements)."]
    Short = 0,
    #[doc = "1: Select long ringo (many elements)."]
    Long = 1,
}
impl From<S> for bool {
    #[inline(always)]
    fn from(variant: S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S` reader - Select short or long ringo (for all ringos types)."]
pub type SR = crate::BitReader<S>;
impl SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S {
        match self.bits {
            false => S::Short,
            true => S::Long,
        }
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == S::Short
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == S::Long
    }
}
#[doc = "Field `S` writer - Select short or long ringo (for all ringos types)."]
pub type SW<'a, REG> = crate::BitWriter<'a, REG, S>;
impl<'a, REG> SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(S::Short)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(S::Long)
    }
}
#[doc = "Ringo frequency output divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fs {
    #[doc = "0: High frequency output (frequency lower than 100 MHz)."]
    Fast = 0,
    #[doc = "1: Low frequency output (frequency lower than 10 MHz)."]
    Slow = 1,
}
impl From<Fs> for bool {
    #[inline(always)]
    fn from(variant: Fs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FS` reader - Ringo frequency output divider."]
pub type FsR = crate::BitReader<Fs>;
impl FsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fs {
        match self.bits {
            false => Fs::Fast,
            true => Fs::Slow,
        }
    }
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Fs::Fast
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Fs::Slow
    }
}
#[doc = "Field `FS` writer - Ringo frequency output divider."]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG, Fs>;
impl<'a, REG> FsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::Fast)
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Fs::Slow)
    }
}
#[doc = "Ringo module Power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pd {
    #[doc = "0: The Ringo module is enabled."]
    PoweredOn = 0,
    #[doc = "1: The Ringo module is disabled."]
    PoweredDown = 1,
}
impl From<Pd> for bool {
    #[inline(always)]
    fn from(variant: Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Ringo module Power control."]
pub type PdR = crate::BitReader<Pd>;
impl PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd {
        match self.bits {
            false => Pd::PoweredOn,
            true => Pd::PoweredDown,
        }
    }
    #[doc = "The Ringo module is enabled."]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == Pd::PoweredOn
    }
    #[doc = "The Ringo module is disabled."]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == Pd::PoweredDown
    }
}
#[doc = "Field `PD` writer - Ringo module Power control."]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG, Pd>;
impl<'a, REG> PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Ringo module is enabled."]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(Pd::PoweredOn)
    }
    #[doc = "The Ringo module is disabled."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pd::PoweredDown)
    }
}
#[doc = ".\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ER24 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<ER24> for bool {
    #[inline(always)]
    fn from(variant: ER24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_R24` reader - ."]
pub type ER24R = crate::BitReader<ER24>;
impl ER24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ER24 {
        match self.bits {
            false => ER24::Disable,
            true => ER24::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ER24::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ER24::Enable
    }
}
#[doc = "Field `E_R24` writer - ."]
pub type ER24W<'a, REG> = crate::BitWriter<'a, REG, ER24>;
impl<'a, REG> ER24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ER24::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ER24::Enable)
    }
}
#[doc = ".\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ER35 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<ER35> for bool {
    #[inline(always)]
    fn from(variant: ER35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_R35` reader - ."]
pub type ER35R = crate::BitReader<ER35>;
impl ER35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ER35 {
        match self.bits {
            false => ER35::Disable,
            true => ER35::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ER35::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ER35::Enable
    }
}
#[doc = "Field `E_R35` writer - ."]
pub type ER35W<'a, REG> = crate::BitWriter<'a, REG, ER35>;
impl<'a, REG> ER35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ER35::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ER35::Enable)
    }
}
#[doc = "Metal 2 (M2) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM2 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<EM2> for bool {
    #[inline(always)]
    fn from(variant: EM2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_M2` reader - Metal 2 (M2) monitor control."]
pub type EM2R = crate::BitReader<EM2>;
impl EM2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM2 {
        match self.bits {
            false => EM2::Disable,
            true => EM2::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM2::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM2::Enable
    }
}
#[doc = "Field `E_M2` writer - Metal 2 (M2) monitor control."]
pub type EM2W<'a, REG> = crate::BitWriter<'a, REG, EM2>;
impl<'a, REG> EM2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EM2::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EM2::Enable)
    }
}
#[doc = "Metal 3 (M3) monitor control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM3 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<EM3> for bool {
    #[inline(always)]
    fn from(variant: EM3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_M3` reader - Metal 3 (M3) monitor control."]
pub type EM3R = crate::BitReader<EM3>;
impl EM3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM3 {
        match self.bits {
            false => EM3::Disable,
            true => EM3::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM3::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM3::Enable
    }
}
#[doc = "Field `E_M3` writer - Metal 3 (M3) monitor control."]
pub type EM3W<'a, REG> = crate::BitWriter<'a, REG, EM3>;
impl<'a, REG> EM3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EM3::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EM3::Enable)
    }
}
#[doc = "Metal 4 (M4) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM4 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<EM4> for bool {
    #[inline(always)]
    fn from(variant: EM4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_M4` reader - Metal 4 (M4) monitor control."]
pub type EM4R = crate::BitReader<EM4>;
impl EM4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM4 {
        match self.bits {
            false => EM4::Disable,
            true => EM4::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM4::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM4::Enable
    }
}
#[doc = "Field `E_M4` writer - Metal 4 (M4) monitor control."]
pub type EM4W<'a, REG> = crate::BitWriter<'a, REG, EM4>;
impl<'a, REG> EM4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EM4::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EM4::Enable)
    }
}
#[doc = "Metal 5 (M5) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM5 {
    #[doc = "0: Ringo is disabled."]
    Disable = 0,
    #[doc = "1: Ringo is enabled."]
    Enable = 1,
}
impl From<EM5> for bool {
    #[inline(always)]
    fn from(variant: EM5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_M5` reader - Metal 5 (M5) monitor control."]
pub type EM5R = crate::BitReader<EM5>;
impl EM5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM5 {
        match self.bits {
            false => EM5::Disable,
            true => EM5::Enable,
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM5::Disable
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EM5::Enable
    }
}
#[doc = "Field `E_M5` writer - Metal 5 (M5) monitor control."]
pub type EM5W<'a, REG> = crate::BitWriter<'a, REG, EM5>;
impl<'a, REG> EM5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EM5::Disable)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EM5::Enable)
    }
}
#[doc = "Field `DIVISOR` reader - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DivisorR = crate::FieldReader;
#[doc = "Field `DIVISOR` writer - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIV_UPDATE_REQ` reader - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type DivUpdateReqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn e_r24(&self) -> ER24R {
        ER24R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn e_r35(&self) -> ER35R {
        ER35R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub fn e_m2(&self) -> EM2R {
        EM2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub fn e_m3(&self) -> EM3R {
        EM3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub fn e_m4(&self) -> EM4R {
        EM4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub fn e_m5(&self) -> EM5R {
        EM5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn div_update_req(&self) -> DivUpdateReqR {
        DivUpdateReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn s(&mut self) -> SW<'_, Ringo1CtrlSpec> {
        SW::new(self, 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, Ringo1CtrlSpec> {
        FsW::new(self, 1)
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, Ringo1CtrlSpec> {
        PdW::new(self, 2)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn e_r24(&mut self) -> ER24W<'_, Ringo1CtrlSpec> {
        ER24W::new(self, 3)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn e_r35(&mut self) -> ER35W<'_, Ringo1CtrlSpec> {
        ER35W::new(self, 4)
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub fn e_m2(&mut self) -> EM2W<'_, Ringo1CtrlSpec> {
        EM2W::new(self, 5)
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub fn e_m3(&mut self) -> EM3W<'_, Ringo1CtrlSpec> {
        EM3W::new(self, 6)
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub fn e_m4(&mut self) -> EM4W<'_, Ringo1CtrlSpec> {
        EM4W::new(self, 7)
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub fn e_m5(&mut self) -> EM5W<'_, Ringo1CtrlSpec> {
        EM5W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DivisorW<'_, Ringo1CtrlSpec> {
        DivisorW::new(self, 16)
    }
}
#[doc = "Second Ring Oscillator module control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringo1CtrlSpec;
impl crate::RegisterSpec for Ringo1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringo1_ctrl::R`](R) reader structure"]
impl crate::Readable for Ringo1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ringo1_ctrl::W`](W) writer structure"]
impl crate::Writable for Ringo1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RINGO1_CTRL to value 0x40"]
impl crate::Resettable for Ringo1CtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}

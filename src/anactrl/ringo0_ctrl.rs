#[doc = "Register `RINGO0_CTRL` reader"]
pub type R = crate::R<Ringo0CtrlSpec>;
#[doc = "Register `RINGO0_CTRL` writer"]
pub type W = crate::W<Ringo0CtrlSpec>;
#[doc = "Select short or long ringo (for all ringos types).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sl {
    #[doc = "0: Select short ringo (few elements)."]
    Short = 0,
    #[doc = "1: Select long ringo (many elements)."]
    Long = 1,
}
impl From<Sl> for bool {
    #[inline(always)]
    fn from(variant: Sl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SL` reader - Select short or long ringo (for all ringos types)."]
pub type SlR = crate::BitReader<Sl>;
impl SlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sl {
        match self.bits {
            false => Sl::Short,
            true => Sl::Long,
        }
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == Sl::Short
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == Sl::Long
    }
}
#[doc = "Field `SL` writer - Select short or long ringo (for all ringos types)."]
pub type SlW<'a, REG> = crate::BitWriter<'a, REG, Sl>;
impl<'a, REG> SlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(Sl::Short)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(Sl::Long)
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
#[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwnSwp {
    #[doc = "0: Normal mode."]
    Normal = 0,
    #[doc = "1: P-Monitor mode. Measure with weak P transistor."]
    PMonitor = 1,
    #[doc = "2: P-Monitor mode. Measure with weak N transistor."]
    NMonitor = 2,
    #[doc = "3: Don't use."]
    Forbidden = 3,
}
impl From<SwnSwp> for u8 {
    #[inline(always)]
    fn from(variant: SwnSwp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwnSwp {
    type Ux = u8;
}
impl crate::IsEnum for SwnSwp {}
#[doc = "Field `SWN_SWP` reader - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub type SwnSwpR = crate::FieldReader<SwnSwp>;
impl SwnSwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwnSwp {
        match self.bits {
            0 => SwnSwp::Normal,
            1 => SwnSwp::PMonitor,
            2 => SwnSwp::NMonitor,
            3 => SwnSwp::Forbidden,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SwnSwp::Normal
    }
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    #[inline(always)]
    pub fn is_p_monitor(&self) -> bool {
        *self == SwnSwp::PMonitor
    }
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    #[inline(always)]
    pub fn is_n_monitor(&self) -> bool {
        *self == SwnSwp::NMonitor
    }
    #[doc = "Don't use."]
    #[inline(always)]
    pub fn is_forbidden(&self) -> bool {
        *self == SwnSwp::Forbidden
    }
}
#[doc = "Field `SWN_SWP` writer - PN-Ringos (P-Transistor and N-Transistor processing) control."]
pub type SwnSwpW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwnSwp, crate::Safe>;
impl<'a, REG> SwnSwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SwnSwp::Normal)
    }
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    #[inline(always)]
    pub fn p_monitor(self) -> &'a mut crate::W<REG> {
        self.variant(SwnSwp::PMonitor)
    }
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    #[inline(always)]
    pub fn n_monitor(self) -> &'a mut crate::W<REG> {
        self.variant(SwnSwp::NMonitor)
    }
    #[doc = "Don't use."]
    #[inline(always)]
    pub fn forbidden(self) -> &'a mut crate::W<REG> {
        self.variant(SwnSwp::Forbidden)
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
#[doc = "First NAND2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENd0 {
    #[doc = "0: First NAND2-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: First NAND2-based ringo is enabled."]
    Enable = 1,
}
impl From<ENd0> for bool {
    #[inline(always)]
    fn from(variant: ENd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_ND0` reader - First NAND2-based ringo control."]
pub type ENd0R = crate::BitReader<ENd0>;
impl ENd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENd0 {
        match self.bits {
            false => ENd0::Disable,
            true => ENd0::Enable,
        }
    }
    #[doc = "First NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENd0::Disable
    }
    #[doc = "First NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENd0::Enable
    }
}
#[doc = "Field `E_ND0` writer - First NAND2-based ringo control."]
pub type ENd0W<'a, REG> = crate::BitWriter<'a, REG, ENd0>;
impl<'a, REG> ENd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENd0::Disable)
    }
    #[doc = "First NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENd0::Enable)
    }
}
#[doc = "Second NAND2-based ringo control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENd1 {
    #[doc = "0: Second NAND2-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: Second NAND2-based ringo is enabled."]
    Enable = 1,
}
impl From<ENd1> for bool {
    #[inline(always)]
    fn from(variant: ENd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_ND1` reader - Second NAND2-based ringo control."]
pub type ENd1R = crate::BitReader<ENd1>;
impl ENd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENd1 {
        match self.bits {
            false => ENd1::Disable,
            true => ENd1::Enable,
        }
    }
    #[doc = "Second NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENd1::Disable
    }
    #[doc = "Second NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENd1::Enable
    }
}
#[doc = "Field `E_ND1` writer - Second NAND2-based ringo control."]
pub type ENd1W<'a, REG> = crate::BitWriter<'a, REG, ENd1>;
impl<'a, REG> ENd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second NAND2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENd1::Disable)
    }
    #[doc = "Second NAND2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENd1::Enable)
    }
}
#[doc = "First NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENr0 {
    #[doc = "0: First NOR2-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: First NOR2-based ringo is enabled."]
    Enable = 1,
}
impl From<ENr0> for bool {
    #[inline(always)]
    fn from(variant: ENr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_NR0` reader - First NOR2-based ringo control."]
pub type ENr0R = crate::BitReader<ENr0>;
impl ENr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENr0 {
        match self.bits {
            false => ENr0::Disable,
            true => ENr0::Enable,
        }
    }
    #[doc = "First NOR2-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENr0::Disable
    }
    #[doc = "First NOR2-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENr0::Enable
    }
}
#[doc = "Field `E_NR0` writer - First NOR2-based ringo control."]
pub type ENr0W<'a, REG> = crate::BitWriter<'a, REG, ENr0>;
impl<'a, REG> ENr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First NOR2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENr0::Disable)
    }
    #[doc = "First NOR2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENr0::Enable)
    }
}
#[doc = "Second NOR2-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENr1 {
    #[doc = "0: Second NORD2-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: Second NORD2-based ringo is enabled."]
    Enable = 1,
}
impl From<ENr1> for bool {
    #[inline(always)]
    fn from(variant: ENr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_NR1` reader - Second NOR2-based ringo control."]
pub type ENr1R = crate::BitReader<ENr1>;
impl ENr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENr1 {
        match self.bits {
            false => ENr1::Disable,
            true => ENr1::Enable,
        }
    }
    #[doc = "Second NORD2-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENr1::Disable
    }
    #[doc = "Second NORD2-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENr1::Enable
    }
}
#[doc = "Field `E_NR1` writer - Second NOR2-based ringo control."]
pub type ENr1W<'a, REG> = crate::BitWriter<'a, REG, ENr1>;
impl<'a, REG> ENr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second NORD2-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENr1::Disable)
    }
    #[doc = "Second NORD2-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENr1::Enable)
    }
}
#[doc = "First Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIv0 {
    #[doc = "0: First INV-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: First INV-based ringo is enabled."]
    Enable = 1,
}
impl From<EIv0> for bool {
    #[inline(always)]
    fn from(variant: EIv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_IV0` reader - First Inverter-based ringo control."]
pub type EIv0R = crate::BitReader<EIv0>;
impl EIv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIv0 {
        match self.bits {
            false => EIv0::Disable,
            true => EIv0::Enable,
        }
    }
    #[doc = "First INV-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EIv0::Disable
    }
    #[doc = "First INV-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EIv0::Enable
    }
}
#[doc = "Field `E_IV0` writer - First Inverter-based ringo control."]
pub type EIv0W<'a, REG> = crate::BitWriter<'a, REG, EIv0>;
impl<'a, REG> EIv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EIv0::Disable)
    }
    #[doc = "First INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EIv0::Enable)
    }
}
#[doc = "Second Inverter-based ringo control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIv1 {
    #[doc = "0: Second INV-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: Second INV-based ringo is enabled."]
    Enable = 1,
}
impl From<EIv1> for bool {
    #[inline(always)]
    fn from(variant: EIv1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_IV1` reader - Second Inverter-based ringo control."]
pub type EIv1R = crate::BitReader<EIv1>;
impl EIv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIv1 {
        match self.bits {
            false => EIv1::Disable,
            true => EIv1::Enable,
        }
    }
    #[doc = "Second INV-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EIv1::Disable
    }
    #[doc = "Second INV-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EIv1::Enable
    }
}
#[doc = "Field `E_IV1` writer - Second Inverter-based ringo control."]
pub type EIv1W<'a, REG> = crate::BitWriter<'a, REG, EIv1>;
impl<'a, REG> EIv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second INV-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EIv1::Disable)
    }
    #[doc = "Second INV-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EIv1::Enable)
    }
}
#[doc = "First PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPn0 {
    #[doc = "0: First PN-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: First PN-based ringo is enabled."]
    Enable = 1,
}
impl From<EPn0> for bool {
    #[inline(always)]
    fn from(variant: EPn0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_PN0` reader - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub type EPn0R = crate::BitReader<EPn0>;
impl EPn0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPn0 {
        match self.bits {
            false => EPn0::Disable,
            true => EPn0::Enable,
        }
    }
    #[doc = "First PN-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EPn0::Disable
    }
    #[doc = "First PN-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EPn0::Enable
    }
}
#[doc = "Field `E_PN0` writer - First PN (P-Transistor and N-Transistor processing) monitor control."]
pub type EPn0W<'a, REG> = crate::BitWriter<'a, REG, EPn0>;
impl<'a, REG> EPn0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EPn0::Disable)
    }
    #[doc = "First PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EPn0::Enable)
    }
}
#[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPn1 {
    #[doc = "0: Second PN-based ringo is disabled."]
    Disable = 0,
    #[doc = "1: Second PN-based ringo is enabled."]
    Enable = 1,
}
impl From<EPn1> for bool {
    #[inline(always)]
    fn from(variant: EPn1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E_PN1` reader - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub type EPn1R = crate::BitReader<EPn1>;
impl EPn1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPn1 {
        match self.bits {
            false => EPn1::Disable,
            true => EPn1::Enable,
        }
    }
    #[doc = "Second PN-based ringo is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EPn1::Disable
    }
    #[doc = "Second PN-based ringo is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EPn1::Enable
    }
}
#[doc = "Field `E_PN1` writer - Second PN (P-Transistor and N-Transistor processing) monitor control."]
pub type EPn1W<'a, REG> = crate::BitWriter<'a, REG, EPn1>;
impl<'a, REG> EPn1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second PN-based ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EPn1::Disable)
    }
    #[doc = "Second PN-based ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EPn1::Enable)
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
    pub fn sl(&self) -> SlR {
        SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub fn swn_swp(&self) -> SwnSwpR {
        SwnSwpR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd0(&self) -> ENd0R {
        ENd0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd1(&self) -> ENd1R {
        ENd1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr0(&self) -> ENr0R {
        ENr0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr1(&self) -> ENr1R {
        ENr1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv0(&self) -> EIv0R {
        EIv0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv1(&self) -> EIv1R {
        EIv1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn0(&self) -> EPn0R {
        EPn0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn1(&self) -> EPn1R {
        EPn1R::new(((self.bits >> 12) & 1) != 0)
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
    pub fn sl(&mut self) -> SlW<'_, Ringo0CtrlSpec> {
        SlW::new(self, 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, Ringo0CtrlSpec> {
        FsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub fn swn_swp(&mut self) -> SwnSwpW<'_, Ringo0CtrlSpec> {
        SwnSwpW::new(self, 2)
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, Ringo0CtrlSpec> {
        PdW::new(self, 4)
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd0(&mut self) -> ENd0W<'_, Ringo0CtrlSpec> {
        ENd0W::new(self, 5)
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline(always)]
    pub fn e_nd1(&mut self) -> ENd1W<'_, Ringo0CtrlSpec> {
        ENd1W::new(self, 6)
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr0(&mut self) -> ENr0W<'_, Ringo0CtrlSpec> {
        ENr0W::new(self, 7)
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline(always)]
    pub fn e_nr1(&mut self) -> ENr1W<'_, Ringo0CtrlSpec> {
        ENr1W::new(self, 8)
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv0(&mut self) -> EIv0W<'_, Ringo0CtrlSpec> {
        EIv0W::new(self, 9)
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline(always)]
    pub fn e_iv1(&mut self) -> EIv1W<'_, Ringo0CtrlSpec> {
        EIv1W::new(self, 10)
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn0(&mut self) -> EPn0W<'_, Ringo0CtrlSpec> {
        EPn0W::new(self, 11)
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub fn e_pn1(&mut self) -> EPn1W<'_, Ringo0CtrlSpec> {
        EPn1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DivisorW<'_, Ringo0CtrlSpec> {
        DivisorW::new(self, 16)
    }
}
#[doc = "First Ring Oscillator module control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringo0CtrlSpec;
impl crate::RegisterSpec for Ringo0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringo0_ctrl::R`](R) reader structure"]
impl crate::Readable for Ringo0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ringo0_ctrl::W`](W) writer structure"]
impl crate::Writable for Ringo0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RINGO0_CTRL to value 0x40"]
impl crate::Resettable for Ringo0CtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}

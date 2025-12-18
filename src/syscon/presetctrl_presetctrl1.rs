#[doc = "Register `PRESETCTRL1` reader"]
pub type R = crate::R<PresetctrlPresetctrl1Spec>;
#[doc = "Register `PRESETCTRL1` writer"]
pub type W = crate::W<PresetctrlPresetctrl1Spec>;
#[doc = "MRT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MrtRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<MrtRst> for bool {
    #[inline(always)]
    fn from(variant: MrtRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT_RST` reader - MRT reset control."]
pub type MrtRstR = crate::BitReader<MrtRst>;
impl MrtRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MrtRst {
        match self.bits {
            false => MrtRst::Released,
            true => MrtRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MrtRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MrtRst::Asserted
    }
}
#[doc = "Field `MRT_RST` writer - MRT reset control."]
pub type MrtRstW<'a, REG> = crate::BitWriter<'a, REG, MrtRst>;
impl<'a, REG> MrtRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRst::Asserted)
    }
}
#[doc = "OS Event Timer reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OstimerRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<OstimerRst> for bool {
    #[inline(always)]
    fn from(variant: OstimerRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTIMER_RST` reader - OS Event Timer reset control."]
pub type OstimerRstR = crate::BitReader<OstimerRst>;
impl OstimerRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OstimerRst {
        match self.bits {
            false => OstimerRst::Released,
            true => OstimerRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == OstimerRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == OstimerRst::Asserted
    }
}
#[doc = "Field `OSTIMER_RST` writer - OS Event Timer reset control."]
pub type OstimerRstW<'a, REG> = crate::BitWriter<'a, REG, OstimerRst>;
impl<'a, REG> OstimerRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(OstimerRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(OstimerRst::Asserted)
    }
}
#[doc = "SCT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<SctRst> for bool {
    #[inline(always)]
    fn from(variant: SctRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_RST` reader - SCT reset control."]
pub type SctRstR = crate::BitReader<SctRst>;
impl SctRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctRst {
        match self.bits {
            false => SctRst::Released,
            true => SctRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SctRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SctRst::Asserted
    }
}
#[doc = "Field `SCT_RST` writer - SCT reset control."]
pub type SctRstW<'a, REG> = crate::BitWriter<'a, REG, SctRst>;
impl<'a, REG> SctRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(SctRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SctRst::Asserted)
    }
}
#[doc = "CAN reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<CanRst> for bool {
    #[inline(always)]
    fn from(variant: CanRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN_RST` reader - CAN reset control."]
pub type CanRstR = crate::BitReader<CanRst>;
impl CanRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CanRst {
        match self.bits {
            false => CanRst::Released,
            true => CanRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CanRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CanRst::Asserted
    }
}
#[doc = "Field `CAN_RST` writer - CAN reset control."]
pub type CanRstW<'a, REG> = crate::BitWriter<'a, REG, CanRst>;
impl<'a, REG> CanRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(CanRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(CanRst::Asserted)
    }
}
#[doc = "UTICK reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtickRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<UtickRst> for bool {
    #[inline(always)]
    fn from(variant: UtickRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK_RST` reader - UTICK reset control."]
pub type UtickRstR = crate::BitReader<UtickRst>;
impl UtickRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UtickRst {
        match self.bits {
            false => UtickRst::Released,
            true => UtickRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == UtickRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == UtickRst::Asserted
    }
}
#[doc = "Field `UTICK_RST` writer - UTICK reset control."]
pub type UtickRstW<'a, REG> = crate::BitWriter<'a, REG, UtickRst>;
impl<'a, REG> UtickRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRst::Asserted)
    }
}
#[doc = "FC0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc0Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0_RST` reader - FC0 reset control."]
pub type Fc0RstR = crate::BitReader<Fc0Rst>;
impl Fc0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc0Rst {
        match self.bits {
            false => Fc0Rst::Released,
            true => Fc0Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc0Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc0Rst::Asserted
    }
}
#[doc = "Field `FC0_RST` writer - FC0 reset control."]
pub type Fc0RstW<'a, REG> = crate::BitWriter<'a, REG, Fc0Rst>;
impl<'a, REG> Fc0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0Rst::Asserted)
    }
}
#[doc = "FC1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc1Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1_RST` reader - FC1 reset control."]
pub type Fc1RstR = crate::BitReader<Fc1Rst>;
impl Fc1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc1Rst {
        match self.bits {
            false => Fc1Rst::Released,
            true => Fc1Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc1Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc1Rst::Asserted
    }
}
#[doc = "Field `FC1_RST` writer - FC1 reset control."]
pub type Fc1RstW<'a, REG> = crate::BitWriter<'a, REG, Fc1Rst>;
impl<'a, REG> Fc1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1Rst::Asserted)
    }
}
#[doc = "FC2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc2Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2_RST` reader - FC2 reset control."]
pub type Fc2RstR = crate::BitReader<Fc2Rst>;
impl Fc2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc2Rst {
        match self.bits {
            false => Fc2Rst::Released,
            true => Fc2Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc2Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc2Rst::Asserted
    }
}
#[doc = "Field `FC2_RST` writer - FC2 reset control."]
pub type Fc2RstW<'a, REG> = crate::BitWriter<'a, REG, Fc2Rst>;
impl<'a, REG> Fc2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2Rst::Asserted)
    }
}
#[doc = "FC3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc3Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc3Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3_RST` reader - FC3 reset control."]
pub type Fc3RstR = crate::BitReader<Fc3Rst>;
impl Fc3RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc3Rst {
        match self.bits {
            false => Fc3Rst::Released,
            true => Fc3Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc3Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc3Rst::Asserted
    }
}
#[doc = "Field `FC3_RST` writer - FC3 reset control."]
pub type Fc3RstW<'a, REG> = crate::BitWriter<'a, REG, Fc3Rst>;
impl<'a, REG> Fc3RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3Rst::Asserted)
    }
}
#[doc = "FC4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc4Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc4Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4_RST` reader - FC4 reset control."]
pub type Fc4RstR = crate::BitReader<Fc4Rst>;
impl Fc4RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc4Rst {
        match self.bits {
            false => Fc4Rst::Released,
            true => Fc4Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc4Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc4Rst::Asserted
    }
}
#[doc = "Field `FC4_RST` writer - FC4 reset control."]
pub type Fc4RstW<'a, REG> = crate::BitWriter<'a, REG, Fc4Rst>;
impl<'a, REG> Fc4RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4Rst::Asserted)
    }
}
#[doc = "FC5 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc5Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc5Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5_RST` reader - FC5 reset control."]
pub type Fc5RstR = crate::BitReader<Fc5Rst>;
impl Fc5RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc5Rst {
        match self.bits {
            false => Fc5Rst::Released,
            true => Fc5Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc5Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc5Rst::Asserted
    }
}
#[doc = "Field `FC5_RST` writer - FC5 reset control."]
pub type Fc5RstW<'a, REG> = crate::BitWriter<'a, REG, Fc5Rst>;
impl<'a, REG> Fc5RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5Rst::Asserted)
    }
}
#[doc = "FC6 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc6Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc6Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6_RST` reader - FC6 reset control."]
pub type Fc6RstR = crate::BitReader<Fc6Rst>;
impl Fc6RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc6Rst {
        match self.bits {
            false => Fc6Rst::Released,
            true => Fc6Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc6Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc6Rst::Asserted
    }
}
#[doc = "Field `FC6_RST` writer - FC6 reset control."]
pub type Fc6RstW<'a, REG> = crate::BitWriter<'a, REG, Fc6Rst>;
impl<'a, REG> Fc6RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6Rst::Asserted)
    }
}
#[doc = "FC7 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Fc7Rst> for bool {
    #[inline(always)]
    fn from(variant: Fc7Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7_RST` reader - FC7 reset control."]
pub type Fc7RstR = crate::BitReader<Fc7Rst>;
impl Fc7RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc7Rst {
        match self.bits {
            false => Fc7Rst::Released,
            true => Fc7Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Fc7Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Fc7Rst::Asserted
    }
}
#[doc = "Field `FC7_RST` writer - FC7 reset control."]
pub type Fc7RstW<'a, REG> = crate::BitWriter<'a, REG, Fc7Rst>;
impl<'a, REG> Fc7RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7Rst::Asserted)
    }
}
#[doc = "Timer 2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer2Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Timer2Rst> for bool {
    #[inline(always)]
    fn from(variant: Timer2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2_RST` reader - Timer 2 reset control."]
pub type Timer2RstR = crate::BitReader<Timer2Rst>;
impl Timer2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer2Rst {
        match self.bits {
            false => Timer2Rst::Released,
            true => Timer2Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Timer2Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Timer2Rst::Asserted
    }
}
#[doc = "Field `TIMER2_RST` writer - Timer 2 reset control."]
pub type Timer2RstW<'a, REG> = crate::BitWriter<'a, REG, Timer2Rst>;
impl<'a, REG> Timer2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2Rst::Asserted)
    }
}
#[doc = "USB0-FS DEV reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0DevRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb0DevRst> for bool {
    #[inline(always)]
    fn from(variant: Usb0DevRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_DEV_RST` reader - USB0-FS DEV reset control."]
pub type Usb0DevRstR = crate::BitReader<Usb0DevRst>;
impl Usb0DevRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0DevRst {
        match self.bits {
            false => Usb0DevRst::Released,
            true => Usb0DevRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb0DevRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb0DevRst::Asserted
    }
}
#[doc = "Field `USB0_DEV_RST` writer - USB0-FS DEV reset control."]
pub type Usb0DevRstW<'a, REG> = crate::BitWriter<'a, REG, Usb0DevRst>;
impl<'a, REG> Usb0DevRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0DevRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0DevRst::Asserted)
    }
}
#[doc = "Timer 0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Timer0Rst> for bool {
    #[inline(always)]
    fn from(variant: Timer0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_RST` reader - Timer 0 reset control."]
pub type Timer0RstR = crate::BitReader<Timer0Rst>;
impl Timer0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0Rst {
        match self.bits {
            false => Timer0Rst::Released,
            true => Timer0Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Timer0Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Timer0Rst::Asserted
    }
}
#[doc = "Field `TIMER0_RST` writer - Timer 0 reset control."]
pub type Timer0RstW<'a, REG> = crate::BitWriter<'a, REG, Timer0Rst>;
impl<'a, REG> Timer0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Rst::Asserted)
    }
}
#[doc = "Timer 1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Timer1Rst> for bool {
    #[inline(always)]
    fn from(variant: Timer1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1_RST` reader - Timer 1 reset control."]
pub type Timer1RstR = crate::BitReader<Timer1Rst>;
impl Timer1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1Rst {
        match self.bits {
            false => Timer1Rst::Released,
            true => Timer1Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Timer1Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Timer1Rst::Asserted
    }
}
#[doc = "Field `TIMER1_RST` writer - Timer 1 reset control."]
pub type Timer1RstW<'a, REG> = crate::BitWriter<'a, REG, Timer1Rst>;
impl<'a, REG> Timer1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Rst::Asserted)
    }
}
impl R {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MrtRstR {
        MrtRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    pub fn ostimer_rst(&self) -> OstimerRstR {
        OstimerRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst(&self) -> SctRstR {
        SctRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN reset control."]
    #[inline(always)]
    pub fn can_rst(&self) -> CanRstR {
        CanRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UtickRstR {
        UtickRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> Fc0RstR {
        Fc0RstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> Fc1RstR {
        Fc1RstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> Fc2RstR {
        Fc2RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> Fc3RstR {
        Fc3RstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> Fc4RstR {
        Fc4RstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> Fc5RstR {
        Fc5RstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> Fc6RstR {
        Fc6RstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> Fc7RstR {
        Fc7RstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    pub fn timer2_rst(&self) -> Timer2RstR {
        Timer2RstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - USB0-FS DEV reset control."]
    #[inline(always)]
    pub fn usb0_dev_rst(&self) -> Usb0DevRstR {
        Usb0DevRstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    pub fn timer0_rst(&self) -> Timer0RstR {
        Timer0RstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    pub fn timer1_rst(&self) -> Timer1RstR {
        Timer1RstR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRT reset control."]
    #[inline(always)]
    pub fn mrt_rst(&mut self) -> MrtRstW<'_, PresetctrlPresetctrl1Spec> {
        MrtRstW::new(self, 0)
    }
    #[doc = "Bit 1 - OS Event Timer reset control."]
    #[inline(always)]
    pub fn ostimer_rst(&mut self) -> OstimerRstW<'_, PresetctrlPresetctrl1Spec> {
        OstimerRstW::new(self, 1)
    }
    #[doc = "Bit 2 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst(&mut self) -> SctRstW<'_, PresetctrlPresetctrl1Spec> {
        SctRstW::new(self, 2)
    }
    #[doc = "Bit 7 - CAN reset control."]
    #[inline(always)]
    pub fn can_rst(&mut self) -> CanRstW<'_, PresetctrlPresetctrl1Spec> {
        CanRstW::new(self, 7)
    }
    #[doc = "Bit 10 - UTICK reset control."]
    #[inline(always)]
    pub fn utick_rst(&mut self) -> UtickRstW<'_, PresetctrlPresetctrl1Spec> {
        UtickRstW::new(self, 10)
    }
    #[doc = "Bit 11 - FC0 reset control."]
    #[inline(always)]
    pub fn fc0_rst(&mut self) -> Fc0RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc0RstW::new(self, 11)
    }
    #[doc = "Bit 12 - FC1 reset control."]
    #[inline(always)]
    pub fn fc1_rst(&mut self) -> Fc1RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc1RstW::new(self, 12)
    }
    #[doc = "Bit 13 - FC2 reset control."]
    #[inline(always)]
    pub fn fc2_rst(&mut self) -> Fc2RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc2RstW::new(self, 13)
    }
    #[doc = "Bit 14 - FC3 reset control."]
    #[inline(always)]
    pub fn fc3_rst(&mut self) -> Fc3RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc3RstW::new(self, 14)
    }
    #[doc = "Bit 15 - FC4 reset control."]
    #[inline(always)]
    pub fn fc4_rst(&mut self) -> Fc4RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc4RstW::new(self, 15)
    }
    #[doc = "Bit 16 - FC5 reset control."]
    #[inline(always)]
    pub fn fc5_rst(&mut self) -> Fc5RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc5RstW::new(self, 16)
    }
    #[doc = "Bit 17 - FC6 reset control."]
    #[inline(always)]
    pub fn fc6_rst(&mut self) -> Fc6RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc6RstW::new(self, 17)
    }
    #[doc = "Bit 18 - FC7 reset control."]
    #[inline(always)]
    pub fn fc7_rst(&mut self) -> Fc7RstW<'_, PresetctrlPresetctrl1Spec> {
        Fc7RstW::new(self, 18)
    }
    #[doc = "Bit 22 - Timer 2 reset control."]
    #[inline(always)]
    pub fn timer2_rst(&mut self) -> Timer2RstW<'_, PresetctrlPresetctrl1Spec> {
        Timer2RstW::new(self, 22)
    }
    #[doc = "Bit 25 - USB0-FS DEV reset control."]
    #[inline(always)]
    pub fn usb0_dev_rst(&mut self) -> Usb0DevRstW<'_, PresetctrlPresetctrl1Spec> {
        Usb0DevRstW::new(self, 25)
    }
    #[doc = "Bit 26 - Timer 0 reset control."]
    #[inline(always)]
    pub fn timer0_rst(&mut self) -> Timer0RstW<'_, PresetctrlPresetctrl1Spec> {
        Timer0RstW::new(self, 26)
    }
    #[doc = "Bit 27 - Timer 1 reset control."]
    #[inline(always)]
    pub fn timer1_rst(&mut self) -> Timer1RstW<'_, PresetctrlPresetctrl1Spec> {
        Timer1RstW::new(self, 27)
    }
}
#[doc = "Peripheral reset control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlPresetctrl1Spec;
impl crate::RegisterSpec for PresetctrlPresetctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrl_presetctrl1::R`](R) reader structure"]
impl crate::Readable for PresetctrlPresetctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`presetctrl_presetctrl1::W`](W) writer structure"]
impl crate::Writable for PresetctrlPresetctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0"]
impl crate::Resettable for PresetctrlPresetctrl1Spec {}

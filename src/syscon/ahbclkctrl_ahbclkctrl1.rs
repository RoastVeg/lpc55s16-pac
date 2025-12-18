#[doc = "Register `AHBCLKCTRL1` reader"]
pub type R = crate::R<AhbclkctrlAhbclkctrl1Spec>;
#[doc = "Register `AHBCLKCTRL1` writer"]
pub type W = crate::W<AhbclkctrlAhbclkctrl1Spec>;
#[doc = "Enables the clock for the MRT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Mrt> for bool {
    #[inline(always)]
    fn from(variant: Mrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT` reader - Enables the clock for the MRT."]
pub type MrtR = crate::BitReader<Mrt>;
impl MrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrt {
        match self.bits {
            false => Mrt::Disable,
            true => Mrt::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mrt::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mrt::Enable
    }
}
#[doc = "Field `MRT` writer - Enables the clock for the MRT."]
pub type MrtW<'a, REG> = crate::BitWriter<'a, REG, Mrt>;
impl<'a, REG> MrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt::Enable)
    }
}
#[doc = "Enables the clock for the OS Event Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostimer {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Ostimer> for bool {
    #[inline(always)]
    fn from(variant: Ostimer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTIMER` reader - Enables the clock for the OS Event Timer."]
pub type OstimerR = crate::BitReader<Ostimer>;
impl OstimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostimer {
        match self.bits {
            false => Ostimer::Disable,
            true => Ostimer::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ostimer::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ostimer::Enable
    }
}
#[doc = "Field `OSTIMER` writer - Enables the clock for the OS Event Timer."]
pub type OstimerW<'a, REG> = crate::BitWriter<'a, REG, Ostimer>;
impl<'a, REG> OstimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimer::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimer::Enable)
    }
}
#[doc = "Enables the clock for the SCT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Sct> for bool {
    #[inline(always)]
    fn from(variant: Sct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` reader - Enables the clock for the SCT."]
pub type SctR = crate::BitReader<Sct>;
impl SctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sct {
        match self.bits {
            false => Sct::Disable,
            true => Sct::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sct::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sct::Enable
    }
}
#[doc = "Field `SCT` writer - Enables the clock for the SCT."]
pub type SctW<'a, REG> = crate::BitWriter<'a, REG, Sct>;
impl<'a, REG> SctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::Enable)
    }
}
#[doc = "Enables the clock for the CAN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Can {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Can> for bool {
    #[inline(always)]
    fn from(variant: Can) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN` reader - Enables the clock for the CAN."]
pub type CanR = crate::BitReader<Can>;
impl CanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Can {
        match self.bits {
            false => Can::Disable,
            true => Can::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Can::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Can::Enable
    }
}
#[doc = "Field `CAN` writer - Enables the clock for the CAN."]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG, Can>;
impl<'a, REG> CanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Can::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Can::Enable)
    }
}
#[doc = "Enables the clock for the UTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Utick> for bool {
    #[inline(always)]
    fn from(variant: Utick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK` reader - Enables the clock for the UTICK."]
pub type UtickR = crate::BitReader<Utick>;
impl UtickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Utick {
        match self.bits {
            false => Utick::Disable,
            true => Utick::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Utick::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Utick::Enable
    }
}
#[doc = "Field `UTICK` writer - Enables the clock for the UTICK."]
pub type UtickW<'a, REG> = crate::BitWriter<'a, REG, Utick>;
impl<'a, REG> UtickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Utick::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Utick::Enable)
    }
}
#[doc = "Enables the clock for the FC0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc0> for bool {
    #[inline(always)]
    fn from(variant: Fc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0` reader - Enables the clock for the FC0."]
pub type Fc0R = crate::BitReader<Fc0>;
impl Fc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc0 {
        match self.bits {
            false => Fc0::Disable,
            true => Fc0::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc0::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc0::Enable
    }
}
#[doc = "Field `FC0` writer - Enables the clock for the FC0."]
pub type Fc0W<'a, REG> = crate::BitWriter<'a, REG, Fc0>;
impl<'a, REG> Fc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0::Enable)
    }
}
#[doc = "Enables the clock for the FC1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc1> for bool {
    #[inline(always)]
    fn from(variant: Fc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1` reader - Enables the clock for the FC1."]
pub type Fc1R = crate::BitReader<Fc1>;
impl Fc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc1 {
        match self.bits {
            false => Fc1::Disable,
            true => Fc1::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc1::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc1::Enable
    }
}
#[doc = "Field `FC1` writer - Enables the clock for the FC1."]
pub type Fc1W<'a, REG> = crate::BitWriter<'a, REG, Fc1>;
impl<'a, REG> Fc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1::Enable)
    }
}
#[doc = "Enables the clock for the FC2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc2> for bool {
    #[inline(always)]
    fn from(variant: Fc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2` reader - Enables the clock for the FC2."]
pub type Fc2R = crate::BitReader<Fc2>;
impl Fc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc2 {
        match self.bits {
            false => Fc2::Disable,
            true => Fc2::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc2::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc2::Enable
    }
}
#[doc = "Field `FC2` writer - Enables the clock for the FC2."]
pub type Fc2W<'a, REG> = crate::BitWriter<'a, REG, Fc2>;
impl<'a, REG> Fc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2::Enable)
    }
}
#[doc = "Enables the clock for the FC3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc3> for bool {
    #[inline(always)]
    fn from(variant: Fc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3` reader - Enables the clock for the FC3."]
pub type Fc3R = crate::BitReader<Fc3>;
impl Fc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc3 {
        match self.bits {
            false => Fc3::Disable,
            true => Fc3::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc3::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc3::Enable
    }
}
#[doc = "Field `FC3` writer - Enables the clock for the FC3."]
pub type Fc3W<'a, REG> = crate::BitWriter<'a, REG, Fc3>;
impl<'a, REG> Fc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3::Enable)
    }
}
#[doc = "Enables the clock for the FC4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc4> for bool {
    #[inline(always)]
    fn from(variant: Fc4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4` reader - Enables the clock for the FC4."]
pub type Fc4R = crate::BitReader<Fc4>;
impl Fc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc4 {
        match self.bits {
            false => Fc4::Disable,
            true => Fc4::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc4::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc4::Enable
    }
}
#[doc = "Field `FC4` writer - Enables the clock for the FC4."]
pub type Fc4W<'a, REG> = crate::BitWriter<'a, REG, Fc4>;
impl<'a, REG> Fc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4::Enable)
    }
}
#[doc = "Enables the clock for the FC5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc5> for bool {
    #[inline(always)]
    fn from(variant: Fc5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5` reader - Enables the clock for the FC5."]
pub type Fc5R = crate::BitReader<Fc5>;
impl Fc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc5 {
        match self.bits {
            false => Fc5::Disable,
            true => Fc5::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc5::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc5::Enable
    }
}
#[doc = "Field `FC5` writer - Enables the clock for the FC5."]
pub type Fc5W<'a, REG> = crate::BitWriter<'a, REG, Fc5>;
impl<'a, REG> Fc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5::Enable)
    }
}
#[doc = "Enables the clock for the FC6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc6> for bool {
    #[inline(always)]
    fn from(variant: Fc6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6` reader - Enables the clock for the FC6."]
pub type Fc6R = crate::BitReader<Fc6>;
impl Fc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc6 {
        match self.bits {
            false => Fc6::Disable,
            true => Fc6::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc6::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc6::Enable
    }
}
#[doc = "Field `FC6` writer - Enables the clock for the FC6."]
pub type Fc6W<'a, REG> = crate::BitWriter<'a, REG, Fc6>;
impl<'a, REG> Fc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6::Enable)
    }
}
#[doc = "Enables the clock for the FC7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fc7> for bool {
    #[inline(always)]
    fn from(variant: Fc7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7` reader - Enables the clock for the FC7."]
pub type Fc7R = crate::BitReader<Fc7>;
impl Fc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc7 {
        match self.bits {
            false => Fc7::Disable,
            true => Fc7::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fc7::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fc7::Enable
    }
}
#[doc = "Field `FC7` writer - Enables the clock for the FC7."]
pub type Fc7W<'a, REG> = crate::BitWriter<'a, REG, Fc7>;
impl<'a, REG> Fc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7::Enable)
    }
}
#[doc = "Enables the clock for the Timer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer2 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Timer2> for bool {
    #[inline(always)]
    fn from(variant: Timer2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2` reader - Enables the clock for the Timer 2."]
pub type Timer2R = crate::BitReader<Timer2>;
impl Timer2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer2 {
        match self.bits {
            false => Timer2::Disable,
            true => Timer2::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer2::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer2::Enable
    }
}
#[doc = "Field `TIMER2` writer - Enables the clock for the Timer 2."]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG, Timer2>;
impl<'a, REG> Timer2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2::Enable)
    }
}
#[doc = "Enables the clock for the USB0-FS device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0Dev {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb0Dev> for bool {
    #[inline(always)]
    fn from(variant: Usb0Dev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_DEV` reader - Enables the clock for the USB0-FS device."]
pub type Usb0DevR = crate::BitReader<Usb0Dev>;
impl Usb0DevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0Dev {
        match self.bits {
            false => Usb0Dev::Disable,
            true => Usb0Dev::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb0Dev::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb0Dev::Enable
    }
}
#[doc = "Field `USB0_DEV` writer - Enables the clock for the USB0-FS device."]
pub type Usb0DevW<'a, REG> = crate::BitWriter<'a, REG, Usb0Dev>;
impl<'a, REG> Usb0DevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Dev::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Dev::Enable)
    }
}
#[doc = "Enables the clock for the Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Timer0> for bool {
    #[inline(always)]
    fn from(variant: Timer0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0` reader - Enables the clock for the Timer 0."]
pub type Timer0R = crate::BitReader<Timer0>;
impl Timer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0 {
        match self.bits {
            false => Timer0::Disable,
            true => Timer0::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer0::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer0::Enable
    }
}
#[doc = "Field `TIMER0` writer - Enables the clock for the Timer 0."]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG, Timer0>;
impl<'a, REG> Timer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0::Enable)
    }
}
#[doc = "Enables the clock for the Timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Timer1> for bool {
    #[inline(always)]
    fn from(variant: Timer1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1` reader - Enables the clock for the Timer 1."]
pub type Timer1R = crate::BitReader<Timer1>;
impl Timer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1 {
        match self.bits {
            false => Timer1::Disable,
            true => Timer1::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer1::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer1::Enable
    }
}
#[doc = "Field `TIMER1` writer - Enables the clock for the Timer 1."]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG, Timer1>;
impl<'a, REG> Timer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&self) -> MrtR {
        MrtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub fn ostimer(&self) -> OstimerR {
        OstimerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the CAN."]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    pub fn utick(&self) -> UtickR {
        UtickR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&self) -> Fc0R {
        Fc0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&self) -> Fc1R {
        Fc1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&self) -> Fc2R {
        Fc2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&self) -> Fc3R {
        Fc3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&self) -> Fc4R {
        Fc4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&self) -> Fc5R {
        Fc5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&self) -> Fc6R {
        Fc6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&self) -> Fc7R {
        Fc7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0-FS device."]
    #[inline(always)]
    pub fn usb0_dev(&self) -> Usb0DevR {
        Usb0DevR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MrtW<'_, AhbclkctrlAhbclkctrl1Spec> {
        MrtW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub fn ostimer(&mut self) -> OstimerW<'_, AhbclkctrlAhbclkctrl1Spec> {
        OstimerW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT."]
    #[inline(always)]
    pub fn sct(&mut self) -> SctW<'_, AhbclkctrlAhbclkctrl1Spec> {
        SctW::new(self, 2)
    }
    #[doc = "Bit 7 - Enables the clock for the CAN."]
    #[inline(always)]
    pub fn can(&mut self) -> CanW<'_, AhbclkctrlAhbclkctrl1Spec> {
        CanW::new(self, 7)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK."]
    #[inline(always)]
    pub fn utick(&mut self) -> UtickW<'_, AhbclkctrlAhbclkctrl1Spec> {
        UtickW::new(self, 10)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&mut self) -> Fc0W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&mut self) -> Fc1W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc1W::new(self, 12)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&mut self) -> Fc2W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc2W::new(self, 13)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&mut self) -> Fc3W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc3W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&mut self) -> Fc4W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc4W::new(self, 15)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&mut self) -> Fc5W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc5W::new(self, 16)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&mut self) -> Fc6W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc6W::new(self, 17)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&mut self) -> Fc7W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Fc7W::new(self, 18)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&mut self) -> Timer2W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Timer2W::new(self, 22)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0-FS device."]
    #[inline(always)]
    pub fn usb0_dev(&mut self) -> Usb0DevW<'_, AhbclkctrlAhbclkctrl1Spec> {
        Usb0DevW::new(self, 25)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&mut self) -> Timer0W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Timer0W::new(self, 26)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&mut self) -> Timer1W<'_, AhbclkctrlAhbclkctrl1Spec> {
        Timer1W::new(self, 27)
    }
}
#[doc = "AHB Clock control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbclkctrlAhbclkctrl1Spec;
impl crate::RegisterSpec for AhbclkctrlAhbclkctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbclkctrl_ahbclkctrl1::R`](R) reader structure"]
impl crate::Readable for AhbclkctrlAhbclkctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbclkctrl_ahbclkctrl1::W`](W) writer structure"]
impl crate::Writable for AhbclkctrlAhbclkctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCLKCTRL1 to value 0"]
impl crate::Resettable for AhbclkctrlAhbclkctrl1Spec {}

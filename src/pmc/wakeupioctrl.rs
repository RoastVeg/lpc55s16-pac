#[doc = "Register `WAKEUPIOCTRL` reader"]
pub type R = crate::R<WakeupioctrlSpec>;
#[doc = "Register `WAKEUPIOCTRL` writer"]
pub type W = crate::W<WakeupioctrlSpec>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup0 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup0> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` reader - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Risingedgewakeup0R = crate::BitReader<Risingedgewakeup0>;
impl Risingedgewakeup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup0 {
        match self.bits {
            false => Risingedgewakeup0::Disable,
            true => Risingedgewakeup0::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup0::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup0::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` writer - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Risingedgewakeup0W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup0>;
impl<'a, REG> Risingedgewakeup0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup0::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup0::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup0 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup0> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` reader - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup0R = crate::BitReader<Fallingedgewakeup0>;
impl Fallingedgewakeup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup0 {
        match self.bits {
            false => Fallingedgewakeup0::Disable,
            true => Fallingedgewakeup0::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup0::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup0::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` writer - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup0W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup0>;
impl<'a, REG> Fallingedgewakeup0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup0::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup0::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup1 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup1> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` reader - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Risingedgewakeup1R = crate::BitReader<Risingedgewakeup1>;
impl Risingedgewakeup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup1 {
        match self.bits {
            false => Risingedgewakeup1::Disable,
            true => Risingedgewakeup1::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup1::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup1::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` writer - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Risingedgewakeup1W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup1>;
impl<'a, REG> Risingedgewakeup1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup1::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup1::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup1 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup1> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` reader - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup1R = crate::BitReader<Fallingedgewakeup1>;
impl Fallingedgewakeup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup1 {
        match self.bits {
            false => Fallingedgewakeup1::Disable,
            true => Fallingedgewakeup1::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup1::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup1::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` writer - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup1W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup1>;
impl<'a, REG> Fallingedgewakeup1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup1::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup1::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup2 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup2> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` reader - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Risingedgewakeup2R = crate::BitReader<Risingedgewakeup2>;
impl Risingedgewakeup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup2 {
        match self.bits {
            false => Risingedgewakeup2::Disable,
            true => Risingedgewakeup2::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup2::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup2::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` writer - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Risingedgewakeup2W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup2>;
impl<'a, REG> Risingedgewakeup2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup2::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup2::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup2 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup2> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` reader - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup2R = crate::BitReader<Fallingedgewakeup2>;
impl Fallingedgewakeup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup2 {
        match self.bits {
            false => Fallingedgewakeup2::Disable,
            true => Fallingedgewakeup2::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup2::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup2::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` writer - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup2W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup2>;
impl<'a, REG> Fallingedgewakeup2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup2::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup2::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup3 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup3> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` reader - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Risingedgewakeup3R = crate::BitReader<Risingedgewakeup3>;
impl Risingedgewakeup3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup3 {
        match self.bits {
            false => Risingedgewakeup3::Disable,
            true => Risingedgewakeup3::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup3::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup3::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` writer - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Risingedgewakeup3W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup3>;
impl<'a, REG> Risingedgewakeup3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup3::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup3::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup3 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup3> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` reader - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup3R = crate::BitReader<Fallingedgewakeup3>;
impl Fallingedgewakeup3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup3 {
        match self.bits {
            false => Fallingedgewakeup3::Disable,
            true => Fallingedgewakeup3::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup3::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup3::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` writer - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup3W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup3>;
impl<'a, REG> Fallingedgewakeup3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup3::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup3::Enable)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modewakeupiopad0 {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PullDown = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PullUp = 2,
    #[doc = "3: Repeater. Repeater mode."]
    Repeater = 3,
}
impl From<Modewakeupiopad0> for u8 {
    #[inline(always)]
    fn from(variant: Modewakeupiopad0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modewakeupiopad0 {
    type Ux = u8;
}
impl crate::IsEnum for Modewakeupiopad0 {}
#[doc = "Field `MODEWAKEUPIOPAD0` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad0R = crate::FieldReader<Modewakeupiopad0>;
impl Modewakeupiopad0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modewakeupiopad0 {
        match self.bits {
            0 => Modewakeupiopad0::Inactive,
            1 => Modewakeupiopad0::PullDown,
            2 => Modewakeupiopad0::PullUp,
            3 => Modewakeupiopad0::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Modewakeupiopad0::Inactive
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Modewakeupiopad0::PullDown
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Modewakeupiopad0::PullUp
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Modewakeupiopad0::Repeater
    }
}
#[doc = "Field `MODEWAKEUPIOPAD0` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Modewakeupiopad0, crate::Safe>;
impl<'a, REG> Modewakeupiopad0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad0::Inactive)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad0::PullDown)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad0::PullUp)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad0::Repeater)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modewakeupiopad1 {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PullDown = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PullUp = 2,
    #[doc = "3: Repeater. Repeater mode."]
    Repeater = 3,
}
impl From<Modewakeupiopad1> for u8 {
    #[inline(always)]
    fn from(variant: Modewakeupiopad1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modewakeupiopad1 {
    type Ux = u8;
}
impl crate::IsEnum for Modewakeupiopad1 {}
#[doc = "Field `MODEWAKEUPIOPAD1` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad1R = crate::FieldReader<Modewakeupiopad1>;
impl Modewakeupiopad1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modewakeupiopad1 {
        match self.bits {
            0 => Modewakeupiopad1::Inactive,
            1 => Modewakeupiopad1::PullDown,
            2 => Modewakeupiopad1::PullUp,
            3 => Modewakeupiopad1::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Modewakeupiopad1::Inactive
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Modewakeupiopad1::PullDown
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Modewakeupiopad1::PullUp
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Modewakeupiopad1::Repeater
    }
}
#[doc = "Field `MODEWAKEUPIOPAD1` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Modewakeupiopad1, crate::Safe>;
impl<'a, REG> Modewakeupiopad1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad1::Inactive)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad1::PullDown)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad1::PullUp)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad1::Repeater)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modewakeupiopad2 {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PullDown = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PullUp = 2,
    #[doc = "3: Repeater. Repeater mode."]
    Repeater = 3,
}
impl From<Modewakeupiopad2> for u8 {
    #[inline(always)]
    fn from(variant: Modewakeupiopad2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modewakeupiopad2 {
    type Ux = u8;
}
impl crate::IsEnum for Modewakeupiopad2 {}
#[doc = "Field `MODEWAKEUPIOPAD2` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad2R = crate::FieldReader<Modewakeupiopad2>;
impl Modewakeupiopad2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modewakeupiopad2 {
        match self.bits {
            0 => Modewakeupiopad2::Inactive,
            1 => Modewakeupiopad2::PullDown,
            2 => Modewakeupiopad2::PullUp,
            3 => Modewakeupiopad2::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Modewakeupiopad2::Inactive
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Modewakeupiopad2::PullDown
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Modewakeupiopad2::PullUp
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Modewakeupiopad2::Repeater
    }
}
#[doc = "Field `MODEWAKEUPIOPAD2` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Modewakeupiopad2, crate::Safe>;
impl<'a, REG> Modewakeupiopad2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad2::Inactive)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad2::PullDown)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad2::PullUp)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad2::Repeater)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modewakeupiopad3 {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PullDown = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PullUp = 2,
    #[doc = "3: Repeater. Repeater mode."]
    Repeater = 3,
}
impl From<Modewakeupiopad3> for u8 {
    #[inline(always)]
    fn from(variant: Modewakeupiopad3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modewakeupiopad3 {
    type Ux = u8;
}
impl crate::IsEnum for Modewakeupiopad3 {}
#[doc = "Field `MODEWAKEUPIOPAD3` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad3R = crate::FieldReader<Modewakeupiopad3>;
impl Modewakeupiopad3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modewakeupiopad3 {
        match self.bits {
            0 => Modewakeupiopad3::Inactive,
            1 => Modewakeupiopad3::PullDown,
            2 => Modewakeupiopad3::PullUp,
            3 => Modewakeupiopad3::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Modewakeupiopad3::Inactive
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Modewakeupiopad3::PullDown
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Modewakeupiopad3::PullUp
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Modewakeupiopad3::Repeater
    }
}
#[doc = "Field `MODEWAKEUPIOPAD3` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type Modewakeupiopad3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Modewakeupiopad3, crate::Safe>;
impl<'a, REG> Modewakeupiopad3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad3::Inactive)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad3::PullDown)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad3::PullUp)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Modewakeupiopad3::Repeater)
    }
}
#[doc = "Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupioEnableCtrl {
    #[doc = "0: WAKEUP IO PAD mode control comes from IOCON."]
    Disable = 0,
    #[doc = "1: WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    Enable = 1,
}
impl From<WakeupioEnableCtrl> for bool {
    #[inline(always)]
    fn from(variant: WakeupioEnableCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUPIO_ENABLE_CTRL` reader - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
pub type WakeupioEnableCtrlR = crate::BitReader<WakeupioEnableCtrl>;
impl WakeupioEnableCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupioEnableCtrl {
        match self.bits {
            false => WakeupioEnableCtrl::Disable,
            true => WakeupioEnableCtrl::Enable,
        }
    }
    #[doc = "WAKEUP IO PAD mode control comes from IOCON."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WakeupioEnableCtrl::Disable
    }
    #[doc = "WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WakeupioEnableCtrl::Enable
    }
}
#[doc = "Field `WAKEUPIO_ENABLE_CTRL` writer - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
pub type WakeupioEnableCtrlW<'a, REG> = crate::BitWriter<'a, REG, WakeupioEnableCtrl>;
impl<'a, REG> WakeupioEnableCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WAKEUP IO PAD mode control comes from IOCON."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupioEnableCtrl::Disable)
    }
    #[doc = "WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupioEnableCtrl::Enable)
    }
}
#[doc = "WAKEUP IO event detector reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupioRstn {
    #[doc = "0: Bloc is reset."]
    Asserted = 0,
    #[doc = "1: Bloc is not reset."]
    Released = 1,
}
impl From<WakeupioRstn> for bool {
    #[inline(always)]
    fn from(variant: WakeupioRstn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUPIO_RSTN` reader - WAKEUP IO event detector reset control."]
pub type WakeupioRstnR = crate::BitReader<WakeupioRstn>;
impl WakeupioRstnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupioRstn {
        match self.bits {
            false => WakeupioRstn::Asserted,
            true => WakeupioRstn::Released,
        }
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WakeupioRstn::Asserted
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WakeupioRstn::Released
    }
}
#[doc = "Field `WAKEUPIO_RSTN` writer - WAKEUP IO event detector reset control."]
pub type WakeupioRstnW<'a, REG> = crate::BitWriter<'a, REG, WakeupioRstn>;
impl<'a, REG> WakeupioRstnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupioRstn::Asserted)
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupioRstn::Released)
    }
}
impl R {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&self) -> Risingedgewakeup0R {
        Risingedgewakeup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&self) -> Fallingedgewakeup0R {
        Fallingedgewakeup0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&self) -> Risingedgewakeup1R {
        Risingedgewakeup1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&self) -> Fallingedgewakeup1R {
        Fallingedgewakeup1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&self) -> Risingedgewakeup2R {
        Risingedgewakeup2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&self) -> Fallingedgewakeup2R {
        Fallingedgewakeup2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&self) -> Risingedgewakeup3R {
        Risingedgewakeup3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&self) -> Fallingedgewakeup3R {
        Fallingedgewakeup3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad0(&self) -> Modewakeupiopad0R {
        Modewakeupiopad0R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad1(&self) -> Modewakeupiopad1R {
        Modewakeupiopad1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad2(&self) -> Modewakeupiopad2R {
        Modewakeupiopad2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad3(&self) -> Modewakeupiopad3R {
        Modewakeupiopad3R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn wakeupio_enable_ctrl(&self) -> WakeupioEnableCtrlR {
        WakeupioEnableCtrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakeupio_rstn(&self) -> WakeupioRstnR {
        WakeupioRstnR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&mut self) -> Risingedgewakeup0W<'_, WakeupioctrlSpec> {
        Risingedgewakeup0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&mut self) -> Fallingedgewakeup0W<'_, WakeupioctrlSpec> {
        Fallingedgewakeup0W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&mut self) -> Risingedgewakeup1W<'_, WakeupioctrlSpec> {
        Risingedgewakeup1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&mut self) -> Fallingedgewakeup1W<'_, WakeupioctrlSpec> {
        Fallingedgewakeup1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&mut self) -> Risingedgewakeup2W<'_, WakeupioctrlSpec> {
        Risingedgewakeup2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&mut self) -> Fallingedgewakeup2W<'_, WakeupioctrlSpec> {
        Fallingedgewakeup2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&mut self) -> Risingedgewakeup3W<'_, WakeupioctrlSpec> {
        Risingedgewakeup3W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&mut self) -> Fallingedgewakeup3W<'_, WakeupioctrlSpec> {
        Fallingedgewakeup3W::new(self, 7)
    }
    #[doc = "Bits 12:13 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad0(&mut self) -> Modewakeupiopad0W<'_, WakeupioctrlSpec> {
        Modewakeupiopad0W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad1(&mut self) -> Modewakeupiopad1W<'_, WakeupioctrlSpec> {
        Modewakeupiopad1W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad2(&mut self) -> Modewakeupiopad2W<'_, WakeupioctrlSpec> {
        Modewakeupiopad2W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad3(&mut self) -> Modewakeupiopad3W<'_, WakeupioctrlSpec> {
        Modewakeupiopad3W::new(self, 18)
    }
    #[doc = "Bit 20 - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn wakeupio_enable_ctrl(&mut self) -> WakeupioEnableCtrlW<'_, WakeupioctrlSpec> {
        WakeupioEnableCtrlW::new(self, 20)
    }
    #[doc = "Bit 21 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakeupio_rstn(&mut self) -> WakeupioRstnW<'_, WakeupioctrlSpec> {
        WakeupioRstnW::new(self, 21)
    }
}
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeupioctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeupioctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupioctrlSpec;
impl crate::RegisterSpec for WakeupioctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeupioctrl::R`](R) reader structure"]
impl crate::Readable for WakeupioctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeupioctrl::W`](W) writer structure"]
impl crate::Writable for WakeupioctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUPIOCTRL to value 0x0020_0000"]
impl crate::Resettable for WakeupioctrlSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}

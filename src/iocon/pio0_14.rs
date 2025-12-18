#[doc = "Register `PIO0_14` reader"]
pub type R = crate::R<Pio0_14Spec>;
#[doc = "Register `PIO0_14` writer"]
pub type W = crate::W<Pio0_14Spec>;
#[doc = "Selects pin function.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: Alternative connection 0."]
    Alt0 = 0,
    #[doc = "1: Alternative connection 1."]
    Alt1 = 1,
    #[doc = "2: Alternative connection 2."]
    Alt2 = 2,
    #[doc = "3: Alternative connection 3."]
    Alt3 = 3,
    #[doc = "4: Alternative connection 4."]
    Alt4 = 4,
    #[doc = "5: Alternative connection 5."]
    Alt5 = 5,
    #[doc = "6: Alternative connection 6."]
    Alt6 = 6,
    #[doc = "7: Alternative connection 7."]
    Alt7 = 7,
}
impl From<Func> for u8 {
    #[inline(always)]
    fn from(variant: Func) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Func {
    type Ux = u8;
}
impl crate::IsEnum for Func {}
#[doc = "Field `FUNC` reader - Selects pin function."]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::Alt0),
            1 => Some(Func::Alt1),
            2 => Some(Func::Alt2),
            3 => Some(Func::Alt3),
            4 => Some(Func::Alt4),
            5 => Some(Func::Alt5),
            6 => Some(Func::Alt6),
            7 => Some(Func::Alt7),
            _ => None,
        }
    }
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == Func::Alt0
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == Func::Alt1
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == Func::Alt2
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == Func::Alt3
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == Func::Alt4
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == Func::Alt5
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == Func::Alt6
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == Func::Alt7
    }
}
#[doc = "Field `FUNC` writer - Selects pin function."]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 4, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt0)
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt1)
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt2)
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt3)
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt4)
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt5)
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt6)
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Alt7)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PullDown = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PullUp = 2,
    #[doc = "3: Repeater. Repeater mode."]
    Repeater = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Inactive,
            1 => Mode::PullDown,
            2 => Mode::PullUp,
            3 => Mode::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Mode::Inactive
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Mode::PullDown
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Mode::PullUp
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Mode::Repeater
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inactive)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullDown)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullUp)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Repeater)
    }
}
#[doc = "Driver slew rate.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slew {
    #[doc = "0: Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    Standard = 0,
    #[doc = "1: Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    Fast = 1,
}
impl From<Slew> for bool {
    #[inline(always)]
    fn from(variant: Slew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEW` reader - Driver slew rate."]
pub type SlewR = crate::BitReader<Slew>;
impl SlewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slew {
        match self.bits {
            false => Slew::Standard,
            true => Slew::Fast,
        }
    }
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Slew::Standard
    }
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Slew::Fast
    }
}
#[doc = "Field `SLEW` writer - Driver slew rate."]
pub type SlewW<'a, REG> = crate::BitWriter<'a, REG, Slew>;
impl<'a, REG> SlewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Standard)
    }
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Fast)
    }
}
#[doc = "Input polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invert {
    #[doc = "0: Disabled. Input function is not inverted."]
    Disabled = 0,
    #[doc = "1: Enabled. Input is function inverted."]
    Enabled = 1,
}
impl From<Invert> for bool {
    #[inline(always)]
    fn from(variant: Invert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVERT` reader - Input polarity."]
pub type InvertR = crate::BitReader<Invert>;
impl InvertR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invert {
        match self.bits {
            false => Invert::Disabled,
            true => Invert::Enabled,
        }
    }
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Invert::Disabled
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Invert::Enabled
    }
}
#[doc = "Field `INVERT` writer - Input polarity."]
pub type InvertW<'a, REG> = crate::BitWriter<'a, REG, Invert>;
impl<'a, REG> InvertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Invert::Disabled)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Invert::Enabled)
    }
}
#[doc = "Select Digital mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digimode {
    #[doc = "0: Disable digital mode. Digital input set to 0."]
    Analog = 0,
    #[doc = "1: Enable Digital mode. Digital input is enabled."]
    Digital = 1,
}
impl From<Digimode> for bool {
    #[inline(always)]
    fn from(variant: Digimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGIMODE` reader - Select Digital mode."]
pub type DigimodeR = crate::BitReader<Digimode>;
impl DigimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digimode {
        match self.bits {
            false => Digimode::Analog,
            true => Digimode::Digital,
        }
    }
    #[doc = "Disable digital mode. Digital input set to 0."]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == Digimode::Analog
    }
    #[doc = "Enable Digital mode. Digital input is enabled."]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == Digimode::Digital
    }
}
#[doc = "Field `DIGIMODE` writer - Select Digital mode."]
pub type DigimodeW<'a, REG> = crate::BitWriter<'a, REG, Digimode>;
impl<'a, REG> DigimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable digital mode. Digital input set to 0."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(Digimode::Analog)
    }
    #[doc = "Enable Digital mode. Digital input is enabled."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(Digimode::Digital)
    }
}
#[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Od {
    #[doc = "0: Normal. Normal push-pull output"]
    Normal = 0,
    #[doc = "1: Open-drain. Simulated open-drain output (high drive disabled)."]
    OpenDrain = 1,
}
impl From<Od> for bool {
    #[inline(always)]
    fn from(variant: Od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
pub type OdR = crate::BitReader<Od>;
impl OdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Od {
        match self.bits {
            false => Od::Normal,
            true => Od::OpenDrain,
        }
    }
    #[doc = "Normal. Normal push-pull output"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Od::Normal
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Od::OpenDrain
    }
}
#[doc = "Field `OD` writer - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
pub type OdW<'a, REG> = crate::BitWriter<'a, REG, Od>;
impl<'a, REG> OdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. Normal push-pull output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Normal)
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Od::OpenDrain)
    }
}
#[doc = "Supply Selection bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssel {
    #[doc = "0: 3V3 Signaling in I2C Mode."]
    Sel3v3 = 0,
    #[doc = "1: 1V8 Signaling in I2C Mode."]
    Sel1v8 = 1,
}
impl From<Ssel> for bool {
    #[inline(always)]
    fn from(variant: Ssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEL` reader - Supply Selection bit."]
pub type SselR = crate::BitReader<Ssel>;
impl SselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssel {
        match self.bits {
            false => Ssel::Sel3v3,
            true => Ssel::Sel1v8,
        }
    }
    #[doc = "3V3 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn is_sel3v3(&self) -> bool {
        *self == Ssel::Sel3v3
    }
    #[doc = "1V8 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn is_sel1v8(&self) -> bool {
        *self == Ssel::Sel1v8
    }
}
#[doc = "Field `SSEL` writer - Supply Selection bit."]
pub type SselW<'a, REG> = crate::BitWriter<'a, REG, Ssel>;
impl<'a, REG> SselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3V3 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn sel3v3(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::Sel3v3)
    }
    #[doc = "1V8 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn sel1v8(self) -> &'a mut crate::W<REG> {
        self.variant(Ssel::Sel1v8)
    }
}
#[doc = "Controls input glitch filter.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filteroff {
    #[doc = "0: Filter enabled."]
    Enabled = 0,
    #[doc = "1: Filter disabled."]
    Disabled = 1,
}
impl From<Filteroff> for bool {
    #[inline(always)]
    fn from(variant: Filteroff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTEROFF` reader - Controls input glitch filter."]
pub type FilteroffR = crate::BitReader<Filteroff>;
impl FilteroffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteroff {
        match self.bits {
            false => Filteroff::Enabled,
            true => Filteroff::Disabled,
        }
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Filteroff::Enabled
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Filteroff::Disabled
    }
}
#[doc = "Field `FILTEROFF` writer - Controls input glitch filter."]
pub type FilteroffW<'a, REG> = crate::BitWriter<'a, REG, Filteroff>;
impl<'a, REG> FilteroffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filteroff::Enabled)
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filteroff::Disabled)
    }
}
#[doc = "Pull-up current source enable in I2C mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecs {
    #[doc = "0: Disabled. IO is in open drain cell."]
    Disabled = 0,
    #[doc = "1: Enabled. Pull resistor is conencted."]
    Enabled = 1,
}
impl From<Ecs> for bool {
    #[inline(always)]
    fn from(variant: Ecs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECS` reader - Pull-up current source enable in I2C mode."]
pub type EcsR = crate::BitReader<Ecs>;
impl EcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecs {
        match self.bits {
            false => Ecs::Disabled,
            true => Ecs::Enabled,
        }
    }
    #[doc = "Disabled. IO is in open drain cell."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ecs::Disabled
    }
    #[doc = "Enabled. Pull resistor is conencted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ecs::Enabled
    }
}
#[doc = "Field `ECS` writer - Pull-up current source enable in I2C mode."]
pub type EcsW<'a, REG> = crate::BitWriter<'a, REG, Ecs>;
impl<'a, REG> EcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. IO is in open drain cell."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ecs::Disabled)
    }
    #[doc = "Enabled. Pull resistor is conencted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ecs::Enabled)
    }
}
#[doc = "Switch between GPIO mode and I2C mode.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Egp {
    #[doc = "0: I2C mode."]
    I2cMode = 0,
    #[doc = "1: GPIO mode."]
    GpioMode = 1,
}
impl From<Egp> for bool {
    #[inline(always)]
    fn from(variant: Egp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGP` reader - Switch between GPIO mode and I2C mode."]
pub type EgpR = crate::BitReader<Egp>;
impl EgpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Egp {
        match self.bits {
            false => Egp::I2cMode,
            true => Egp::GpioMode,
        }
    }
    #[doc = "I2C mode."]
    #[inline(always)]
    pub fn is_i2c_mode(&self) -> bool {
        *self == Egp::I2cMode
    }
    #[doc = "GPIO mode."]
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        *self == Egp::GpioMode
    }
}
#[doc = "Field `EGP` writer - Switch between GPIO mode and I2C mode."]
pub type EgpW<'a, REG> = crate::BitWriter<'a, REG, Egp>;
impl<'a, REG> EgpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C mode."]
    #[inline(always)]
    pub fn i2c_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Egp::I2cMode)
    }
    #[doc = "GPIO mode."]
    #[inline(always)]
    pub fn gpio_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Egp::GpioMode)
    }
}
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cfilter {
    #[doc = "0: I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FastMode = 0,
    #[doc = "1: I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    StandardMode = 1,
}
impl From<I2cfilter> for bool {
    #[inline(always)]
    fn from(variant: I2cfilter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CFILTER` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
pub type I2cfilterR = crate::BitReader<I2cfilter>;
impl I2cfilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cfilter {
        match self.bits {
            false => I2cfilter::FastMode,
            true => I2cfilter::StandardMode,
        }
    }
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    #[inline(always)]
    pub fn is_fast_mode(&self) -> bool {
        *self == I2cfilter::FastMode
    }
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        *self == I2cfilter::StandardMode
    }
}
#[doc = "Field `I2CFILTER` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
pub type I2cfilterW<'a, REG> = crate::BitWriter<'a, REG, I2cfilter>;
impl<'a, REG> I2cfilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    #[inline(always)]
    pub fn fast_mode(self) -> &'a mut crate::W<REG> {
        self.variant(I2cfilter::FastMode)
    }
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut crate::W<REG> {
        self.variant(I2cfilter::StandardMode)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&self) -> SlewR {
        SlewR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&self) -> InvertR {
        InvertR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    pub fn digimode(&self) -> DigimodeR {
        DigimodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Supply Selection bit."]
    #[inline(always)]
    pub fn ssel(&self) -> SselR {
        SselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&self) -> FilteroffR {
        FilteroffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub fn ecs(&self) -> EcsR {
        EcsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub fn egp(&self) -> EgpR {
        EgpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub fn i2cfilter(&self) -> I2cfilterR {
        I2cfilterR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_14")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, Pio0_14Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Pio0_14Spec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&mut self) -> SlewW<'_, Pio0_14Spec> {
        SlewW::new(self, 6)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&mut self) -> InvertW<'_, Pio0_14Spec> {
        InvertW::new(self, 7)
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    pub fn digimode(&mut self) -> DigimodeW<'_, Pio0_14Spec> {
        DigimodeW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, Pio0_14Spec> {
        OdW::new(self, 9)
    }
    #[doc = "Bit 11 - Supply Selection bit."]
    #[inline(always)]
    pub fn ssel(&mut self) -> SselW<'_, Pio0_14Spec> {
        SselW::new(self, 11)
    }
    #[doc = "Bit 12 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&mut self) -> FilteroffW<'_, Pio0_14Spec> {
        FilteroffW::new(self, 12)
    }
    #[doc = "Bit 13 - Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub fn ecs(&mut self) -> EcsW<'_, Pio0_14Spec> {
        EcsW::new(self, 13)
    }
    #[doc = "Bit 14 - Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub fn egp(&mut self) -> EgpW<'_, Pio0_14Spec> {
        EgpW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> I2cfilterW<'_, Pio0_14Spec> {
        I2cfilterW::new(self, 15)
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14\n\nYou can [`read`](crate::Reg::read) this register and get [`pio0_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pio0_14Spec;
impl crate::RegisterSpec for Pio0_14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio0_14::R`](R) reader structure"]
impl crate::Readable for Pio0_14Spec {}
#[doc = "`write(|w| ..)` method takes [`pio0_14::W`](W) writer structure"]
impl crate::Writable for Pio0_14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIO0_14 to value 0x5000"]
impl crate::Resettable for Pio0_14Spec {
    const RESET_VALUE: u32 = 0x5000;
}

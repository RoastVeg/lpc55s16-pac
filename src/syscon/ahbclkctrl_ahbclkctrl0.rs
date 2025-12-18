#[doc = "Register `AHBCLKCTRL0` reader"]
pub type R = crate::R<AhbclkctrlAhbclkctrl0Spec>;
#[doc = "Register `AHBCLKCTRL0` writer"]
pub type W = crate::W<AhbclkctrlAhbclkctrl0Spec>;
#[doc = "Enables the clock for the ROM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rom {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Rom> for bool {
    #[inline(always)]
    fn from(variant: Rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Enables the clock for the ROM."]
pub type RomR = crate::BitReader<Rom>;
impl RomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rom {
        match self.bits {
            false => Rom::Disable,
            true => Rom::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rom::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rom::Enable
    }
}
#[doc = "Field `ROM` writer - Enables the clock for the ROM."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG, Rom>;
impl<'a, REG> RomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Enable)
    }
}
#[doc = "Enables the clock for the SRAM Controller 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramCtrl1 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<SramCtrl1> for bool {
    #[inline(always)]
    fn from(variant: SramCtrl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL1` reader - Enables the clock for the SRAM Controller 1."]
pub type SramCtrl1R = crate::BitReader<SramCtrl1>;
impl SramCtrl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramCtrl1 {
        match self.bits {
            false => SramCtrl1::Disable,
            true => SramCtrl1::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SramCtrl1::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SramCtrl1::Enable
    }
}
#[doc = "Field `SRAM_CTRL1` writer - Enables the clock for the SRAM Controller 1."]
pub type SramCtrl1W<'a, REG> = crate::BitWriter<'a, REG, SramCtrl1>;
impl<'a, REG> SramCtrl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl1::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl1::Enable)
    }
}
#[doc = "Enables the clock for the SRAM Controller 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramCtrl2 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<SramCtrl2> for bool {
    #[inline(always)]
    fn from(variant: SramCtrl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL2` reader - Enables the clock for the SRAM Controller 2."]
pub type SramCtrl2R = crate::BitReader<SramCtrl2>;
impl SramCtrl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramCtrl2 {
        match self.bits {
            false => SramCtrl2::Disable,
            true => SramCtrl2::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SramCtrl2::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SramCtrl2::Enable
    }
}
#[doc = "Field `SRAM_CTRL2` writer - Enables the clock for the SRAM Controller 2."]
pub type SramCtrl2W<'a, REG> = crate::BitWriter<'a, REG, SramCtrl2>;
impl<'a, REG> SramCtrl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl2::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl2::Enable)
    }
}
#[doc = "Enables the clock for the Flash controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flash {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Flash> for bool {
    #[inline(always)]
    fn from(variant: Flash) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH` reader - Enables the clock for the Flash controller."]
pub type FlashR = crate::BitReader<Flash>;
impl FlashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flash {
        match self.bits {
            false => Flash::Disable,
            true => Flash::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Flash::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Flash::Enable
    }
}
#[doc = "Field `FLASH` writer - Enables the clock for the Flash controller."]
pub type FlashW<'a, REG> = crate::BitWriter<'a, REG, Flash>;
impl<'a, REG> FlashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Flash::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Flash::Enable)
    }
}
#[doc = "Enables the clock for the FMC controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmc {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Fmc> for bool {
    #[inline(always)]
    fn from(variant: Fmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMC` reader - Enables the clock for the FMC controller."]
pub type FmcR = crate::BitReader<Fmc>;
impl FmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmc {
        match self.bits {
            false => Fmc::Disable,
            true => Fmc::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fmc::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fmc::Enable
    }
}
#[doc = "Field `FMC` writer - Enables the clock for the FMC controller."]
pub type FmcW<'a, REG> = crate::BitWriter<'a, REG, Fmc>;
impl<'a, REG> FmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fmc::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fmc::Enable)
    }
}
#[doc = "Enables the clock for the Input Mux.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mux {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Mux> for bool {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX` reader - Enables the clock for the Input Mux."]
pub type MuxR = crate::BitReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            false => Mux::Disable,
            true => Mux::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mux::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mux::Enable
    }
}
#[doc = "Field `MUX` writer - Enables the clock for the Input Mux."]
pub type MuxW<'a, REG> = crate::BitWriter<'a, REG, Mux>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Enable)
    }
}
#[doc = "Enables the clock for the I/O controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocon {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Iocon> for bool {
    #[inline(always)]
    fn from(variant: Iocon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables the clock for the I/O controller."]
pub type IoconR = crate::BitReader<Iocon>;
impl IoconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocon {
        match self.bits {
            false => Iocon::Disable,
            true => Iocon::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Iocon::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Iocon::Enable
    }
}
#[doc = "Field `IOCON` writer - Enables the clock for the I/O controller."]
pub type IoconW<'a, REG> = crate::BitWriter<'a, REG, Iocon>;
impl<'a, REG> IoconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Iocon::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Iocon::Enable)
    }
}
#[doc = "Enables the clock for the GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Gpio0> for bool {
    #[inline(always)]
    fn from(variant: Gpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - Enables the clock for the GPIO0."]
pub type Gpio0R = crate::BitReader<Gpio0>;
impl Gpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0 {
        match self.bits {
            false => Gpio0::Disable,
            true => Gpio0::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio0::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio0::Enable
    }
}
#[doc = "Field `GPIO0` writer - Enables the clock for the GPIO0."]
pub type Gpio0W<'a, REG> = crate::BitWriter<'a, REG, Gpio0>;
impl<'a, REG> Gpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Enable)
    }
}
#[doc = "Enables the clock for the GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Gpio1> for bool {
    #[inline(always)]
    fn from(variant: Gpio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1` reader - Enables the clock for the GPIO1."]
pub type Gpio1R = crate::BitReader<Gpio1>;
impl Gpio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1 {
        match self.bits {
            false => Gpio1::Disable,
            true => Gpio1::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio1::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio1::Enable
    }
}
#[doc = "Field `GPIO1` writer - Enables the clock for the GPIO1."]
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG, Gpio1>;
impl<'a, REG> Gpio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Enable)
    }
}
#[doc = "Enables the clock for the Pin interrupt (PINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pint {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Pint> for bool {
    #[inline(always)]
    fn from(variant: Pint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINT` reader - Enables the clock for the Pin interrupt (PINT)."]
pub type PintR = crate::BitReader<Pint>;
impl PintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pint {
        match self.bits {
            false => Pint::Disable,
            true => Pint::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pint::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pint::Enable
    }
}
#[doc = "Field `PINT` writer - Enables the clock for the Pin interrupt (PINT)."]
pub type PintW<'a, REG> = crate::BitWriter<'a, REG, Pint>;
impl<'a, REG> PintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pint::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pint::Enable)
    }
}
#[doc = "Enables the clock for the Group interrupt (GINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gint {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Gint> for bool {
    #[inline(always)]
    fn from(variant: Gint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GINT` reader - Enables the clock for the Group interrupt (GINT)."]
pub type GintR = crate::BitReader<Gint>;
impl GintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gint {
        match self.bits {
            false => Gint::Disable,
            true => Gint::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gint::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gint::Enable
    }
}
#[doc = "Field `GINT` writer - Enables the clock for the Group interrupt (GINT)."]
pub type GintW<'a, REG> = crate::BitWriter<'a, REG, Gint>;
impl<'a, REG> GintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gint::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gint::Enable)
    }
}
#[doc = "Enables the clock for the DMA0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Dma0> for bool {
    #[inline(always)]
    fn from(variant: Dma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` reader - Enables the clock for the DMA0."]
pub type Dma0R = crate::BitReader<Dma0>;
impl Dma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0 {
        match self.bits {
            false => Dma0::Disable,
            true => Dma0::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dma0::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dma0::Enable
    }
}
#[doc = "Field `DMA0` writer - Enables the clock for the DMA0."]
pub type Dma0W<'a, REG> = crate::BitWriter<'a, REG, Dma0>;
impl<'a, REG> Dma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Enable)
    }
}
#[doc = "Enables the clock for the CRCGEN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcgen {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Crcgen> for bool {
    #[inline(always)]
    fn from(variant: Crcgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCGEN` reader - Enables the clock for the CRCGEN."]
pub type CrcgenR = crate::BitReader<Crcgen>;
impl CrcgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcgen {
        match self.bits {
            false => Crcgen::Disable,
            true => Crcgen::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crcgen::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Crcgen::Enable
    }
}
#[doc = "Field `CRCGEN` writer - Enables the clock for the CRCGEN."]
pub type CrcgenW<'a, REG> = crate::BitWriter<'a, REG, Crcgen>;
impl<'a, REG> CrcgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcgen::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcgen::Enable)
    }
}
#[doc = "Enables the clock for the Watchdog Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Wwdt> for bool {
    #[inline(always)]
    fn from(variant: Wwdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT` reader - Enables the clock for the Watchdog Timer."]
pub type WwdtR = crate::BitReader<Wwdt>;
impl WwdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt {
        match self.bits {
            false => Wwdt::Disable,
            true => Wwdt::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wwdt::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wwdt::Enable
    }
}
#[doc = "Field `WWDT` writer - Enables the clock for the Watchdog Timer."]
pub type WwdtW<'a, REG> = crate::BitWriter<'a, REG, Wwdt>;
impl<'a, REG> WwdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt::Enable)
    }
}
#[doc = "Enables the clock for the Real Time Clock (RTC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - Enables the clock for the Real Time Clock (RTC)."]
pub type RtcR = crate::BitReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            false => Rtc::Disable,
            true => Rtc::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtc::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtc::Enable
    }
}
#[doc = "Field `RTC` writer - Enables the clock for the Real Time Clock (RTC)."]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Enable)
    }
}
#[doc = "Enables the clock for the Inter CPU communication Mailbox.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mailbox {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Mailbox> for bool {
    #[inline(always)]
    fn from(variant: Mailbox) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX` reader - Enables the clock for the Inter CPU communication Mailbox."]
pub type MailboxR = crate::BitReader<Mailbox>;
impl MailboxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mailbox {
        match self.bits {
            false => Mailbox::Disable,
            true => Mailbox::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mailbox::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mailbox::Enable
    }
}
#[doc = "Field `MAILBOX` writer - Enables the clock for the Inter CPU communication Mailbox."]
pub type MailboxW<'a, REG> = crate::BitWriter<'a, REG, Mailbox>;
impl<'a, REG> MailboxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mailbox::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mailbox::Enable)
    }
}
#[doc = "Enables the clock for the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Adc> for bool {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Enables the clock for the ADC."]
pub type AdcR = crate::BitReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            false => Adc::Disable,
            true => Adc::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adc::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adc::Enable
    }
}
#[doc = "Field `ADC` writer - Enables the clock for the ADC."]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG, Adc>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&self) -> SramCtrl1R {
        SramCtrl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&self) -> SramCtrl2R {
        SramCtrl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&self) -> FmcR {
        FmcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&self) -> IoconR {
        IoconR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&self) -> PintR {
        PintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&self) -> GintR {
        GintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&self) -> Dma0R {
        Dma0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&self) -> CrcgenR {
        CrcgenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&self) -> WwdtR {
        WwdtR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&self) -> MailboxR {
        MailboxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> RomW<'_, AhbclkctrlAhbclkctrl0Spec> {
        RomW::new(self, 1)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&mut self) -> SramCtrl1W<'_, AhbclkctrlAhbclkctrl0Spec> {
        SramCtrl1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&mut self) -> SramCtrl2W<'_, AhbclkctrlAhbclkctrl0Spec> {
        SramCtrl2W::new(self, 4)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&mut self) -> FlashW<'_, AhbclkctrlAhbclkctrl0Spec> {
        FlashW::new(self, 7)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&mut self) -> FmcW<'_, AhbclkctrlAhbclkctrl0Spec> {
        FmcW::new(self, 8)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, AhbclkctrlAhbclkctrl0Spec> {
        MuxW::new(self, 11)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IoconW<'_, AhbclkctrlAhbclkctrl0Spec> {
        IoconW::new(self, 13)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> Gpio0W<'_, AhbclkctrlAhbclkctrl0Spec> {
        Gpio0W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<'_, AhbclkctrlAhbclkctrl0Spec> {
        Gpio1W::new(self, 15)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&mut self) -> PintW<'_, AhbclkctrlAhbclkctrl0Spec> {
        PintW::new(self, 18)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&mut self) -> GintW<'_, AhbclkctrlAhbclkctrl0Spec> {
        GintW::new(self, 19)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&mut self) -> Dma0W<'_, AhbclkctrlAhbclkctrl0Spec> {
        Dma0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&mut self) -> CrcgenW<'_, AhbclkctrlAhbclkctrl0Spec> {
        CrcgenW::new(self, 21)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WwdtW<'_, AhbclkctrlAhbclkctrl0Spec> {
        WwdtW::new(self, 22)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, AhbclkctrlAhbclkctrl0Spec> {
        RtcW::new(self, 23)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&mut self) -> MailboxW<'_, AhbclkctrlAhbclkctrl0Spec> {
        MailboxW::new(self, 26)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<'_, AhbclkctrlAhbclkctrl0Spec> {
        AdcW::new(self, 27)
    }
}
#[doc = "AHB Clock control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbclkctrlAhbclkctrl0Spec;
impl crate::RegisterSpec for AhbclkctrlAhbclkctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbclkctrl_ahbclkctrl0::R`](R) reader structure"]
impl crate::Readable for AhbclkctrlAhbclkctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbclkctrl_ahbclkctrl0::W`](W) writer structure"]
impl crate::Writable for AhbclkctrlAhbclkctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCLKCTRL0 to value 0x0180"]
impl crate::Resettable for AhbclkctrlAhbclkctrl0Spec {
    const RESET_VALUE: u32 = 0x0180;
}

#[doc = "Register `AHBCLKCTRL2` reader"]
pub type R = crate::R<AhbclkctrlAhbclkctrl2Spec>;
#[doc = "Register `AHBCLKCTRL2` writer"]
pub type W = crate::W<AhbclkctrlAhbclkctrl2Spec>;
#[doc = "Enables the clock for the DMA1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Dma1> for bool {
    #[inline(always)]
    fn from(variant: Dma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1` reader - Enables the clock for the DMA1."]
pub type Dma1R = crate::BitReader<Dma1>;
impl Dma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1 {
        match self.bits {
            false => Dma1::Disable,
            true => Dma1::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dma1::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dma1::Enable
    }
}
#[doc = "Field `DMA1` writer - Enables the clock for the DMA1."]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG, Dma1>;
impl<'a, REG> Dma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1::Enable)
    }
}
#[doc = "Enables the clock for the Comparator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comp {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Comp> for bool {
    #[inline(always)]
    fn from(variant: Comp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP` reader - Enables the clock for the Comparator."]
pub type CompR = crate::BitReader<Comp>;
impl CompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comp {
        match self.bits {
            false => Comp::Disable,
            true => Comp::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comp::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comp::Enable
    }
}
#[doc = "Field `COMP` writer - Enables the clock for the Comparator."]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG, Comp>;
impl<'a, REG> CompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comp::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comp::Enable)
    }
}
#[doc = "Enables the clock for the USB1-HS Host.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Host {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb1Host> for bool {
    #[inline(always)]
    fn from(variant: Usb1Host) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_HOST` reader - Enables the clock for the USB1-HS Host."]
pub type Usb1HostR = crate::BitReader<Usb1Host>;
impl Usb1HostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Host {
        match self.bits {
            false => Usb1Host::Disable,
            true => Usb1Host::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb1Host::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb1Host::Enable
    }
}
#[doc = "Field `USB1_HOST` writer - Enables the clock for the USB1-HS Host."]
pub type Usb1HostW<'a, REG> = crate::BitWriter<'a, REG, Usb1Host>;
impl<'a, REG> Usb1HostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Host::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Host::Enable)
    }
}
#[doc = "Enables the clock for the USB1-HS device.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Dev {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb1Dev> for bool {
    #[inline(always)]
    fn from(variant: Usb1Dev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_DEV` reader - Enables the clock for the USB1-HS device."]
pub type Usb1DevR = crate::BitReader<Usb1Dev>;
impl Usb1DevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Dev {
        match self.bits {
            false => Usb1Dev::Disable,
            true => Usb1Dev::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb1Dev::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb1Dev::Enable
    }
}
#[doc = "Field `USB1_DEV` writer - Enables the clock for the USB1-HS device."]
pub type Usb1DevW<'a, REG> = crate::BitWriter<'a, REG, Usb1Dev>;
impl<'a, REG> Usb1DevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Dev::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Dev::Enable)
    }
}
#[doc = "Enables the clock for the USB1-HS RAM.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Ram {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb1Ram> for bool {
    #[inline(always)]
    fn from(variant: Usb1Ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_RAM` reader - Enables the clock for the USB1-HS RAM."]
pub type Usb1RamR = crate::BitReader<Usb1Ram>;
impl Usb1RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Ram {
        match self.bits {
            false => Usb1Ram::Disable,
            true => Usb1Ram::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb1Ram::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb1Ram::Enable
    }
}
#[doc = "Field `USB1_RAM` writer - Enables the clock for the USB1-HS RAM."]
pub type Usb1RamW<'a, REG> = crate::BitWriter<'a, REG, Usb1Ram>;
impl<'a, REG> Usb1RamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Ram::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Ram::Enable)
    }
}
#[doc = "Enables the clock for the USB1-HS PHY.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Phy {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb1Phy> for bool {
    #[inline(always)]
    fn from(variant: Usb1Phy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_PHY` reader - Enables the clock for the USB1-HS PHY."]
pub type Usb1PhyR = crate::BitReader<Usb1Phy>;
impl Usb1PhyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Phy {
        match self.bits {
            false => Usb1Phy::Disable,
            true => Usb1Phy::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb1Phy::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb1Phy::Enable
    }
}
#[doc = "Field `USB1_PHY` writer - Enables the clock for the USB1-HS PHY."]
pub type Usb1PhyW<'a, REG> = crate::BitWriter<'a, REG, Usb1Phy>;
impl<'a, REG> Usb1PhyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Phy::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Phy::Enable)
    }
}
#[doc = "Enables the clock for the Frequency meter.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freqme {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Freqme> for bool {
    #[inline(always)]
    fn from(variant: Freqme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME` reader - Enables the clock for the Frequency meter."]
pub type FreqmeR = crate::BitReader<Freqme>;
impl FreqmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freqme {
        match self.bits {
            false => Freqme::Disable,
            true => Freqme::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Freqme::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Freqme::Enable
    }
}
#[doc = "Field `FREQME` writer - Enables the clock for the Frequency meter."]
pub type FreqmeW<'a, REG> = crate::BitWriter<'a, REG, Freqme>;
impl<'a, REG> FreqmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Freqme::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Freqme::Enable)
    }
}
#[doc = "Enables the clock for the code watchdog.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdog {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Cdog> for bool {
    #[inline(always)]
    fn from(variant: Cdog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDOG` reader - Enables the clock for the code watchdog."]
pub type CdogR = crate::BitReader<Cdog>;
impl CdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdog {
        match self.bits {
            false => Cdog::Disable,
            true => Cdog::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cdog::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cdog::Enable
    }
}
#[doc = "Field `CDOG` writer - Enables the clock for the code watchdog."]
pub type CdogW<'a, REG> = crate::BitWriter<'a, REG, Cdog>;
impl<'a, REG> CdogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdog::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdog::Enable)
    }
}
#[doc = "Enables the clock for the RNG.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Enables the clock for the RNG."]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::Disable,
            true => Rng::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rng::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rng::Enable
    }
}
#[doc = "Field `RNG` writer - Enables the clock for the RNG."]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Enable)
    }
}
#[doc = "SYSCTL block clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysctl {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Sysctl> for bool {
    #[inline(always)]
    fn from(variant: Sysctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCTL` reader - SYSCTL block clock."]
pub type SysctlR = crate::BitReader<Sysctl>;
impl SysctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysctl {
        match self.bits {
            false => Sysctl::Disable,
            true => Sysctl::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sysctl::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sysctl::Enable
    }
}
#[doc = "Field `SYSCTL` writer - SYSCTL block clock."]
pub type SysctlW<'a, REG> = crate::BitWriter<'a, REG, Sysctl>;
impl<'a, REG> SysctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sysctl::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sysctl::Enable)
    }
}
#[doc = "Enables the clock for the USB0-FS Host Master.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0Hostm {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb0Hostm> for bool {
    #[inline(always)]
    fn from(variant: Usb0Hostm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTM` reader - Enables the clock for the USB0-FS Host Master."]
pub type Usb0HostmR = crate::BitReader<Usb0Hostm>;
impl Usb0HostmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0Hostm {
        match self.bits {
            false => Usb0Hostm::Disable,
            true => Usb0Hostm::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb0Hostm::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb0Hostm::Enable
    }
}
#[doc = "Field `USB0_HOSTM` writer - Enables the clock for the USB0-FS Host Master."]
pub type Usb0HostmW<'a, REG> = crate::BitWriter<'a, REG, Usb0Hostm>;
impl<'a, REG> Usb0HostmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Hostm::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Hostm::Enable)
    }
}
#[doc = "Enables the clock for the USB0-FS Host Slave.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0Hosts {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Usb0Hosts> for bool {
    #[inline(always)]
    fn from(variant: Usb0Hosts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTS` reader - Enables the clock for the USB0-FS Host Slave."]
pub type Usb0HostsR = crate::BitReader<Usb0Hosts>;
impl Usb0HostsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0Hosts {
        match self.bits {
            false => Usb0Hosts::Disable,
            true => Usb0Hosts::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb0Hosts::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb0Hosts::Enable
    }
}
#[doc = "Field `USB0_HOSTS` writer - Enables the clock for the USB0-FS Host Slave."]
pub type Usb0HostsW<'a, REG> = crate::BitWriter<'a, REG, Usb0Hosts>;
impl<'a, REG> Usb0HostsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Hosts::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Hosts::Enable)
    }
}
#[doc = "Enables the clock for the HASH_AES.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashAes {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<HashAes> for bool {
    #[inline(always)]
    fn from(variant: HashAes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_AES` reader - Enables the clock for the HASH_AES."]
pub type HashAesR = crate::BitReader<HashAes>;
impl HashAesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashAes {
        match self.bits {
            false => HashAes::Disable,
            true => HashAes::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HashAes::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HashAes::Enable
    }
}
#[doc = "Field `HASH_AES` writer - Enables the clock for the HASH_AES."]
pub type HashAesW<'a, REG> = crate::BitWriter<'a, REG, HashAes>;
impl<'a, REG> HashAesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HashAes::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HashAes::Enable)
    }
}
#[doc = "Enables the clock for the PLU LUT.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plulut {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Plulut> for bool {
    #[inline(always)]
    fn from(variant: Plulut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLULUT` reader - Enables the clock for the PLU LUT."]
pub type PlulutR = crate::BitReader<Plulut>;
impl PlulutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plulut {
        match self.bits {
            false => Plulut::Disable,
            true => Plulut::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Plulut::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Plulut::Enable
    }
}
#[doc = "Field `PLULUT` writer - Enables the clock for the PLU LUT."]
pub type PlulutW<'a, REG> = crate::BitWriter<'a, REG, Plulut>;
impl<'a, REG> PlulutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Plulut::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Plulut::Enable)
    }
}
#[doc = "Enables the clock for the Timer 3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer3 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Timer3> for bool {
    #[inline(always)]
    fn from(variant: Timer3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER3` reader - Enables the clock for the Timer 3."]
pub type Timer3R = crate::BitReader<Timer3>;
impl Timer3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer3 {
        match self.bits {
            false => Timer3::Disable,
            true => Timer3::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer3::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer3::Enable
    }
}
#[doc = "Field `TIMER3` writer - Enables the clock for the Timer 3."]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG, Timer3>;
impl<'a, REG> Timer3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3::Enable)
    }
}
#[doc = "Enables the clock for the Timer 4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer4 {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Timer4> for bool {
    #[inline(always)]
    fn from(variant: Timer4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER4` reader - Enables the clock for the Timer 4."]
pub type Timer4R = crate::BitReader<Timer4>;
impl Timer4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer4 {
        match self.bits {
            false => Timer4::Disable,
            true => Timer4::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer4::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer4::Enable
    }
}
#[doc = "Field `TIMER4` writer - Enables the clock for the Timer 4."]
pub type Timer4W<'a, REG> = crate::BitWriter<'a, REG, Timer4>;
impl<'a, REG> Timer4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer4::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer4::Enable)
    }
}
#[doc = "Enables the clock for the PUF reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` reader - Enables the clock for the PUF reset control."]
pub type PufR = crate::BitReader<Puf>;
impl PufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Puf {
        match self.bits {
            false => Puf::Disable,
            true => Puf::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Puf::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Puf::Enable
    }
}
#[doc = "Field `PUF` writer - Enables the clock for the PUF reset control."]
pub type PufW<'a, REG> = crate::BitWriter<'a, REG, Puf>;
impl<'a, REG> PufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::Enable)
    }
}
#[doc = "Enables the clock for the Casper.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` reader - Enables the clock for the Casper."]
pub type CasperR = crate::BitReader<Casper>;
impl CasperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Casper {
        match self.bits {
            false => Casper::Disable,
            true => Casper::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Casper::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Casper::Enable
    }
}
#[doc = "Field `CASPER` writer - Enables the clock for the Casper."]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Enable)
    }
}
#[doc = "Enables the clock for the analog control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalogCtrl {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<AnalogCtrl> for bool {
    #[inline(always)]
    fn from(variant: AnalogCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANALOG_CTRL` reader - Enables the clock for the analog control."]
pub type AnalogCtrlR = crate::BitReader<AnalogCtrl>;
impl AnalogCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnalogCtrl {
        match self.bits {
            false => AnalogCtrl::Disable,
            true => AnalogCtrl::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AnalogCtrl::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AnalogCtrl::Enable
    }
}
#[doc = "Field `ANALOG_CTRL` writer - Enables the clock for the analog control."]
pub type AnalogCtrlW<'a, REG> = crate::BitWriter<'a, REG, AnalogCtrl>;
impl<'a, REG> AnalogCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogCtrl::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogCtrl::Enable)
    }
}
#[doc = "Enables the clock for the HS LSPI.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsLspi {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<HsLspi> for bool {
    #[inline(always)]
    fn from(variant: HsLspi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_LSPI` reader - Enables the clock for the HS LSPI."]
pub type HsLspiR = crate::BitReader<HsLspi>;
impl HsLspiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsLspi {
        match self.bits {
            false => HsLspi::Disable,
            true => HsLspi::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HsLspi::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HsLspi::Enable
    }
}
#[doc = "Field `HS_LSPI` writer - Enables the clock for the HS LSPI."]
pub type HsLspiW<'a, REG> = crate::BitWriter<'a, REG, HsLspi>;
impl<'a, REG> HsLspiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspi::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspi::Enable)
    }
}
#[doc = "Enables the clock for the GPIO secure.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioSec {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<GpioSec> for bool {
    #[inline(always)]
    fn from(variant: GpioSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC` reader - Enables the clock for the GPIO secure."]
pub type GpioSecR = crate::BitReader<GpioSec>;
impl GpioSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioSec {
        match self.bits {
            false => GpioSec::Disable,
            true => GpioSec::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GpioSec::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GpioSec::Enable
    }
}
#[doc = "Field `GPIO_SEC` writer - Enables the clock for the GPIO secure."]
pub type GpioSecW<'a, REG> = crate::BitWriter<'a, REG, GpioSec>;
impl<'a, REG> GpioSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSec::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSec::Enable)
    }
}
#[doc = "Enables the clock for the GPIO secure int.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioSecInt {
    #[doc = "0: Disable Clock."]
    Disable = 0,
    #[doc = "1: Enable Clock."]
    Enable = 1,
}
impl From<GpioSecInt> for bool {
    #[inline(always)]
    fn from(variant: GpioSecInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_INT` reader - Enables the clock for the GPIO secure int."]
pub type GpioSecIntR = crate::BitReader<GpioSecInt>;
impl GpioSecIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioSecInt {
        match self.bits {
            false => GpioSecInt::Disable,
            true => GpioSecInt::Enable,
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GpioSecInt::Disable
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GpioSecInt::Enable
    }
}
#[doc = "Field `GPIO_SEC_INT` writer - Enables the clock for the GPIO secure int."]
pub type GpioSecIntW<'a, REG> = crate::BitWriter<'a, REG, GpioSecInt>;
impl<'a, REG> GpioSecIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecInt::Disable)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecInt::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1-HS Host."]
    #[inline(always)]
    pub fn usb1_host(&self) -> Usb1HostR {
        Usb1HostR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1-HS device."]
    #[inline(always)]
    pub fn usb1_dev(&self) -> Usb1DevR {
        Usb1DevR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1-HS RAM."]
    #[inline(always)]
    pub fn usb1_ram(&self) -> Usb1RamR {
        Usb1RamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the USB1-HS PHY."]
    #[inline(always)]
    pub fn usb1_phy(&self) -> Usb1PhyR {
        Usb1PhyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&self) -> FreqmeR {
        FreqmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the code watchdog."]
    #[inline(always)]
    pub fn cdog(&self) -> CdogR {
        CdogR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&self) -> SysctlR {
        SysctlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB0-FS Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&self) -> Usb0HostmR {
        Usb0HostmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB0-FS Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&self) -> Usb0HostsR {
        Usb0HostsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&self) -> HashAesR {
        HashAesR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&self) -> PlulutR {
        PlulutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&self) -> PufR {
        PufR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&self) -> CasperR {
        CasperR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&self) -> AnalogCtrlR {
        AnalogCtrlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&self) -> HsLspiR {
        HsLspiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&self) -> GpioSecR {
        GpioSecR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&self) -> GpioSecIntR {
        GpioSecIntR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRL_AHBCLKCTRL2")
            .field("dma1", &self.dma1())
            .field("comp", &self.comp())
            .field("usb1_host", &self.usb1_host())
            .field("usb1_dev", &self.usb1_dev())
            .field("usb1_ram", &self.usb1_ram())
            .field("usb1_phy", &self.usb1_phy())
            .field("freqme", &self.freqme())
            .field("cdog", &self.cdog())
            .field("rng", &self.rng())
            .field("sysctl", &self.sysctl())
            .field("usb0_hostm", &self.usb0_hostm())
            .field("usb0_hosts", &self.usb0_hosts())
            .field("hash_aes", &self.hash_aes())
            .field("plulut", &self.plulut())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("puf", &self.puf())
            .field("casper", &self.casper())
            .field("analog_ctrl", &self.analog_ctrl())
            .field("hs_lspi", &self.hs_lspi())
            .field("gpio_sec", &self.gpio_sec())
            .field("gpio_sec_int", &self.gpio_sec_int())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&mut self) -> Dma1W<'_, AhbclkctrlAhbclkctrl2Spec> {
        Dma1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, AhbclkctrlAhbclkctrl2Spec> {
        CompW::new(self, 2)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1-HS Host."]
    #[inline(always)]
    pub fn usb1_host(&mut self) -> Usb1HostW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb1HostW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1-HS device."]
    #[inline(always)]
    pub fn usb1_dev(&mut self) -> Usb1DevW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb1DevW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1-HS RAM."]
    #[inline(always)]
    pub fn usb1_ram(&mut self) -> Usb1RamW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb1RamW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables the clock for the USB1-HS PHY."]
    #[inline(always)]
    pub fn usb1_phy(&mut self) -> Usb1PhyW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb1PhyW::new(self, 7)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&mut self) -> FreqmeW<'_, AhbclkctrlAhbclkctrl2Spec> {
        FreqmeW::new(self, 8)
    }
    #[doc = "Bit 11 - Enables the clock for the code watchdog."]
    #[inline(always)]
    pub fn cdog(&mut self) -> CdogW<'_, AhbclkctrlAhbclkctrl2Spec> {
        CdogW::new(self, 11)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&mut self) -> RngW<'_, AhbclkctrlAhbclkctrl2Spec> {
        RngW::new(self, 13)
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&mut self) -> SysctlW<'_, AhbclkctrlAhbclkctrl2Spec> {
        SysctlW::new(self, 15)
    }
    #[doc = "Bit 16 - Enables the clock for the USB0-FS Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&mut self) -> Usb0HostmW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb0HostmW::new(self, 16)
    }
    #[doc = "Bit 17 - Enables the clock for the USB0-FS Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&mut self) -> Usb0HostsW<'_, AhbclkctrlAhbclkctrl2Spec> {
        Usb0HostsW::new(self, 17)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&mut self) -> HashAesW<'_, AhbclkctrlAhbclkctrl2Spec> {
        HashAesW::new(self, 18)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&mut self) -> PlulutW<'_, AhbclkctrlAhbclkctrl2Spec> {
        PlulutW::new(self, 20)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&mut self) -> Timer3W<'_, AhbclkctrlAhbclkctrl2Spec> {
        Timer3W::new(self, 21)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&mut self) -> Timer4W<'_, AhbclkctrlAhbclkctrl2Spec> {
        Timer4W::new(self, 22)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&mut self) -> PufW<'_, AhbclkctrlAhbclkctrl2Spec> {
        PufW::new(self, 23)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&mut self) -> CasperW<'_, AhbclkctrlAhbclkctrl2Spec> {
        CasperW::new(self, 24)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&mut self) -> AnalogCtrlW<'_, AhbclkctrlAhbclkctrl2Spec> {
        AnalogCtrlW::new(self, 27)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&mut self) -> HsLspiW<'_, AhbclkctrlAhbclkctrl2Spec> {
        HsLspiW::new(self, 28)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&mut self) -> GpioSecW<'_, AhbclkctrlAhbclkctrl2Spec> {
        GpioSecW::new(self, 29)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&mut self) -> GpioSecIntW<'_, AhbclkctrlAhbclkctrl2Spec> {
        GpioSecIntW::new(self, 30)
    }
}
#[doc = "AHB Clock control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbclkctrlAhbclkctrl2Spec;
impl crate::RegisterSpec for AhbclkctrlAhbclkctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbclkctrl_ahbclkctrl2::R`](R) reader structure"]
impl crate::Readable for AhbclkctrlAhbclkctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbclkctrl_ahbclkctrl2::W`](W) writer structure"]
impl crate::Writable for AhbclkctrlAhbclkctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCLKCTRL2 to value 0"]
impl crate::Resettable for AhbclkctrlAhbclkctrl2Spec {}

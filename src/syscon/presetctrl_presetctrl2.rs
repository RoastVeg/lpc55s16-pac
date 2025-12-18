#[doc = "Register `PRESETCTRL2` reader"]
pub type R = crate::R<PresetctrlPresetctrl2Spec>;
#[doc = "Register `PRESETCTRL2` writer"]
pub type W = crate::W<PresetctrlPresetctrl2Spec>;
#[doc = "DMA1 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Dma1Rst> for bool {
    #[inline(always)]
    fn from(variant: Dma1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1_RST` reader - DMA1 reset control."]
pub type Dma1RstR = crate::BitReader<Dma1Rst>;
impl Dma1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1Rst {
        match self.bits {
            false => Dma1Rst::Released,
            true => Dma1Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Dma1Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Dma1Rst::Asserted
    }
}
#[doc = "Field `DMA1_RST` writer - DMA1 reset control."]
pub type Dma1RstW<'a, REG> = crate::BitWriter<'a, REG, Dma1Rst>;
impl<'a, REG> Dma1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rst::Asserted)
    }
}
#[doc = "Comparator reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<CompRst> for bool {
    #[inline(always)]
    fn from(variant: CompRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP_RST` reader - Comparator reset control."]
pub type CompRstR = crate::BitReader<CompRst>;
impl CompRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CompRst {
        match self.bits {
            false => CompRst::Released,
            true => CompRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CompRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CompRst::Asserted
    }
}
#[doc = "Field `COMP_RST` writer - Comparator reset control."]
pub type CompRstW<'a, REG> = crate::BitWriter<'a, REG, CompRst>;
impl<'a, REG> CompRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(CompRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(CompRst::Asserted)
    }
}
#[doc = "USB1-HS Host reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1HostRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb1HostRst> for bool {
    #[inline(always)]
    fn from(variant: Usb1HostRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_HOST_RST` reader - USB1-HS Host reset control."]
pub type Usb1HostRstR = crate::BitReader<Usb1HostRst>;
impl Usb1HostRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1HostRst {
        match self.bits {
            false => Usb1HostRst::Released,
            true => Usb1HostRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb1HostRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb1HostRst::Asserted
    }
}
#[doc = "Field `USB1_HOST_RST` writer - USB1-HS Host reset control."]
pub type Usb1HostRstW<'a, REG> = crate::BitWriter<'a, REG, Usb1HostRst>;
impl<'a, REG> Usb1HostRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1HostRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1HostRst::Asserted)
    }
}
#[doc = "USB1-HS dev reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1DevRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb1DevRst> for bool {
    #[inline(always)]
    fn from(variant: Usb1DevRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_DEV_RST` reader - USB1-HS dev reset control."]
pub type Usb1DevRstR = crate::BitReader<Usb1DevRst>;
impl Usb1DevRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1DevRst {
        match self.bits {
            false => Usb1DevRst::Released,
            true => Usb1DevRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb1DevRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb1DevRst::Asserted
    }
}
#[doc = "Field `USB1_DEV_RST` writer - USB1-HS dev reset control."]
pub type Usb1DevRstW<'a, REG> = crate::BitWriter<'a, REG, Usb1DevRst>;
impl<'a, REG> Usb1DevRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1DevRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1DevRst::Asserted)
    }
}
#[doc = "USB1-HS RAM reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1RamRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb1RamRst> for bool {
    #[inline(always)]
    fn from(variant: Usb1RamRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_RAM_RST` reader - USB1-HS RAM reset control."]
pub type Usb1RamRstR = crate::BitReader<Usb1RamRst>;
impl Usb1RamRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1RamRst {
        match self.bits {
            false => Usb1RamRst::Released,
            true => Usb1RamRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb1RamRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb1RamRst::Asserted
    }
}
#[doc = "Field `USB1_RAM_RST` writer - USB1-HS RAM reset control."]
pub type Usb1RamRstW<'a, REG> = crate::BitWriter<'a, REG, Usb1RamRst>;
impl<'a, REG> Usb1RamRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1RamRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1RamRst::Asserted)
    }
}
#[doc = "USB1-HS PHY reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1PhyRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb1PhyRst> for bool {
    #[inline(always)]
    fn from(variant: Usb1PhyRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_PHY_RST` reader - USB1-HS PHY reset control."]
pub type Usb1PhyRstR = crate::BitReader<Usb1PhyRst>;
impl Usb1PhyRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1PhyRst {
        match self.bits {
            false => Usb1PhyRst::Released,
            true => Usb1PhyRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb1PhyRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb1PhyRst::Asserted
    }
}
#[doc = "Field `USB1_PHY_RST` writer - USB1-HS PHY reset control."]
pub type Usb1PhyRstW<'a, REG> = crate::BitWriter<'a, REG, Usb1PhyRst>;
impl<'a, REG> Usb1PhyRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1PhyRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1PhyRst::Asserted)
    }
}
#[doc = "Frequency meter reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<FreqmeRst> for bool {
    #[inline(always)]
    fn from(variant: FreqmeRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_RST` reader - Frequency meter reset control."]
pub type FreqmeRstR = crate::BitReader<FreqmeRst>;
impl FreqmeRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FreqmeRst {
        match self.bits {
            false => FreqmeRst::Released,
            true => FreqmeRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FreqmeRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FreqmeRst::Asserted
    }
}
#[doc = "Field `FREQME_RST` writer - Frequency meter reset control."]
pub type FreqmeRstW<'a, REG> = crate::BitWriter<'a, REG, FreqmeRst>;
impl<'a, REG> FreqmeRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRst::Asserted)
    }
}
#[doc = "Code Watchdog reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdogRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<CdogRst> for bool {
    #[inline(always)]
    fn from(variant: CdogRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDOG_RST` reader - Code Watchdog reset control."]
pub type CdogRstR = crate::BitReader<CdogRst>;
impl CdogRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CdogRst {
        match self.bits {
            false => CdogRst::Released,
            true => CdogRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CdogRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CdogRst::Asserted
    }
}
#[doc = "Field `CDOG_RST` writer - Code Watchdog reset control."]
pub type CdogRstW<'a, REG> = crate::BitWriter<'a, REG, CdogRst>;
impl<'a, REG> CdogRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(CdogRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(CdogRst::Asserted)
    }
}
#[doc = "RNG reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<RngRst> for bool {
    #[inline(always)]
    fn from(variant: RngRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_RST` reader - RNG reset control."]
pub type RngRstR = crate::BitReader<RngRst>;
impl RngRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RngRst {
        match self.bits {
            false => RngRst::Released,
            true => RngRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RngRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RngRst::Asserted
    }
}
#[doc = "Field `RNG_RST` writer - RNG reset control."]
pub type RngRstW<'a, REG> = crate::BitWriter<'a, REG, RngRst>;
impl<'a, REG> RngRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(RngRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(RngRst::Asserted)
    }
}
#[doc = "SYSCTL Block reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysctlRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<SysctlRst> for bool {
    #[inline(always)]
    fn from(variant: SysctlRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCTL_RST` reader - SYSCTL Block reset."]
pub type SysctlRstR = crate::BitReader<SysctlRst>;
impl SysctlRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysctlRst {
        match self.bits {
            false => SysctlRst::Released,
            true => SysctlRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SysctlRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SysctlRst::Asserted
    }
}
#[doc = "Field `SYSCTL_RST` writer - SYSCTL Block reset."]
pub type SysctlRstW<'a, REG> = crate::BitWriter<'a, REG, SysctlRst>;
impl<'a, REG> SysctlRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(SysctlRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SysctlRst::Asserted)
    }
}
#[doc = "USB0-FS Host Master reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0HostmRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb0HostmRst> for bool {
    #[inline(always)]
    fn from(variant: Usb0HostmRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTM_RST` reader - USB0-FS Host Master reset control."]
pub type Usb0HostmRstR = crate::BitReader<Usb0HostmRst>;
impl Usb0HostmRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0HostmRst {
        match self.bits {
            false => Usb0HostmRst::Released,
            true => Usb0HostmRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb0HostmRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb0HostmRst::Asserted
    }
}
#[doc = "Field `USB0_HOSTM_RST` writer - USB0-FS Host Master reset control."]
pub type Usb0HostmRstW<'a, REG> = crate::BitWriter<'a, REG, Usb0HostmRst>;
impl<'a, REG> Usb0HostmRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0HostmRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0HostmRst::Asserted)
    }
}
#[doc = "USB0-FS Host Slave reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0HostsRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Usb0HostsRst> for bool {
    #[inline(always)]
    fn from(variant: Usb0HostsRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTS_RST` reader - USB0-FS Host Slave reset control."]
pub type Usb0HostsRstR = crate::BitReader<Usb0HostsRst>;
impl Usb0HostsRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0HostsRst {
        match self.bits {
            false => Usb0HostsRst::Released,
            true => Usb0HostsRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Usb0HostsRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Usb0HostsRst::Asserted
    }
}
#[doc = "Field `USB0_HOSTS_RST` writer - USB0-FS Host Slave reset control."]
pub type Usb0HostsRstW<'a, REG> = crate::BitWriter<'a, REG, Usb0HostsRst>;
impl<'a, REG> Usb0HostsRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0HostsRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0HostsRst::Asserted)
    }
}
#[doc = "HASH_AES reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashAesRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<HashAesRst> for bool {
    #[inline(always)]
    fn from(variant: HashAesRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_AES_RST` reader - HASH_AES reset control."]
pub type HashAesRstR = crate::BitReader<HashAesRst>;
impl HashAesRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashAesRst {
        match self.bits {
            false => HashAesRst::Released,
            true => HashAesRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HashAesRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HashAesRst::Asserted
    }
}
#[doc = "Field `HASH_AES_RST` writer - HASH_AES reset control."]
pub type HashAesRstW<'a, REG> = crate::BitWriter<'a, REG, HashAesRst>;
impl<'a, REG> HashAesRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(HashAesRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(HashAesRst::Asserted)
    }
}
#[doc = "PLU LUT reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlulutRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<PlulutRst> for bool {
    #[inline(always)]
    fn from(variant: PlulutRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLULUT_RST` reader - PLU LUT reset control."]
pub type PlulutRstR = crate::BitReader<PlulutRst>;
impl PlulutRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlulutRst {
        match self.bits {
            false => PlulutRst::Released,
            true => PlulutRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PlulutRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PlulutRst::Asserted
    }
}
#[doc = "Field `PLULUT_RST` writer - PLU LUT reset control."]
pub type PlulutRstW<'a, REG> = crate::BitWriter<'a, REG, PlulutRst>;
impl<'a, REG> PlulutRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(PlulutRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(PlulutRst::Asserted)
    }
}
#[doc = "Timer 3 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer3Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Timer3Rst> for bool {
    #[inline(always)]
    fn from(variant: Timer3Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER3_RST` reader - Timer 3 reset control."]
pub type Timer3RstR = crate::BitReader<Timer3Rst>;
impl Timer3RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer3Rst {
        match self.bits {
            false => Timer3Rst::Released,
            true => Timer3Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Timer3Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Timer3Rst::Asserted
    }
}
#[doc = "Field `TIMER3_RST` writer - Timer 3 reset control."]
pub type Timer3RstW<'a, REG> = crate::BitWriter<'a, REG, Timer3Rst>;
impl<'a, REG> Timer3RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3Rst::Asserted)
    }
}
#[doc = "Timer 4 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer4Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Timer4Rst> for bool {
    #[inline(always)]
    fn from(variant: Timer4Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER4_RST` reader - Timer 4 reset control."]
pub type Timer4RstR = crate::BitReader<Timer4Rst>;
impl Timer4RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer4Rst {
        match self.bits {
            false => Timer4Rst::Released,
            true => Timer4Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Timer4Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Timer4Rst::Asserted
    }
}
#[doc = "Field `TIMER4_RST` writer - Timer 4 reset control."]
pub type Timer4RstW<'a, REG> = crate::BitWriter<'a, REG, Timer4Rst>;
impl<'a, REG> Timer4RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Timer4Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Timer4Rst::Asserted)
    }
}
#[doc = "PUF reset control reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PufRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<PufRst> for bool {
    #[inline(always)]
    fn from(variant: PufRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_RST` reader - PUF reset control reset control."]
pub type PufRstR = crate::BitReader<PufRst>;
impl PufRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PufRst {
        match self.bits {
            false => PufRst::Released,
            true => PufRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PufRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PufRst::Asserted
    }
}
#[doc = "Field `PUF_RST` writer - PUF reset control reset control."]
pub type PufRstW<'a, REG> = crate::BitWriter<'a, REG, PufRst>;
impl<'a, REG> PufRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(PufRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(PufRst::Asserted)
    }
}
#[doc = "Casper reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<CasperRst> for bool {
    #[inline(always)]
    fn from(variant: CasperRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_RST` reader - Casper reset control."]
pub type CasperRstR = crate::BitReader<CasperRst>;
impl CasperRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperRst {
        match self.bits {
            false => CasperRst::Released,
            true => CasperRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CasperRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CasperRst::Asserted
    }
}
#[doc = "Field `CASPER_RST` writer - Casper reset control."]
pub type CasperRstW<'a, REG> = crate::BitWriter<'a, REG, CasperRst>;
impl<'a, REG> CasperRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRst::Asserted)
    }
}
#[doc = "analog control reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalogCtrlRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<AnalogCtrlRst> for bool {
    #[inline(always)]
    fn from(variant: AnalogCtrlRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANALOG_CTRL_RST` reader - analog control reset control."]
pub type AnalogCtrlRstR = crate::BitReader<AnalogCtrlRst>;
impl AnalogCtrlRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnalogCtrlRst {
        match self.bits {
            false => AnalogCtrlRst::Released,
            true => AnalogCtrlRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == AnalogCtrlRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == AnalogCtrlRst::Asserted
    }
}
#[doc = "Field `ANALOG_CTRL_RST` writer - analog control reset control."]
pub type AnalogCtrlRstW<'a, REG> = crate::BitWriter<'a, REG, AnalogCtrlRst>;
impl<'a, REG> AnalogCtrlRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogCtrlRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogCtrlRst::Asserted)
    }
}
#[doc = "HS LSPI reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsLspiRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<HsLspiRst> for bool {
    #[inline(always)]
    fn from(variant: HsLspiRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_LSPI_RST` reader - HS LSPI reset control."]
pub type HsLspiRstR = crate::BitReader<HsLspiRst>;
impl HsLspiRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsLspiRst {
        match self.bits {
            false => HsLspiRst::Released,
            true => HsLspiRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HsLspiRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HsLspiRst::Asserted
    }
}
#[doc = "Field `HS_LSPI_RST` writer - HS LSPI reset control."]
pub type HsLspiRstW<'a, REG> = crate::BitWriter<'a, REG, HsLspiRst>;
impl<'a, REG> HsLspiRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRst::Asserted)
    }
}
#[doc = "GPIO secure reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioSecRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<GpioSecRst> for bool {
    #[inline(always)]
    fn from(variant: GpioSecRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_RST` reader - GPIO secure reset control."]
pub type GpioSecRstR = crate::BitReader<GpioSecRst>;
impl GpioSecRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioSecRst {
        match self.bits {
            false => GpioSecRst::Released,
            true => GpioSecRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GpioSecRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GpioSecRst::Asserted
    }
}
#[doc = "Field `GPIO_SEC_RST` writer - GPIO secure reset control."]
pub type GpioSecRstW<'a, REG> = crate::BitWriter<'a, REG, GpioSecRst>;
impl<'a, REG> GpioSecRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecRst::Asserted)
    }
}
#[doc = "GPIO secure int reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioSecIntRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<GpioSecIntRst> for bool {
    #[inline(always)]
    fn from(variant: GpioSecIntRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` reader - GPIO secure int reset control."]
pub type GpioSecIntRstR = crate::BitReader<GpioSecIntRst>;
impl GpioSecIntRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioSecIntRst {
        match self.bits {
            false => GpioSecIntRst::Released,
            true => GpioSecIntRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GpioSecIntRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GpioSecIntRst::Asserted
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` writer - GPIO secure int reset control."]
pub type GpioSecIntRstW<'a, REG> = crate::BitWriter<'a, REG, GpioSecIntRst>;
impl<'a, REG> GpioSecIntRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecIntRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSecIntRst::Asserted)
    }
}
impl R {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&self) -> Dma1RstR {
        Dma1RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&self) -> CompRstR {
        CompRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - USB1-HS Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&self) -> Usb1HostRstR {
        Usb1HostRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB1-HS dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&self) -> Usb1DevRstR {
        Usb1DevRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB1-HS RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&self) -> Usb1RamRstR {
        Usb1RamRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB1-HS PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&self) -> Usb1PhyRstR {
        Usb1PhyRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&self) -> FreqmeRstR {
        FreqmeRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Code Watchdog reset control."]
    #[inline(always)]
    pub fn cdog_rst(&self) -> CdogRstR {
        CdogRstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RngRstR {
        RngRstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&self) -> SysctlRstR {
        SysctlRstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB0-FS Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&self) -> Usb0HostmRstR {
        Usb0HostmRstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB0-FS Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&self) -> Usb0HostsRstR {
        Usb0HostsRstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&self) -> HashAesRstR {
        HashAesRstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&self) -> PlulutRstR {
        PlulutRstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&self) -> Timer3RstR {
        Timer3RstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&self) -> Timer4RstR {
        Timer4RstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&self) -> PufRstR {
        PufRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&self) -> CasperRstR {
        CasperRstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&self) -> AnalogCtrlRstR {
        AnalogCtrlRstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&self) -> HsLspiRstR {
        HsLspiRstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&self) -> GpioSecRstR {
        GpioSecRstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&self) -> GpioSecIntRstR {
        GpioSecIntRstR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRL_PRESETCTRL2")
            .field("dma1_rst", &self.dma1_rst())
            .field("comp_rst", &self.comp_rst())
            .field("usb1_host_rst", &self.usb1_host_rst())
            .field("usb1_dev_rst", &self.usb1_dev_rst())
            .field("usb1_ram_rst", &self.usb1_ram_rst())
            .field("usb1_phy_rst", &self.usb1_phy_rst())
            .field("freqme_rst", &self.freqme_rst())
            .field("cdog_rst", &self.cdog_rst())
            .field("rng_rst", &self.rng_rst())
            .field("sysctl_rst", &self.sysctl_rst())
            .field("usb0_hostm_rst", &self.usb0_hostm_rst())
            .field("usb0_hosts_rst", &self.usb0_hosts_rst())
            .field("hash_aes_rst", &self.hash_aes_rst())
            .field("plulut_rst", &self.plulut_rst())
            .field("timer3_rst", &self.timer3_rst())
            .field("timer4_rst", &self.timer4_rst())
            .field("puf_rst", &self.puf_rst())
            .field("casper_rst", &self.casper_rst())
            .field("analog_ctrl_rst", &self.analog_ctrl_rst())
            .field("hs_lspi_rst", &self.hs_lspi_rst())
            .field("gpio_sec_rst", &self.gpio_sec_rst())
            .field("gpio_sec_int_rst", &self.gpio_sec_int_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&mut self) -> Dma1RstW<'_, PresetctrlPresetctrl2Spec> {
        Dma1RstW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&mut self) -> CompRstW<'_, PresetctrlPresetctrl2Spec> {
        CompRstW::new(self, 2)
    }
    #[doc = "Bit 4 - USB1-HS Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&mut self) -> Usb1HostRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb1HostRstW::new(self, 4)
    }
    #[doc = "Bit 5 - USB1-HS dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&mut self) -> Usb1DevRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb1DevRstW::new(self, 5)
    }
    #[doc = "Bit 6 - USB1-HS RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&mut self) -> Usb1RamRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb1RamRstW::new(self, 6)
    }
    #[doc = "Bit 7 - USB1-HS PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&mut self) -> Usb1PhyRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb1PhyRstW::new(self, 7)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&mut self) -> FreqmeRstW<'_, PresetctrlPresetctrl2Spec> {
        FreqmeRstW::new(self, 8)
    }
    #[doc = "Bit 11 - Code Watchdog reset control."]
    #[inline(always)]
    pub fn cdog_rst(&mut self) -> CdogRstW<'_, PresetctrlPresetctrl2Spec> {
        CdogRstW::new(self, 11)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RngRstW<'_, PresetctrlPresetctrl2Spec> {
        RngRstW::new(self, 13)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&mut self) -> SysctlRstW<'_, PresetctrlPresetctrl2Spec> {
        SysctlRstW::new(self, 15)
    }
    #[doc = "Bit 16 - USB0-FS Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&mut self) -> Usb0HostmRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb0HostmRstW::new(self, 16)
    }
    #[doc = "Bit 17 - USB0-FS Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&mut self) -> Usb0HostsRstW<'_, PresetctrlPresetctrl2Spec> {
        Usb0HostsRstW::new(self, 17)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&mut self) -> HashAesRstW<'_, PresetctrlPresetctrl2Spec> {
        HashAesRstW::new(self, 18)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&mut self) -> PlulutRstW<'_, PresetctrlPresetctrl2Spec> {
        PlulutRstW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&mut self) -> Timer3RstW<'_, PresetctrlPresetctrl2Spec> {
        Timer3RstW::new(self, 21)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&mut self) -> Timer4RstW<'_, PresetctrlPresetctrl2Spec> {
        Timer4RstW::new(self, 22)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&mut self) -> PufRstW<'_, PresetctrlPresetctrl2Spec> {
        PufRstW::new(self, 23)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&mut self) -> CasperRstW<'_, PresetctrlPresetctrl2Spec> {
        CasperRstW::new(self, 24)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&mut self) -> AnalogCtrlRstW<'_, PresetctrlPresetctrl2Spec> {
        AnalogCtrlRstW::new(self, 27)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&mut self) -> HsLspiRstW<'_, PresetctrlPresetctrl2Spec> {
        HsLspiRstW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&mut self) -> GpioSecRstW<'_, PresetctrlPresetctrl2Spec> {
        GpioSecRstW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&mut self) -> GpioSecIntRstW<'_, PresetctrlPresetctrl2Spec> {
        GpioSecIntRstW::new(self, 30)
    }
}
#[doc = "Peripheral reset control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlPresetctrl2Spec;
impl crate::RegisterSpec for PresetctrlPresetctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrl_presetctrl2::R`](R) reader structure"]
impl crate::Readable for PresetctrlPresetctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`presetctrl_presetctrl2::W`](W) writer structure"]
impl crate::Writable for PresetctrlPresetctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRL2 to value 0"]
impl crate::Resettable for PresetctrlPresetctrl2Spec {}

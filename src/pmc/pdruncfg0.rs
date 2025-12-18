#[doc = "Register `PDRUNCFG0` reader"]
pub type R = crate::R<Pdruncfg0Spec>;
#[doc = "Register `PDRUNCFG0` writer"]
pub type W = crate::W<Pdruncfg0Spec>;
#[doc = "Controls power to VBAT Brown Out Detector (BOD).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenBodvbat {
    #[doc = "0: BOD VBAT is powered."]
    Poweredon = 0,
    #[doc = "1: BOD VBAT is powered down."]
    Poweredoff = 1,
}
impl From<PdenBodvbat> for bool {
    #[inline(always)]
    fn from(variant: PdenBodvbat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_BODVBAT` reader - Controls power to VBAT Brown Out Detector (BOD)."]
pub type PdenBodvbatR = crate::BitReader<PdenBodvbat>;
impl PdenBodvbatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenBodvbat {
        match self.bits {
            false => PdenBodvbat::Poweredon,
            true => PdenBodvbat::Poweredoff,
        }
    }
    #[doc = "BOD VBAT is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenBodvbat::Poweredon
    }
    #[doc = "BOD VBAT is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenBodvbat::Poweredoff
    }
}
#[doc = "Field `PDEN_BODVBAT` writer - Controls power to VBAT Brown Out Detector (BOD)."]
pub type PdenBodvbatW<'a, REG> = crate::BitWriter<'a, REG, PdenBodvbat>;
impl<'a, REG> PdenBodvbatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOD VBAT is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenBodvbat::Poweredon)
    }
    #[doc = "BOD VBAT is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenBodvbat::Poweredoff)
    }
}
#[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenFro32k {
    #[doc = "0: FRO32KHz is powered."]
    Poweredon = 0,
    #[doc = "1: FRO32KHz is powered down."]
    Poweredoff = 1,
}
impl From<PdenFro32k> for bool {
    #[inline(always)]
    fn from(variant: PdenFro32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_FRO32K` reader - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
pub type PdenFro32kR = crate::BitReader<PdenFro32k>;
impl PdenFro32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenFro32k {
        match self.bits {
            false => PdenFro32k::Poweredon,
            true => PdenFro32k::Poweredoff,
        }
    }
    #[doc = "FRO32KHz is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenFro32k::Poweredon
    }
    #[doc = "FRO32KHz is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenFro32k::Poweredoff
    }
}
#[doc = "Field `PDEN_FRO32K` writer - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
pub type PdenFro32kW<'a, REG> = crate::BitWriter<'a, REG, PdenFro32k>;
impl<'a, REG> PdenFro32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRO32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenFro32k::Poweredon)
    }
    #[doc = "FRO32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenFro32k::Poweredoff)
    }
}
#[doc = "Controls power to crystal 32 KHz.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenXtal32k {
    #[doc = "0: Crystal 32KHz is powered."]
    Poweredon = 0,
    #[doc = "1: Crystal 32KHz is powered down."]
    Poweredoff = 1,
}
impl From<PdenXtal32k> for bool {
    #[inline(always)]
    fn from(variant: PdenXtal32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_XTAL32K` reader - Controls power to crystal 32 KHz."]
pub type PdenXtal32kR = crate::BitReader<PdenXtal32k>;
impl PdenXtal32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenXtal32k {
        match self.bits {
            false => PdenXtal32k::Poweredon,
            true => PdenXtal32k::Poweredoff,
        }
    }
    #[doc = "Crystal 32KHz is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenXtal32k::Poweredon
    }
    #[doc = "Crystal 32KHz is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenXtal32k::Poweredoff
    }
}
#[doc = "Field `PDEN_XTAL32K` writer - Controls power to crystal 32 KHz."]
pub type PdenXtal32kW<'a, REG> = crate::BitWriter<'a, REG, PdenXtal32k>;
impl<'a, REG> PdenXtal32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Crystal 32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenXtal32k::Poweredon)
    }
    #[doc = "Crystal 32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenXtal32k::Poweredoff)
    }
}
#[doc = "Controls power to high speed crystal.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenXtal32m {
    #[doc = "0: High speed crystal is powered."]
    Poweredon = 0,
    #[doc = "1: High speed crystal is powered down."]
    Poweredoff = 1,
}
impl From<PdenXtal32m> for bool {
    #[inline(always)]
    fn from(variant: PdenXtal32m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_XTAL32M` reader - Controls power to high speed crystal."]
pub type PdenXtal32mR = crate::BitReader<PdenXtal32m>;
impl PdenXtal32mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenXtal32m {
        match self.bits {
            false => PdenXtal32m::Poweredon,
            true => PdenXtal32m::Poweredoff,
        }
    }
    #[doc = "High speed crystal is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenXtal32m::Poweredon
    }
    #[doc = "High speed crystal is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenXtal32m::Poweredoff
    }
}
#[doc = "Field `PDEN_XTAL32M` writer - Controls power to high speed crystal."]
pub type PdenXtal32mW<'a, REG> = crate::BitWriter<'a, REG, PdenXtal32m>;
impl<'a, REG> PdenXtal32mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed crystal is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenXtal32m::Poweredon)
    }
    #[doc = "High speed crystal is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenXtal32m::Poweredoff)
    }
}
#[doc = "Controls power to System PLL (also refered as PLL0).\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenPll0 {
    #[doc = "0: PLL0 is powered."]
    Poweredon = 0,
    #[doc = "1: PLL0 is powered down."]
    Poweredoff = 1,
}
impl From<PdenPll0> for bool {
    #[inline(always)]
    fn from(variant: PdenPll0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_PLL0` reader - Controls power to System PLL (also refered as PLL0)."]
pub type PdenPll0R = crate::BitReader<PdenPll0>;
impl PdenPll0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenPll0 {
        match self.bits {
            false => PdenPll0::Poweredon,
            true => PdenPll0::Poweredoff,
        }
    }
    #[doc = "PLL0 is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenPll0::Poweredon
    }
    #[doc = "PLL0 is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenPll0::Poweredoff
    }
}
#[doc = "Field `PDEN_PLL0` writer - Controls power to System PLL (also refered as PLL0)."]
pub type PdenPll0W<'a, REG> = crate::BitWriter<'a, REG, PdenPll0>;
impl<'a, REG> PdenPll0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL0 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll0::Poweredon)
    }
    #[doc = "PLL0 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll0::Poweredoff)
    }
}
#[doc = "Controls power to USB PLL (also refered as PLL1).\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenPll1 {
    #[doc = "0: PLL1 is powered."]
    Poweredon = 0,
    #[doc = "1: PLL1 is powered down."]
    Poweredoff = 1,
}
impl From<PdenPll1> for bool {
    #[inline(always)]
    fn from(variant: PdenPll1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_PLL1` reader - Controls power to USB PLL (also refered as PLL1)."]
pub type PdenPll1R = crate::BitReader<PdenPll1>;
impl PdenPll1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenPll1 {
        match self.bits {
            false => PdenPll1::Poweredon,
            true => PdenPll1::Poweredoff,
        }
    }
    #[doc = "PLL1 is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenPll1::Poweredon
    }
    #[doc = "PLL1 is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenPll1::Poweredoff
    }
}
#[doc = "Field `PDEN_PLL1` writer - Controls power to USB PLL (also refered as PLL1)."]
pub type PdenPll1W<'a, REG> = crate::BitWriter<'a, REG, PdenPll1>;
impl<'a, REG> PdenPll1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL1 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll1::Poweredon)
    }
    #[doc = "PLL1 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll1::Poweredoff)
    }
}
#[doc = "Controls power to USB Full Speed phy.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenUsbfsphy {
    #[doc = "0: USB Full Speed phy is powered."]
    Poweredon = 0,
    #[doc = "1: USB Full Speed phy is powered down."]
    Poweredoff = 1,
}
impl From<PdenUsbfsphy> for bool {
    #[inline(always)]
    fn from(variant: PdenUsbfsphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_USBFSPHY` reader - Controls power to USB Full Speed phy."]
pub type PdenUsbfsphyR = crate::BitReader<PdenUsbfsphy>;
impl PdenUsbfsphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenUsbfsphy {
        match self.bits {
            false => PdenUsbfsphy::Poweredon,
            true => PdenUsbfsphy::Poweredoff,
        }
    }
    #[doc = "USB Full Speed phy is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenUsbfsphy::Poweredon
    }
    #[doc = "USB Full Speed phy is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenUsbfsphy::Poweredoff
    }
}
#[doc = "Field `PDEN_USBFSPHY` writer - Controls power to USB Full Speed phy."]
pub type PdenUsbfsphyW<'a, REG> = crate::BitWriter<'a, REG, PdenUsbfsphy>;
impl<'a, REG> PdenUsbfsphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB Full Speed phy is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenUsbfsphy::Poweredon)
    }
    #[doc = "USB Full Speed phy is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenUsbfsphy::Poweredoff)
    }
}
#[doc = "Controls power to USB High Speed Phy.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenUsbhsphy {
    #[doc = "0: USB HS phy is powered."]
    Poweredon = 0,
    #[doc = "1: USB HS phy is powered down."]
    Poweredoff = 1,
}
impl From<PdenUsbhsphy> for bool {
    #[inline(always)]
    fn from(variant: PdenUsbhsphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_USBHSPHY` reader - Controls power to USB High Speed Phy."]
pub type PdenUsbhsphyR = crate::BitReader<PdenUsbhsphy>;
impl PdenUsbhsphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenUsbhsphy {
        match self.bits {
            false => PdenUsbhsphy::Poweredon,
            true => PdenUsbhsphy::Poweredoff,
        }
    }
    #[doc = "USB HS phy is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenUsbhsphy::Poweredon
    }
    #[doc = "USB HS phy is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenUsbhsphy::Poweredoff
    }
}
#[doc = "Field `PDEN_USBHSPHY` writer - Controls power to USB High Speed Phy."]
pub type PdenUsbhsphyW<'a, REG> = crate::BitWriter<'a, REG, PdenUsbhsphy>;
impl<'a, REG> PdenUsbhsphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB HS phy is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenUsbhsphy::Poweredon)
    }
    #[doc = "USB HS phy is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenUsbhsphy::Poweredoff)
    }
}
#[doc = "Controls power to Analog Comparator.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenComp {
    #[doc = "0: Analog Comparator is powered."]
    Poweredon = 0,
    #[doc = "1: Analog Comparator is powered down."]
    Poweredoff = 1,
}
impl From<PdenComp> for bool {
    #[inline(always)]
    fn from(variant: PdenComp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_COMP` reader - Controls power to Analog Comparator."]
pub type PdenCompR = crate::BitReader<PdenComp>;
impl PdenCompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenComp {
        match self.bits {
            false => PdenComp::Poweredon,
            true => PdenComp::Poweredoff,
        }
    }
    #[doc = "Analog Comparator is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenComp::Poweredon
    }
    #[doc = "Analog Comparator is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenComp::Poweredoff
    }
}
#[doc = "Field `PDEN_COMP` writer - Controls power to Analog Comparator."]
pub type PdenCompW<'a, REG> = crate::BitWriter<'a, REG, PdenComp>;
impl<'a, REG> PdenCompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Comparator is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenComp::Poweredon)
    }
    #[doc = "Analog Comparator is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenComp::Poweredoff)
    }
}
#[doc = "Controls power to USB high speed LDO.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenLdousbhs {
    #[doc = "0: USB high speed LDO is powered."]
    Poweredon = 0,
    #[doc = "1: USB high speed LDO is powered down."]
    Poweredoff = 1,
}
impl From<PdenLdousbhs> for bool {
    #[inline(always)]
    fn from(variant: PdenLdousbhs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_LDOUSBHS` reader - Controls power to USB high speed LDO."]
pub type PdenLdousbhsR = crate::BitReader<PdenLdousbhs>;
impl PdenLdousbhsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenLdousbhs {
        match self.bits {
            false => PdenLdousbhs::Poweredon,
            true => PdenLdousbhs::Poweredoff,
        }
    }
    #[doc = "USB high speed LDO is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenLdousbhs::Poweredon
    }
    #[doc = "USB high speed LDO is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenLdousbhs::Poweredoff
    }
}
#[doc = "Field `PDEN_LDOUSBHS` writer - Controls power to USB high speed LDO."]
pub type PdenLdousbhsW<'a, REG> = crate::BitWriter<'a, REG, PdenLdousbhs>;
impl<'a, REG> PdenLdousbhsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB high speed LDO is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenLdousbhs::Poweredon)
    }
    #[doc = "USB high speed LDO is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenLdousbhs::Poweredoff)
    }
}
#[doc = "Controls power to auxiliary biasing (AUXBIAS)\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenAuxbias {
    #[doc = "0: auxiliary biasing is powered."]
    Poweredon = 0,
    #[doc = "1: auxiliary biasing is powered down."]
    Poweredoff = 1,
}
impl From<PdenAuxbias> for bool {
    #[inline(always)]
    fn from(variant: PdenAuxbias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_AUXBIAS` reader - Controls power to auxiliary biasing (AUXBIAS)"]
pub type PdenAuxbiasR = crate::BitReader<PdenAuxbias>;
impl PdenAuxbiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenAuxbias {
        match self.bits {
            false => PdenAuxbias::Poweredon,
            true => PdenAuxbias::Poweredoff,
        }
    }
    #[doc = "auxiliary biasing is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenAuxbias::Poweredon
    }
    #[doc = "auxiliary biasing is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenAuxbias::Poweredoff
    }
}
#[doc = "Field `PDEN_AUXBIAS` writer - Controls power to auxiliary biasing (AUXBIAS)"]
pub type PdenAuxbiasW<'a, REG> = crate::BitWriter<'a, REG, PdenAuxbias>;
impl<'a, REG> PdenAuxbiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "auxiliary biasing is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenAuxbias::Poweredon)
    }
    #[doc = "auxiliary biasing is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenAuxbias::Poweredoff)
    }
}
#[doc = "Controls power to high speed crystal LDO.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenLdoxo32m {
    #[doc = "0: High speed crystal LDO is powered."]
    Poweredon = 0,
    #[doc = "1: High speed crystal LDO is powered down."]
    Poweredoff = 1,
}
impl From<PdenLdoxo32m> for bool {
    #[inline(always)]
    fn from(variant: PdenLdoxo32m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_LDOXO32M` reader - Controls power to high speed crystal LDO."]
pub type PdenLdoxo32mR = crate::BitReader<PdenLdoxo32m>;
impl PdenLdoxo32mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenLdoxo32m {
        match self.bits {
            false => PdenLdoxo32m::Poweredon,
            true => PdenLdoxo32m::Poweredoff,
        }
    }
    #[doc = "High speed crystal LDO is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenLdoxo32m::Poweredon
    }
    #[doc = "High speed crystal LDO is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenLdoxo32m::Poweredoff
    }
}
#[doc = "Field `PDEN_LDOXO32M` writer - Controls power to high speed crystal LDO."]
pub type PdenLdoxo32mW<'a, REG> = crate::BitWriter<'a, REG, PdenLdoxo32m>;
impl<'a, REG> PdenLdoxo32mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed crystal LDO is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenLdoxo32m::Poweredon)
    }
    #[doc = "High speed crystal LDO is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenLdoxo32m::Poweredoff)
    }
}
#[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenRng {
    #[doc = "0: TRNG clocks are powered."]
    Poweredon = 0,
    #[doc = "1: TRNG clocks are powered down."]
    Poweredoff = 1,
}
impl From<PdenRng> for bool {
    #[inline(always)]
    fn from(variant: PdenRng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_RNG` reader - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
pub type PdenRngR = crate::BitReader<PdenRng>;
impl PdenRngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenRng {
        match self.bits {
            false => PdenRng::Poweredon,
            true => PdenRng::Poweredoff,
        }
    }
    #[doc = "TRNG clocks are powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenRng::Poweredon
    }
    #[doc = "TRNG clocks are powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenRng::Poweredoff
    }
}
#[doc = "Field `PDEN_RNG` writer - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
pub type PdenRngW<'a, REG> = crate::BitWriter<'a, REG, PdenRng>;
impl<'a, REG> PdenRngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRNG clocks are powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenRng::Poweredon)
    }
    #[doc = "TRNG clocks are powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenRng::Poweredoff)
    }
}
#[doc = "Controls power to System PLL (PLL0) Spread Spectrum module.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdenPll0Sscg {
    #[doc = "0: PLL0 Sread spectrum module is powered."]
    Poweredon = 0,
    #[doc = "1: PLL0 Sread spectrum module is powered down."]
    Poweredoff = 1,
}
impl From<PdenPll0Sscg> for bool {
    #[inline(always)]
    fn from(variant: PdenPll0Sscg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN_PLL0_SSCG` reader - Controls power to System PLL (PLL0) Spread Spectrum module."]
pub type PdenPll0SscgR = crate::BitReader<PdenPll0Sscg>;
impl PdenPll0SscgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdenPll0Sscg {
        match self.bits {
            false => PdenPll0Sscg::Poweredon,
            true => PdenPll0Sscg::Poweredoff,
        }
    }
    #[doc = "PLL0 Sread spectrum module is powered."]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PdenPll0Sscg::Poweredon
    }
    #[doc = "PLL0 Sread spectrum module is powered down."]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PdenPll0Sscg::Poweredoff
    }
}
#[doc = "Field `PDEN_PLL0_SSCG` writer - Controls power to System PLL (PLL0) Spread Spectrum module."]
pub type PdenPll0SscgW<'a, REG> = crate::BitWriter<'a, REG, PdenPll0Sscg>;
impl<'a, REG> PdenPll0SscgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL0 Sread spectrum module is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll0Sscg::Poweredon)
    }
    #[doc = "PLL0 Sread spectrum module is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut crate::W<REG> {
        self.variant(PdenPll0Sscg::Poweredoff)
    }
}
impl R {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&self) -> PdenBodvbatR {
        PdenBodvbatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&self) -> PdenFro32kR {
        PdenFro32kR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&self) -> PdenXtal32kR {
        PdenXtal32kR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Controls power to high speed crystal."]
    #[inline(always)]
    pub fn pden_xtal32m(&self) -> PdenXtal32mR {
        PdenXtal32mR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub fn pden_pll0(&self) -> PdenPll0R {
        PdenPll0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub fn pden_pll1(&self) -> PdenPll1R {
        PdenPll1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub fn pden_usbfsphy(&self) -> PdenUsbfsphyR {
        PdenUsbfsphyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub fn pden_usbhsphy(&self) -> PdenUsbhsphyR {
        PdenUsbhsphyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&self) -> PdenCompR {
        PdenCompR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline(always)]
    pub fn pden_ldousbhs(&self) -> PdenLdousbhsR {
        PdenLdousbhsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&self) -> PdenAuxbiasR {
        PdenAuxbiasR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&self) -> PdenLdoxo32mR {
        PdenLdoxo32mR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PdenRngR {
        PdenRngR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&self) -> PdenPll0SscgR {
        PdenPll0SscgR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDRUNCFG0")
            .field("pden_bodvbat", &self.pden_bodvbat())
            .field("pden_fro32k", &self.pden_fro32k())
            .field("pden_xtal32k", &self.pden_xtal32k())
            .field("pden_xtal32m", &self.pden_xtal32m())
            .field("pden_pll0", &self.pden_pll0())
            .field("pden_pll1", &self.pden_pll1())
            .field("pden_usbfsphy", &self.pden_usbfsphy())
            .field("pden_usbhsphy", &self.pden_usbhsphy())
            .field("pden_comp", &self.pden_comp())
            .field("pden_ldousbhs", &self.pden_ldousbhs())
            .field("pden_auxbias", &self.pden_auxbias())
            .field("pden_ldoxo32m", &self.pden_ldoxo32m())
            .field("pden_rng", &self.pden_rng())
            .field("pden_pll0_sscg", &self.pden_pll0_sscg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&mut self) -> PdenBodvbatW<'_, Pdruncfg0Spec> {
        PdenBodvbatW::new(self, 3)
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&mut self) -> PdenFro32kW<'_, Pdruncfg0Spec> {
        PdenFro32kW::new(self, 6)
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&mut self) -> PdenXtal32kW<'_, Pdruncfg0Spec> {
        PdenXtal32kW::new(self, 7)
    }
    #[doc = "Bit 8 - Controls power to high speed crystal."]
    #[inline(always)]
    pub fn pden_xtal32m(&mut self) -> PdenXtal32mW<'_, Pdruncfg0Spec> {
        PdenXtal32mW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub fn pden_pll0(&mut self) -> PdenPll0W<'_, Pdruncfg0Spec> {
        PdenPll0W::new(self, 9)
    }
    #[doc = "Bit 10 - Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub fn pden_pll1(&mut self) -> PdenPll1W<'_, Pdruncfg0Spec> {
        PdenPll1W::new(self, 10)
    }
    #[doc = "Bit 11 - Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub fn pden_usbfsphy(&mut self) -> PdenUsbfsphyW<'_, Pdruncfg0Spec> {
        PdenUsbfsphyW::new(self, 11)
    }
    #[doc = "Bit 12 - Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub fn pden_usbhsphy(&mut self) -> PdenUsbhsphyW<'_, Pdruncfg0Spec> {
        PdenUsbhsphyW::new(self, 12)
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&mut self) -> PdenCompW<'_, Pdruncfg0Spec> {
        PdenCompW::new(self, 13)
    }
    #[doc = "Bit 18 - Controls power to USB high speed LDO."]
    #[inline(always)]
    pub fn pden_ldousbhs(&mut self) -> PdenLdousbhsW<'_, Pdruncfg0Spec> {
        PdenLdousbhsW::new(self, 18)
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&mut self) -> PdenAuxbiasW<'_, Pdruncfg0Spec> {
        PdenAuxbiasW::new(self, 19)
    }
    #[doc = "Bit 20 - Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&mut self) -> PdenLdoxo32mW<'_, Pdruncfg0Spec> {
        PdenLdoxo32mW::new(self, 20)
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PdenRngW<'_, Pdruncfg0Spec> {
        PdenRngW::new(self, 22)
    }
    #[doc = "Bit 23 - Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&mut self) -> PdenPll0SscgW<'_, Pdruncfg0Spec> {
        PdenPll0SscgW::new(self, 23)
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg0Spec;
impl crate::RegisterSpec for Pdruncfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdruncfg0::R`](R) reader structure"]
impl crate::Readable for Pdruncfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdruncfg0::W`](W) writer structure"]
impl crate::Writable for Pdruncfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDRUNCFG0 to value 0x00de_ffc0"]
impl crate::Resettable for Pdruncfg0Spec {
    const RESET_VALUE: u32 = 0x00de_ffc0;
}

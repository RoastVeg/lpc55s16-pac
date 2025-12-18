#[doc = "Register `PRESETCTRL0` reader"]
pub type R = crate::R<PresetctrlPresetctrl0Spec>;
#[doc = "Register `PRESETCTRL0` writer"]
pub type W = crate::W<PresetctrlPresetctrl0Spec>;
#[doc = "ROM reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<RomRst> for bool {
    #[inline(always)]
    fn from(variant: RomRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_RST` reader - ROM reset control."]
pub type RomRstR = crate::BitReader<RomRst>;
impl RomRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RomRst {
        match self.bits {
            false => RomRst::Released,
            true => RomRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RomRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RomRst::Asserted
    }
}
#[doc = "Field `ROM_RST` writer - ROM reset control."]
pub type RomRstW<'a, REG> = crate::BitWriter<'a, REG, RomRst>;
impl<'a, REG> RomRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(RomRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(RomRst::Asserted)
    }
}
#[doc = "SRAM Controller 1 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramCtrl1Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<SramCtrl1Rst> for bool {
    #[inline(always)]
    fn from(variant: SramCtrl1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL1_RST` reader - SRAM Controller 1 reset control."]
pub type SramCtrl1RstR = crate::BitReader<SramCtrl1Rst>;
impl SramCtrl1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramCtrl1Rst {
        match self.bits {
            false => SramCtrl1Rst::Released,
            true => SramCtrl1Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SramCtrl1Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SramCtrl1Rst::Asserted
    }
}
#[doc = "Field `SRAM_CTRL1_RST` writer - SRAM Controller 1 reset control."]
pub type SramCtrl1RstW<'a, REG> = crate::BitWriter<'a, REG, SramCtrl1Rst>;
impl<'a, REG> SramCtrl1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl1Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl1Rst::Asserted)
    }
}
#[doc = "SRAM Controller 2 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramCtrl2Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<SramCtrl2Rst> for bool {
    #[inline(always)]
    fn from(variant: SramCtrl2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL2_RST` reader - SRAM Controller 2 reset control."]
pub type SramCtrl2RstR = crate::BitReader<SramCtrl2Rst>;
impl SramCtrl2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramCtrl2Rst {
        match self.bits {
            false => SramCtrl2Rst::Released,
            true => SramCtrl2Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SramCtrl2Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SramCtrl2Rst::Asserted
    }
}
#[doc = "Field `SRAM_CTRL2_RST` writer - SRAM Controller 2 reset control."]
pub type SramCtrl2RstW<'a, REG> = crate::BitWriter<'a, REG, SramCtrl2Rst>;
impl<'a, REG> SramCtrl2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl2Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SramCtrl2Rst::Asserted)
    }
}
#[doc = "Flash controller reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<FlashRst> for bool {
    #[inline(always)]
    fn from(variant: FlashRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_RST` reader - Flash controller reset control."]
pub type FlashRstR = crate::BitReader<FlashRst>;
impl FlashRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashRst {
        match self.bits {
            false => FlashRst::Released,
            true => FlashRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FlashRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FlashRst::Asserted
    }
}
#[doc = "Field `FLASH_RST` writer - Flash controller reset control."]
pub type FlashRstW<'a, REG> = crate::BitWriter<'a, REG, FlashRst>;
impl<'a, REG> FlashRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRst::Asserted)
    }
}
#[doc = "FMC controller reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmcRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<FmcRst> for bool {
    #[inline(always)]
    fn from(variant: FmcRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMC_RST` reader - FMC controller reset control."]
pub type FmcRstR = crate::BitReader<FmcRst>;
impl FmcRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FmcRst {
        match self.bits {
            false => FmcRst::Released,
            true => FmcRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FmcRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FmcRst::Asserted
    }
}
#[doc = "Field `FMC_RST` writer - FMC controller reset control."]
pub type FmcRstW<'a, REG> = crate::BitWriter<'a, REG, FmcRst>;
impl<'a, REG> FmcRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(FmcRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(FmcRst::Asserted)
    }
}
#[doc = "Input Mux reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<MuxRst> for bool {
    #[inline(always)]
    fn from(variant: MuxRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_RST` reader - Input Mux reset control."]
pub type MuxRstR = crate::BitReader<MuxRst>;
impl MuxRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxRst {
        match self.bits {
            false => MuxRst::Released,
            true => MuxRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MuxRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MuxRst::Asserted
    }
}
#[doc = "Field `MUX_RST` writer - Input Mux reset control."]
pub type MuxRstW<'a, REG> = crate::BitWriter<'a, REG, MuxRst>;
impl<'a, REG> MuxRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(MuxRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(MuxRst::Asserted)
    }
}
#[doc = "I/O controller reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoconRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<IoconRst> for bool {
    #[inline(always)]
    fn from(variant: IoconRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON_RST` reader - I/O controller reset control."]
pub type IoconRstR = crate::BitReader<IoconRst>;
impl IoconRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IoconRst {
        match self.bits {
            false => IoconRst::Released,
            true => IoconRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == IoconRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == IoconRst::Asserted
    }
}
#[doc = "Field `IOCON_RST` writer - I/O controller reset control."]
pub type IoconRstW<'a, REG> = crate::BitWriter<'a, REG, IoconRst>;
impl<'a, REG> IoconRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRst::Asserted)
    }
}
#[doc = "GPIO0 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Gpio0Rst> for bool {
    #[inline(always)]
    fn from(variant: Gpio0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0_RST` reader - GPIO0 reset control."]
pub type Gpio0RstR = crate::BitReader<Gpio0Rst>;
impl Gpio0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0Rst {
        match self.bits {
            false => Gpio0Rst::Released,
            true => Gpio0Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Gpio0Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Gpio0Rst::Asserted
    }
}
#[doc = "Field `GPIO0_RST` writer - GPIO0 reset control."]
pub type Gpio0RstW<'a, REG> = crate::BitWriter<'a, REG, Gpio0Rst>;
impl<'a, REG> Gpio0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rst::Asserted)
    }
}
#[doc = "GPIO1 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Gpio1Rst> for bool {
    #[inline(always)]
    fn from(variant: Gpio1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1_RST` reader - GPIO1 reset control."]
pub type Gpio1RstR = crate::BitReader<Gpio1Rst>;
impl Gpio1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1Rst {
        match self.bits {
            false => Gpio1Rst::Released,
            true => Gpio1Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Gpio1Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Gpio1Rst::Asserted
    }
}
#[doc = "Field `GPIO1_RST` writer - GPIO1 reset control."]
pub type Gpio1RstW<'a, REG> = crate::BitWriter<'a, REG, Gpio1Rst>;
impl<'a, REG> Gpio1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rst::Asserted)
    }
}
#[doc = "Pin interrupt (PINT) reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PintRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<PintRst> for bool {
    #[inline(always)]
    fn from(variant: PintRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINT_RST` reader - Pin interrupt (PINT) reset control."]
pub type PintRstR = crate::BitReader<PintRst>;
impl PintRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PintRst {
        match self.bits {
            false => PintRst::Released,
            true => PintRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PintRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PintRst::Asserted
    }
}
#[doc = "Field `PINT_RST` writer - Pin interrupt (PINT) reset control."]
pub type PintRstW<'a, REG> = crate::BitWriter<'a, REG, PintRst>;
impl<'a, REG> PintRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(PintRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(PintRst::Asserted)
    }
}
#[doc = "Group interrupt (GINT) reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GintRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<GintRst> for bool {
    #[inline(always)]
    fn from(variant: GintRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GINT_RST` reader - Group interrupt (GINT) reset control."]
pub type GintRstR = crate::BitReader<GintRst>;
impl GintRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GintRst {
        match self.bits {
            false => GintRst::Released,
            true => GintRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GintRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GintRst::Asserted
    }
}
#[doc = "Field `GINT_RST` writer - Group interrupt (GINT) reset control."]
pub type GintRstW<'a, REG> = crate::BitWriter<'a, REG, GintRst>;
impl<'a, REG> GintRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(GintRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(GintRst::Asserted)
    }
}
#[doc = "DMA0 reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0Rst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<Dma0Rst> for bool {
    #[inline(always)]
    fn from(variant: Dma0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0_RST` reader - DMA0 reset control."]
pub type Dma0RstR = crate::BitReader<Dma0Rst>;
impl Dma0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0Rst {
        match self.bits {
            false => Dma0Rst::Released,
            true => Dma0Rst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Dma0Rst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Dma0Rst::Asserted
    }
}
#[doc = "Field `DMA0_RST` writer - DMA0 reset control."]
pub type Dma0RstW<'a, REG> = crate::BitWriter<'a, REG, Dma0Rst>;
impl<'a, REG> Dma0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rst::Asserted)
    }
}
#[doc = "CRCGEN reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcgenRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<CrcgenRst> for bool {
    #[inline(always)]
    fn from(variant: CrcgenRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCGEN_RST` reader - CRCGEN reset control."]
pub type CrcgenRstR = crate::BitReader<CrcgenRst>;
impl CrcgenRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcgenRst {
        match self.bits {
            false => CrcgenRst::Released,
            true => CrcgenRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CrcgenRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CrcgenRst::Asserted
    }
}
#[doc = "Field `CRCGEN_RST` writer - CRCGEN reset control."]
pub type CrcgenRstW<'a, REG> = crate::BitWriter<'a, REG, CrcgenRst>;
impl<'a, REG> CrcgenRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(CrcgenRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(CrcgenRst::Asserted)
    }
}
#[doc = "Watchdog Timer reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WwdtRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<WwdtRst> for bool {
    #[inline(always)]
    fn from(variant: WwdtRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT_RST` reader - Watchdog Timer reset control."]
pub type WwdtRstR = crate::BitReader<WwdtRst>;
impl WwdtRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WwdtRst {
        match self.bits {
            false => WwdtRst::Released,
            true => WwdtRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WwdtRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WwdtRst::Asserted
    }
}
#[doc = "Field `WWDT_RST` writer - Watchdog Timer reset control."]
pub type WwdtRstW<'a, REG> = crate::BitWriter<'a, REG, WwdtRst>;
impl<'a, REG> WwdtRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRst::Asserted)
    }
}
#[doc = "Real Time Clock (RTC) reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<RtcRst> for bool {
    #[inline(always)]
    fn from(variant: RtcRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_RST` reader - Real Time Clock (RTC) reset control."]
pub type RtcRstR = crate::BitReader<RtcRst>;
impl RtcRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcRst {
        match self.bits {
            false => RtcRst::Released,
            true => RtcRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RtcRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RtcRst::Asserted
    }
}
#[doc = "Field `RTC_RST` writer - Real Time Clock (RTC) reset control."]
pub type RtcRstW<'a, REG> = crate::BitWriter<'a, REG, RtcRst>;
impl<'a, REG> RtcRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRst::Asserted)
    }
}
#[doc = "Inter CPU communication Mailbox reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MailboxRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<MailboxRst> for bool {
    #[inline(always)]
    fn from(variant: MailboxRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX_RST` reader - Inter CPU communication Mailbox reset control."]
pub type MailboxRstR = crate::BitReader<MailboxRst>;
impl MailboxRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MailboxRst {
        match self.bits {
            false => MailboxRst::Released,
            true => MailboxRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MailboxRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MailboxRst::Asserted
    }
}
#[doc = "Field `MAILBOX_RST` writer - Inter CPU communication Mailbox reset control."]
pub type MailboxRstW<'a, REG> = crate::BitWriter<'a, REG, MailboxRst>;
impl<'a, REG> MailboxRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRst::Asserted)
    }
}
#[doc = "ADC reset control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcRst {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1: Bloc is reset."]
    Asserted = 1,
}
impl From<AdcRst> for bool {
    #[inline(always)]
    fn from(variant: AdcRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_RST` reader - ADC reset control."]
pub type AdcRstR = crate::BitReader<AdcRst>;
impl AdcRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcRst {
        match self.bits {
            false => AdcRst::Released,
            true => AdcRst::Asserted,
        }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == AdcRst::Released
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == AdcRst::Asserted
    }
}
#[doc = "Field `ADC_RST` writer - ADC reset control."]
pub type AdcRstW<'a, REG> = crate::BitWriter<'a, REG, AdcRst>;
impl<'a, REG> AdcRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRst::Released)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRst::Asserted)
    }
}
impl R {
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&self) -> RomRstR {
        RomRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&self) -> SramCtrl1RstR {
        SramCtrl1RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&self) -> SramCtrl2RstR {
        SramCtrl2RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FlashRstR {
        FlashRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FmcRstR {
        FmcRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MuxRstR {
        MuxRstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IoconRstR {
        IoconRstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> Gpio0RstR {
        Gpio0RstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> Gpio1RstR {
        Gpio1RstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PintRstR {
        PintRstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GintRstR {
        GintRstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> Dma0RstR {
        Dma0RstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&self) -> CrcgenRstR {
        CrcgenRstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WwdtRstR {
        WwdtRstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&self) -> RtcRstR {
        RtcRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&self) -> MailboxRstR {
        MailboxRstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&self) -> AdcRstR {
        AdcRstR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRL_PRESETCTRL0")
            .field("rom_rst", &self.rom_rst())
            .field("sram_ctrl1_rst", &self.sram_ctrl1_rst())
            .field("sram_ctrl2_rst", &self.sram_ctrl2_rst())
            .field("flash_rst", &self.flash_rst())
            .field("fmc_rst", &self.fmc_rst())
            .field("mux_rst", &self.mux_rst())
            .field("iocon_rst", &self.iocon_rst())
            .field("gpio0_rst", &self.gpio0_rst())
            .field("gpio1_rst", &self.gpio1_rst())
            .field("pint_rst", &self.pint_rst())
            .field("gint_rst", &self.gint_rst())
            .field("dma0_rst", &self.dma0_rst())
            .field("crcgen_rst", &self.crcgen_rst())
            .field("wwdt_rst", &self.wwdt_rst())
            .field("rtc_rst", &self.rtc_rst())
            .field("mailbox_rst", &self.mailbox_rst())
            .field("adc_rst", &self.adc_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&mut self) -> RomRstW<'_, PresetctrlPresetctrl0Spec> {
        RomRstW::new(self, 1)
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&mut self) -> SramCtrl1RstW<'_, PresetctrlPresetctrl0Spec> {
        SramCtrl1RstW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&mut self) -> SramCtrl2RstW<'_, PresetctrlPresetctrl0Spec> {
        SramCtrl2RstW::new(self, 4)
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> FlashRstW<'_, PresetctrlPresetctrl0Spec> {
        FlashRstW::new(self, 7)
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> FmcRstW<'_, PresetctrlPresetctrl0Spec> {
        FmcRstW::new(self, 8)
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> MuxRstW<'_, PresetctrlPresetctrl0Spec> {
        MuxRstW::new(self, 11)
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> IoconRstW<'_, PresetctrlPresetctrl0Spec> {
        IoconRstW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> Gpio0RstW<'_, PresetctrlPresetctrl0Spec> {
        Gpio0RstW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> Gpio1RstW<'_, PresetctrlPresetctrl0Spec> {
        Gpio1RstW::new(self, 15)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> PintRstW<'_, PresetctrlPresetctrl0Spec> {
        PintRstW::new(self, 18)
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> GintRstW<'_, PresetctrlPresetctrl0Spec> {
        GintRstW::new(self, 19)
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> Dma0RstW<'_, PresetctrlPresetctrl0Spec> {
        Dma0RstW::new(self, 20)
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&mut self) -> CrcgenRstW<'_, PresetctrlPresetctrl0Spec> {
        CrcgenRstW::new(self, 21)
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> WwdtRstW<'_, PresetctrlPresetctrl0Spec> {
        WwdtRstW::new(self, 22)
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&mut self) -> RtcRstW<'_, PresetctrlPresetctrl0Spec> {
        RtcRstW::new(self, 23)
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&mut self) -> MailboxRstW<'_, PresetctrlPresetctrl0Spec> {
        MailboxRstW::new(self, 26)
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&mut self) -> AdcRstW<'_, PresetctrlPresetctrl0Spec> {
        AdcRstW::new(self, 27)
    }
}
#[doc = "Peripheral reset control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetctrlPresetctrl0Spec;
impl crate::RegisterSpec for PresetctrlPresetctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presetctrl_presetctrl0::R`](R) reader structure"]
impl crate::Readable for PresetctrlPresetctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`presetctrl_presetctrl0::W`](W) writer structure"]
impl crate::Writable for PresetctrlPresetctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESETCTRL0 to value 0"]
impl crate::Resettable for PresetctrlPresetctrl0Spec {}

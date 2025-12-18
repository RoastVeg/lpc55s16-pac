#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<ClockCtrlSpec>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<ClockCtrlSpec>;
#[doc = "Enable XTAL32MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtal32mhzFreqmEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<Xtal32mhzFreqmEna> for bool {
    #[inline(always)]
    fn from(variant: Xtal32mhzFreqmEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTAL32MHZ_FREQM_ENA` reader - Enable XTAL32MHz clock for Frequency Measure module."]
pub type Xtal32mhzFreqmEnaR = crate::BitReader<Xtal32mhzFreqmEna>;
impl Xtal32mhzFreqmEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtal32mhzFreqmEna {
        match self.bits {
            false => Xtal32mhzFreqmEna::Disable,
            true => Xtal32mhzFreqmEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Xtal32mhzFreqmEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Xtal32mhzFreqmEna::Enable
    }
}
#[doc = "Field `XTAL32MHZ_FREQM_ENA` writer - Enable XTAL32MHz clock for Frequency Measure module."]
pub type Xtal32mhzFreqmEnaW<'a, REG> = crate::BitWriter<'a, REG, Xtal32mhzFreqmEna>;
impl<'a, REG> Xtal32mhzFreqmEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Xtal32mhzFreqmEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Xtal32mhzFreqmEna::Enable)
    }
}
#[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fro1mhzUtickEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<Fro1mhzUtickEna> for bool {
    #[inline(always)]
    fn from(variant: Fro1mhzUtickEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO1MHZ_UTICK_ENA` reader - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub type Fro1mhzUtickEnaR = crate::BitReader<Fro1mhzUtickEna>;
impl Fro1mhzUtickEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fro1mhzUtickEna {
        match self.bits {
            false => Fro1mhzUtickEna::Disable,
            true => Fro1mhzUtickEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fro1mhzUtickEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fro1mhzUtickEna::Enable
    }
}
#[doc = "Field `FRO1MHZ_UTICK_ENA` writer - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub type Fro1mhzUtickEnaW<'a, REG> = crate::BitWriter<'a, REG, Fro1mhzUtickEna>;
impl<'a, REG> Fro1mhzUtickEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro1mhzUtickEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro1mhzUtickEna::Enable)
    }
}
#[doc = "Enable FRO 12MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fro12mhzFreqmEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<Fro12mhzFreqmEna> for bool {
    #[inline(always)]
    fn from(variant: Fro12mhzFreqmEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO12MHZ_FREQM_ENA` reader - Enable FRO 12MHz clock for Frequency Measure module."]
pub type Fro12mhzFreqmEnaR = crate::BitReader<Fro12mhzFreqmEna>;
impl Fro12mhzFreqmEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fro12mhzFreqmEna {
        match self.bits {
            false => Fro12mhzFreqmEna::Disable,
            true => Fro12mhzFreqmEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fro12mhzFreqmEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fro12mhzFreqmEna::Enable
    }
}
#[doc = "Field `FRO12MHZ_FREQM_ENA` writer - Enable FRO 12MHz clock for Frequency Measure module."]
pub type Fro12mhzFreqmEnaW<'a, REG> = crate::BitWriter<'a, REG, Fro12mhzFreqmEna>;
impl<'a, REG> Fro12mhzFreqmEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro12mhzFreqmEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro12mhzFreqmEna::Enable)
    }
}
#[doc = "Enable FRO 96MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FroHfFreqmEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<FroHfFreqmEna> for bool {
    #[inline(always)]
    fn from(variant: FroHfFreqmEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO_HF_FREQM_ENA` reader - Enable FRO 96MHz clock for Frequency Measure module."]
pub type FroHfFreqmEnaR = crate::BitReader<FroHfFreqmEna>;
impl FroHfFreqmEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FroHfFreqmEna {
        match self.bits {
            false => FroHfFreqmEna::Disable,
            true => FroHfFreqmEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FroHfFreqmEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FroHfFreqmEna::Enable
    }
}
#[doc = "Field `FRO_HF_FREQM_ENA` writer - Enable FRO 96MHz clock for Frequency Measure module."]
pub type FroHfFreqmEnaW<'a, REG> = crate::BitWriter<'a, REG, FroHfFreqmEna>;
impl<'a, REG> FroHfFreqmEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FroHfFreqmEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FroHfFreqmEna::Enable)
    }
}
#[doc = "Enable clock_in clock for clock module.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkinEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<ClkinEna> for bool {
    #[inline(always)]
    fn from(variant: ClkinEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKIN_ENA` reader - Enable clock_in clock for clock module."]
pub type ClkinEnaR = crate::BitReader<ClkinEna>;
impl ClkinEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkinEna {
        match self.bits {
            false => ClkinEna::Disable,
            true => ClkinEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ClkinEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ClkinEna::Enable
    }
}
#[doc = "Field `CLKIN_ENA` writer - Enable clock_in clock for clock module."]
pub type ClkinEnaW<'a, REG> = crate::BitWriter<'a, REG, ClkinEna>;
impl<'a, REG> ClkinEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkinEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkinEna::Enable)
    }
}
#[doc = "Enable FRO 1MHz clock for clock muxing in clock gen.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fro1mhzClkEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<Fro1mhzClkEna> for bool {
    #[inline(always)]
    fn from(variant: Fro1mhzClkEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO1MHZ_CLK_ENA` reader - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub type Fro1mhzClkEnaR = crate::BitReader<Fro1mhzClkEna>;
impl Fro1mhzClkEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fro1mhzClkEna {
        match self.bits {
            false => Fro1mhzClkEna::Disable,
            true => Fro1mhzClkEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fro1mhzClkEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fro1mhzClkEna::Enable
    }
}
#[doc = "Field `FRO1MHZ_CLK_ENA` writer - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub type Fro1mhzClkEnaW<'a, REG> = crate::BitWriter<'a, REG, Fro1mhzClkEna>;
impl<'a, REG> Fro1mhzClkEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro1mhzClkEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fro1mhzClkEna::Enable)
    }
}
#[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnaFro12mClkEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<AnaFro12mClkEna> for bool {
    #[inline(always)]
    fn from(variant: AnaFro12mClkEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANA_FRO12M_CLK_ENA` reader - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub type AnaFro12mClkEnaR = crate::BitReader<AnaFro12mClkEna>;
impl AnaFro12mClkEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnaFro12mClkEna {
        match self.bits {
            false => AnaFro12mClkEna::Disable,
            true => AnaFro12mClkEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AnaFro12mClkEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AnaFro12mClkEna::Enable
    }
}
#[doc = "Field `ANA_FRO12M_CLK_ENA` writer - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub type AnaFro12mClkEnaW<'a, REG> = crate::BitWriter<'a, REG, AnaFro12mClkEna>;
impl<'a, REG> AnaFro12mClkEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AnaFro12mClkEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AnaFro12mClkEna::Enable)
    }
}
#[doc = "Enable clock for cristal oscilator calibration.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XoCalClkEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<XoCalClkEna> for bool {
    #[inline(always)]
    fn from(variant: XoCalClkEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XO_CAL_CLK_ENA` reader - Enable clock for cristal oscilator calibration."]
pub type XoCalClkEnaR = crate::BitReader<XoCalClkEna>;
impl XoCalClkEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XoCalClkEna {
        match self.bits {
            false => XoCalClkEna::Disable,
            true => XoCalClkEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XoCalClkEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XoCalClkEna::Enable
    }
}
#[doc = "Field `XO_CAL_CLK_ENA` writer - Enable clock for cristal oscilator calibration."]
pub type XoCalClkEnaW<'a, REG> = crate::BitWriter<'a, REG, XoCalClkEna>;
impl<'a, REG> XoCalClkEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(XoCalClkEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(XoCalClkEna::Enable)
    }
}
#[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluDeglitchClkEna {
    #[doc = "0: The clock is not enabled."]
    Disable = 0,
    #[doc = "1: The clock is enabled."]
    Enable = 1,
}
impl From<PluDeglitchClkEna> for bool {
    #[inline(always)]
    fn from(variant: PluDeglitchClkEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` reader - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub type PluDeglitchClkEnaR = crate::BitReader<PluDeglitchClkEna>;
impl PluDeglitchClkEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PluDeglitchClkEna {
        match self.bits {
            false => PluDeglitchClkEna::Disable,
            true => PluDeglitchClkEna::Enable,
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PluDeglitchClkEna::Disable
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PluDeglitchClkEna::Enable
    }
}
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` writer - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub type PluDeglitchClkEnaW<'a, REG> = crate::BitWriter<'a, REG, PluDeglitchClkEna>;
impl<'a, REG> PluDeglitchClkEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PluDeglitchClkEna::Disable)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PluDeglitchClkEna::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn xtal32mhz_freqm_ena(&self) -> Xtal32mhzFreqmEnaR {
        Xtal32mhzFreqmEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub fn fro1mhz_utick_ena(&self) -> Fro1mhzUtickEnaR {
        Fro1mhzUtickEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro12mhz_freqm_ena(&self) -> Fro12mhzFreqmEnaR {
        Fro12mhzFreqmEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro_hf_freqm_ena(&self) -> FroHfFreqmEnaR {
        FroHfFreqmEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    pub fn clkin_ena(&self) -> ClkinEnaR {
        ClkinEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub fn fro1mhz_clk_ena(&self) -> Fro1mhzClkEnaR {
        Fro1mhzClkEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub fn ana_fro12m_clk_ena(&self) -> AnaFro12mClkEnaR {
        AnaFro12mClkEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub fn xo_cal_clk_ena(&self) -> XoCalClkEnaR {
        XoCalClkEnaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub fn plu_deglitch_clk_ena(&self) -> PluDeglitchClkEnaR {
        PluDeglitchClkEnaR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_CTRL")
            .field("xtal32mhz_freqm_ena", &self.xtal32mhz_freqm_ena())
            .field("fro1mhz_utick_ena", &self.fro1mhz_utick_ena())
            .field("fro12mhz_freqm_ena", &self.fro12mhz_freqm_ena())
            .field("fro_hf_freqm_ena", &self.fro_hf_freqm_ena())
            .field("clkin_ena", &self.clkin_ena())
            .field("fro1mhz_clk_ena", &self.fro1mhz_clk_ena())
            .field("ana_fro12m_clk_ena", &self.ana_fro12m_clk_ena())
            .field("xo_cal_clk_ena", &self.xo_cal_clk_ena())
            .field("plu_deglitch_clk_ena", &self.plu_deglitch_clk_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn xtal32mhz_freqm_ena(&mut self) -> Xtal32mhzFreqmEnaW<'_, ClockCtrlSpec> {
        Xtal32mhzFreqmEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub fn fro1mhz_utick_ena(&mut self) -> Fro1mhzUtickEnaW<'_, ClockCtrlSpec> {
        Fro1mhzUtickEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro12mhz_freqm_ena(&mut self) -> Fro12mhzFreqmEnaW<'_, ClockCtrlSpec> {
        Fro12mhzFreqmEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro_hf_freqm_ena(&mut self) -> FroHfFreqmEnaW<'_, ClockCtrlSpec> {
        FroHfFreqmEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    pub fn clkin_ena(&mut self) -> ClkinEnaW<'_, ClockCtrlSpec> {
        ClkinEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub fn fro1mhz_clk_ena(&mut self) -> Fro1mhzClkEnaW<'_, ClockCtrlSpec> {
        Fro1mhzClkEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub fn ana_fro12m_clk_ena(&mut self) -> AnaFro12mClkEnaW<'_, ClockCtrlSpec> {
        AnaFro12mClkEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub fn xo_cal_clk_ena(&mut self) -> XoCalClkEnaW<'_, ClockCtrlSpec> {
        XoCalClkEnaW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub fn plu_deglitch_clk_ena(&mut self) -> PluDeglitchClkEnaW<'_, ClockCtrlSpec> {
        PluDeglitchClkEnaW::new(self, 9)
    }
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtrlSpec;
impl crate::RegisterSpec for ClockCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for ClockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for ClockCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0x01"]
impl crate::Resettable for ClockCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}

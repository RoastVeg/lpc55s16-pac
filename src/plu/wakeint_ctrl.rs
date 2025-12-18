#[doc = "Register `WAKEINT_CTRL` reader"]
pub type R = crate::R<WakeintCtrlSpec>;
#[doc = "Register `WAKEINT_CTRL` writer"]
pub type W = crate::W<WakeintCtrlSpec>;
#[doc = "Field `MASK` reader - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "control input of the PLU, add filtering for glitch.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterMode {
    #[doc = "0: Bypass mode."]
    Bypass = 0,
    #[doc = "1: Filter 1 clock period."]
    Filter1clk = 1,
    #[doc = "2: Filter 2 clock period."]
    Filter2clk = 2,
    #[doc = "3: Filter 3 clock period."]
    Filter3clk = 3,
}
impl From<FilterMode> for u8 {
    #[inline(always)]
    fn from(variant: FilterMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FilterMode {
    type Ux = u8;
}
impl crate::IsEnum for FilterMode {}
#[doc = "Field `FILTER_MODE` reader - control input of the PLU, add filtering for glitch."]
pub type FilterModeR = crate::FieldReader<FilterMode>;
impl FilterModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FilterMode {
        match self.bits {
            0 => FilterMode::Bypass,
            1 => FilterMode::Filter1clk,
            2 => FilterMode::Filter2clk,
            3 => FilterMode::Filter3clk,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == FilterMode::Bypass
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        *self == FilterMode::Filter1clk
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        *self == FilterMode::Filter2clk
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        *self == FilterMode::Filter3clk
    }
}
#[doc = "Field `FILTER_MODE` writer - control input of the PLU, add filtering for glitch."]
pub type FilterModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, FilterMode, crate::Safe>;
impl<'a, REG> FilterModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(FilterMode::Bypass)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut crate::W<REG> {
        self.variant(FilterMode::Filter1clk)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut crate::W<REG> {
        self.variant(FilterMode::Filter2clk)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut crate::W<REG> {
        self.variant(FilterMode::Filter3clk)
    }
}
#[doc = "hclk is divided by 2**filter_clksel.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterClksel {
    #[doc = "0: Selects the 1 MHz low-power oscillator as the filter clock."]
    Fro1mhz = 0,
    #[doc = "1: Selects the 12 Mhz FRO as the filter clock."]
    Fro12mhz = 1,
    #[doc = "2: Selects a third filter clock source, if provided."]
    OtherClock = 2,
}
impl From<FilterClksel> for u8 {
    #[inline(always)]
    fn from(variant: FilterClksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FilterClksel {
    type Ux = u8;
}
impl crate::IsEnum for FilterClksel {}
#[doc = "Field `FILTER_CLKSEL` reader - hclk is divided by 2**filter_clksel."]
pub type FilterClkselR = crate::FieldReader<FilterClksel>;
impl FilterClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FilterClksel> {
        match self.bits {
            0 => Some(FilterClksel::Fro1mhz),
            1 => Some(FilterClksel::Fro12mhz),
            2 => Some(FilterClksel::OtherClock),
            _ => None,
        }
    }
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    #[inline(always)]
    pub fn is_fro1mhz(&self) -> bool {
        *self == FilterClksel::Fro1mhz
    }
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    #[inline(always)]
    pub fn is_fro12mhz(&self) -> bool {
        *self == FilterClksel::Fro12mhz
    }
    #[doc = "Selects a third filter clock source, if provided."]
    #[inline(always)]
    pub fn is_other_clock(&self) -> bool {
        *self == FilterClksel::OtherClock
    }
}
#[doc = "Field `FILTER_CLKSEL` writer - hclk is divided by 2**filter_clksel."]
pub type FilterClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, FilterClksel>;
impl<'a, REG> FilterClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    #[inline(always)]
    pub fn fro1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(FilterClksel::Fro1mhz)
    }
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    #[inline(always)]
    pub fn fro12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(FilterClksel::Fro12mhz)
    }
    #[doc = "Selects a third filter clock source, if provided."]
    #[inline(always)]
    pub fn other_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FilterClksel::OtherClock)
    }
}
#[doc = "Field `LATCH_ENABLE` reader - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub type LatchEnableR = crate::BitReader;
#[doc = "Field `LATCH_ENABLE` writer - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub type LatchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLEAR` reader - Write to clear wakeint_latched"]
pub type IntrClearR = crate::BitReader;
#[doc = "Field `INTR_CLEAR` writer - Write to clear wakeint_latched"]
pub type IntrClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub fn filter_mode(&self) -> FilterModeR {
        FilterModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub fn filter_clksel(&self) -> FilterClkselR {
        FilterClkselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&self) -> LatchEnableR {
        LatchEnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&self) -> IntrClearR {
        IntrClearR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEINT_CTRL")
            .field("mask", &self.mask())
            .field("filter_mode", &self.filter_mode())
            .field("filter_clksel", &self.filter_clksel())
            .field("latch_enable", &self.latch_enable())
            .field("intr_clear", &self.intr_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, WakeintCtrlSpec> {
        MaskW::new(self, 0)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub fn filter_mode(&mut self) -> FilterModeW<'_, WakeintCtrlSpec> {
        FilterModeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub fn filter_clksel(&mut self) -> FilterClkselW<'_, WakeintCtrlSpec> {
        FilterClkselW::new(self, 10)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&mut self) -> LatchEnableW<'_, WakeintCtrlSpec> {
        LatchEnableW::new(self, 12)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&mut self) -> IntrClearW<'_, WakeintCtrlSpec> {
        IntrClearW::new(self, 13)
    }
}
#[doc = "Wakeup interrupt control for PLU\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeint_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeint_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeintCtrlSpec;
impl crate::RegisterSpec for WakeintCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeint_ctrl::R`](R) reader structure"]
impl crate::Readable for WakeintCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeint_ctrl::W`](W) writer structure"]
impl crate::Writable for WakeintCtrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2000;
}
#[doc = "`reset()` method sets WAKEINT_CTRL to value 0"]
impl crate::Resettable for WakeintCtrlSpec {}

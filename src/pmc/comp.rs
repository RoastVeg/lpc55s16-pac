#[doc = "Register `COMP` reader"]
pub type R = crate::R<CompSpec>;
#[doc = "Register `COMP` writer"]
pub type W = crate::W<CompSpec>;
#[doc = "Hysteris when hyst = '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hyst {
    #[doc = "0: Hysteresis is disable."]
    Disable = 0,
    #[doc = "1: Hysteresis is enable."]
    Enable = 1,
}
impl From<Hyst> for bool {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST` reader - Hysteris when hyst = '1'."]
pub type HystR = crate::BitReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            false => Hyst::Disable,
            true => Hyst::Enable,
        }
    }
    #[doc = "Hysteresis is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hyst::Disable
    }
    #[doc = "Hysteresis is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hyst::Enable
    }
}
#[doc = "Field `HYST` writer - Hysteris when hyst = '1'."]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG, Hyst>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hysteresis is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Disable)
    }
    #[doc = "Hysteresis is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Enable)
    }
}
#[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrefinput {
    #[doc = "0: Select internal VREF."]
    Internalref = 0,
    #[doc = "1: Select VDDA."]
    Vdda = 1,
}
impl From<Vrefinput> for bool {
    #[inline(always)]
    fn from(variant: Vrefinput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFINPUT` reader - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub type VrefinputR = crate::BitReader<Vrefinput>;
impl VrefinputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefinput {
        match self.bits {
            false => Vrefinput::Internalref,
            true => Vrefinput::Vdda,
        }
    }
    #[doc = "Select internal VREF."]
    #[inline(always)]
    pub fn is_internalref(&self) -> bool {
        *self == Vrefinput::Internalref
    }
    #[doc = "Select VDDA."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Vrefinput::Vdda
    }
}
#[doc = "Field `VREFINPUT` writer - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub type VrefinputW<'a, REG> = crate::BitWriter<'a, REG, Vrefinput>;
impl<'a, REG> VrefinputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select internal VREF."]
    #[inline(always)]
    pub fn internalref(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefinput::Internalref)
    }
    #[doc = "Select VDDA."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefinput::Vdda)
    }
}
#[doc = "Low power mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowpower {
    #[doc = "0: High speed mode."]
    Highspeed = 0,
    #[doc = "1: Low power mode (Low speed)."]
    Lowspeed = 1,
}
impl From<Lowpower> for bool {
    #[inline(always)]
    fn from(variant: Lowpower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWPOWER` reader - Low power mode."]
pub type LowpowerR = crate::BitReader<Lowpower>;
impl LowpowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowpower {
        match self.bits {
            false => Lowpower::Highspeed,
            true => Lowpower::Lowspeed,
        }
    }
    #[doc = "High speed mode."]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == Lowpower::Highspeed
    }
    #[doc = "Low power mode (Low speed)."]
    #[inline(always)]
    pub fn is_lowspeed(&self) -> bool {
        *self == Lowpower::Lowspeed
    }
}
#[doc = "Field `LOWPOWER` writer - Low power mode."]
pub type LowpowerW<'a, REG> = crate::BitWriter<'a, REG, Lowpower>;
impl<'a, REG> LowpowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed mode."]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut crate::W<REG> {
        self.variant(Lowpower::Highspeed)
    }
    #[doc = "Low power mode (Low speed)."]
    #[inline(always)]
    pub fn lowspeed(self) -> &'a mut crate::W<REG> {
        self.variant(Lowpower::Lowspeed)
    }
}
#[doc = "Control word for P multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmux {
    #[doc = "0: VREF (See fiedl VREFINPUT)."]
    Vref = 0,
    #[doc = "1: Pin P0_0."]
    Cmp0A = 1,
    #[doc = "2: Pin P0_9."]
    Cmp0B = 2,
    #[doc = "3: Pin P0_18."]
    Cmp0C = 3,
    #[doc = "4: Pin P1_14."]
    Cmp0D = 4,
    #[doc = "5: Pin P2_23."]
    Cmp0E = 5,
}
impl From<Pmux> for u8 {
    #[inline(always)]
    fn from(variant: Pmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmux {
    type Ux = u8;
}
impl crate::IsEnum for Pmux {}
#[doc = "Field `PMUX` reader - Control word for P multiplexer:."]
pub type PmuxR = crate::FieldReader<Pmux>;
impl PmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmux> {
        match self.bits {
            0 => Some(Pmux::Vref),
            1 => Some(Pmux::Cmp0A),
            2 => Some(Pmux::Cmp0B),
            3 => Some(Pmux::Cmp0C),
            4 => Some(Pmux::Cmp0D),
            5 => Some(Pmux::Cmp0E),
            _ => None,
        }
    }
    #[doc = "VREF (See fiedl VREFINPUT)."]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Pmux::Vref
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        *self == Pmux::Cmp0A
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        *self == Pmux::Cmp0B
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        *self == Pmux::Cmp0C
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        *self == Pmux::Cmp0D
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        *self == Pmux::Cmp0E
    }
}
#[doc = "Field `PMUX` writer - Control word for P multiplexer:."]
pub type PmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pmux>;
impl<'a, REG> PmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREF (See fiedl VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Vref)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Cmp0A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Cmp0B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Cmp0C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Cmp0D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmux::Cmp0E)
    }
}
#[doc = "Control word for N multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nmux {
    #[doc = "0: VREF (See field VREFINPUT)."]
    Vref = 0,
    #[doc = "1: Pin P0_0."]
    Cmp0A = 1,
    #[doc = "2: Pin P0_9."]
    Cmp0B = 2,
    #[doc = "3: Pin P0_18."]
    Cmp0C = 3,
    #[doc = "4: Pin P1_14."]
    Cmp0D = 4,
    #[doc = "5: Pin P2_23."]
    Cmp0E = 5,
}
impl From<Nmux> for u8 {
    #[inline(always)]
    fn from(variant: Nmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nmux {
    type Ux = u8;
}
impl crate::IsEnum for Nmux {}
#[doc = "Field `NMUX` reader - Control word for N multiplexer:."]
pub type NmuxR = crate::FieldReader<Nmux>;
impl NmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nmux> {
        match self.bits {
            0 => Some(Nmux::Vref),
            1 => Some(Nmux::Cmp0A),
            2 => Some(Nmux::Cmp0B),
            3 => Some(Nmux::Cmp0C),
            4 => Some(Nmux::Cmp0D),
            5 => Some(Nmux::Cmp0E),
            _ => None,
        }
    }
    #[doc = "VREF (See field VREFINPUT)."]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Nmux::Vref
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        *self == Nmux::Cmp0A
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        *self == Nmux::Cmp0B
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        *self == Nmux::Cmp0C
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        *self == Nmux::Cmp0D
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        *self == Nmux::Cmp0E
    }
}
#[doc = "Field `NMUX` writer - Control word for N multiplexer:."]
pub type NmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nmux>;
impl<'a, REG> NmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREF (See field VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Vref)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Cmp0A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Cmp0B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Cmp0C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Cmp0D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut crate::W<REG> {
        self.variant(Nmux::Cmp0E)
    }
}
#[doc = "Field `VREF` reader - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub type VrefR = crate::FieldReader;
#[doc = "Field `VREF` writer - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub type VrefW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Control the filtering of the Analog Comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FiltercgfSamplemode {
    #[doc = "0: Bypass mode."]
    Bypass = 0,
    #[doc = "1: Filter 1 clock period."]
    Filter1clk = 1,
    #[doc = "2: Filter 2 clock period."]
    Filter2clk = 2,
    #[doc = "3: Filter 3 clock period."]
    Filter3clk = 3,
}
impl From<FiltercgfSamplemode> for u8 {
    #[inline(always)]
    fn from(variant: FiltercgfSamplemode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FiltercgfSamplemode {
    type Ux = u8;
}
impl crate::IsEnum for FiltercgfSamplemode {}
#[doc = "Field `FILTERCGF_SAMPLEMODE` reader - Control the filtering of the Analog Comparator output."]
pub type FiltercgfSamplemodeR = crate::FieldReader<FiltercgfSamplemode>;
impl FiltercgfSamplemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FiltercgfSamplemode {
        match self.bits {
            0 => FiltercgfSamplemode::Bypass,
            1 => FiltercgfSamplemode::Filter1clk,
            2 => FiltercgfSamplemode::Filter2clk,
            3 => FiltercgfSamplemode::Filter3clk,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == FiltercgfSamplemode::Bypass
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        *self == FiltercgfSamplemode::Filter1clk
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        *self == FiltercgfSamplemode::Filter2clk
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        *self == FiltercgfSamplemode::Filter3clk
    }
}
#[doc = "Field `FILTERCGF_SAMPLEMODE` writer - Control the filtering of the Analog Comparator output."]
pub type FiltercgfSamplemodeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, FiltercgfSamplemode, crate::Safe>;
impl<'a, REG> FiltercgfSamplemodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfSamplemode::Bypass)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfSamplemode::Filter1clk)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfSamplemode::Filter2clk)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfSamplemode::Filter3clk)
    }
}
#[doc = "Filter Clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FiltercgfClkdiv {
    #[doc = "0: Filter clock period duration equals 1 Analog Comparator clock period."]
    Filter1clkPeriod = 0,
    #[doc = "1: Filter clock period duration equals 2 Analog Comparator clock period."]
    Filter2clkPeriod = 1,
    #[doc = "2: Filter clock period duration equals 4 Analog Comparator clock period."]
    Filter4clkPeriod = 2,
    #[doc = "3: Filter clock period duration equals 8 Analog Comparator clock period."]
    Filter8clkPeriod = 3,
    #[doc = "4: Filter clock period duration equals 16 Analog Comparator clock period."]
    Filter16clkPeriod = 4,
    #[doc = "5: Filter clock period duration equals 32 Analog Comparator clock period."]
    Filter32clkPeriod = 5,
    #[doc = "6: Filter clock period duration equals 64 Analog Comparator clock period."]
    Filter64clkPeriod = 6,
    #[doc = "7: Filter clock period duration equals 128 Analog Comparator clock period."]
    Filter128clkPeriod = 7,
}
impl From<FiltercgfClkdiv> for u8 {
    #[inline(always)]
    fn from(variant: FiltercgfClkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FiltercgfClkdiv {
    type Ux = u8;
}
impl crate::IsEnum for FiltercgfClkdiv {}
#[doc = "Field `FILTERCGF_CLKDIV` reader - Filter Clock divider."]
pub type FiltercgfClkdivR = crate::FieldReader<FiltercgfClkdiv>;
impl FiltercgfClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FiltercgfClkdiv {
        match self.bits {
            0 => FiltercgfClkdiv::Filter1clkPeriod,
            1 => FiltercgfClkdiv::Filter2clkPeriod,
            2 => FiltercgfClkdiv::Filter4clkPeriod,
            3 => FiltercgfClkdiv::Filter8clkPeriod,
            4 => FiltercgfClkdiv::Filter16clkPeriod,
            5 => FiltercgfClkdiv::Filter32clkPeriod,
            6 => FiltercgfClkdiv::Filter64clkPeriod,
            7 => FiltercgfClkdiv::Filter128clkPeriod,
            _ => unreachable!(),
        }
    }
    #[doc = "Filter clock period duration equals 1 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_1clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter1clkPeriod
    }
    #[doc = "Filter clock period duration equals 2 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_2clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter2clkPeriod
    }
    #[doc = "Filter clock period duration equals 4 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_4clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter4clkPeriod
    }
    #[doc = "Filter clock period duration equals 8 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_8clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter8clkPeriod
    }
    #[doc = "Filter clock period duration equals 16 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_16clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter16clkPeriod
    }
    #[doc = "Filter clock period duration equals 32 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_32clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter32clkPeriod
    }
    #[doc = "Filter clock period duration equals 64 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_64clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter64clkPeriod
    }
    #[doc = "Filter clock period duration equals 128 Analog Comparator clock period."]
    #[inline(always)]
    pub fn is_filter_128clk_period(&self) -> bool {
        *self == FiltercgfClkdiv::Filter128clkPeriod
    }
}
#[doc = "Field `FILTERCGF_CLKDIV` writer - Filter Clock divider."]
pub type FiltercgfClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, FiltercgfClkdiv, crate::Safe>;
impl<'a, REG> FiltercgfClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter clock period duration equals 1 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_1clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter1clkPeriod)
    }
    #[doc = "Filter clock period duration equals 2 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_2clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter2clkPeriod)
    }
    #[doc = "Filter clock period duration equals 4 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_4clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter4clkPeriod)
    }
    #[doc = "Filter clock period duration equals 8 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_8clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter8clkPeriod)
    }
    #[doc = "Filter clock period duration equals 16 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_16clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter16clkPeriod)
    }
    #[doc = "Filter clock period duration equals 32 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_32clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter32clkPeriod)
    }
    #[doc = "Filter clock period duration equals 64 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_64clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter64clkPeriod)
    }
    #[doc = "Filter clock period duration equals 128 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_128clk_period(self) -> &'a mut crate::W<REG> {
        self.variant(FiltercgfClkdiv::Filter128clkPeriod)
    }
}
impl R {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&self) -> VrefinputR {
        VrefinputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&self) -> LowpowerR {
        LowpowerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&self) -> PmuxR {
        PmuxR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&self) -> NmuxR {
        NmuxR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&self) -> VrefR {
        VrefR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&self) -> FiltercgfSamplemodeR {
        FiltercgfSamplemodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Filter Clock divider."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&self) -> FiltercgfClkdivR {
        FiltercgfClkdivR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CompSpec> {
        HystW::new(self, 1)
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&mut self) -> VrefinputW<'_, CompSpec> {
        VrefinputW::new(self, 2)
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LowpowerW<'_, CompSpec> {
        LowpowerW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&mut self) -> PmuxW<'_, CompSpec> {
        PmuxW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&mut self) -> NmuxW<'_, CompSpec> {
        NmuxW::new(self, 7)
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&mut self) -> VrefW<'_, CompSpec> {
        VrefW::new(self, 10)
    }
    #[doc = "Bits 16:17 - Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&mut self) -> FiltercgfSamplemodeW<'_, CompSpec> {
        FiltercgfSamplemodeW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Filter Clock divider."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&mut self) -> FiltercgfClkdivW<'_, CompSpec> {
        FiltercgfClkdivW::new(self, 18)
    }
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompSpec;
impl crate::RegisterSpec for CompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp::R`](R) reader structure"]
impl crate::Readable for CompSpec {}
#[doc = "`write(|w| ..)` method takes [`comp::W`](W) writer structure"]
impl crate::Writable for CompSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP to value 0x0a"]
impl crate::Resettable for CompSpec {
    const RESET_VALUE: u32 = 0x0a;
}

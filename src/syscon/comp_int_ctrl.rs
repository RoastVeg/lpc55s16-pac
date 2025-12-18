#[doc = "Register `COMP_INT_CTRL` reader"]
pub type R = crate::R<CompIntCtrlSpec>;
#[doc = "Register `COMP_INT_CTRL` writer"]
pub type W = crate::W<CompIntCtrlSpec>;
#[doc = "Analog Comparator interrupt enable control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnable {
    #[doc = "0: interrupt disable."]
    IntDisable = 0,
    #[doc = "1: interrupt enable."]
    IntEnable = 1,
}
impl From<IntEnable> for bool {
    #[inline(always)]
    fn from(variant: IntEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_ENABLE` reader - Analog Comparator interrupt enable control:."]
pub type IntEnableR = crate::BitReader<IntEnable>;
impl IntEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnable {
        match self.bits {
            false => IntEnable::IntDisable,
            true => IntEnable::IntEnable,
        }
    }
    #[doc = "interrupt disable."]
    #[inline(always)]
    pub fn is_int_disable(&self) -> bool {
        *self == IntEnable::IntDisable
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub fn is_int_enable(&self) -> bool {
        *self == IntEnable::IntEnable
    }
}
#[doc = "Field `INT_ENABLE` writer - Analog Comparator interrupt enable control:."]
pub type IntEnableW<'a, REG> = crate::BitWriter<'a, REG, IntEnable>;
impl<'a, REG> IntEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disable."]
    #[inline(always)]
    pub fn int_disable(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnable::IntDisable)
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub fn int_enable(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnable::IntEnable)
    }
}
#[doc = "Analog Comparator interrupt clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntClear {
    #[doc = "0: No effect."]
    None = 0,
    #[doc = "1: Clear the interrupt. Self-cleared bit."]
    Clear = 1,
}
impl From<IntClear> for bool {
    #[inline(always)]
    fn from(variant: IntClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_CLEAR` reader - Analog Comparator interrupt clear."]
pub type IntClearR = crate::BitReader<IntClear>;
impl IntClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntClear {
        match self.bits {
            false => IntClear::None,
            true => IntClear::Clear,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IntClear::None
    }
    #[doc = "Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IntClear::Clear
    }
}
#[doc = "Field `INT_CLEAR` writer - Analog Comparator interrupt clear."]
pub type IntClearW<'a, REG> = crate::BitWriter<'a, REG, IntClear>;
impl<'a, REG> IntClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(IntClear::None)
    }
    #[doc = "Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IntClear::Clear)
    }
}
#[doc = "Comparator interrupt type selector:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntCtrl {
    #[doc = "0: The analog comparator interrupt edge sensitive is disabled."]
    EdgeDisable = 0,
    #[doc = "1: The analog comparator interrupt level sensitive is disabled."]
    LvlDisable = 1,
    #[doc = "2: analog comparator interrupt is rising edge sensitive."]
    EdgeRising = 2,
    #[doc = "3: Analog Comparator interrupt is high level sensitive."]
    LvlHigh = 3,
    #[doc = "4: analog comparator interrupt is falling edge sensitive."]
    EdgeFalling = 4,
    #[doc = "5: Analog Comparator interrupt is low level sensitive."]
    LvlLow = 5,
    #[doc = "6: analog comparator interrupt is rising and falling edge sensitive."]
    EdgeBoth = 6,
    #[doc = "7: The analog comparator interrupt level sensitive is disabled."]
    LvlDis2 = 7,
}
impl From<IntCtrl> for u8 {
    #[inline(always)]
    fn from(variant: IntCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntCtrl {
    type Ux = u8;
}
impl crate::IsEnum for IntCtrl {}
#[doc = "Field `INT_CTRL` reader - Comparator interrupt type selector:."]
pub type IntCtrlR = crate::FieldReader<IntCtrl>;
impl IntCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntCtrl {
        match self.bits {
            0 => IntCtrl::EdgeDisable,
            1 => IntCtrl::LvlDisable,
            2 => IntCtrl::EdgeRising,
            3 => IntCtrl::LvlHigh,
            4 => IntCtrl::EdgeFalling,
            5 => IntCtrl::LvlLow,
            6 => IntCtrl::EdgeBoth,
            7 => IntCtrl::LvlDis2,
            _ => unreachable!(),
        }
    }
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    #[inline(always)]
    pub fn is_edge_disable(&self) -> bool {
        *self == IntCtrl::EdgeDisable
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn is_lvl_disable(&self) -> bool {
        *self == IntCtrl::LvlDisable
    }
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    #[inline(always)]
    pub fn is_edge_rising(&self) -> bool {
        *self == IntCtrl::EdgeRising
    }
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    #[inline(always)]
    pub fn is_lvl_high(&self) -> bool {
        *self == IntCtrl::LvlHigh
    }
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    #[inline(always)]
    pub fn is_edge_falling(&self) -> bool {
        *self == IntCtrl::EdgeFalling
    }
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    #[inline(always)]
    pub fn is_lvl_low(&self) -> bool {
        *self == IntCtrl::LvlLow
    }
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    #[inline(always)]
    pub fn is_edge_both(&self) -> bool {
        *self == IntCtrl::EdgeBoth
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn is_lvl_dis2(&self) -> bool {
        *self == IntCtrl::LvlDis2
    }
}
#[doc = "Field `INT_CTRL` writer - Comparator interrupt type selector:."]
pub type IntCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, IntCtrl, crate::Safe>;
impl<'a, REG> IntCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    #[inline(always)]
    pub fn edge_disable(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::EdgeDisable)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_disable(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::LvlDisable)
    }
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    #[inline(always)]
    pub fn edge_rising(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::EdgeRising)
    }
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    #[inline(always)]
    pub fn lvl_high(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::LvlHigh)
    }
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    #[inline(always)]
    pub fn edge_falling(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::EdgeFalling)
    }
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    #[inline(always)]
    pub fn lvl_low(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::LvlLow)
    }
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    #[inline(always)]
    pub fn edge_both(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::EdgeBoth)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_dis2(self) -> &'a mut crate::W<REG> {
        self.variant(IntCtrl::LvlDis2)
    }
}
#[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSource {
    #[doc = "0: Select Analog Comparator filtered output as input for interrupt detection."]
    FilterInt = 0,
    #[doc = "1: Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RawInt = 1,
}
impl From<IntSource> for bool {
    #[inline(always)]
    fn from(variant: IntSource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_SOURCE` reader - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
pub type IntSourceR = crate::BitReader<IntSource>;
impl IntSourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntSource {
        match self.bits {
            false => IntSource::FilterInt,
            true => IntSource::RawInt,
        }
    }
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    #[inline(always)]
    pub fn is_filter_int(&self) -> bool {
        *self == IntSource::FilterInt
    }
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    #[inline(always)]
    pub fn is_raw_int(&self) -> bool {
        *self == IntSource::RawInt
    }
}
#[doc = "Field `INT_SOURCE` writer - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
pub type IntSourceW<'a, REG> = crate::BitWriter<'a, REG, IntSource>;
impl<'a, REG> IntSourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    #[inline(always)]
    pub fn filter_int(self) -> &'a mut crate::W<REG> {
        self.variant(IntSource::FilterInt)
    }
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    #[inline(always)]
    pub fn raw_int(self) -> &'a mut crate::W<REG> {
        self.variant(IntSource::RawInt)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub fn int_enable(&self) -> IntEnableR {
        IntEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    pub fn int_clear(&self) -> IntClearR {
        IntClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    pub fn int_ctrl(&self) -> IntCtrlR {
        IntCtrlR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub fn int_source(&self) -> IntSourceR {
        IntSourceR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> IntEnableW<'_, CompIntCtrlSpec> {
        IntEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    pub fn int_clear(&mut self) -> IntClearW<'_, CompIntCtrlSpec> {
        IntClearW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    pub fn int_ctrl(&mut self) -> IntCtrlW<'_, CompIntCtrlSpec> {
        IntCtrlW::new(self, 2)
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub fn int_source(&mut self) -> IntSourceW<'_, CompIntCtrlSpec> {
        IntSourceW::new(self, 5)
    }
}
#[doc = "Comparator Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompIntCtrlSpec;
impl crate::RegisterSpec for CompIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_int_ctrl::R`](R) reader structure"]
impl crate::Readable for CompIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_int_ctrl::W`](W) writer structure"]
impl crate::Writable for CompIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_INT_CTRL to value 0"]
impl crate::Resettable for CompIntCtrlSpec {}

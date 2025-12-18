#[doc = "Register `DUMMY_CTRL` reader"]
pub type R = crate::R<DummyCtrlSpec>;
#[doc = "Register `DUMMY_CTRL` writer"]
pub type W = crate::W<DummyCtrlSpec>;
#[doc = "Control High speed Crystal oscillator mode of the ADC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xo32mAdcClkMode {
    #[doc = "0: High speed Crystal oscillator output to ADC is disabled."]
    Disable = 0,
    #[doc = "1: High speed Crystal oscillator output to ADC is enable."]
    XoAdcEnable = 1,
}
impl From<Xo32mAdcClkMode> for u8 {
    #[inline(always)]
    fn from(variant: Xo32mAdcClkMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xo32mAdcClkMode {
    type Ux = u8;
}
impl crate::IsEnum for Xo32mAdcClkMode {}
#[doc = "Field `XO32M_ADC_CLK_MODE` reader - Control High speed Crystal oscillator mode of the ADC clock."]
pub type Xo32mAdcClkModeR = crate::FieldReader<Xo32mAdcClkMode>;
impl Xo32mAdcClkModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xo32mAdcClkMode> {
        match self.bits {
            0 => Some(Xo32mAdcClkMode::Disable),
            1 => Some(Xo32mAdcClkMode::XoAdcEnable),
            _ => None,
        }
    }
    #[doc = "High speed Crystal oscillator output to ADC is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Xo32mAdcClkMode::Disable
    }
    #[doc = "High speed Crystal oscillator output to ADC is enable."]
    #[inline(always)]
    pub fn is_xo_adc_enable(&self) -> bool {
        *self == Xo32mAdcClkMode::XoAdcEnable
    }
}
#[doc = "Field `XO32M_ADC_CLK_MODE` writer - Control High speed Crystal oscillator mode of the ADC clock."]
pub type Xo32mAdcClkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xo32mAdcClkMode>;
impl<'a, REG> Xo32mAdcClkModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed Crystal oscillator output to ADC is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Xo32mAdcClkMode::Disable)
    }
    #[doc = "High speed Crystal oscillator output to ADC is enable."]
    #[inline(always)]
    pub fn xo_adc_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Xo32mAdcClkMode::XoAdcEnable)
    }
}
impl R {
    #[doc = "Bits 10:11 - Control High speed Crystal oscillator mode of the ADC clock."]
    #[inline(always)]
    pub fn xo32m_adc_clk_mode(&self) -> Xo32mAdcClkModeR {
        Xo32mAdcClkModeR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Control High speed Crystal oscillator mode of the ADC clock."]
    #[inline(always)]
    pub fn xo32m_adc_clk_mode(&mut self) -> Xo32mAdcClkModeW<'_, DummyCtrlSpec> {
        Xo32mAdcClkModeW::new(self, 10)
    }
}
#[doc = "Dummy Control bus to analog modules\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummyCtrlSpec;
impl crate::RegisterSpec for DummyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy_ctrl::R`](R) reader structure"]
impl crate::Readable for DummyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dummy_ctrl::W`](W) writer structure"]
impl crate::Writable for DummyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY_CTRL to value 0"]
impl crate::Resettable for DummyCtrlSpec {}

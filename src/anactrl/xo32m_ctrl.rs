#[doc = "Register `XO32M_CTRL` reader"]
pub type R = crate::R<Xo32mCtrlSpec>;
#[doc = "Register `XO32M_CTRL` writer"]
pub type W = crate::W<Xo32mCtrlSpec>;
#[doc = "Field `SLAVE` reader - Xo in slave mode."]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SLAVE` writer - Xo in slave mode."]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC_CAP_IN` reader - Tune capa banks of High speed Crystal Oscillator input pin"]
pub type OscCapInR = crate::FieldReader;
#[doc = "Field `OSC_CAP_IN` writer - Tune capa banks of High speed Crystal Oscillator input pin"]
pub type OscCapInW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OSC_CAP_OUT` reader - Tune capa banks of High speed Crystal Oscillator output pin"]
pub type OscCapOutR = crate::FieldReader;
#[doc = "Field `OSC_CAP_OUT` writer - Tune capa banks of High speed Crystal Oscillator output pin"]
pub type OscCapOutW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Bypass enable of XO AC buffer enable in pll and top level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcbufPassEnable {
    #[doc = "0: XO AC buffer bypass is disabled."]
    Disable = 0,
    #[doc = "1: XO AC buffer bypass is enabled."]
    Enable = 1,
}
impl From<AcbufPassEnable> for bool {
    #[inline(always)]
    fn from(variant: AcbufPassEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACBUF_PASS_ENABLE` reader - Bypass enable of XO AC buffer enable in pll and top level."]
pub type AcbufPassEnableR = crate::BitReader<AcbufPassEnable>;
impl AcbufPassEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcbufPassEnable {
        match self.bits {
            false => AcbufPassEnable::Disable,
            true => AcbufPassEnable::Enable,
        }
    }
    #[doc = "XO AC buffer bypass is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AcbufPassEnable::Disable
    }
    #[doc = "XO AC buffer bypass is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AcbufPassEnable::Enable
    }
}
#[doc = "Field `ACBUF_PASS_ENABLE` writer - Bypass enable of XO AC buffer enable in pll and top level."]
pub type AcbufPassEnableW<'a, REG> = crate::BitWriter<'a, REG, AcbufPassEnable>;
impl<'a, REG> AcbufPassEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XO AC buffer bypass is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AcbufPassEnable::Disable)
    }
    #[doc = "XO AC buffer bypass is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AcbufPassEnable::Enable)
    }
}
#[doc = "Enable High speed Crystal oscillator output to USB HS PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnablePllUsbOut {
    #[doc = "0: High speed Crystal oscillator output to USB HS PLL is disabled."]
    Disable = 0,
    #[doc = "1: High speed Crystal oscillator output to USB HS PLL is enabled."]
    Enable = 1,
}
impl From<EnablePllUsbOut> for bool {
    #[inline(always)]
    fn from(variant: EnablePllUsbOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_PLL_USB_OUT` reader - Enable High speed Crystal oscillator output to USB HS PLL."]
pub type EnablePllUsbOutR = crate::BitReader<EnablePllUsbOut>;
impl EnablePllUsbOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnablePllUsbOut {
        match self.bits {
            false => EnablePllUsbOut::Disable,
            true => EnablePllUsbOut::Enable,
        }
    }
    #[doc = "High speed Crystal oscillator output to USB HS PLL is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnablePllUsbOut::Disable
    }
    #[doc = "High speed Crystal oscillator output to USB HS PLL is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnablePllUsbOut::Enable
    }
}
#[doc = "Field `ENABLE_PLL_USB_OUT` writer - Enable High speed Crystal oscillator output to USB HS PLL."]
pub type EnablePllUsbOutW<'a, REG> = crate::BitWriter<'a, REG, EnablePllUsbOut>;
impl<'a, REG> EnablePllUsbOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed Crystal oscillator output to USB HS PLL is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnablePllUsbOut::Disable)
    }
    #[doc = "High speed Crystal oscillator output to USB HS PLL is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnablePllUsbOut::Enable)
    }
}
#[doc = "Enable High speed Crystal oscillator output to CPU system.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableSystemClkOut {
    #[doc = "0: High speed Crystal oscillator output to CPU system is disabled."]
    Disable = 0,
    #[doc = "1: High speed Crystal oscillator output to CPU system is enabled."]
    Enable = 1,
}
impl From<EnableSystemClkOut> for bool {
    #[inline(always)]
    fn from(variant: EnableSystemClkOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` reader - Enable High speed Crystal oscillator output to CPU system."]
pub type EnableSystemClkOutR = crate::BitReader<EnableSystemClkOut>;
impl EnableSystemClkOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableSystemClkOut {
        match self.bits {
            false => EnableSystemClkOut::Disable,
            true => EnableSystemClkOut::Enable,
        }
    }
    #[doc = "High speed Crystal oscillator output to CPU system is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnableSystemClkOut::Disable
    }
    #[doc = "High speed Crystal oscillator output to CPU system is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnableSystemClkOut::Enable
    }
}
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` writer - Enable High speed Crystal oscillator output to CPU system."]
pub type EnableSystemClkOutW<'a, REG> = crate::BitWriter<'a, REG, EnableSystemClkOut>;
impl<'a, REG> EnableSystemClkOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed Crystal oscillator output to CPU system is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSystemClkOut::Disable)
    }
    #[doc = "High speed Crystal oscillator output to CPU system is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSystemClkOut::Enable)
    }
}
impl R {
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Tune capa banks of High speed Crystal Oscillator input pin"]
    #[inline(always)]
    pub fn osc_cap_in(&self) -> OscCapInR {
        OscCapInR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Tune capa banks of High speed Crystal Oscillator output pin"]
    #[inline(always)]
    pub fn osc_cap_out(&self) -> OscCapOutR {
        OscCapOutR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&self) -> AcbufPassEnableR {
        AcbufPassEnableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&self) -> EnablePllUsbOutR {
        EnablePllUsbOutR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&self) -> EnableSystemClkOutR {
        EnableSystemClkOutR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&mut self) -> SlaveW<'_, Xo32mCtrlSpec> {
        SlaveW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Tune capa banks of High speed Crystal Oscillator input pin"]
    #[inline(always)]
    pub fn osc_cap_in(&mut self) -> OscCapInW<'_, Xo32mCtrlSpec> {
        OscCapInW::new(self, 8)
    }
    #[doc = "Bits 15:21 - Tune capa banks of High speed Crystal Oscillator output pin"]
    #[inline(always)]
    pub fn osc_cap_out(&mut self) -> OscCapOutW<'_, Xo32mCtrlSpec> {
        OscCapOutW::new(self, 15)
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&mut self) -> AcbufPassEnableW<'_, Xo32mCtrlSpec> {
        AcbufPassEnableW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&mut self) -> EnablePllUsbOutW<'_, Xo32mCtrlSpec> {
        EnablePllUsbOutW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&mut self) -> EnableSystemClkOutW<'_, Xo32mCtrlSpec> {
        EnableSystemClkOutW::new(self, 24)
    }
}
#[doc = "High speed Crystal Oscillator Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32m_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo32m_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xo32mCtrlSpec;
impl crate::RegisterSpec for Xo32mCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xo32m_ctrl::R`](R) reader structure"]
impl crate::Readable for Xo32mCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xo32m_ctrl::W`](W) writer structure"]
impl crate::Writable for Xo32mCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XO32M_CTRL to value 0x0021_428a"]
impl crate::Resettable for Xo32mCtrlSpec {
    const RESET_VALUE: u32 = 0x0021_428a;
}

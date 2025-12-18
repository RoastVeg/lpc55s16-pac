#[doc = "Register `ANACTRL_SET` reader"]
pub type R = crate::R<AnactrlSetSpec>;
#[doc = "Register `ANACTRL_SET` writer"]
pub type W = crate::W<AnactrlSetSpec>;
#[doc = "Field `LVI_EN` reader - Vow voltage detector enable bit."]
pub type LviEnR = crate::BitReader;
#[doc = "Field `LVI_EN` writer - Vow voltage detector enable bit."]
pub type LviEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFD_CLK_SEL` reader - For normal USB operation, this bit field must remain at value 2'b00."]
pub type PfdClkSelR = crate::FieldReader;
#[doc = "Field `PFD_CLK_SEL` writer - For normal USB operation, this bit field must remain at value 2'b00."]
pub type PfdClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevPulldown {
    #[doc = "0: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    Value0 = 0,
    #[doc = "1: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    Value1 = 1,
}
impl From<DevPulldown> for bool {
    #[inline(always)]
    fn from(variant: DevPulldown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_PULLDOWN` reader - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DevPulldownR = crate::BitReader<DevPulldown>;
impl DevPulldownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevPulldown {
        match self.bits {
            false => DevPulldown::Value0,
            true => DevPulldown::Value1,
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DevPulldown::Value0
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DevPulldown::Value1
    }
}
#[doc = "Field `DEV_PULLDOWN` writer - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DevPulldownW<'a, REG> = crate::BitWriter<'a, REG, DevPulldown>;
impl<'a, REG> DevPulldownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(DevPulldown::Value0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DevPulldown::Value1)
    }
}
impl R {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&self) -> LviEnR {
        LviEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&self) -> PfdClkSelR {
        PfdClkSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DevPulldownR {
        DevPulldownR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&mut self) -> LviEnW<'_, AnactrlSetSpec> {
        LviEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&mut self) -> PfdClkSelW<'_, AnactrlSetSpec> {
        PfdClkSelW::new(self, 2)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DevPulldownW<'_, AnactrlSetSpec> {
        DevPulldownW::new(self, 10)
    }
}
#[doc = "USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnactrlSetSpec;
impl crate::RegisterSpec for AnactrlSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anactrl_set::R`](R) reader structure"]
impl crate::Readable for AnactrlSetSpec {}
#[doc = "`write(|w| ..)` method takes [`anactrl_set::W`](W) writer structure"]
impl crate::Writable for AnactrlSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANACTRL_SET to value 0x0a00_0402"]
impl crate::Resettable for AnactrlSetSpec {
    const RESET_VALUE: u32 = 0x0a00_0402;
}

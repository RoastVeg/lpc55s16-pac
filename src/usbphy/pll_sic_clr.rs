#[doc = "Register `PLL_SIC_CLR` reader"]
pub type R = crate::R<PllSicClrSpec>;
#[doc = "Register `PLL_SIC_CLR` writer"]
pub type W = crate::W<PllSicClrSpec>;
#[doc = "Field `PLL_EN_USB_CLKS` reader - Enables the USB clock from PLL to USB PHY"]
pub type PllEnUsbClksR = crate::BitReader;
#[doc = "Field `PLL_EN_USB_CLKS` writer - Enables the USB clock from PLL to USB PHY"]
pub type PllEnUsbClksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_POWER` reader - Power up the USB PLL"]
pub type PllPowerR = crate::BitReader;
#[doc = "Field `PLL_POWER` writer - Power up the USB PLL"]
pub type PllPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_ENABLE` reader - Enables the clock output from the USB PLL"]
pub type PllEnableR = crate::BitReader;
#[doc = "Field `PLL_ENABLE` writer - Enables the clock output from the USB PLL"]
pub type PllEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference bias power down select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefbiasPwdSel {
    #[doc = "0: Selects PLL_POWER to control the reference bias"]
    Value0 = 0,
    #[doc = "1: Selects REFBIAS_PWD to control the reference bias"]
    Value1 = 1,
}
impl From<RefbiasPwdSel> for bool {
    #[inline(always)]
    fn from(variant: RefbiasPwdSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` reader - Reference bias power down select."]
pub type RefbiasPwdSelR = crate::BitReader<RefbiasPwdSel>;
impl RefbiasPwdSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefbiasPwdSel {
        match self.bits {
            false => RefbiasPwdSel::Value0,
            true => RefbiasPwdSel::Value1,
        }
    }
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RefbiasPwdSel::Value0
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RefbiasPwdSel::Value1
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` writer - Reference bias power down select."]
pub type RefbiasPwdSelW<'a, REG> = crate::BitWriter<'a, REG, RefbiasPwdSel>;
impl<'a, REG> RefbiasPwdSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(RefbiasPwdSel::Value0)
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RefbiasPwdSel::Value1)
    }
}
#[doc = "Field `REFBIAS_PWD` reader - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
pub type RefbiasPwdR = crate::BitReader;
#[doc = "Field `REFBIAS_PWD` writer - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
pub type RefbiasPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_REG_ENABLE` reader - This field controls the USB PLL regulator, set to enable the regulator"]
pub type PllRegEnableR = crate::BitReader;
#[doc = "Field `PLL_REG_ENABLE` writer - This field controls the USB PLL regulator, set to enable the regulator"]
pub type PllRegEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This field controls the USB PLL feedback loop divider\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PllDivSel {
    #[doc = "0: Divide by 13"]
    Value0 = 0,
    #[doc = "1: Divide by 15"]
    Value1 = 1,
    #[doc = "2: Divide by 16"]
    Value2 = 2,
    #[doc = "3: Divide by 20"]
    Value3 = 3,
    #[doc = "4: Divide by 22"]
    Value4 = 4,
    #[doc = "5: Divide by 25"]
    Value5 = 5,
    #[doc = "6: Divide by 30"]
    Value6 = 6,
    #[doc = "7: Divide by 240"]
    Value7 = 7,
}
impl From<PllDivSel> for u8 {
    #[inline(always)]
    fn from(variant: PllDivSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PllDivSel {
    type Ux = u8;
}
impl crate::IsEnum for PllDivSel {}
#[doc = "Field `PLL_DIV_SEL` reader - This field controls the USB PLL feedback loop divider"]
pub type PllDivSelR = crate::FieldReader<PllDivSel>;
impl PllDivSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllDivSel {
        match self.bits {
            0 => PllDivSel::Value0,
            1 => PllDivSel::Value1,
            2 => PllDivSel::Value2,
            3 => PllDivSel::Value3,
            4 => PllDivSel::Value4,
            5 => PllDivSel::Value5,
            6 => PllDivSel::Value6,
            7 => PllDivSel::Value7,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PllDivSel::Value0
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PllDivSel::Value1
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PllDivSel::Value2
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PllDivSel::Value3
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PllDivSel::Value4
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PllDivSel::Value5
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PllDivSel::Value6
    }
    #[doc = "Divide by 240"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PllDivSel::Value7
    }
}
#[doc = "Field `PLL_DIV_SEL` writer - This field controls the USB PLL feedback loop divider"]
pub type PllDivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, PllDivSel, crate::Safe>;
impl<'a, REG> PllDivSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value0)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value2)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value3)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value4)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value5)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value6)
    }
    #[doc = "Divide by 240"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PllDivSel::Value7)
    }
}
#[doc = "Field `PLL_PREDIV` reader - This is selection between /1 or /2 to expand the range of ref input clock."]
pub type PllPredivR = crate::BitReader;
#[doc = "Field `PLL_PREDIV` writer - This is selection between /1 or /2 to expand the range of ref input clock."]
pub type PllPredivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB PLL lock status indicator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLock {
    #[doc = "0: PLL is not currently locked"]
    Value0 = 0,
    #[doc = "1: PLL is currently locked"]
    Value1 = 1,
}
impl From<PllLock> for bool {
    #[inline(always)]
    fn from(variant: PllLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK` reader - USB PLL lock status indicator"]
pub type PllLockR = crate::BitReader<PllLock>;
impl PllLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLock {
        match self.bits {
            false => PllLock::Value0,
            true => PllLock::Value1,
        }
    }
    #[doc = "PLL is not currently locked"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PllLock::Value0
    }
    #[doc = "PLL is currently locked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PllLock::Value1
    }
}
impl R {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PllEnUsbClksR {
        PllEnUsbClksR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    pub fn pll_power(&self) -> PllPowerR {
        PllPowerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PllEnableR {
        PllEnableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    pub fn refbias_pwd_sel(&self) -> RefbiasPwdSelR {
        RefbiasPwdSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub fn refbias_pwd(&self) -> RefbiasPwdR {
        RefbiasPwdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&self) -> PllRegEnableR {
        PllRegEnableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PllDivSelR {
        PllDivSelR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub fn pll_prediv(&self) -> PllPredivR {
        PllPredivR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_CLR")
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_prediv", &self.pll_prediv())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&mut self) -> PllEnUsbClksW<'_, PllSicClrSpec> {
        PllEnUsbClksW::new(self, 6)
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    pub fn pll_power(&mut self) -> PllPowerW<'_, PllSicClrSpec> {
        PllPowerW::new(self, 12)
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub fn pll_enable(&mut self) -> PllEnableW<'_, PllSicClrSpec> {
        PllEnableW::new(self, 13)
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    pub fn refbias_pwd_sel(&mut self) -> RefbiasPwdSelW<'_, PllSicClrSpec> {
        RefbiasPwdSelW::new(self, 19)
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub fn refbias_pwd(&mut self) -> RefbiasPwdW<'_, PllSicClrSpec> {
        RefbiasPwdW::new(self, 20)
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&mut self) -> PllRegEnableW<'_, PllSicClrSpec> {
        PllRegEnableW::new(self, 21)
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&mut self) -> PllDivSelW<'_, PllSicClrSpec> {
        PllDivSelW::new(self, 22)
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub fn pll_prediv(&mut self) -> PllPredivW<'_, PllSicClrSpec> {
        PllPredivW::new(self, 30)
    }
}
#[doc = "USB PHY PLL Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_sic_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sic_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSicClrSpec;
impl crate::RegisterSpec for PllSicClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_sic_clr::R`](R) reader structure"]
impl crate::Readable for PllSicClrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_sic_clr::W`](W) writer structure"]
impl crate::Writable for PllSicClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_SIC_CLR to value 0x00d1_2000"]
impl crate::Resettable for PllSicClrSpec {
    const RESET_VALUE: u32 = 0x00d1_2000;
}

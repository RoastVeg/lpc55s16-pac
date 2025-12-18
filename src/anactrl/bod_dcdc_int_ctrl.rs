#[doc = "Register `BOD_DCDC_INT_CTRL` reader"]
pub type R = crate::R<BodDcdcIntCtrlSpec>;
#[doc = "Register `BOD_DCDC_INT_CTRL` writer"]
pub type W = crate::W<BodDcdcIntCtrlSpec>;
#[doc = "BOD VBAT interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodvbatIntEnable {
    #[doc = "0: BOD VBAT interrupt is disabled."]
    Disable = 0,
    #[doc = "1: BOD VBAT interrupt is enabled."]
    Enable = 1,
}
impl From<BodvbatIntEnable> for bool {
    #[inline(always)]
    fn from(variant: BodvbatIntEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` reader - BOD VBAT interrupt control."]
pub type BodvbatIntEnableR = crate::BitReader<BodvbatIntEnable>;
impl BodvbatIntEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodvbatIntEnable {
        match self.bits {
            false => BodvbatIntEnable::Disable,
            true => BodvbatIntEnable::Enable,
        }
    }
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodvbatIntEnable::Disable
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodvbatIntEnable::Enable
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` writer - BOD VBAT interrupt control."]
pub type BodvbatIntEnableW<'a, REG> = crate::BitWriter<'a, REG, BodvbatIntEnable>;
impl<'a, REG> BodvbatIntEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatIntEnable::Disable)
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodvbatIntEnable::Enable)
    }
}
#[doc = "Field `BODVBAT_INT_CLEAR` reader - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BodvbatIntClearR = crate::BitReader;
#[doc = "Field `BODVBAT_INT_CLEAR` writer - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BodvbatIntClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BOD CORE interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodcoreIntEnable {
    #[doc = "0: BOD CORE interrupt is disabled."]
    Disable = 0,
    #[doc = "1: BOD CORE interrupt is enabled."]
    Enable = 1,
}
impl From<BodcoreIntEnable> for bool {
    #[inline(always)]
    fn from(variant: BodcoreIntEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORE_INT_ENABLE` reader - BOD CORE interrupt control."]
pub type BodcoreIntEnableR = crate::BitReader<BodcoreIntEnable>;
impl BodcoreIntEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodcoreIntEnable {
        match self.bits {
            false => BodcoreIntEnable::Disable,
            true => BodcoreIntEnable::Enable,
        }
    }
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BodcoreIntEnable::Disable
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BodcoreIntEnable::Enable
    }
}
#[doc = "Field `BODCORE_INT_ENABLE` writer - BOD CORE interrupt control."]
pub type BodcoreIntEnableW<'a, REG> = crate::BitWriter<'a, REG, BodcoreIntEnable>;
impl<'a, REG> BodcoreIntEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreIntEnable::Disable)
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BodcoreIntEnable::Enable)
    }
}
#[doc = "Field `BODCORE_INT_CLEAR` reader - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BodcoreIntClearR = crate::BitReader;
#[doc = "Field `BODCORE_INT_CLEAR` writer - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type BodcoreIntClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DCDC interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcIntEnable {
    #[doc = "0: DCDC interrupt is disabled."]
    Disable = 0,
    #[doc = "1: DCDC interrupt is enabled."]
    Enable = 1,
}
impl From<DcdcIntEnable> for bool {
    #[inline(always)]
    fn from(variant: DcdcIntEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_INT_ENABLE` reader - DCDC interrupt control."]
pub type DcdcIntEnableR = crate::BitReader<DcdcIntEnable>;
impl DcdcIntEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcIntEnable {
        match self.bits {
            false => DcdcIntEnable::Disable,
            true => DcdcIntEnable::Enable,
        }
    }
    #[doc = "DCDC interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DcdcIntEnable::Disable
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DcdcIntEnable::Enable
    }
}
#[doc = "Field `DCDC_INT_ENABLE` writer - DCDC interrupt control."]
pub type DcdcIntEnableW<'a, REG> = crate::BitWriter<'a, REG, DcdcIntEnable>;
impl<'a, REG> DcdcIntEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCDC interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcIntEnable::Disable)
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcIntEnable::Enable)
    }
}
#[doc = "Field `DCDC_INT_CLEAR` reader - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type DcdcIntClearR = crate::BitReader;
#[doc = "Field `DCDC_INT_CLEAR` writer - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub type DcdcIntClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&self) -> BodvbatIntEnableR {
        BodvbatIntEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&self) -> BodvbatIntClearR {
        BodvbatIntClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&self) -> BodcoreIntEnableR {
        BodcoreIntEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&self) -> BodcoreIntClearR {
        BodcoreIntClearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&self) -> DcdcIntEnableR {
        DcdcIntEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&self) -> DcdcIntClearR {
        DcdcIntClearR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&mut self) -> BodvbatIntEnableW<'_, BodDcdcIntCtrlSpec> {
        BodvbatIntEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&mut self) -> BodvbatIntClearW<'_, BodDcdcIntCtrlSpec> {
        BodvbatIntClearW::new(self, 1)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&mut self) -> BodcoreIntEnableW<'_, BodDcdcIntCtrlSpec> {
        BodcoreIntEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&mut self) -> BodcoreIntClearW<'_, BodDcdcIntCtrlSpec> {
        BodcoreIntClearW::new(self, 3)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&mut self) -> DcdcIntEnableW<'_, BodDcdcIntCtrlSpec> {
        DcdcIntEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&mut self) -> DcdcIntClearW<'_, BodDcdcIntCtrlSpec> {
        DcdcIntClearW::new(self, 5)
    }
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bod_dcdc_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_dcdc_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodDcdcIntCtrlSpec;
impl crate::RegisterSpec for BodDcdcIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod_dcdc_int_ctrl::R`](R) reader structure"]
impl crate::Readable for BodDcdcIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bod_dcdc_int_ctrl::W`](W) writer structure"]
impl crate::Writable for BodDcdcIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOD_DCDC_INT_CTRL to value 0"]
impl crate::Resettable for BodDcdcIntCtrlSpec {}

#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL0` reader"]
pub type R = crate::R<SecCtrlApbBridge1MemCtrl0Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL0` writer"]
pub type W = crate::W<SecCtrlApbBridge1MemCtrl0Spec>;
#[doc = "Power Management Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmcRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<PmcRule> for u8 {
    #[inline(always)]
    fn from(variant: PmcRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmcRule {
    type Ux = u8;
}
impl crate::IsEnum for PmcRule {}
#[doc = "Field `PMC_RULE` reader - Power Management Controller"]
pub type PmcRuleR = crate::FieldReader<PmcRule>;
impl PmcRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmcRule {
        match self.bits {
            0 => PmcRule::EnumNsNp,
            1 => PmcRule::EnumNsP,
            2 => PmcRule::EnumSNp,
            3 => PmcRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PmcRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PmcRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PmcRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PmcRule::EnumSP
    }
}
#[doc = "Field `PMC_RULE` writer - Power Management Controller"]
pub type PmcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmcRule, crate::Safe>;
impl<'a, REG> PmcRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PmcRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PmcRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PmcRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PmcRule::EnumSP)
    }
}
#[doc = "System Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysctrlRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SysctrlRule> for u8 {
    #[inline(always)]
    fn from(variant: SysctrlRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysctrlRule {
    type Ux = u8;
}
impl crate::IsEnum for SysctrlRule {}
#[doc = "Field `SYSCTRL_RULE` reader - System Controller"]
pub type SysctrlRuleR = crate::FieldReader<SysctrlRule>;
impl SysctrlRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysctrlRule {
        match self.bits {
            0 => SysctrlRule::EnumNsNp,
            1 => SysctrlRule::EnumNsP,
            2 => SysctrlRule::EnumSNp,
            3 => SysctrlRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SysctrlRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SysctrlRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SysctrlRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SysctrlRule::EnumSP
    }
}
#[doc = "Field `SYSCTRL_RULE` writer - System Controller"]
pub type SysctrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysctrlRule, crate::Safe>;
impl<'a, REG> SysctrlRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SysctrlRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SysctrlRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SysctrlRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SysctrlRule::EnumSP)
    }
}
#[doc = "SPI FILTER control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiFilterRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SpiFilterRule> for u8 {
    #[inline(always)]
    fn from(variant: SpiFilterRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpiFilterRule {
    type Ux = u8;
}
impl crate::IsEnum for SpiFilterRule {}
#[doc = "Field `SPI_FILTER_RULE` reader - SPI FILTER control"]
pub type SpiFilterRuleR = crate::FieldReader<SpiFilterRule>;
impl SpiFilterRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiFilterRule {
        match self.bits {
            0 => SpiFilterRule::EnumNsNp,
            1 => SpiFilterRule::EnumNsP,
            2 => SpiFilterRule::EnumSNp,
            3 => SpiFilterRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SpiFilterRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SpiFilterRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SpiFilterRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SpiFilterRule::EnumSP
    }
}
#[doc = "Field `SPI_FILTER_RULE` writer - SPI FILTER control"]
pub type SpiFilterRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SpiFilterRule, crate::Safe>;
impl<'a, REG> SpiFilterRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SpiFilterRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SpiFilterRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SpiFilterRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SpiFilterRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Management Controller"]
    #[inline(always)]
    pub fn pmc_rule(&self) -> PmcRuleR {
        PmcRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 12:13 - System Controller"]
    #[inline(always)]
    pub fn sysctrl_rule(&self) -> SysctrlRuleR {
        SysctrlRuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SPI FILTER control"]
    #[inline(always)]
    pub fn spi_filter_rule(&self) -> SpiFilterRuleR {
        SpiFilterRuleR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Management Controller"]
    #[inline(always)]
    pub fn pmc_rule(&mut self) -> PmcRuleW<'_, SecCtrlApbBridge1MemCtrl0Spec> {
        PmcRuleW::new(self, 0)
    }
    #[doc = "Bits 12:13 - System Controller"]
    #[inline(always)]
    pub fn sysctrl_rule(&mut self) -> SysctrlRuleW<'_, SecCtrlApbBridge1MemCtrl0Spec> {
        SysctrlRuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - SPI FILTER control"]
    #[inline(always)]
    pub fn spi_filter_rule(&mut self) -> SpiFilterRuleW<'_, SecCtrlApbBridge1MemCtrl0Spec> {
        SpiFilterRuleW::new(self, 16)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge1MemCtrl0Spec;
impl crate::RegisterSpec for SecCtrlApbBridge1MemCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge1_mem_ctrl0::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge1MemCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge1_mem_ctrl0::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge1MemCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 to value 0"]
impl crate::Resettable for SecCtrlApbBridge1MemCtrl0Spec {}

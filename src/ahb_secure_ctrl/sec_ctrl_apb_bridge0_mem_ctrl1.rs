#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` reader"]
pub type R = crate::R<SecCtrlApbBridge0MemCtrl1Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL1` writer"]
pub type W = crate::W<SecCtrlApbBridge0MemCtrl1Spec>;
#[doc = "Standard counter/Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctimer0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ctimer0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ctimer0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctimer0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ctimer0Rule {}
#[doc = "Field `CTIMER0_RULE` reader - Standard counter/Timer 0"]
pub type Ctimer0RuleR = crate::FieldReader<Ctimer0Rule>;
impl Ctimer0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer0Rule {
        match self.bits {
            0 => Ctimer0Rule::EnumNsNp,
            1 => Ctimer0Rule::EnumNsP,
            2 => Ctimer0Rule::EnumSNp,
            3 => Ctimer0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ctimer0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ctimer0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ctimer0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ctimer0Rule::EnumSP
    }
}
#[doc = "Field `CTIMER0_RULE` writer - Standard counter/Timer 0"]
pub type Ctimer0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctimer0Rule, crate::Safe>;
impl<'a, REG> Ctimer0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Rule::EnumSP)
    }
}
#[doc = "Standard counter/Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctimer1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ctimer1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ctimer1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctimer1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ctimer1Rule {}
#[doc = "Field `CTIMER1_RULE` reader - Standard counter/Timer 1"]
pub type Ctimer1RuleR = crate::FieldReader<Ctimer1Rule>;
impl Ctimer1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer1Rule {
        match self.bits {
            0 => Ctimer1Rule::EnumNsNp,
            1 => Ctimer1Rule::EnumNsP,
            2 => Ctimer1Rule::EnumSNp,
            3 => Ctimer1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ctimer1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ctimer1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ctimer1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ctimer1Rule::EnumSP
    }
}
#[doc = "Field `CTIMER1_RULE` writer - Standard counter/Timer 1"]
pub type Ctimer1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctimer1Rule, crate::Safe>;
impl<'a, REG> Ctimer1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Rule::EnumSP)
    }
}
#[doc = "Windiwed wtachdog Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WwdtRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<WwdtRule> for u8 {
    #[inline(always)]
    fn from(variant: WwdtRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WwdtRule {
    type Ux = u8;
}
impl crate::IsEnum for WwdtRule {}
#[doc = "Field `WWDT_RULE` reader - Windiwed wtachdog Timer"]
pub type WwdtRuleR = crate::FieldReader<WwdtRule>;
impl WwdtRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WwdtRule {
        match self.bits {
            0 => WwdtRule::EnumNsNp,
            1 => WwdtRule::EnumNsP,
            2 => WwdtRule::EnumSNp,
            3 => WwdtRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == WwdtRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == WwdtRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == WwdtRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == WwdtRule::EnumSP
    }
}
#[doc = "Field `WWDT_RULE` writer - Windiwed wtachdog Timer"]
pub type WwdtRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, WwdtRule, crate::Safe>;
impl<'a, REG> WwdtRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtRule::EnumSP)
    }
}
#[doc = "Multi-rate Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MrtRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<MrtRule> for u8 {
    #[inline(always)]
    fn from(variant: MrtRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MrtRule {
    type Ux = u8;
}
impl crate::IsEnum for MrtRule {}
#[doc = "Field `MRT_RULE` reader - Multi-rate Timer"]
pub type MrtRuleR = crate::FieldReader<MrtRule>;
impl MrtRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MrtRule {
        match self.bits {
            0 => MrtRule::EnumNsNp,
            1 => MrtRule::EnumNsP,
            2 => MrtRule::EnumSNp,
            3 => MrtRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MrtRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MrtRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MrtRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MrtRule::EnumSP
    }
}
#[doc = "Field `MRT_RULE` writer - Multi-rate Timer"]
pub type MrtRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, MrtRule, crate::Safe>;
impl<'a, REG> MrtRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(MrtRule::EnumSP)
    }
}
#[doc = "Micro-Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UtickRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<UtickRule> for u8 {
    #[inline(always)]
    fn from(variant: UtickRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UtickRule {
    type Ux = u8;
}
impl crate::IsEnum for UtickRule {}
#[doc = "Field `UTICK_RULE` reader - Micro-Timer"]
pub type UtickRuleR = crate::FieldReader<UtickRule>;
impl UtickRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UtickRule {
        match self.bits {
            0 => UtickRule::EnumNsNp,
            1 => UtickRule::EnumNsP,
            2 => UtickRule::EnumSNp,
            3 => UtickRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UtickRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UtickRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UtickRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UtickRule::EnumSP
    }
}
#[doc = "Field `UTICK_RULE` writer - Micro-Timer"]
pub type UtickRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, UtickRule, crate::Safe>;
impl<'a, REG> UtickRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(UtickRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    pub fn ctimer0_rule(&self) -> Ctimer0RuleR {
        Ctimer0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    pub fn ctimer1_rule(&self) -> Ctimer1RuleR {
        Ctimer1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    pub fn wwdt_rule(&self) -> WwdtRuleR {
        WwdtRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    pub fn mrt_rule(&self) -> MrtRuleR {
        MrtRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    pub fn utick_rule(&self) -> UtickRuleR {
        UtickRuleR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 0"]
    #[inline(always)]
    pub fn ctimer0_rule(&mut self) -> Ctimer0RuleW<'_, SecCtrlApbBridge0MemCtrl1Spec> {
        Ctimer0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 1"]
    #[inline(always)]
    pub fn ctimer1_rule(&mut self) -> Ctimer1RuleW<'_, SecCtrlApbBridge0MemCtrl1Spec> {
        Ctimer1RuleW::new(self, 4)
    }
    #[doc = "Bits 16:17 - Windiwed wtachdog Timer"]
    #[inline(always)]
    pub fn wwdt_rule(&mut self) -> WwdtRuleW<'_, SecCtrlApbBridge0MemCtrl1Spec> {
        WwdtRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Multi-rate Timer"]
    #[inline(always)]
    pub fn mrt_rule(&mut self) -> MrtRuleW<'_, SecCtrlApbBridge0MemCtrl1Spec> {
        MrtRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Micro-Timer"]
    #[inline(always)]
    pub fn utick_rule(&mut self) -> UtickRuleW<'_, SecCtrlApbBridge0MemCtrl1Spec> {
        UtickRuleW::new(self, 24)
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge0MemCtrl1Spec;
impl crate::RegisterSpec for SecCtrlApbBridge0MemCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge0_mem_ctrl1::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge0MemCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge0_mem_ctrl1::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge0MemCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 to value 0"]
impl crate::Resettable for SecCtrlApbBridge0MemCtrl1Spec {}

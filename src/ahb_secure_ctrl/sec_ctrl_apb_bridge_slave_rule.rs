#[doc = "Register `SEC_CTRL_APB_BRIDGE_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlApbBridgeSlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlApbBridgeSlaveRuleSpec>;
#[doc = "Security access rules for the whole APB Bridge 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apbbridge0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Apbbridge0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Apbbridge0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apbbridge0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Apbbridge0Rule {}
#[doc = "Field `APBBRIDGE0_RULE` reader - Security access rules for the whole APB Bridge 0"]
pub type Apbbridge0RuleR = crate::FieldReader<Apbbridge0Rule>;
impl Apbbridge0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbbridge0Rule {
        match self.bits {
            0 => Apbbridge0Rule::EnumNsNp,
            1 => Apbbridge0Rule::EnumNsP,
            2 => Apbbridge0Rule::EnumSNp,
            3 => Apbbridge0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Apbbridge0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Apbbridge0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Apbbridge0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Apbbridge0Rule::EnumSP
    }
}
#[doc = "Field `APBBRIDGE0_RULE` writer - Security access rules for the whole APB Bridge 0"]
pub type Apbbridge0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Apbbridge0Rule, crate::Safe>;
impl<'a, REG> Apbbridge0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge0Rule::EnumSP)
    }
}
#[doc = "Security access rules for the whole APB Bridge 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apbbridge1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Apbbridge1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Apbbridge1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apbbridge1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Apbbridge1Rule {}
#[doc = "Field `APBBRIDGE1_RULE` reader - Security access rules for the whole APB Bridge 1"]
pub type Apbbridge1RuleR = crate::FieldReader<Apbbridge1Rule>;
impl Apbbridge1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apbbridge1Rule {
        match self.bits {
            0 => Apbbridge1Rule::EnumNsNp,
            1 => Apbbridge1Rule::EnumNsP,
            2 => Apbbridge1Rule::EnumSNp,
            3 => Apbbridge1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Apbbridge1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Apbbridge1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Apbbridge1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Apbbridge1Rule::EnumSP
    }
}
#[doc = "Field `APBBRIDGE1_RULE` writer - Security access rules for the whole APB Bridge 1"]
pub type Apbbridge1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Apbbridge1Rule, crate::Safe>;
impl<'a, REG> Apbbridge1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Apbbridge1Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole APB Bridge 0"]
    #[inline(always)]
    pub fn apbbridge0_rule(&self) -> Apbbridge0RuleR {
        Apbbridge0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole APB Bridge 1"]
    #[inline(always)]
    pub fn apbbridge1_rule(&self) -> Apbbridge1RuleR {
        Apbbridge1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE_SLAVE_RULE")
            .field("apbbridge0_rule", &self.apbbridge0_rule())
            .field("apbbridge1_rule", &self.apbbridge1_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole APB Bridge 0"]
    #[inline(always)]
    pub fn apbbridge0_rule(&mut self) -> Apbbridge0RuleW<'_, SecCtrlApbBridgeSlaveRuleSpec> {
        Apbbridge0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole APB Bridge 1"]
    #[inline(always)]
    pub fn apbbridge1_rule(&mut self) -> Apbbridge1RuleW<'_, SecCtrlApbBridgeSlaveRuleSpec> {
        Apbbridge1RuleW::new(self, 4)
    }
}
#[doc = "Security access rules for both APB Bridges slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridgeSlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlApbBridgeSlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridgeSlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridgeSlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlApbBridgeSlaveRuleSpec {}

#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE1_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort9Slave1RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE1_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort9Slave1RuleSpec>;
#[doc = "Secure High Speed GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Gpio1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Gpio1Rule {}
#[doc = "Field `GPIO1_RULE` reader - Secure High Speed GPIO"]
pub type Gpio1RuleR = crate::FieldReader<Gpio1Rule>;
impl Gpio1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1Rule {
        match self.bits {
            0 => Gpio1Rule::EnumNsNp,
            1 => Gpio1Rule::EnumNsP,
            2 => Gpio1Rule::EnumSNp,
            3 => Gpio1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Gpio1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Gpio1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Gpio1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Gpio1Rule::EnumSP
    }
}
#[doc = "Field `GPIO1_RULE` writer - Secure High Speed GPIO"]
pub type Gpio1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1Rule, crate::Safe>;
impl<'a, REG> Gpio1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1Rule::EnumSP)
    }
}
#[doc = "AHB Secure Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbSecCtrlRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AhbSecCtrlRule> for u8 {
    #[inline(always)]
    fn from(variant: AhbSecCtrlRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbSecCtrlRule {
    type Ux = u8;
}
impl crate::IsEnum for AhbSecCtrlRule {}
#[doc = "Field `AHB_SEC_CTRL_RULE` reader - AHB Secure Controller"]
pub type AhbSecCtrlRuleR = crate::FieldReader<AhbSecCtrlRule>;
impl AhbSecCtrlRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbSecCtrlRule {
        match self.bits {
            0 => AhbSecCtrlRule::EnumNsNp,
            1 => AhbSecCtrlRule::EnumNsP,
            2 => AhbSecCtrlRule::EnumSNp,
            3 => AhbSecCtrlRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AhbSecCtrlRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AhbSecCtrlRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AhbSecCtrlRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AhbSecCtrlRule::EnumSP
    }
}
#[doc = "Field `AHB_SEC_CTRL_RULE` writer - AHB Secure Controller"]
pub type AhbSecCtrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, AhbSecCtrlRule, crate::Safe>;
impl<'a, REG> AhbSecCtrlRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Secure High Speed GPIO"]
    #[inline(always)]
    pub fn gpio1_rule(&self) -> Gpio1RuleR {
        Gpio1RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - AHB Secure Controller"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_rule(&self) -> AhbSecCtrlRuleR {
        AhbSecCtrlRuleR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Secure High Speed GPIO"]
    #[inline(always)]
    pub fn gpio1_rule(&mut self) -> Gpio1RuleW<'_, SecCtrlAhbPort9Slave1RuleSpec> {
        Gpio1RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - AHB Secure Controller"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_rule(&mut self) -> AhbSecCtrlRuleW<'_, SecCtrlAhbPort9Slave1RuleSpec> {
        AhbSecCtrlRuleW::new(self, 4)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port9_slave1_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port9_slave1_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort9Slave1RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort9Slave1RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port9_slave1_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort9Slave1RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port9_slave1_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort9Slave1RuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT9_SLAVE1_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort9Slave1RuleSpec {}

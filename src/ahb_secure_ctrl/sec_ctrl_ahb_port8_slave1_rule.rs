#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort8Slave1RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort8Slave1RuleSpec>;
#[doc = "Flexcomm interface 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm7Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm7Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm7Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm7Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm7Rule {}
#[doc = "Field `FLEXCOMM7_RULE` reader - Flexcomm interface 7"]
pub type Flexcomm7RuleR = crate::FieldReader<Flexcomm7Rule>;
impl Flexcomm7RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7Rule {
        match self.bits {
            0 => Flexcomm7Rule::EnumNsNp,
            1 => Flexcomm7Rule::EnumNsP,
            2 => Flexcomm7Rule::EnumSNp,
            3 => Flexcomm7Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm7Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm7Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm7Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm7Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM7_RULE` writer - Flexcomm interface 7"]
pub type Flexcomm7RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm7Rule, crate::Safe>;
impl<'a, REG> Flexcomm7RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rule::EnumSP)
    }
}
#[doc = "Debug mailbox (aka ISP-AP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DbgMailboxRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<DbgMailboxRule> for u8 {
    #[inline(always)]
    fn from(variant: DbgMailboxRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DbgMailboxRule {
    type Ux = u8;
}
impl crate::IsEnum for DbgMailboxRule {}
#[doc = "Field `DBG_MAILBOX_RULE` reader - Debug mailbox (aka ISP-AP)"]
pub type DbgMailboxRuleR = crate::FieldReader<DbgMailboxRule>;
impl DbgMailboxRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgMailboxRule {
        match self.bits {
            0 => DbgMailboxRule::EnumNsNp,
            1 => DbgMailboxRule::EnumNsP,
            2 => DbgMailboxRule::EnumSNp,
            3 => DbgMailboxRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == DbgMailboxRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DbgMailboxRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DbgMailboxRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DbgMailboxRule::EnumSP
    }
}
#[doc = "Field `DBG_MAILBOX_RULE` writer - Debug mailbox (aka ISP-AP)"]
pub type DbgMailboxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, DbgMailboxRule, crate::Safe>;
impl<'a, REG> DbgMailboxRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(DbgMailboxRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(DbgMailboxRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(DbgMailboxRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(DbgMailboxRule::EnumSP)
    }
}
#[doc = "CAN-FD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Can0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Can0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Can0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Can0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Can0Rule {}
#[doc = "Field `CAN0_RULE` reader - CAN-FD"]
pub type Can0RuleR = crate::FieldReader<Can0Rule>;
impl Can0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Can0Rule {
        match self.bits {
            0 => Can0Rule::EnumNsNp,
            1 => Can0Rule::EnumNsP,
            2 => Can0Rule::EnumSNp,
            3 => Can0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Can0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Can0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Can0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Can0Rule::EnumSP
    }
}
#[doc = "Field `CAN0_RULE` writer - CAN-FD"]
pub type Can0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Can0Rule, crate::Safe>;
impl<'a, REG> Can0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Can0Rule::EnumSP)
    }
}
#[doc = "High Speed SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HsLspiRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<HsLspiRule> for u8 {
    #[inline(always)]
    fn from(variant: HsLspiRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HsLspiRule {
    type Ux = u8;
}
impl crate::IsEnum for HsLspiRule {}
#[doc = "Field `HS_LSPI_RULE` reader - High Speed SPI"]
pub type HsLspiRuleR = crate::FieldReader<HsLspiRule>;
impl HsLspiRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsLspiRule {
        match self.bits {
            0 => HsLspiRule::EnumNsNp,
            1 => HsLspiRule::EnumNsP,
            2 => HsLspiRule::EnumSNp,
            3 => HsLspiRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HsLspiRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HsLspiRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HsLspiRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HsLspiRule::EnumSP
    }
}
#[doc = "Field `HS_LSPI_RULE` writer - High Speed SPI"]
pub type HsLspiRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, HsLspiRule, crate::Safe>;
impl<'a, REG> HsLspiRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(HsLspiRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flexcomm interface 7"]
    #[inline(always)]
    pub fn flexcomm7_rule(&self) -> Flexcomm7RuleR {
        Flexcomm7RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Debug mailbox (aka ISP-AP)"]
    #[inline(always)]
    pub fn dbg_mailbox_rule(&self) -> DbgMailboxRuleR {
        DbgMailboxRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CAN-FD"]
    #[inline(always)]
    pub fn can0_rule(&self) -> Can0RuleR {
        Can0RuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - High Speed SPI"]
    #[inline(always)]
    pub fn hs_lspi_rule(&self) -> HsLspiRuleR {
        HsLspiRuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flexcomm interface 7"]
    #[inline(always)]
    pub fn flexcomm7_rule(&mut self) -> Flexcomm7RuleW<'_, SecCtrlAhbPort8Slave1RuleSpec> {
        Flexcomm7RuleW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Debug mailbox (aka ISP-AP)"]
    #[inline(always)]
    pub fn dbg_mailbox_rule(&mut self) -> DbgMailboxRuleW<'_, SecCtrlAhbPort8Slave1RuleSpec> {
        DbgMailboxRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - CAN-FD"]
    #[inline(always)]
    pub fn can0_rule(&mut self) -> Can0RuleW<'_, SecCtrlAhbPort8Slave1RuleSpec> {
        Can0RuleW::new(self, 20)
    }
    #[doc = "Bits 28:29 - High Speed SPI"]
    #[inline(always)]
    pub fn hs_lspi_rule(&mut self) -> HsLspiRuleW<'_, SecCtrlAhbPort8Slave1RuleSpec> {
        HsLspiRuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port8_slave1_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port8_slave1_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort8Slave1RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort8Slave1RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port8_slave1_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort8Slave1RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port8_slave1_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort8Slave1RuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE1_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort8Slave1RuleSpec {}

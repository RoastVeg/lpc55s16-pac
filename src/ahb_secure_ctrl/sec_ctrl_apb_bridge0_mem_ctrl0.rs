#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL0` reader"]
pub type R = crate::R<SecCtrlApbBridge0MemCtrl0Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL0` writer"]
pub type W = crate::W<SecCtrlApbBridge0MemCtrl0Spec>;
#[doc = "System Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SysconRule> for u8 {
    #[inline(always)]
    fn from(variant: SysconRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconRule {
    type Ux = u8;
}
impl crate::IsEnum for SysconRule {}
#[doc = "Field `SYSCON_RULE` reader - System Configuration"]
pub type SysconRuleR = crate::FieldReader<SysconRule>;
impl SysconRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconRule {
        match self.bits {
            0 => SysconRule::EnumNsNp,
            1 => SysconRule::EnumNsP,
            2 => SysconRule::EnumSNp,
            3 => SysconRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SysconRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SysconRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SysconRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SysconRule::EnumSP
    }
}
#[doc = "Field `SYSCON_RULE` writer - System Configuration"]
pub type SysconRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconRule, crate::Safe>;
impl<'a, REG> SysconRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SysconRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SysconRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SysconRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SysconRule::EnumSP)
    }
}
#[doc = "I/O Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IoconRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<IoconRule> for u8 {
    #[inline(always)]
    fn from(variant: IoconRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IoconRule {
    type Ux = u8;
}
impl crate::IsEnum for IoconRule {}
#[doc = "Field `IOCON_RULE` reader - I/O Configuration"]
pub type IoconRuleR = crate::FieldReader<IoconRule>;
impl IoconRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IoconRule {
        match self.bits {
            0 => IoconRule::EnumNsNp,
            1 => IoconRule::EnumNsP,
            2 => IoconRule::EnumSNp,
            3 => IoconRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == IoconRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == IoconRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == IoconRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == IoconRule::EnumSP
    }
}
#[doc = "Field `IOCON_RULE` writer - I/O Configuration"]
pub type IoconRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, IoconRule, crate::Safe>;
impl<'a, REG> IoconRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(IoconRule::EnumSP)
    }
}
#[doc = "GPIO input Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gint0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Gint0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Gint0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gint0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Gint0Rule {}
#[doc = "Field `GINT0_RULE` reader - GPIO input Interrupt 0"]
pub type Gint0RuleR = crate::FieldReader<Gint0Rule>;
impl Gint0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gint0Rule {
        match self.bits {
            0 => Gint0Rule::EnumNsNp,
            1 => Gint0Rule::EnumNsP,
            2 => Gint0Rule::EnumSNp,
            3 => Gint0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Gint0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Gint0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Gint0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Gint0Rule::EnumSP
    }
}
#[doc = "Field `GINT0_RULE` writer - GPIO input Interrupt 0"]
pub type Gint0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gint0Rule, crate::Safe>;
impl<'a, REG> Gint0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gint0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gint0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gint0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gint0Rule::EnumSP)
    }
}
#[doc = "GPIO input Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gint1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Gint1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Gint1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gint1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Gint1Rule {}
#[doc = "Field `GINT1_RULE` reader - GPIO input Interrupt 1"]
pub type Gint1RuleR = crate::FieldReader<Gint1Rule>;
impl Gint1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gint1Rule {
        match self.bits {
            0 => Gint1Rule::EnumNsNp,
            1 => Gint1Rule::EnumNsP,
            2 => Gint1Rule::EnumSNp,
            3 => Gint1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Gint1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Gint1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Gint1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Gint1Rule::EnumSP
    }
}
#[doc = "Field `GINT1_RULE` writer - GPIO input Interrupt 1"]
pub type Gint1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gint1Rule, crate::Safe>;
impl<'a, REG> Gint1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gint1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gint1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gint1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gint1Rule::EnumSP)
    }
}
#[doc = "Pin Interrupt and Pattern match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PintRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<PintRule> for u8 {
    #[inline(always)]
    fn from(variant: PintRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PintRule {
    type Ux = u8;
}
impl crate::IsEnum for PintRule {}
#[doc = "Field `PINT_RULE` reader - Pin Interrupt and Pattern match"]
pub type PintRuleR = crate::FieldReader<PintRule>;
impl PintRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PintRule {
        match self.bits {
            0 => PintRule::EnumNsNp,
            1 => PintRule::EnumNsP,
            2 => PintRule::EnumSNp,
            3 => PintRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PintRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PintRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PintRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PintRule::EnumSP
    }
}
#[doc = "Field `PINT_RULE` writer - Pin Interrupt and Pattern match"]
pub type PintRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, PintRule, crate::Safe>;
impl<'a, REG> PintRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PintRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PintRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PintRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PintRule::EnumSP)
    }
}
#[doc = "Secure Pin Interrupt and Pattern match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecPintRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SecPintRule> for u8 {
    #[inline(always)]
    fn from(variant: SecPintRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecPintRule {
    type Ux = u8;
}
impl crate::IsEnum for SecPintRule {}
#[doc = "Field `SEC_PINT_RULE` reader - Secure Pin Interrupt and Pattern match"]
pub type SecPintRuleR = crate::FieldReader<SecPintRule>;
impl SecPintRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecPintRule {
        match self.bits {
            0 => SecPintRule::EnumNsNp,
            1 => SecPintRule::EnumNsP,
            2 => SecPintRule::EnumSNp,
            3 => SecPintRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SecPintRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SecPintRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SecPintRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SecPintRule::EnumSP
    }
}
#[doc = "Field `SEC_PINT_RULE` writer - Secure Pin Interrupt and Pattern match"]
pub type SecPintRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecPintRule, crate::Safe>;
impl<'a, REG> SecPintRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SecPintRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SecPintRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SecPintRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SecPintRule::EnumSP)
    }
}
#[doc = "Peripheral input multiplexing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputmuxRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<InputmuxRule> for u8 {
    #[inline(always)]
    fn from(variant: InputmuxRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InputmuxRule {
    type Ux = u8;
}
impl crate::IsEnum for InputmuxRule {}
#[doc = "Field `INPUTMUX_RULE` reader - Peripheral input multiplexing"]
pub type InputmuxRuleR = crate::FieldReader<InputmuxRule>;
impl InputmuxRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputmuxRule {
        match self.bits {
            0 => InputmuxRule::EnumNsNp,
            1 => InputmuxRule::EnumNsP,
            2 => InputmuxRule::EnumSNp,
            3 => InputmuxRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == InputmuxRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == InputmuxRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == InputmuxRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == InputmuxRule::EnumSP
    }
}
#[doc = "Field `INPUTMUX_RULE` writer - Peripheral input multiplexing"]
pub type InputmuxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, InputmuxRule, crate::Safe>;
impl<'a, REG> InputmuxRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(InputmuxRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(InputmuxRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(InputmuxRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(InputmuxRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline(always)]
    pub fn syscon_rule(&self) -> SysconRuleR {
        SysconRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline(always)]
    pub fn iocon_rule(&self) -> IoconRuleR {
        IoconRuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline(always)]
    pub fn gint0_rule(&self) -> Gint0RuleR {
        Gint0RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline(always)]
    pub fn gint1_rule(&self) -> Gint1RuleR {
        Gint1RuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn pint_rule(&self) -> PintRuleR {
        PintRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn sec_pint_rule(&self) -> SecPintRuleR {
        SecPintRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral input multiplexing"]
    #[inline(always)]
    pub fn inputmux_rule(&self) -> InputmuxRuleR {
        InputmuxRuleR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline(always)]
    pub fn syscon_rule(&mut self) -> SysconRuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        SysconRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline(always)]
    pub fn iocon_rule(&mut self) -> IoconRuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        IoconRuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline(always)]
    pub fn gint0_rule(&mut self) -> Gint0RuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        Gint0RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline(always)]
    pub fn gint1_rule(&mut self) -> Gint1RuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        Gint1RuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn pint_rule(&mut self) -> PintRuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        PintRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline(always)]
    pub fn sec_pint_rule(&mut self) -> SecPintRuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        SecPintRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Peripheral input multiplexing"]
    #[inline(always)]
    pub fn inputmux_rule(&mut self) -> InputmuxRuleW<'_, SecCtrlApbBridge0MemCtrl0Spec> {
        InputmuxRuleW::new(self, 24)
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge0MemCtrl0Spec;
impl crate::RegisterSpec for SecCtrlApbBridge0MemCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge0_mem_ctrl0::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge0MemCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge0_mem_ctrl0::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge0MemCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 to value 0"]
impl crate::Resettable for SecCtrlApbBridge0MemCtrl0Spec {}

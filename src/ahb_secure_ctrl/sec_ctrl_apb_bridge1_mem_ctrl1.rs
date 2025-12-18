#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL1` reader"]
pub type R = crate::R<SecCtrlApbBridge1MemCtrl1Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL1` writer"]
pub type W = crate::W<SecCtrlApbBridge1MemCtrl1Spec>;
#[doc = "Standard counter/Timer 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctimer2Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ctimer2Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ctimer2Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctimer2Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ctimer2Rule {}
#[doc = "Field `CTIMER2_RULE` reader - Standard counter/Timer 2"]
pub type Ctimer2RuleR = crate::FieldReader<Ctimer2Rule>;
impl Ctimer2RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer2Rule {
        match self.bits {
            0 => Ctimer2Rule::EnumNsNp,
            1 => Ctimer2Rule::EnumNsP,
            2 => Ctimer2Rule::EnumSNp,
            3 => Ctimer2Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ctimer2Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ctimer2Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ctimer2Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ctimer2Rule::EnumSP
    }
}
#[doc = "Field `CTIMER2_RULE` writer - Standard counter/Timer 2"]
pub type Ctimer2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctimer2Rule, crate::Safe>;
impl<'a, REG> Ctimer2RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Rule::EnumSP)
    }
}
#[doc = "Standard counter/Timer 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctimer3Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ctimer3Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ctimer3Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctimer3Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ctimer3Rule {}
#[doc = "Field `CTIMER3_RULE` reader - Standard counter/Timer 3"]
pub type Ctimer3RuleR = crate::FieldReader<Ctimer3Rule>;
impl Ctimer3RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer3Rule {
        match self.bits {
            0 => Ctimer3Rule::EnumNsNp,
            1 => Ctimer3Rule::EnumNsP,
            2 => Ctimer3Rule::EnumSNp,
            3 => Ctimer3Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ctimer3Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ctimer3Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ctimer3Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ctimer3Rule::EnumSP
    }
}
#[doc = "Field `CTIMER3_RULE` writer - Standard counter/Timer 3"]
pub type Ctimer3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctimer3Rule, crate::Safe>;
impl<'a, REG> Ctimer3RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Rule::EnumSP)
    }
}
#[doc = "Standard counter/Timer 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctimer4Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ctimer4Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ctimer4Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctimer4Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ctimer4Rule {}
#[doc = "Field `CTIMER4_RULE` reader - Standard counter/Timer 4"]
pub type Ctimer4RuleR = crate::FieldReader<Ctimer4Rule>;
impl Ctimer4RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer4Rule {
        match self.bits {
            0 => Ctimer4Rule::EnumNsNp,
            1 => Ctimer4Rule::EnumNsP,
            2 => Ctimer4Rule::EnumSNp,
            3 => Ctimer4Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ctimer4Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ctimer4Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ctimer4Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ctimer4Rule::EnumSP
    }
}
#[doc = "Field `CTIMER4_RULE` writer - Standard counter/Timer 4"]
pub type Ctimer4RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctimer4Rule, crate::Safe>;
impl<'a, REG> Ctimer4RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Rule::EnumSP)
    }
}
#[doc = "Real Time Counter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RtcRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<RtcRule> for u8 {
    #[inline(always)]
    fn from(variant: RtcRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtcRule {
    type Ux = u8;
}
impl crate::IsEnum for RtcRule {}
#[doc = "Field `RTC_RULE` reader - Real Time Counter"]
pub type RtcRuleR = crate::FieldReader<RtcRule>;
impl RtcRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcRule {
        match self.bits {
            0 => RtcRule::EnumNsNp,
            1 => RtcRule::EnumNsP,
            2 => RtcRule::EnumSNp,
            3 => RtcRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RtcRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RtcRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RtcRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RtcRule::EnumSP
    }
}
#[doc = "Field `RTC_RULE` writer - Real Time Counter"]
pub type RtcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, RtcRule, crate::Safe>;
impl<'a, REG> RtcRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(RtcRule::EnumSP)
    }
}
#[doc = "OS Event Timer\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OseventRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<OseventRule> for u8 {
    #[inline(always)]
    fn from(variant: OseventRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OseventRule {
    type Ux = u8;
}
impl crate::IsEnum for OseventRule {}
#[doc = "Field `OSEVENT_RULE` reader - OS Event Timer"]
pub type OseventRuleR = crate::FieldReader<OseventRule>;
impl OseventRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OseventRule {
        match self.bits {
            0 => OseventRule::EnumNsNp,
            1 => OseventRule::EnumNsP,
            2 => OseventRule::EnumSNp,
            3 => OseventRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == OseventRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == OseventRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == OseventRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == OseventRule::EnumSP
    }
}
#[doc = "Field `OSEVENT_RULE` writer - OS Event Timer"]
pub type OseventRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, OseventRule, crate::Safe>;
impl<'a, REG> OseventRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(OseventRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(OseventRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(OseventRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(OseventRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    pub fn ctimer2_rule(&self) -> Ctimer2RuleR {
        Ctimer2RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    pub fn ctimer3_rule(&self) -> Ctimer3RuleR {
        Ctimer3RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    pub fn ctimer4_rule(&self) -> Ctimer4RuleR {
        Ctimer4RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    pub fn rtc_rule(&self) -> RtcRuleR {
        RtcRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    pub fn osevent_rule(&self) -> OseventRuleR {
        OseventRuleR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL1")
            .field("ctimer2_rule", &self.ctimer2_rule())
            .field("ctimer3_rule", &self.ctimer3_rule())
            .field("ctimer4_rule", &self.ctimer4_rule())
            .field("rtc_rule", &self.rtc_rule())
            .field("osevent_rule", &self.osevent_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline(always)]
    pub fn ctimer2_rule(&mut self) -> Ctimer2RuleW<'_, SecCtrlApbBridge1MemCtrl1Spec> {
        Ctimer2RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline(always)]
    pub fn ctimer3_rule(&mut self) -> Ctimer3RuleW<'_, SecCtrlApbBridge1MemCtrl1Spec> {
        Ctimer3RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline(always)]
    pub fn ctimer4_rule(&mut self) -> Ctimer4RuleW<'_, SecCtrlApbBridge1MemCtrl1Spec> {
        Ctimer4RuleW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline(always)]
    pub fn rtc_rule(&mut self) -> RtcRuleW<'_, SecCtrlApbBridge1MemCtrl1Spec> {
        RtcRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline(always)]
    pub fn osevent_rule(&mut self) -> OseventRuleW<'_, SecCtrlApbBridge1MemCtrl1Spec> {
        OseventRuleW::new(self, 20)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge1MemCtrl1Spec;
impl crate::RegisterSpec for SecCtrlApbBridge1MemCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge1_mem_ctrl1::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge1MemCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge1_mem_ctrl1::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge1MemCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 to value 0"]
impl crate::Resettable for SecCtrlApbBridge1MemCtrl1Spec {}

#[doc = "Register `SEC_CTRL_RAM2_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRam2SlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAM2_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRam2SlaveRuleSpec>;
#[doc = "Security access rules for the whole RAM2 : 0x2000_C000 - 0x2000_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ram2Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ram2Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ram2Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram2Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ram2Rule {}
#[doc = "Field `RAM2_RULE` reader - Security access rules for the whole RAM2 : 0x2000_C000 - 0x2000_FFFF"]
pub type Ram2RuleR = crate::FieldReader<Ram2Rule>;
impl Ram2RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram2Rule {
        match self.bits {
            0 => Ram2Rule::EnumNsNp,
            1 => Ram2Rule::EnumNsP,
            2 => Ram2Rule::EnumSNp,
            3 => Ram2Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ram2Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ram2Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ram2Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ram2Rule::EnumSP
    }
}
#[doc = "Field `RAM2_RULE` writer - Security access rules for the whole RAM2 : 0x2000_C000 - 0x2000_FFFF"]
pub type Ram2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ram2Rule, crate::Safe>;
impl<'a, REG> Ram2RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM2 : 0x2000_C000 - 0x2000_FFFF"]
    #[inline(always)]
    pub fn ram2_rule(&self) -> Ram2RuleR {
        Ram2RuleR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM2 : 0x2000_C000 - 0x2000_FFFF"]
    #[inline(always)]
    pub fn ram2_rule(&mut self) -> Ram2RuleW<'_, SecCtrlRam2SlaveRuleSpec> {
        Ram2RuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAM2 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram2_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram2_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam2SlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRam2SlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram2_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam2SlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram2_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam2SlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM2_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRam2SlaveRuleSpec {}

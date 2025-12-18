#[doc = "Register `SEC_CTRL_RAM1_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRam1SlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAM1_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRam1SlaveRuleSpec>;
#[doc = "Security access rules for the whole RAM1 : 0x2000_8000 - 0x2000_BFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ram1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ram1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ram1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ram1Rule {}
#[doc = "Field `RAM1_RULE` reader - Security access rules for the whole RAM1 : 0x2000_8000 - 0x2000_BFFF"]
pub type Ram1RuleR = crate::FieldReader<Ram1Rule>;
impl Ram1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram1Rule {
        match self.bits {
            0 => Ram1Rule::EnumNsNp,
            1 => Ram1Rule::EnumNsP,
            2 => Ram1Rule::EnumSNp,
            3 => Ram1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ram1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ram1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ram1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ram1Rule::EnumSP
    }
}
#[doc = "Field `RAM1_RULE` writer - Security access rules for the whole RAM1 : 0x2000_8000 - 0x2000_BFFF"]
pub type Ram1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ram1Rule, crate::Safe>;
impl<'a, REG> Ram1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM1 : 0x2000_8000 - 0x2000_BFFF"]
    #[inline(always)]
    pub fn ram1_rule(&self) -> Ram1RuleR {
        Ram1RuleR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM1 : 0x2000_8000 - 0x2000_BFFF"]
    #[inline(always)]
    pub fn ram1_rule(&mut self) -> Ram1RuleW<'_, SecCtrlRam1SlaveRuleSpec> {
        Ram1RuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAM1 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram1_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram1_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam1SlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRam1SlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram1_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam1SlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram1_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam1SlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM1_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRam1SlaveRuleSpec {}

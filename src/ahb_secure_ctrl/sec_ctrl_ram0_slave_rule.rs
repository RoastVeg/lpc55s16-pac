#[doc = "Register `SEC_CTRL_RAM0_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRam0SlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAM0_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRam0SlaveRuleSpec>;
#[doc = "Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_7FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ram0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ram0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ram0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ram0Rule {}
#[doc = "Field `RAM0_RULE` reader - Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_7FFF"]
pub type Ram0RuleR = crate::FieldReader<Ram0Rule>;
impl Ram0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram0Rule {
        match self.bits {
            0 => Ram0Rule::EnumNsNp,
            1 => Ram0Rule::EnumNsP,
            2 => Ram0Rule::EnumSNp,
            3 => Ram0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ram0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ram0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ram0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ram0Rule::EnumSP
    }
}
#[doc = "Field `RAM0_RULE` writer - Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_7FFF"]
pub type Ram0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ram0Rule, crate::Safe>;
impl<'a, REG> Ram0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_7FFF"]
    #[inline(always)]
    pub fn ram0_rule(&self) -> Ram0RuleR {
        Ram0RuleR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM0_SLAVE_RULE")
            .field("ram0_rule", &self.ram0_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM0 : 0x2000_0000 - 0x2000_7FFF"]
    #[inline(always)]
    pub fn ram0_rule(&mut self) -> Ram0RuleW<'_, SecCtrlRam0SlaveRuleSpec> {
        Ram0RuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAM0 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram0_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram0_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam0SlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRam0SlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram0_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam0SlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram0_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam0SlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM0_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRam0SlaveRuleSpec {}

#[doc = "Register `SEC_CTRL_RAMX_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRamxSlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAMX_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRamxSlaveRuleSpec>;
#[doc = "Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RamxRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<RamxRule> for u8 {
    #[inline(always)]
    fn from(variant: RamxRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RamxRule {
    type Ux = u8;
}
impl crate::IsEnum for RamxRule {}
#[doc = "Field `RAMX_RULE` reader - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
pub type RamxRuleR = crate::FieldReader<RamxRule>;
impl RamxRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamxRule {
        match self.bits {
            0 => RamxRule::EnumNsNp,
            1 => RamxRule::EnumNsP,
            2 => RamxRule::EnumSNp,
            3 => RamxRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RamxRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RamxRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RamxRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RamxRule::EnumSP
    }
}
#[doc = "Field `RAMX_RULE` writer - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
pub type RamxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, RamxRule, crate::Safe>;
impl<'a, REG> RamxRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(RamxRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(RamxRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(RamxRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(RamxRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[inline(always)]
    pub fn ramx_rule(&self) -> RamxRuleR {
        RamxRuleR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAMX_SLAVE_RULE")
            .field("ramx_rule", &self.ramx_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAMX : 0x0400_0000 - 0x0400_7FFF"]
    #[inline(always)]
    pub fn ramx_rule(&mut self) -> RamxRuleW<'_, SecCtrlRamxSlaveRuleSpec> {
        RamxRuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAMX slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ramx_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ramx_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRamxSlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRamxSlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ramx_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRamxSlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ramx_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRamxSlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAMX_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRamxSlaveRuleSpec {}

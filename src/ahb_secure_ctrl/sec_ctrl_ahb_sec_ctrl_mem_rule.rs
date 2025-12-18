#[doc = "Register `SEC_CTRL_AHB_SEC_CTRL_MEM_RULE` reader"]
pub type R = crate::R<SecCtrlAhbSecCtrlMemRuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_SEC_CTRL_MEM_RULE` writer"]
pub type W = crate::W<SecCtrlAhbSecCtrlMemRuleSpec>;
#[doc = "Address space: 0x400A_0000 - 0x400A_CFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbSecCtrlSect0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AhbSecCtrlSect0Rule> for u8 {
    #[inline(always)]
    fn from(variant: AhbSecCtrlSect0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbSecCtrlSect0Rule {
    type Ux = u8;
}
impl crate::IsEnum for AhbSecCtrlSect0Rule {}
#[doc = "Field `AHB_SEC_CTRL_SECT_0_RULE` reader - Address space: 0x400A_0000 - 0x400A_CFFF"]
pub type AhbSecCtrlSect0RuleR = crate::FieldReader<AhbSecCtrlSect0Rule>;
impl AhbSecCtrlSect0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbSecCtrlSect0Rule {
        match self.bits {
            0 => AhbSecCtrlSect0Rule::EnumNsNp,
            1 => AhbSecCtrlSect0Rule::EnumNsP,
            2 => AhbSecCtrlSect0Rule::EnumSNp,
            3 => AhbSecCtrlSect0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AhbSecCtrlSect0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AhbSecCtrlSect0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AhbSecCtrlSect0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AhbSecCtrlSect0Rule::EnumSP
    }
}
#[doc = "Field `AHB_SEC_CTRL_SECT_0_RULE` writer - Address space: 0x400A_0000 - 0x400A_CFFF"]
pub type AhbSecCtrlSect0RuleW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, AhbSecCtrlSect0Rule, crate::Safe>;
impl<'a, REG> AhbSecCtrlSect0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect0Rule::EnumSP)
    }
}
#[doc = "Address space: 0x400A_D000 - 0x400A_DFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbSecCtrlSect1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AhbSecCtrlSect1Rule> for u8 {
    #[inline(always)]
    fn from(variant: AhbSecCtrlSect1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbSecCtrlSect1Rule {
    type Ux = u8;
}
impl crate::IsEnum for AhbSecCtrlSect1Rule {}
#[doc = "Field `AHB_SEC_CTRL_SECT_1_RULE` reader - Address space: 0x400A_D000 - 0x400A_DFFF"]
pub type AhbSecCtrlSect1RuleR = crate::FieldReader<AhbSecCtrlSect1Rule>;
impl AhbSecCtrlSect1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbSecCtrlSect1Rule {
        match self.bits {
            0 => AhbSecCtrlSect1Rule::EnumNsNp,
            1 => AhbSecCtrlSect1Rule::EnumNsP,
            2 => AhbSecCtrlSect1Rule::EnumSNp,
            3 => AhbSecCtrlSect1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AhbSecCtrlSect1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AhbSecCtrlSect1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AhbSecCtrlSect1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AhbSecCtrlSect1Rule::EnumSP
    }
}
#[doc = "Field `AHB_SEC_CTRL_SECT_1_RULE` writer - Address space: 0x400A_D000 - 0x400A_DFFF"]
pub type AhbSecCtrlSect1RuleW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, AhbSecCtrlSect1Rule, crate::Safe>;
impl<'a, REG> AhbSecCtrlSect1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect1Rule::EnumSP)
    }
}
#[doc = "Address space: 0x400A_E000 - 0x400A_EFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbSecCtrlSect2Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AhbSecCtrlSect2Rule> for u8 {
    #[inline(always)]
    fn from(variant: AhbSecCtrlSect2Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbSecCtrlSect2Rule {
    type Ux = u8;
}
impl crate::IsEnum for AhbSecCtrlSect2Rule {}
#[doc = "Field `AHB_SEC_CTRL_SECT_2_RULE` reader - Address space: 0x400A_E000 - 0x400A_EFFF"]
pub type AhbSecCtrlSect2RuleR = crate::FieldReader<AhbSecCtrlSect2Rule>;
impl AhbSecCtrlSect2RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbSecCtrlSect2Rule {
        match self.bits {
            0 => AhbSecCtrlSect2Rule::EnumNsNp,
            1 => AhbSecCtrlSect2Rule::EnumNsP,
            2 => AhbSecCtrlSect2Rule::EnumSNp,
            3 => AhbSecCtrlSect2Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AhbSecCtrlSect2Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AhbSecCtrlSect2Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AhbSecCtrlSect2Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AhbSecCtrlSect2Rule::EnumSP
    }
}
#[doc = "Field `AHB_SEC_CTRL_SECT_2_RULE` writer - Address space: 0x400A_E000 - 0x400A_EFFF"]
pub type AhbSecCtrlSect2RuleW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, AhbSecCtrlSect2Rule, crate::Safe>;
impl<'a, REG> AhbSecCtrlSect2RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect2Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect2Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect2Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect2Rule::EnumSP)
    }
}
#[doc = "Address space: 0x400A_F000 - 0x400A_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbSecCtrlSect3Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AhbSecCtrlSect3Rule> for u8 {
    #[inline(always)]
    fn from(variant: AhbSecCtrlSect3Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbSecCtrlSect3Rule {
    type Ux = u8;
}
impl crate::IsEnum for AhbSecCtrlSect3Rule {}
#[doc = "Field `AHB_SEC_CTRL_SECT_3_RULE` reader - Address space: 0x400A_F000 - 0x400A_FFFF"]
pub type AhbSecCtrlSect3RuleR = crate::FieldReader<AhbSecCtrlSect3Rule>;
impl AhbSecCtrlSect3RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbSecCtrlSect3Rule {
        match self.bits {
            0 => AhbSecCtrlSect3Rule::EnumNsNp,
            1 => AhbSecCtrlSect3Rule::EnumNsP,
            2 => AhbSecCtrlSect3Rule::EnumSNp,
            3 => AhbSecCtrlSect3Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AhbSecCtrlSect3Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AhbSecCtrlSect3Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AhbSecCtrlSect3Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AhbSecCtrlSect3Rule::EnumSP
    }
}
#[doc = "Field `AHB_SEC_CTRL_SECT_3_RULE` writer - Address space: 0x400A_F000 - 0x400A_FFFF"]
pub type AhbSecCtrlSect3RuleW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, AhbSecCtrlSect3Rule, crate::Safe>;
impl<'a, REG> AhbSecCtrlSect3RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect3Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect3Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect3Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AhbSecCtrlSect3Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_0_rule(&self) -> AhbSecCtrlSect0RuleR {
        AhbSecCtrlSect0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_1_rule(&self) -> AhbSecCtrlSect1RuleR {
        AhbSecCtrlSect1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_2_rule(&self) -> AhbSecCtrlSect2RuleR {
        AhbSecCtrlSect2RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_3_rule(&self) -> AhbSecCtrlSect3RuleR {
        AhbSecCtrlSect3RuleR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address space: 0x400A_0000 - 0x400A_CFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_0_rule(
        &mut self,
    ) -> AhbSecCtrlSect0RuleW<'_, SecCtrlAhbSecCtrlMemRuleSpec> {
        AhbSecCtrlSect0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Address space: 0x400A_D000 - 0x400A_DFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_1_rule(
        &mut self,
    ) -> AhbSecCtrlSect1RuleW<'_, SecCtrlAhbSecCtrlMemRuleSpec> {
        AhbSecCtrlSect1RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Address space: 0x400A_E000 - 0x400A_EFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_2_rule(
        &mut self,
    ) -> AhbSecCtrlSect2RuleW<'_, SecCtrlAhbSecCtrlMemRuleSpec> {
        AhbSecCtrlSect2RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Address space: 0x400A_F000 - 0x400A_FFFF"]
    #[inline(always)]
    pub fn ahb_sec_ctrl_sect_3_rule(
        &mut self,
    ) -> AhbSecCtrlSect3RuleW<'_, SecCtrlAhbSecCtrlMemRuleSpec> {
        AhbSecCtrlSect3RuleW::new(self, 12)
    }
}
#[doc = "Security access rules for AHB_SEC_CTRL_AHB.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_sec_ctrl_mem_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_sec_ctrl_mem_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbSecCtrlMemRuleSpec;
impl crate::RegisterSpec for SecCtrlAhbSecCtrlMemRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_sec_ctrl_mem_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbSecCtrlMemRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_sec_ctrl_mem_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbSecCtrlMemRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_SEC_CTRL_MEM_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbSecCtrlMemRuleSpec {}

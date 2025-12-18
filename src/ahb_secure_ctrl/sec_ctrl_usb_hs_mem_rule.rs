#[doc = "Register `SEC_CTRL_USB_HS_MEM_RULE` reader"]
pub type R = crate::R<SecCtrlUsbHsMemRuleSpec>;
#[doc = "Register `SEC_CTRL_USB_HS_MEM_RULE` writer"]
pub type W = crate::W<SecCtrlUsbHsMemRuleSpec>;
#[doc = "Address space: 0x2001_0000 - 0x2001_0FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SramSect0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SramSect0Rule> for u8 {
    #[inline(always)]
    fn from(variant: SramSect0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SramSect0Rule {
    type Ux = u8;
}
impl crate::IsEnum for SramSect0Rule {}
#[doc = "Field `SRAM_SECT_0_RULE` reader - Address space: 0x2001_0000 - 0x2001_0FFF"]
pub type SramSect0RuleR = crate::FieldReader<SramSect0Rule>;
impl SramSect0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramSect0Rule {
        match self.bits {
            0 => SramSect0Rule::EnumNsNp,
            1 => SramSect0Rule::EnumNsP,
            2 => SramSect0Rule::EnumSNp,
            3 => SramSect0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SramSect0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SramSect0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SramSect0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SramSect0Rule::EnumSP
    }
}
#[doc = "Field `SRAM_SECT_0_RULE` writer - Address space: 0x2001_0000 - 0x2001_0FFF"]
pub type SramSect0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SramSect0Rule, crate::Safe>;
impl<'a, REG> SramSect0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect0Rule::EnumSP)
    }
}
#[doc = "Address space: 0x2001_1000 - 0x2001_1FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SramSect1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SramSect1Rule> for u8 {
    #[inline(always)]
    fn from(variant: SramSect1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SramSect1Rule {
    type Ux = u8;
}
impl crate::IsEnum for SramSect1Rule {}
#[doc = "Field `SRAM_SECT_1_RULE` reader - Address space: 0x2001_1000 - 0x2001_1FFF"]
pub type SramSect1RuleR = crate::FieldReader<SramSect1Rule>;
impl SramSect1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramSect1Rule {
        match self.bits {
            0 => SramSect1Rule::EnumNsNp,
            1 => SramSect1Rule::EnumNsP,
            2 => SramSect1Rule::EnumSNp,
            3 => SramSect1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SramSect1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SramSect1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SramSect1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SramSect1Rule::EnumSP
    }
}
#[doc = "Field `SRAM_SECT_1_RULE` writer - Address space: 0x2001_1000 - 0x2001_1FFF"]
pub type SramSect1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SramSect1Rule, crate::Safe>;
impl<'a, REG> SramSect1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect1Rule::EnumSP)
    }
}
#[doc = "Address space: 0x2001_2000 - 0x2001_2FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SramSect2Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SramSect2Rule> for u8 {
    #[inline(always)]
    fn from(variant: SramSect2Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SramSect2Rule {
    type Ux = u8;
}
impl crate::IsEnum for SramSect2Rule {}
#[doc = "Field `SRAM_SECT_2_RULE` reader - Address space: 0x2001_2000 - 0x2001_2FFF"]
pub type SramSect2RuleR = crate::FieldReader<SramSect2Rule>;
impl SramSect2RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramSect2Rule {
        match self.bits {
            0 => SramSect2Rule::EnumNsNp,
            1 => SramSect2Rule::EnumNsP,
            2 => SramSect2Rule::EnumSNp,
            3 => SramSect2Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SramSect2Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SramSect2Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SramSect2Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SramSect2Rule::EnumSP
    }
}
#[doc = "Field `SRAM_SECT_2_RULE` writer - Address space: 0x2001_2000 - 0x2001_2FFF"]
pub type SramSect2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SramSect2Rule, crate::Safe>;
impl<'a, REG> SramSect2RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect2Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect2Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect2Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect2Rule::EnumSP)
    }
}
#[doc = "Address space: 0x2001_3000 - 0x2001_3FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SramSect3Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SramSect3Rule> for u8 {
    #[inline(always)]
    fn from(variant: SramSect3Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SramSect3Rule {
    type Ux = u8;
}
impl crate::IsEnum for SramSect3Rule {}
#[doc = "Field `SRAM_SECT_3_RULE` reader - Address space: 0x2001_3000 - 0x2001_3FFF"]
pub type SramSect3RuleR = crate::FieldReader<SramSect3Rule>;
impl SramSect3RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramSect3Rule {
        match self.bits {
            0 => SramSect3Rule::EnumNsNp,
            1 => SramSect3Rule::EnumNsP,
            2 => SramSect3Rule::EnumSNp,
            3 => SramSect3Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SramSect3Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SramSect3Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SramSect3Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SramSect3Rule::EnumSP
    }
}
#[doc = "Field `SRAM_SECT_3_RULE` writer - Address space: 0x2001_3000 - 0x2001_3FFF"]
pub type SramSect3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SramSect3Rule, crate::Safe>;
impl<'a, REG> SramSect3RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect3Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect3Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect3Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SramSect3Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Address space: 0x2001_0000 - 0x2001_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&self) -> SramSect0RuleR {
        SramSect0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Address space: 0x2001_1000 - 0x2001_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&self) -> SramSect1RuleR {
        SramSect1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Address space: 0x2001_2000 - 0x2001_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&self) -> SramSect2RuleR {
        SramSect2RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Address space: 0x2001_3000 - 0x2001_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&self) -> SramSect3RuleR {
        SramSect3RuleR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_USB_HS_MEM_RULE")
            .field("sram_sect_0_rule", &self.sram_sect_0_rule())
            .field("sram_sect_1_rule", &self.sram_sect_1_rule())
            .field("sram_sect_2_rule", &self.sram_sect_2_rule())
            .field("sram_sect_3_rule", &self.sram_sect_3_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Address space: 0x2001_0000 - 0x2001_0FFF"]
    #[inline(always)]
    pub fn sram_sect_0_rule(&mut self) -> SramSect0RuleW<'_, SecCtrlUsbHsMemRuleSpec> {
        SramSect0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Address space: 0x2001_1000 - 0x2001_1FFF"]
    #[inline(always)]
    pub fn sram_sect_1_rule(&mut self) -> SramSect1RuleW<'_, SecCtrlUsbHsMemRuleSpec> {
        SramSect1RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Address space: 0x2001_2000 - 0x2001_2FFF"]
    #[inline(always)]
    pub fn sram_sect_2_rule(&mut self) -> SramSect2RuleW<'_, SecCtrlUsbHsMemRuleSpec> {
        SramSect2RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Address space: 0x2001_3000 - 0x2001_3FFF"]
    #[inline(always)]
    pub fn sram_sect_3_rule(&mut self) -> SramSect3RuleW<'_, SecCtrlUsbHsMemRuleSpec> {
        SramSect3RuleW::new(self, 12)
    }
}
#[doc = "Security access rules for RAM_USB_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_usb_hs_mem_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_usb_hs_mem_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlUsbHsMemRuleSpec;
impl crate::RegisterSpec for SecCtrlUsbHsMemRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_usb_hs_mem_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlUsbHsMemRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_usb_hs_mem_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlUsbHsMemRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_USB_HS_MEM_RULE to value 0"]
impl crate::Resettable for SecCtrlUsbHsMemRuleSpec {}

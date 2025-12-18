#[doc = "Register `SEC_CTRL_AHB_PORT7_SLAVE0_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort7Slave0RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT7_SLAVE0_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort7Slave0RuleSpec>;
#[doc = "DMA Controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Dma0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Dma0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Dma0Rule {}
#[doc = "Field `DMA0_RULE` reader - DMA Controller"]
pub type Dma0RuleR = crate::FieldReader<Dma0Rule>;
impl Dma0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0Rule {
        match self.bits {
            0 => Dma0Rule::EnumNsNp,
            1 => Dma0Rule::EnumNsP,
            2 => Dma0Rule::EnumSNp,
            3 => Dma0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Dma0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Dma0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Dma0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Dma0Rule::EnumSP
    }
}
#[doc = "Field `DMA0_RULE` writer - DMA Controller"]
pub type Dma0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0Rule, crate::Safe>;
impl<'a, REG> Dma0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Rule::EnumSP)
    }
}
#[doc = "USB Full-speed device\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FsUsbDevRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<FsUsbDevRule> for u8 {
    #[inline(always)]
    fn from(variant: FsUsbDevRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FsUsbDevRule {
    type Ux = u8;
}
impl crate::IsEnum for FsUsbDevRule {}
#[doc = "Field `FS_USB_DEV_RULE` reader - USB Full-speed device"]
pub type FsUsbDevRuleR = crate::FieldReader<FsUsbDevRule>;
impl FsUsbDevRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsUsbDevRule {
        match self.bits {
            0 => FsUsbDevRule::EnumNsNp,
            1 => FsUsbDevRule::EnumNsP,
            2 => FsUsbDevRule::EnumSNp,
            3 => FsUsbDevRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FsUsbDevRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FsUsbDevRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FsUsbDevRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FsUsbDevRule::EnumSP
    }
}
#[doc = "Field `FS_USB_DEV_RULE` writer - USB Full-speed device"]
pub type FsUsbDevRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, FsUsbDevRule, crate::Safe>;
impl<'a, REG> FsUsbDevRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(FsUsbDevRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(FsUsbDevRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(FsUsbDevRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(FsUsbDevRule::EnumSP)
    }
}
#[doc = "SCTimer\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SctRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<SctRule> for u8 {
    #[inline(always)]
    fn from(variant: SctRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SctRule {
    type Ux = u8;
}
impl crate::IsEnum for SctRule {}
#[doc = "Field `SCT_RULE` reader - SCTimer"]
pub type SctRuleR = crate::FieldReader<SctRule>;
impl SctRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctRule {
        match self.bits {
            0 => SctRule::EnumNsNp,
            1 => SctRule::EnumNsP,
            2 => SctRule::EnumSNp,
            3 => SctRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SctRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SctRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SctRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SctRule::EnumSP
    }
}
#[doc = "Field `SCT_RULE` writer - SCTimer"]
pub type SctRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, SctRule, crate::Safe>;
impl<'a, REG> SctRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(SctRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(SctRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(SctRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(SctRule::EnumSP)
    }
}
#[doc = "Flexcomm interface 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm0Rule {}
#[doc = "Field `FLEXCOMM0_RULE` reader - Flexcomm interface 0"]
pub type Flexcomm0RuleR = crate::FieldReader<Flexcomm0Rule>;
impl Flexcomm0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0Rule {
        match self.bits {
            0 => Flexcomm0Rule::EnumNsNp,
            1 => Flexcomm0Rule::EnumNsP,
            2 => Flexcomm0Rule::EnumSNp,
            3 => Flexcomm0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm0Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM0_RULE` writer - Flexcomm interface 0"]
pub type Flexcomm0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm0Rule, crate::Safe>;
impl<'a, REG> Flexcomm0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rule::EnumSP)
    }
}
#[doc = "Flexcomm interface 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm1Rule {}
#[doc = "Field `FLEXCOMM1_RULE` reader - Flexcomm interface 1"]
pub type Flexcomm1RuleR = crate::FieldReader<Flexcomm1Rule>;
impl Flexcomm1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1Rule {
        match self.bits {
            0 => Flexcomm1Rule::EnumNsNp,
            1 => Flexcomm1Rule::EnumNsP,
            2 => Flexcomm1Rule::EnumSNp,
            3 => Flexcomm1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm1Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM1_RULE` writer - Flexcomm interface 1"]
pub type Flexcomm1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm1Rule, crate::Safe>;
impl<'a, REG> Flexcomm1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    pub fn dma0_rule(&self) -> Dma0RuleR {
        Dma0RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    pub fn fs_usb_dev_rule(&self) -> FsUsbDevRuleR {
        FsUsbDevRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    pub fn sct_rule(&self) -> SctRuleR {
        SctRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    pub fn flexcomm0_rule(&self) -> Flexcomm0RuleR {
        Flexcomm0RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    pub fn flexcomm1_rule(&self) -> Flexcomm1RuleR {
        Flexcomm1RuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT7_SLAVE0_RULE")
            .field("dma0_rule", &self.dma0_rule())
            .field("fs_usb_dev_rule", &self.fs_usb_dev_rule())
            .field("sct_rule", &self.sct_rule())
            .field("flexcomm0_rule", &self.flexcomm0_rule())
            .field("flexcomm1_rule", &self.flexcomm1_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    pub fn dma0_rule(&mut self) -> Dma0RuleW<'_, SecCtrlAhbPort7Slave0RuleSpec> {
        Dma0RuleW::new(self, 8)
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    pub fn fs_usb_dev_rule(&mut self) -> FsUsbDevRuleW<'_, SecCtrlAhbPort7Slave0RuleSpec> {
        FsUsbDevRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    pub fn sct_rule(&mut self) -> SctRuleW<'_, SecCtrlAhbPort7Slave0RuleSpec> {
        SctRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    pub fn flexcomm0_rule(&mut self) -> Flexcomm0RuleW<'_, SecCtrlAhbPort7Slave0RuleSpec> {
        Flexcomm0RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    pub fn flexcomm1_rule(&mut self) -> Flexcomm1RuleW<'_, SecCtrlAhbPort7Slave0RuleSpec> {
        Flexcomm1RuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port7_slave0_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port7_slave0_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort7Slave0RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort7Slave0RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port7_slave0_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort7Slave0RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port7_slave0_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort7Slave0RuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT7_SLAVE0_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort7Slave0RuleSpec {}

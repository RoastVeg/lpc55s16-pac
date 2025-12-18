#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort9Slave0RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort9Slave0RuleSpec>;
#[doc = "ADC\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AdcRule> for u8 {
    #[inline(always)]
    fn from(variant: AdcRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcRule {
    type Ux = u8;
}
impl crate::IsEnum for AdcRule {}
#[doc = "Field `ADC_RULE` reader - ADC"]
pub type AdcRuleR = crate::FieldReader<AdcRule>;
impl AdcRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcRule {
        match self.bits {
            0 => AdcRule::EnumNsNp,
            1 => AdcRule::EnumNsP,
            2 => AdcRule::EnumSNp,
            3 => AdcRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AdcRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AdcRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AdcRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AdcRule::EnumSP
    }
}
#[doc = "Field `ADC_RULE` writer - ADC"]
pub type AdcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, AdcRule, crate::Safe>;
impl<'a, REG> AdcRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AdcRule::EnumSP)
    }
}
#[doc = "USB Full Speed Host registers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbFsHostRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<UsbFsHostRule> for u8 {
    #[inline(always)]
    fn from(variant: UsbFsHostRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbFsHostRule {
    type Ux = u8;
}
impl crate::IsEnum for UsbFsHostRule {}
#[doc = "Field `USB_FS_HOST_RULE` reader - USB Full Speed Host registers."]
pub type UsbFsHostRuleR = crate::FieldReader<UsbFsHostRule>;
impl UsbFsHostRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbFsHostRule {
        match self.bits {
            0 => UsbFsHostRule::EnumNsNp,
            1 => UsbFsHostRule::EnumNsP,
            2 => UsbFsHostRule::EnumSNp,
            3 => UsbFsHostRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UsbFsHostRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UsbFsHostRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UsbFsHostRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UsbFsHostRule::EnumSP
    }
}
#[doc = "Field `USB_FS_HOST_RULE` writer - USB Full Speed Host registers."]
pub type UsbFsHostRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, UsbFsHostRule, crate::Safe>;
impl<'a, REG> UsbFsHostRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsHostRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsHostRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsHostRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsHostRule::EnumSP)
    }
}
#[doc = "USB High speed host registers\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbHsHostRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<UsbHsHostRule> for u8 {
    #[inline(always)]
    fn from(variant: UsbHsHostRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbHsHostRule {
    type Ux = u8;
}
impl crate::IsEnum for UsbHsHostRule {}
#[doc = "Field `USB_HS_HOST_RULE` reader - USB High speed host registers"]
pub type UsbHsHostRuleR = crate::FieldReader<UsbHsHostRule>;
impl UsbHsHostRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbHsHostRule {
        match self.bits {
            0 => UsbHsHostRule::EnumNsNp,
            1 => UsbHsHostRule::EnumNsP,
            2 => UsbHsHostRule::EnumSNp,
            3 => UsbHsHostRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UsbHsHostRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UsbHsHostRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UsbHsHostRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UsbHsHostRule::EnumSP
    }
}
#[doc = "Field `USB_HS_HOST_RULE` writer - USB High speed host registers"]
pub type UsbHsHostRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, UsbHsHostRule, crate::Safe>;
impl<'a, REG> UsbHsHostRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsHostRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsHostRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsHostRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsHostRule::EnumSP)
    }
}
#[doc = "SHA-2 crypto registers\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HashRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<HashRule> for u8 {
    #[inline(always)]
    fn from(variant: HashRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HashRule {
    type Ux = u8;
}
impl crate::IsEnum for HashRule {}
#[doc = "Field `HASH_RULE` reader - SHA-2 crypto registers"]
pub type HashRuleR = crate::FieldReader<HashRule>;
impl HashRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashRule {
        match self.bits {
            0 => HashRule::EnumNsNp,
            1 => HashRule::EnumNsP,
            2 => HashRule::EnumSNp,
            3 => HashRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HashRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HashRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HashRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HashRule::EnumSP
    }
}
#[doc = "Field `HASH_RULE` writer - SHA-2 crypto registers"]
pub type HashRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, HashRule, crate::Safe>;
impl<'a, REG> HashRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(HashRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(HashRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(HashRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(HashRule::EnumSP)
    }
}
#[doc = "RSA/ECC crypto accelerator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CasperRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<CasperRule> for u8 {
    #[inline(always)]
    fn from(variant: CasperRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CasperRule {
    type Ux = u8;
}
impl crate::IsEnum for CasperRule {}
#[doc = "Field `CASPER_RULE` reader - RSA/ECC crypto accelerator"]
pub type CasperRuleR = crate::FieldReader<CasperRule>;
impl CasperRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperRule {
        match self.bits {
            0 => CasperRule::EnumNsNp,
            1 => CasperRule::EnumNsP,
            2 => CasperRule::EnumSNp,
            3 => CasperRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CasperRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CasperRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CasperRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CasperRule::EnumSP
    }
}
#[doc = "Field `CASPER_RULE` writer - RSA/ECC crypto accelerator"]
pub type CasperRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, CasperRule, crate::Safe>;
impl<'a, REG> CasperRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(CasperRule::EnumSP)
    }
}
#[doc = "DMA Controller (Secure)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Dma1Rule> for u8 {
    #[inline(always)]
    fn from(variant: Dma1Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1Rule {
    type Ux = u8;
}
impl crate::IsEnum for Dma1Rule {}
#[doc = "Field `DMA1_RULE` reader - DMA Controller (Secure)"]
pub type Dma1RuleR = crate::FieldReader<Dma1Rule>;
impl Dma1RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1Rule {
        match self.bits {
            0 => Dma1Rule::EnumNsNp,
            1 => Dma1Rule::EnumNsP,
            2 => Dma1Rule::EnumSNp,
            3 => Dma1Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Dma1Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Dma1Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Dma1Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Dma1Rule::EnumSP
    }
}
#[doc = "Field `DMA1_RULE` writer - DMA Controller (Secure)"]
pub type Dma1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma1Rule, crate::Safe>;
impl<'a, REG> Dma1RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&self) -> AdcRuleR {
        AdcRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - USB Full Speed Host registers."]
    #[inline(always)]
    pub fn usb_fs_host_rule(&self) -> UsbFsHostRuleR {
        UsbFsHostRuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - USB High speed host registers"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&self) -> UsbHsHostRuleR {
        UsbHsHostRuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&self) -> HashRuleR {
        HashRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&self) -> CasperRuleR {
        CasperRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&self) -> Dma1RuleR {
        Dma1RuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT9_SLAVE0_RULE")
            .field("adc_rule", &self.adc_rule())
            .field("usb_fs_host_rule", &self.usb_fs_host_rule())
            .field("usb_hs_host_rule", &self.usb_hs_host_rule())
            .field("hash_rule", &self.hash_rule())
            .field("casper_rule", &self.casper_rule())
            .field("dma1_rule", &self.dma1_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&mut self) -> AdcRuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        AdcRuleW::new(self, 0)
    }
    #[doc = "Bits 8:9 - USB Full Speed Host registers."]
    #[inline(always)]
    pub fn usb_fs_host_rule(&mut self) -> UsbFsHostRuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        UsbFsHostRuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - USB High speed host registers"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&mut self) -> UsbHsHostRuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        UsbHsHostRuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&mut self) -> HashRuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        HashRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&mut self) -> CasperRuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        CasperRuleW::new(self, 20)
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&mut self) -> Dma1RuleW<'_, SecCtrlAhbPort9Slave0RuleSpec> {
        Dma1RuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port9_slave0_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port9_slave0_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort9Slave0RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort9Slave0RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port9_slave0_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort9Slave0RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port9_slave0_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort9Slave0RuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT9_SLAVE0_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort9Slave0RuleSpec {}

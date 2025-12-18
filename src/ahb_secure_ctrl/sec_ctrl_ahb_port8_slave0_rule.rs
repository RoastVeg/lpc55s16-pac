#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort8Slave0RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort8Slave0RuleSpec>;
#[doc = "USB high Speed device registers\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbHsDevRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<UsbHsDevRule> for u8 {
    #[inline(always)]
    fn from(variant: UsbHsDevRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbHsDevRule {
    type Ux = u8;
}
impl crate::IsEnum for UsbHsDevRule {}
#[doc = "Field `USB_HS_DEV_RULE` reader - USB high Speed device registers"]
pub type UsbHsDevRuleR = crate::FieldReader<UsbHsDevRule>;
impl UsbHsDevRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbHsDevRule {
        match self.bits {
            0 => UsbHsDevRule::EnumNsNp,
            1 => UsbHsDevRule::EnumNsP,
            2 => UsbHsDevRule::EnumSNp,
            3 => UsbHsDevRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UsbHsDevRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UsbHsDevRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UsbHsDevRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UsbHsDevRule::EnumSP
    }
}
#[doc = "Field `USB_HS_DEV_RULE` writer - USB high Speed device registers"]
pub type UsbHsDevRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, UsbHsDevRule, crate::Safe>;
impl<'a, REG> UsbHsDevRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsDevRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsDevRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsDevRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbHsDevRule::EnumSP)
    }
}
#[doc = "CRC engine\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CrcRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<CrcRule> for u8 {
    #[inline(always)]
    fn from(variant: CrcRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CrcRule {
    type Ux = u8;
}
impl crate::IsEnum for CrcRule {}
#[doc = "Field `CRC_RULE` reader - CRC engine"]
pub type CrcRuleR = crate::FieldReader<CrcRule>;
impl CrcRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcRule {
        match self.bits {
            0 => CrcRule::EnumNsNp,
            1 => CrcRule::EnumNsP,
            2 => CrcRule::EnumSNp,
            3 => CrcRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CrcRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CrcRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CrcRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CrcRule::EnumSP
    }
}
#[doc = "Field `CRC_RULE` writer - CRC engine"]
pub type CrcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, CrcRule, crate::Safe>;
impl<'a, REG> CrcRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRule::EnumSP)
    }
}
#[doc = "Flexcomm interface 5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm5Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm5Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm5Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm5Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm5Rule {}
#[doc = "Field `FLEXCOMM5_RULE` reader - Flexcomm interface 5"]
pub type Flexcomm5RuleR = crate::FieldReader<Flexcomm5Rule>;
impl Flexcomm5RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5Rule {
        match self.bits {
            0 => Flexcomm5Rule::EnumNsNp,
            1 => Flexcomm5Rule::EnumNsP,
            2 => Flexcomm5Rule::EnumSNp,
            3 => Flexcomm5Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm5Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm5Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm5Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm5Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM5_RULE` writer - Flexcomm interface 5"]
pub type Flexcomm5RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm5Rule, crate::Safe>;
impl<'a, REG> Flexcomm5RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rule::EnumSP)
    }
}
#[doc = "Flexcomm interface 6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm6Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm6Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm6Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm6Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm6Rule {}
#[doc = "Field `FLEXCOMM6_RULE` reader - Flexcomm interface 6"]
pub type Flexcomm6RuleR = crate::FieldReader<Flexcomm6Rule>;
impl Flexcomm6RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6Rule {
        match self.bits {
            0 => Flexcomm6Rule::EnumNsNp,
            1 => Flexcomm6Rule::EnumNsP,
            2 => Flexcomm6Rule::EnumSNp,
            3 => Flexcomm6Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm6Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm6Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm6Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm6Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM6_RULE` writer - Flexcomm interface 6"]
pub type Flexcomm6RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm6Rule, crate::Safe>;
impl<'a, REG> Flexcomm6RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 16:17 - USB high Speed device registers"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&self) -> UsbHsDevRuleR {
        UsbHsDevRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CRC engine"]
    #[inline(always)]
    pub fn crc_rule(&self) -> CrcRuleR {
        CrcRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5"]
    #[inline(always)]
    pub fn flexcomm5_rule(&self) -> Flexcomm5RuleR {
        Flexcomm5RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6"]
    #[inline(always)]
    pub fn flexcomm6_rule(&self) -> Flexcomm6RuleR {
        Flexcomm6RuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_AHB_PORT8_SLAVE0_RULE")
            .field("usb_hs_dev_rule", &self.usb_hs_dev_rule())
            .field("crc_rule", &self.crc_rule())
            .field("flexcomm5_rule", &self.flexcomm5_rule())
            .field("flexcomm6_rule", &self.flexcomm6_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:17 - USB high Speed device registers"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&mut self) -> UsbHsDevRuleW<'_, SecCtrlAhbPort8Slave0RuleSpec> {
        UsbHsDevRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - CRC engine"]
    #[inline(always)]
    pub fn crc_rule(&mut self) -> CrcRuleW<'_, SecCtrlAhbPort8Slave0RuleSpec> {
        CrcRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5"]
    #[inline(always)]
    pub fn flexcomm5_rule(&mut self) -> Flexcomm5RuleW<'_, SecCtrlAhbPort8Slave0RuleSpec> {
        Flexcomm5RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6"]
    #[inline(always)]
    pub fn flexcomm6_rule(&mut self) -> Flexcomm6RuleW<'_, SecCtrlAhbPort8Slave0RuleSpec> {
        Flexcomm6RuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port8_slave0_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port8_slave0_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort8Slave0RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort8Slave0RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port8_slave0_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort8Slave0RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port8_slave0_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort8Slave0RuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE0_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort8Slave0RuleSpec {}

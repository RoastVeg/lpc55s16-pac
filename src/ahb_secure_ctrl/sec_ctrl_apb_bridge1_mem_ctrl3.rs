#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL3` reader"]
pub type R = crate::R<SecCtrlApbBridge1MemCtrl3Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL3` writer"]
pub type W = crate::W<SecCtrlApbBridge1MemCtrl3Spec>;
#[doc = "USB High Speed Phy controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbhphyRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<UsbhphyRule> for u8 {
    #[inline(always)]
    fn from(variant: UsbhphyRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbhphyRule {
    type Ux = u8;
}
impl crate::IsEnum for UsbhphyRule {}
#[doc = "Field `USBHPHY_RULE` reader - USB High Speed Phy controller"]
pub type UsbhphyRuleR = crate::FieldReader<UsbhphyRule>;
impl UsbhphyRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhphyRule {
        match self.bits {
            0 => UsbhphyRule::EnumNsNp,
            1 => UsbhphyRule::EnumNsP,
            2 => UsbhphyRule::EnumSNp,
            3 => UsbhphyRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == UsbhphyRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == UsbhphyRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == UsbhphyRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == UsbhphyRule::EnumSP
    }
}
#[doc = "Field `USBHPHY_RULE` writer - USB High Speed Phy controller"]
pub type UsbhphyRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, UsbhphyRule, crate::Safe>;
impl<'a, REG> UsbhphyRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhphyRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhphyRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhphyRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhphyRule::EnumSP)
    }
}
#[doc = "True Random Number Generator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RngRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<RngRule> for u8 {
    #[inline(always)]
    fn from(variant: RngRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RngRule {
    type Ux = u8;
}
impl crate::IsEnum for RngRule {}
#[doc = "Field `RNG_RULE` reader - True Random Number Generator"]
pub type RngRuleR = crate::FieldReader<RngRule>;
impl RngRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RngRule {
        match self.bits {
            0 => RngRule::EnumNsNp,
            1 => RngRule::EnumNsP,
            2 => RngRule::EnumSNp,
            3 => RngRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RngRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RngRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RngRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RngRule::EnumSP
    }
}
#[doc = "Field `RNG_RULE` writer - True Random Number Generator"]
pub type RngRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, RngRule, crate::Safe>;
impl<'a, REG> RngRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(RngRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(RngRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(RngRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(RngRule::EnumSP)
    }
}
#[doc = "PUF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PufRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<PufRule> for u8 {
    #[inline(always)]
    fn from(variant: PufRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PufRule {
    type Ux = u8;
}
impl crate::IsEnum for PufRule {}
#[doc = "Field `PUF_RULE` reader - PUF"]
pub type PufRuleR = crate::FieldReader<PufRule>;
impl PufRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PufRule {
        match self.bits {
            0 => PufRule::EnumNsNp,
            1 => PufRule::EnumNsP,
            2 => PufRule::EnumSNp,
            3 => PufRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PufRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PufRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PufRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PufRule::EnumSP
    }
}
#[doc = "Field `PUF_RULE` writer - PUF"]
pub type PufRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, PufRule, crate::Safe>;
impl<'a, REG> PufRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PufRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PufRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PufRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PufRule::EnumSP)
    }
}
#[doc = "Programmable Look-Up logic\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PluRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<PluRule> for u8 {
    #[inline(always)]
    fn from(variant: PluRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PluRule {
    type Ux = u8;
}
impl crate::IsEnum for PluRule {}
#[doc = "Field `PLU_RULE` reader - Programmable Look-Up logic"]
pub type PluRuleR = crate::FieldReader<PluRule>;
impl PluRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PluRule {
        match self.bits {
            0 => PluRule::EnumNsNp,
            1 => PluRule::EnumNsP,
            2 => PluRule::EnumSNp,
            3 => PluRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PluRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PluRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PluRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PluRule::EnumSP
    }
}
#[doc = "Field `PLU_RULE` writer - Programmable Look-Up logic"]
pub type PluRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, PluRule, crate::Safe>;
impl<'a, REG> PluRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PluRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PluRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PluRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PluRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - USB High Speed Phy controller"]
    #[inline(always)]
    pub fn usbhphy_rule(&self) -> UsbhphyRuleR {
        UsbhphyRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - True Random Number Generator"]
    #[inline(always)]
    pub fn rng_rule(&self) -> RngRuleR {
        RngRuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PUF"]
    #[inline(always)]
    pub fn puf_rule(&self) -> PufRuleR {
        PufRuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Programmable Look-Up logic"]
    #[inline(always)]
    pub fn plu_rule(&self) -> PluRuleR {
        PluRuleR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE1_MEM_CTRL3")
            .field("usbhphy_rule", &self.usbhphy_rule())
            .field("rng_rule", &self.rng_rule())
            .field("puf_rule", &self.puf_rule())
            .field("plu_rule", &self.plu_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - USB High Speed Phy controller"]
    #[inline(always)]
    pub fn usbhphy_rule(&mut self) -> UsbhphyRuleW<'_, SecCtrlApbBridge1MemCtrl3Spec> {
        UsbhphyRuleW::new(self, 0)
    }
    #[doc = "Bits 8:9 - True Random Number Generator"]
    #[inline(always)]
    pub fn rng_rule(&mut self) -> RngRuleW<'_, SecCtrlApbBridge1MemCtrl3Spec> {
        RngRuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - PUF"]
    #[inline(always)]
    pub fn puf_rule(&mut self) -> PufRuleW<'_, SecCtrlApbBridge1MemCtrl3Spec> {
        PufRuleW::new(self, 12)
    }
    #[doc = "Bits 20:21 - Programmable Look-Up logic"]
    #[inline(always)]
    pub fn plu_rule(&mut self) -> PluRuleW<'_, SecCtrlApbBridge1MemCtrl3Spec> {
        PluRuleW::new(self, 20)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge1MemCtrl3Spec;
impl crate::RegisterSpec for SecCtrlApbBridge1MemCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge1_mem_ctrl3::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge1MemCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge1_mem_ctrl3::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge1MemCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 to value 0"]
impl crate::Resettable for SecCtrlApbBridge1MemCtrl3Spec {}

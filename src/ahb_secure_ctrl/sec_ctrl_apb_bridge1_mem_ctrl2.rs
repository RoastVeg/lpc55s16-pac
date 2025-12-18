#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL2` reader"]
pub type R = crate::R<SecCtrlApbBridge1MemCtrl2Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE1_MEM_CTRL2` writer"]
pub type W = crate::W<SecCtrlApbBridge1MemCtrl2Spec>;
#[doc = "Flash Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashCtrlRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<FlashCtrlRule> for u8 {
    #[inline(always)]
    fn from(variant: FlashCtrlRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FlashCtrlRule {
    type Ux = u8;
}
impl crate::IsEnum for FlashCtrlRule {}
#[doc = "Field `FLASH_CTRL_RULE` reader - Flash Controller"]
pub type FlashCtrlRuleR = crate::FieldReader<FlashCtrlRule>;
impl FlashCtrlRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashCtrlRule {
        match self.bits {
            0 => FlashCtrlRule::EnumNsNp,
            1 => FlashCtrlRule::EnumNsP,
            2 => FlashCtrlRule::EnumSNp,
            3 => FlashCtrlRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FlashCtrlRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FlashCtrlRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FlashCtrlRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FlashCtrlRule::EnumSP
    }
}
#[doc = "Field `FLASH_CTRL_RULE` writer - Flash Controller"]
pub type FlashCtrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, FlashCtrlRule, crate::Safe>;
impl<'a, REG> FlashCtrlRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(FlashCtrlRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(FlashCtrlRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(FlashCtrlRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(FlashCtrlRule::EnumSP)
    }
}
#[doc = "Prince\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PrinceRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<PrinceRule> for u8 {
    #[inline(always)]
    fn from(variant: PrinceRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PrinceRule {
    type Ux = u8;
}
impl crate::IsEnum for PrinceRule {}
#[doc = "Field `PRINCE_RULE` reader - Prince"]
pub type PrinceRuleR = crate::FieldReader<PrinceRule>;
impl PrinceRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrinceRule {
        match self.bits {
            0 => PrinceRule::EnumNsNp,
            1 => PrinceRule::EnumNsP,
            2 => PrinceRule::EnumSNp,
            3 => PrinceRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PrinceRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PrinceRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PrinceRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PrinceRule::EnumSP
    }
}
#[doc = "Field `PRINCE_RULE` writer - Prince"]
pub type PrinceRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, PrinceRule, crate::Safe>;
impl<'a, REG> PrinceRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PrinceRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PrinceRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PrinceRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PrinceRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 16:17 - Flash Controller"]
    #[inline(always)]
    pub fn flash_ctrl_rule(&self) -> FlashCtrlRuleR {
        FlashCtrlRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Prince"]
    #[inline(always)]
    pub fn prince_rule(&self) -> PrinceRuleR {
        PrinceRuleR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Flash Controller"]
    #[inline(always)]
    pub fn flash_ctrl_rule(&mut self) -> FlashCtrlRuleW<'_, SecCtrlApbBridge1MemCtrl2Spec> {
        FlashCtrlRuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Prince"]
    #[inline(always)]
    pub fn prince_rule(&mut self) -> PrinceRuleW<'_, SecCtrlApbBridge1MemCtrl2Spec> {
        PrinceRuleW::new(self, 20)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge1_mem_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge1_mem_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge1MemCtrl2Spec;
impl crate::RegisterSpec for SecCtrlApbBridge1MemCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge1_mem_ctrl2::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge1MemCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge1_mem_ctrl2::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge1MemCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 to value 0"]
impl crate::Resettable for SecCtrlApbBridge1MemCtrl2Spec {}

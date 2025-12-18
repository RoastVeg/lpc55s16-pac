#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL2` reader"]
pub type R = crate::R<SecCtrlApbBridge0MemCtrl2Spec>;
#[doc = "Register `SEC_CTRL_APB_BRIDGE0_MEM_CTRL2` writer"]
pub type W = crate::W<SecCtrlApbBridge0MemCtrl2Spec>;
#[doc = "Analog Modules controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AnactrlRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<AnactrlRule> for u8 {
    #[inline(always)]
    fn from(variant: AnactrlRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AnactrlRule {
    type Ux = u8;
}
impl crate::IsEnum for AnactrlRule {}
#[doc = "Field `ANACTRL_RULE` reader - Analog Modules controller"]
pub type AnactrlRuleR = crate::FieldReader<AnactrlRule>;
impl AnactrlRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnactrlRule {
        match self.bits {
            0 => AnactrlRule::EnumNsNp,
            1 => AnactrlRule::EnumNsP,
            2 => AnactrlRule::EnumSNp,
            3 => AnactrlRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AnactrlRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AnactrlRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AnactrlRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AnactrlRule::EnumSP
    }
}
#[doc = "Field `ANACTRL_RULE` writer - Analog Modules controller"]
pub type AnactrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, AnactrlRule, crate::Safe>;
impl<'a, REG> AnactrlRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(AnactrlRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(AnactrlRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(AnactrlRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(AnactrlRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 12:13 - Analog Modules controller"]
    #[inline(always)]
    pub fn anactrl_rule(&self) -> AnactrlRuleR {
        AnactrlRuleR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_APB_BRIDGE0_MEM_CTRL2")
            .field("anactrl_rule", &self.anactrl_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:13 - Analog Modules controller"]
    #[inline(always)]
    pub fn anactrl_rule(&mut self) -> AnactrlRuleW<'_, SecCtrlApbBridge0MemCtrl2Spec> {
        AnactrlRuleW::new(self, 12)
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_apb_bridge0_mem_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_apb_bridge0_mem_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlApbBridge0MemCtrl2Spec;
impl crate::RegisterSpec for SecCtrlApbBridge0MemCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_apb_bridge0_mem_ctrl2::R`](R) reader structure"]
impl crate::Readable for SecCtrlApbBridge0MemCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_apb_bridge0_mem_ctrl2::W`](W) writer structure"]
impl crate::Writable for SecCtrlApbBridge0MemCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 to value 0"]
impl crate::Resettable for SecCtrlApbBridge0MemCtrl2Spec {}

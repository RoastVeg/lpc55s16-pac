#[doc = "Register `SEC_CTRL_USB_HS_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlUsbHsSlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_USB_HS_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlUsbHsSlaveRuleSpec>;
#[doc = "Security access rules for the whole USB High Speed RAM : 0x2001_0000 - 0x2001_3FFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RamUsbHsRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<RamUsbHsRule> for u8 {
    #[inline(always)]
    fn from(variant: RamUsbHsRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RamUsbHsRule {
    type Ux = u8;
}
impl crate::IsEnum for RamUsbHsRule {}
#[doc = "Field `RAM_USB_HS_RULE` reader - Security access rules for the whole USB High Speed RAM : 0x2001_0000 - 0x2001_3FFF"]
pub type RamUsbHsRuleR = crate::FieldReader<RamUsbHsRule>;
impl RamUsbHsRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamUsbHsRule {
        match self.bits {
            0 => RamUsbHsRule::EnumNsNp,
            1 => RamUsbHsRule::EnumNsP,
            2 => RamUsbHsRule::EnumSNp,
            3 => RamUsbHsRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RamUsbHsRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RamUsbHsRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RamUsbHsRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RamUsbHsRule::EnumSP
    }
}
#[doc = "Field `RAM_USB_HS_RULE` writer - Security access rules for the whole USB High Speed RAM : 0x2001_0000 - 0x2001_3FFF"]
pub type RamUsbHsRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, RamUsbHsRule, crate::Safe>;
impl<'a, REG> RamUsbHsRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(RamUsbHsRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(RamUsbHsRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(RamUsbHsRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(RamUsbHsRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole USB High Speed RAM : 0x2001_0000 - 0x2001_3FFF"]
    #[inline(always)]
    pub fn ram_usb_hs_rule(&self) -> RamUsbHsRuleR {
        RamUsbHsRuleR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_USB_HS_SLAVE_RULE")
            .field("ram_usb_hs_rule", &self.ram_usb_hs_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole USB High Speed RAM : 0x2001_0000 - 0x2001_3FFF"]
    #[inline(always)]
    pub fn ram_usb_hs_rule(&mut self) -> RamUsbHsRuleW<'_, SecCtrlUsbHsSlaveRuleSpec> {
        RamUsbHsRuleW::new(self, 0)
    }
}
#[doc = "Security access rules for USB High speed RAM slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_usb_hs_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_usb_hs_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlUsbHsSlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlUsbHsSlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_usb_hs_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlUsbHsSlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_usb_hs_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlUsbHsSlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_USB_HS_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlUsbHsSlaveRuleSpec {}

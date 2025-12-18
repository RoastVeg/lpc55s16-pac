#[doc = "Register `SEC_CTRL_FLASH_ROM_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlFlashRomSlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_FLASH_ROM_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlFlashRomSlaveRuleSpec>;
#[doc = "Security access rules for the whole FLASH : 0x0000_0000 - 0x0003_FFFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<FlashRule> for u8 {
    #[inline(always)]
    fn from(variant: FlashRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FlashRule {
    type Ux = u8;
}
impl crate::IsEnum for FlashRule {}
#[doc = "Field `FLASH_RULE` reader - Security access rules for the whole FLASH : 0x0000_0000 - 0x0003_FFFF"]
pub type FlashRuleR = crate::FieldReader<FlashRule>;
impl FlashRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashRule {
        match self.bits {
            0 => FlashRule::EnumNsNp,
            1 => FlashRule::EnumNsP,
            2 => FlashRule::EnumSNp,
            3 => FlashRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FlashRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FlashRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FlashRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FlashRule::EnumSP
    }
}
#[doc = "Field `FLASH_RULE` writer - Security access rules for the whole FLASH : 0x0000_0000 - 0x0003_FFFF"]
pub type FlashRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, FlashRule, crate::Safe>;
impl<'a, REG> FlashRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRule::EnumSP)
    }
}
#[doc = "Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RomRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<RomRule> for u8 {
    #[inline(always)]
    fn from(variant: RomRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RomRule {
    type Ux = u8;
}
impl crate::IsEnum for RomRule {}
#[doc = "Field `ROM_RULE` reader - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
pub type RomRuleR = crate::FieldReader<RomRule>;
impl RomRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RomRule {
        match self.bits {
            0 => RomRule::EnumNsNp,
            1 => RomRule::EnumNsP,
            2 => RomRule::EnumSNp,
            3 => RomRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RomRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RomRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RomRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RomRule::EnumSP
    }
}
#[doc = "Field `ROM_RULE` writer - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
pub type RomRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, RomRule, crate::Safe>;
impl<'a, REG> RomRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(RomRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(RomRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(RomRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(RomRule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0003_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&self) -> FlashRuleR {
        FlashRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&self) -> RomRuleR {
        RomRuleR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_FLASH_ROM_SLAVE_RULE")
            .field("flash_rule", &self.flash_rule())
            .field("rom_rule", &self.rom_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole FLASH : 0x0000_0000 - 0x0003_FFFF"]
    #[inline(always)]
    pub fn flash_rule(&mut self) -> FlashRuleW<'_, SecCtrlFlashRomSlaveRuleSpec> {
        FlashRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Security access rules for the whole ROM : 0x0300_0000 - 0x0301_FFFF"]
    #[inline(always)]
    pub fn rom_rule(&mut self) -> RomRuleW<'_, SecCtrlFlashRomSlaveRuleSpec> {
        RomRuleW::new(self, 4)
    }
}
#[doc = "Security access rules for Flash and ROM slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_flash_rom_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_flash_rom_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlFlashRomSlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlFlashRomSlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_flash_rom_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlFlashRomSlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_flash_rom_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlFlashRomSlaveRuleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_FLASH_ROM_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlFlashRomSlaveRuleSpec {}

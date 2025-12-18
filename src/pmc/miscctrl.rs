#[doc = "Register `MISCCTRL` reader"]
pub type R = crate::R<MiscctrlSpec>;
#[doc = "Register `MISCCTRL` writer"]
pub type W = crate::W<MiscctrlSpec>;
#[doc = "Select LDO Deep Sleep reference source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldodeepsleepref {
    #[doc = "0: LDO DEEP Sleep uses Flash buffer biasing as reference."]
    Flashbuffer = 0,
    #[doc = "1: LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    Bgp0p8v = 1,
}
impl From<Ldodeepsleepref> for bool {
    #[inline(always)]
    fn from(variant: Ldodeepsleepref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDODEEPSLEEPREF` reader - Select LDO Deep Sleep reference source."]
pub type LdodeepsleeprefR = crate::BitReader<Ldodeepsleepref>;
impl LdodeepsleeprefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldodeepsleepref {
        match self.bits {
            false => Ldodeepsleepref::Flashbuffer,
            true => Ldodeepsleepref::Bgp0p8v,
        }
    }
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    #[inline(always)]
    pub fn is_flashbuffer(&self) -> bool {
        *self == Ldodeepsleepref::Flashbuffer
    }
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    #[inline(always)]
    pub fn is_bgp0p8v(&self) -> bool {
        *self == Ldodeepsleepref::Bgp0p8v
    }
}
#[doc = "Field `LDODEEPSLEEPREF` writer - Select LDO Deep Sleep reference source."]
pub type LdodeepsleeprefW<'a, REG> = crate::BitWriter<'a, REG, Ldodeepsleepref>;
impl<'a, REG> LdodeepsleeprefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    #[inline(always)]
    pub fn flashbuffer(self) -> &'a mut crate::W<REG> {
        self.variant(Ldodeepsleepref::Flashbuffer)
    }
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    #[inline(always)]
    pub fn bgp0p8v(self) -> &'a mut crate::W<REG> {
        self.variant(Ldodeepsleepref::Bgp0p8v)
    }
}
#[doc = "Control the activation of LDO MEM High Z mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldomemhighzmode {
    #[doc = "0: LDO MEM High Z mode is disabled."]
    Disable = 0,
    #[doc = "1: LDO MEM High Z mode is enabled."]
    Enable = 1,
}
impl From<Ldomemhighzmode> for bool {
    #[inline(always)]
    fn from(variant: Ldomemhighzmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` reader - Control the activation of LDO MEM High Z mode."]
pub type LdomemhighzmodeR = crate::BitReader<Ldomemhighzmode>;
impl LdomemhighzmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldomemhighzmode {
        match self.bits {
            false => Ldomemhighzmode::Disable,
            true => Ldomemhighzmode::Enable,
        }
    }
    #[doc = "LDO MEM High Z mode is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ldomemhighzmode::Disable
    }
    #[doc = "LDO MEM High Z mode is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ldomemhighzmode::Enable
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` writer - Control the activation of LDO MEM High Z mode."]
pub type LdomemhighzmodeW<'a, REG> = crate::BitWriter<'a, REG, Ldomemhighzmode>;
impl<'a, REG> LdomemhighzmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO MEM High Z mode is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ldomemhighzmode::Disable)
    }
    #[doc = "LDO MEM High Z mode is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ldomemhighzmode::Enable)
    }
}
#[doc = "Field `LOWPWR_FLASH_BUF` reader - no description available"]
pub type LowpwrFlashBufR = crate::BitReader;
#[doc = "Field `LOWPWR_FLASH_BUF` writer - no description available"]
pub type LowpwrFlashBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISCCTRL_3_11` reader - Reserved."]
pub type Miscctrl3_11R = crate::FieldReader<u16>;
#[doc = "Field `MISCCTRL_3_11` writer - Reserved."]
pub type Miscctrl3_11W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableBleed {
    #[doc = "0: LDO_MEM bleed current is enabled."]
    BleedEnable = 0,
    #[doc = "1: LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    BleedDisable = 1,
}
impl From<DisableBleed> for bool {
    #[inline(always)]
    fn from(variant: DisableBleed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE_BLEED` reader - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DisableBleedR = crate::BitReader<DisableBleed>;
impl DisableBleedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableBleed {
        match self.bits {
            false => DisableBleed::BleedEnable,
            true => DisableBleed::BleedDisable,
        }
    }
    #[doc = "LDO_MEM bleed current is enabled."]
    #[inline(always)]
    pub fn is_bleed_enable(&self) -> bool {
        *self == DisableBleed::BleedEnable
    }
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    #[inline(always)]
    pub fn is_bleed_disable(&self) -> bool {
        *self == DisableBleed::BleedDisable
    }
}
#[doc = "Field `DISABLE_BLEED` writer - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DisableBleedW<'a, REG> = crate::BitWriter<'a, REG, DisableBleed>;
impl<'a, REG> DisableBleedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO_MEM bleed current is enabled."]
    #[inline(always)]
    pub fn bleed_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableBleed::BleedEnable)
    }
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    #[inline(always)]
    pub fn bleed_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableBleed::BleedDisable)
    }
}
#[doc = "Field `MISCCTRL_13_15` reader - Reserved."]
pub type Miscctrl13_15R = crate::FieldReader;
#[doc = "Field `MISCCTRL_13_15` writer - Reserved."]
pub type Miscctrl13_15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub fn ldodeepsleepref(&self) -> LdodeepsleeprefR {
        LdodeepsleeprefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&self) -> LdomemhighzmodeR {
        LdomemhighzmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn lowpwr_flash_buf(&self) -> LowpwrFlashBufR {
        LowpwrFlashBufR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_3_11(&self) -> Miscctrl3_11R {
        Miscctrl3_11R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&self) -> DisableBleedR {
        DisableBleedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_13_15(&self) -> Miscctrl13_15R {
        Miscctrl13_15R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub fn ldodeepsleepref(&mut self) -> LdodeepsleeprefW<'_, MiscctrlSpec> {
        LdodeepsleeprefW::new(self, 0)
    }
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&mut self) -> LdomemhighzmodeW<'_, MiscctrlSpec> {
        LdomemhighzmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn lowpwr_flash_buf(&mut self) -> LowpwrFlashBufW<'_, MiscctrlSpec> {
        LowpwrFlashBufW::new(self, 2)
    }
    #[doc = "Bits 3:11 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_3_11(&mut self) -> Miscctrl3_11W<'_, MiscctrlSpec> {
        Miscctrl3_11W::new(self, 3)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&mut self) -> DisableBleedW<'_, MiscctrlSpec> {
        DisableBleedW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_13_15(&mut self) -> Miscctrl13_15W<'_, MiscctrlSpec> {
        Miscctrl13_15W::new(self, 13)
    }
}
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`miscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscctrlSpec;
impl crate::RegisterSpec for MiscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miscctrl::R`](R) reader structure"]
impl crate::Readable for MiscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`miscctrl::W`](W) writer structure"]
impl crate::Writable for MiscctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MiscctrlSpec {}

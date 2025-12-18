#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Power Management Controller Main Finite State Machine (FSM) status.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsmmain {
    #[doc = "0: POWER UP : The IC is powering up."]
    FsmmainPowerup = 0,
    #[doc = "1: ACTIVE : Power up is completed. The IC is in normal functional operation mode."]
    FsmmainActive = 1,
    #[doc = "2: POWER DOWN : the IC has entered POWER DOWN mode."]
    FsmmainPowerdown = 2,
    #[doc = "3: DEEP SLEEP: The IC has entered DEEP SLEEP mode."]
    FsmmainDeepsleep = 3,
    #[doc = "6: DEEP POWER DOWN : The IC entred DEEP POWER DOWN mode."]
    FsmmainDeeppowerdown = 6,
    #[doc = "7: IC Structural TEST Mode : The IC has entered in IC Test mode."]
    FsmmainDftActive = 7,
}
impl From<Fsmmain> for u8 {
    #[inline(always)]
    fn from(variant: Fsmmain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsmmain {
    type Ux = u8;
}
impl crate::IsEnum for Fsmmain {}
#[doc = "Field `FSMMAIN` reader - Power Management Controller Main Finite State Machine (FSM) status."]
pub type FsmmainR = crate::FieldReader<Fsmmain>;
impl FsmmainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsmmain> {
        match self.bits {
            0 => Some(Fsmmain::FsmmainPowerup),
            1 => Some(Fsmmain::FsmmainActive),
            2 => Some(Fsmmain::FsmmainPowerdown),
            3 => Some(Fsmmain::FsmmainDeepsleep),
            6 => Some(Fsmmain::FsmmainDeeppowerdown),
            7 => Some(Fsmmain::FsmmainDftActive),
            _ => None,
        }
    }
    #[doc = "POWER UP : The IC is powering up."]
    #[inline(always)]
    pub fn is_fsmmain_powerup(&self) -> bool {
        *self == Fsmmain::FsmmainPowerup
    }
    #[doc = "ACTIVE : Power up is completed. The IC is in normal functional operation mode."]
    #[inline(always)]
    pub fn is_fsmmain_active(&self) -> bool {
        *self == Fsmmain::FsmmainActive
    }
    #[doc = "POWER DOWN : the IC has entered POWER DOWN mode."]
    #[inline(always)]
    pub fn is_fsmmain_powerdown(&self) -> bool {
        *self == Fsmmain::FsmmainPowerdown
    }
    #[doc = "DEEP SLEEP: The IC has entered DEEP SLEEP mode."]
    #[inline(always)]
    pub fn is_fsmmain_deepsleep(&self) -> bool {
        *self == Fsmmain::FsmmainDeepsleep
    }
    #[doc = "DEEP POWER DOWN : The IC entred DEEP POWER DOWN mode."]
    #[inline(always)]
    pub fn is_fsmmain_deeppowerdown(&self) -> bool {
        *self == Fsmmain::FsmmainDeeppowerdown
    }
    #[doc = "IC Structural TEST Mode : The IC has entered in IC Test mode."]
    #[inline(always)]
    pub fn is_fsmmain_dft_active(&self) -> bool {
        *self == Fsmmain::FsmmainDftActive
    }
}
#[doc = "Field `FSMPWUP` reader - POWER UP Finite State Machine (FSM) status."]
pub type FsmpwupR = crate::FieldReader;
#[doc = "Field `FSMDSLP` reader - DEEP SLEEP Finite State Machine (FSM) status."]
pub type FsmdslpR = crate::FieldReader;
#[doc = "Field `FSMPWDN` reader - POWER DOWN Finite State Machine (FSM) status."]
pub type FsmpwdnR = crate::FieldReader;
#[doc = "Field `FSMDPWD` reader - DEEP POWER DOWN Finite State Machine (FSM) status."]
pub type FsmdpwdR = crate::FieldReader;
#[doc = "Latest IC Boot cause:.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootmode {
    #[doc = "0: Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    Powerup = 0,
    #[doc = "1: Latest IC boot was from DEEP SLEEP low power mode.."]
    Deepsleep = 1,
    #[doc = "2: Latest IC boot was from POWER DOWN low power mode.."]
    Powerdown = 2,
    #[doc = "3: Latest IC boot was from DEEP POWER DOWN low power mode.."]
    Deeppowerdown = 3,
}
impl From<Bootmode> for u8 {
    #[inline(always)]
    fn from(variant: Bootmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootmode {
    type Ux = u8;
}
impl crate::IsEnum for Bootmode {}
#[doc = "Field `BOOTMODE` reader - Latest IC Boot cause:."]
pub type BootmodeR = crate::FieldReader<Bootmode>;
impl BootmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootmode {
        match self.bits {
            0 => Bootmode::Powerup,
            1 => Bootmode::Deepsleep,
            2 => Bootmode::Powerdown,
            3 => Bootmode::Deeppowerdown,
            _ => unreachable!(),
        }
    }
    #[doc = "Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    #[inline(always)]
    pub fn is_powerup(&self) -> bool {
        *self == Bootmode::Powerup
    }
    #[doc = "Latest IC boot was from DEEP SLEEP low power mode.."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == Bootmode::Deepsleep
    }
    #[doc = "Latest IC boot was from POWER DOWN low power mode.."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Bootmode::Powerdown
    }
    #[doc = "Latest IC boot was from DEEP POWER DOWN low power mode.."]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == Bootmode::Deeppowerdown
    }
}
#[doc = "Field `WAFERTESTDONEVECT` reader - Indicates cuurent status of wafer test level."]
pub type WafertestdonevectR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Power Management Controller Main Finite State Machine (FSM) status."]
    #[inline(always)]
    pub fn fsmmain(&self) -> FsmmainR {
        FsmmainR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - POWER UP Finite State Machine (FSM) status."]
    #[inline(always)]
    pub fn fsmpwup(&self) -> FsmpwupR {
        FsmpwupR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - DEEP SLEEP Finite State Machine (FSM) status."]
    #[inline(always)]
    pub fn fsmdslp(&self) -> FsmdslpR {
        FsmdslpR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - POWER DOWN Finite State Machine (FSM) status."]
    #[inline(always)]
    pub fn fsmpwdn(&self) -> FsmpwdnR {
        FsmpwdnR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - DEEP POWER DOWN Finite State Machine (FSM) status."]
    #[inline(always)]
    pub fn fsmdpwd(&self) -> FsmdpwdR {
        FsmdpwdR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:19 - Latest IC Boot cause:."]
    #[inline(always)]
    pub fn bootmode(&self) -> BootmodeR {
        BootmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Indicates cuurent status of wafer test level."]
    #[inline(always)]
    pub fn wafertestdonevect(&self) -> WafertestdonevectR {
        WafertestdonevectR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("fsmmain", &self.fsmmain())
            .field("fsmpwup", &self.fsmpwup())
            .field("fsmdslp", &self.fsmdslp())
            .field("fsmpwdn", &self.fsmpwdn())
            .field("fsmdpwd", &self.fsmdpwd())
            .field("bootmode", &self.bootmode())
            .field("wafertestdonevect", &self.wafertestdonevect())
            .finish()
    }
}
impl W {}
#[doc = "Power Management Controller FSM (Finite State Machines) status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}

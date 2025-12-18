#[doc = "Register `DEVCMDSTAT` reader"]
pub type R = crate::R<DevcmdstatSpec>;
#[doc = "Register `DEVCMDSTAT` writer"]
pub type W = crate::W<DevcmdstatSpec>;
#[doc = "Field `DEV_ADDR` reader - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub type DevAddrR = crate::FieldReader;
#[doc = "Field `DEV_ADDR` writer - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub type DevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DEV_EN` reader - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub type DevEnR = crate::BitReader;
#[doc = "Field `DEV_EN` writer - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub type DevEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP` reader - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Forces the NEEDCLK output to always be on:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceNeedclk {
    #[doc = "0: USB_NEEDCLK has normal function."]
    Normal = 0,
    #[doc = "1: USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    AlwaysOn = 1,
}
impl From<ForceNeedclk> for bool {
    #[inline(always)]
    fn from(variant: ForceNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_NEEDCLK` reader - Forces the NEEDCLK output to always be on:"]
pub type ForceNeedclkR = crate::BitReader<ForceNeedclk>;
impl ForceNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceNeedclk {
        match self.bits {
            false => ForceNeedclk::Normal,
            true => ForceNeedclk::AlwaysOn,
        }
    }
    #[doc = "USB_NEEDCLK has normal function."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ForceNeedclk::Normal
    }
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == ForceNeedclk::AlwaysOn
    }
}
#[doc = "Field `FORCE_NEEDCLK` writer - Forces the NEEDCLK output to always be on:"]
pub type ForceNeedclkW<'a, REG> = crate::BitWriter<'a, REG, ForceNeedclk>;
impl<'a, REG> ForceNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB_NEEDCLK has normal function."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ForceNeedclk::Normal)
    }
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut crate::W<REG> {
        self.variant(ForceNeedclk::AlwaysOn)
    }
}
#[doc = "LPM Supported:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpmSup {
    #[doc = "0: LPM not supported."]
    No = 0,
    #[doc = "1: LPM supported."]
    Yes = 1,
}
impl From<LpmSup> for bool {
    #[inline(always)]
    fn from(variant: LpmSup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM_SUP` reader - LPM Supported:"]
pub type LpmSupR = crate::BitReader<LpmSup>;
impl LpmSupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpmSup {
        match self.bits {
            false => LpmSup::No,
            true => LpmSup::Yes,
        }
    }
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LpmSup::No
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == LpmSup::Yes
    }
}
#[doc = "Field `LPM_SUP` writer - LPM Supported:"]
pub type LpmSupW<'a, REG> = crate::BitWriter<'a, REG, LpmSup>;
impl<'a, REG> LpmSupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(LpmSup::No)
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(LpmSup::Yes)
    }
}
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntonnakAo {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    Disabled = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    Enabled = 1,
}
impl From<IntonnakAo> for bool {
    #[inline(always)]
    fn from(variant: IntonnakAo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type IntonnakAoR = crate::BitReader<IntonnakAo>;
impl IntonnakAoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntonnakAo {
        match self.bits {
            false => IntonnakAo::Disabled,
            true => IntonnakAo::Enabled,
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IntonnakAo::Disabled
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IntonnakAo::Enabled
    }
}
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type IntonnakAoW<'a, REG> = crate::BitWriter<'a, REG, IntonnakAo>;
impl<'a, REG> IntonnakAoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakAo::Disabled)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakAo::Enabled)
    }
}
#[doc = "Interrupt on NAK for interrupt and bulk IN EP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntonnakAi {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    Disabled = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    Enabled = 1,
}
impl From<IntonnakAi> for bool {
    #[inline(always)]
    fn from(variant: IntonnakAi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP"]
pub type IntonnakAiR = crate::BitReader<IntonnakAi>;
impl IntonnakAiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntonnakAi {
        match self.bits {
            false => IntonnakAi::Disabled,
            true => IntonnakAi::Enabled,
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IntonnakAi::Disabled
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IntonnakAi::Enabled
    }
}
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP"]
pub type IntonnakAiW<'a, REG> = crate::BitWriter<'a, REG, IntonnakAi>;
impl<'a, REG> IntonnakAiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakAi::Disabled)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakAi::Enabled)
    }
}
#[doc = "Interrupt on NAK for control OUT EP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntonnakCo {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    Disabled = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    Enabled = 1,
}
impl From<IntonnakCo> for bool {
    #[inline(always)]
    fn from(variant: IntonnakCo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP"]
pub type IntonnakCoR = crate::BitReader<IntonnakCo>;
impl IntonnakCoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntonnakCo {
        match self.bits {
            false => IntonnakCo::Disabled,
            true => IntonnakCo::Enabled,
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IntonnakCo::Disabled
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IntonnakCo::Enabled
    }
}
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP"]
pub type IntonnakCoW<'a, REG> = crate::BitWriter<'a, REG, IntonnakCo>;
impl<'a, REG> IntonnakCoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakCo::Disabled)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakCo::Enabled)
    }
}
#[doc = "Interrupt on NAK for control IN EP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntonnakCi {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    Disabled = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    Enabled = 1,
}
impl From<IntonnakCi> for bool {
    #[inline(always)]
    fn from(variant: IntonnakCi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP"]
pub type IntonnakCiR = crate::BitReader<IntonnakCi>;
impl IntonnakCiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntonnakCi {
        match self.bits {
            false => IntonnakCi::Disabled,
            true => IntonnakCi::Enabled,
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IntonnakCi::Disabled
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IntonnakCi::Enabled
    }
}
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP"]
pub type IntonnakCiW<'a, REG> = crate::BitWriter<'a, REG, IntonnakCi>;
impl<'a, REG> IntonnakCiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakCi::Disabled)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonnakCi::Enabled)
    }
}
#[doc = "Field `DCON` reader - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
pub type DconR = crate::BitReader;
#[doc = "Field `DCON` writer - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
pub type DconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSUS` reader - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub type DsusR = crate::BitReader;
#[doc = "Field `DSUS` writer - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub type DsusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub type LpmSusR = crate::BitReader;
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub type LpmSusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
pub type LpmRewpR = crate::BitReader;
#[doc = "Field `DCON_C` reader - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub type DconCR = crate::BitReader;
#[doc = "Field `DCON_C` writer - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub type DconCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSUS_C` reader - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub type DsusCR = crate::BitReader;
#[doc = "Field `DSUS_C` writer - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub type DsusCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRES_C` reader - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub type DresCR = crate::BitReader;
#[doc = "Field `DRES_C` writer - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub type DresCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDEBOUNCED` reader - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
pub type VbusdebouncedR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DevAddrR {
        DevAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&self) -> DevEnR {
        DevEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:"]
    #[inline(always)]
    pub fn force_needclk(&self) -> ForceNeedclkR {
        ForceNeedclkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LpmSupR {
        LpmSupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> IntonnakAoR {
        IntonnakAoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> IntonnakAiR {
        IntonnakAiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&self) -> IntonnakCoR {
        IntonnakCoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> IntonnakCiR {
        IntonnakCiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
    #[inline(always)]
    pub fn dcon(&self) -> DconR {
        DconR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&self) -> DsusR {
        DsusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LpmSusR {
        LpmSusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LpmRewpR {
        LpmRewpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DconCR {
        DconCR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DsusCR {
        DsusCR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&self) -> DresCR {
        DresCR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    pub fn vbusdebounced(&self) -> VbusdebouncedR {
        VbusdebouncedR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVCMDSTAT")
            .field("dev_addr", &self.dev_addr())
            .field("dev_en", &self.dev_en())
            .field("setup", &self.setup())
            .field("force_needclk", &self.force_needclk())
            .field("lpm_sup", &self.lpm_sup())
            .field("intonnak_ao", &self.intonnak_ao())
            .field("intonnak_ai", &self.intonnak_ai())
            .field("intonnak_co", &self.intonnak_co())
            .field("intonnak_ci", &self.intonnak_ci())
            .field("dcon", &self.dcon())
            .field("dsus", &self.dsus())
            .field("lpm_sus", &self.lpm_sus())
            .field("lpm_rewp", &self.lpm_rewp())
            .field("dcon_c", &self.dcon_c())
            .field("dsus_c", &self.dsus_c())
            .field("dres_c", &self.dres_c())
            .field("vbusdebounced", &self.vbusdebounced())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DevAddrW<'_, DevcmdstatSpec> {
        DevAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&mut self) -> DevEnW<'_, DevcmdstatSpec> {
        DevEnW::new(self, 7)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&mut self) -> SetupW<'_, DevcmdstatSpec> {
        SetupW::new(self, 8)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:"]
    #[inline(always)]
    pub fn force_needclk(&mut self) -> ForceNeedclkW<'_, DevcmdstatSpec> {
        ForceNeedclkW::new(self, 9)
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> LpmSupW<'_, DevcmdstatSpec> {
        LpmSupW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> IntonnakAoW<'_, DevcmdstatSpec> {
        IntonnakAoW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> IntonnakAiW<'_, DevcmdstatSpec> {
        IntonnakAiW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> IntonnakCoW<'_, DevcmdstatSpec> {
        IntonnakCoW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> IntonnakCiW<'_, DevcmdstatSpec> {
        IntonnakCiW::new(self, 15)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
    #[inline(always)]
    pub fn dcon(&mut self) -> DconW<'_, DevcmdstatSpec> {
        DconW::new(self, 16)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&mut self) -> DsusW<'_, DevcmdstatSpec> {
        DsusW::new(self, 17)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> LpmSusW<'_, DevcmdstatSpec> {
        LpmSusW::new(self, 19)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&mut self) -> DconCW<'_, DevcmdstatSpec> {
        DconCW::new(self, 24)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&mut self) -> DsusCW<'_, DevcmdstatSpec> {
        DsusCW::new(self, 25)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&mut self) -> DresCW<'_, DevcmdstatSpec> {
        DresCW::new(self, 26)
    }
}
#[doc = "USB Device Command/Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`devcmdstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcmdstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevcmdstatSpec;
impl crate::RegisterSpec for DevcmdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devcmdstat::R`](R) reader structure"]
impl crate::Readable for DevcmdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`devcmdstat::W`](W) writer structure"]
impl crate::Writable for DevcmdstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DevcmdstatSpec {
    const RESET_VALUE: u32 = 0x0800;
}

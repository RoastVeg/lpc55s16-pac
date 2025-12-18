#[doc = "Register `CTRL_TOG` reader"]
pub type R = crate::R<CtrlTogSpec>;
#[doc = "Register `CTRL_TOG` writer"]
pub type W = crate::W<CtrlTogSpec>;
#[doc = "Field `ENHOSTDISCONDETECT` reader - For host mode, enables high-speed disconnect detector"]
pub type EnhostdiscondetectR = crate::BitReader;
#[doc = "Field `ENHOSTDISCONDETECT` writer - For host mode, enables high-speed disconnect detector"]
pub type EnhostdiscondetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENIRQHOSTDISCON` reader - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type EnirqhostdisconR = crate::BitReader;
#[doc = "Field `ENIRQHOSTDISCON` writer - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type EnirqhostdisconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Indicates that the device has disconnected in High-Speed mode"]
pub type HostdiscondetectIrqR = crate::BitReader;
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Indicates that the device has disconnected in High-Speed mode"]
pub type HostdiscondetectIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endevplugindet {
    #[doc = "0: Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    Value0 = 0,
    #[doc = "1: Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    Value1 = 1,
}
impl From<Endevplugindet> for bool {
    #[inline(always)]
    fn from(variant: Endevplugindet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEVPLUGINDET` reader - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type EndevplugindetR = crate::BitReader<Endevplugindet>;
impl EndevplugindetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endevplugindet {
        match self.bits {
            false => Endevplugindet::Value0,
            true => Endevplugindet::Value1,
        }
    }
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Endevplugindet::Value0
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Endevplugindet::Value1
    }
}
#[doc = "Field `ENDEVPLUGINDET` writer - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type EndevplugindetW<'a, REG> = crate::BitWriter<'a, REG, Endevplugindet>;
impl<'a, REG> EndevplugindetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Endevplugindet::Value0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Endevplugindet::Value1)
    }
}
#[doc = "Field `DEVPLUGIN_POLARITY` reader - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DevpluginPolarityR = crate::BitReader;
#[doc = "Field `DEVPLUGIN_POLARITY` writer - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DevpluginPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMEIRQSTICKY` reader - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type ResumeirqstickyR = crate::BitReader;
#[doc = "Field `RESUMEIRQSTICKY` writer - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type ResumeirqstickyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENIRQRESUMEDETECT` reader - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub type EnirqresumedetectR = crate::BitReader;
#[doc = "Field `ENIRQRESUMEDETECT` writer - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub type EnirqresumedetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_IRQ` reader - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub type ResumeIrqR = crate::BitReader;
#[doc = "Field `RESUME_IRQ` writer - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub type ResumeIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVPLUGIN_IRQ` reader - Indicates that the device is connected"]
pub type DevpluginIrqR = crate::BitReader;
#[doc = "Field `DEVPLUGIN_IRQ` writer - Indicates that the device is connected"]
pub type DevpluginIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUTMILEVEL2` reader - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type Enutmilevel2R = crate::BitReader;
#[doc = "Field `ENUTMILEVEL2` writer - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type Enutmilevel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUTMILEVEL3` reader - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type Enutmilevel3R = crate::BitReader;
#[doc = "Field `ENUTMILEVEL3` writer - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type Enutmilevel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENIRQWAKEUP` reader - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub type EnirqwakeupR = crate::BitReader;
#[doc = "Field `ENIRQWAKEUP` writer - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub type EnirqwakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_IRQ` reader - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub type WakeupIrqR = crate::BitReader;
#[doc = "Field `WAKEUP_IRQ` writer - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub type WakeupIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORESUME_EN` reader - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AutoresumeEnR = crate::BitReader;
#[doc = "Field `AUTORESUME_EN` writer - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AutoresumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type EnautoclrClkgateR = crate::BitReader;
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type EnautoclrClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type EnautoclrPhyPwdR = crate::BitReader;
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type EnautoclrPhyPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDPDMCHG_WKUP` reader - Enable DP DM change wake-up: Not for customer use"]
pub type EndpdmchgWkupR = crate::BitReader;
#[doc = "Field `ENDPDMCHG_WKUP` writer - Enable DP DM change wake-up: Not for customer use"]
pub type EndpdmchgWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENVBUSCHG_WKUP` reader - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub type EnvbuschgWkupR = crate::BitReader;
#[doc = "Field `ENVBUSCHG_WKUP` writer - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub type EnvbuschgWkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOCLR_USBCLKGATE` reader - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type EnautoclrUsbclkgateR = crate::BitReader;
#[doc = "Field `ENAUTOCLR_USBCLKGATE` writer - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type EnautoclrUsbclkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOSET_USBCLKS` reader - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type EnautosetUsbclksR = crate::BitReader;
#[doc = "Field `ENAUTOSET_USBCLKS` writer - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type EnautosetUsbclksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_FORCE_LS_SE0` reader - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HostForceLsSe0R = crate::BitReader;
#[doc = "Field `HOST_FORCE_LS_SE0` writer - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HostForceLsSe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_SUSPENDM` reader - Used by the PHY to indicate a powered-down state"]
pub type UtmiSuspendmR = crate::BitReader;
#[doc = "Field `UTMI_SUSPENDM` writer - Used by the PHY to indicate a powered-down state"]
pub type UtmiSuspendmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGATE` reader - Gate UTMI Clocks"]
pub type ClkgateR = crate::BitReader;
#[doc = "Field `CLKGATE` writer - Gate UTMI Clocks"]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRST` reader - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SftrstR = crate::BitReader;
#[doc = "Field `SFTRST` writer - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SftrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> EnhostdiscondetectR {
        EnhostdiscondetectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&self) -> EnirqhostdisconR {
        EnirqhostdisconR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HostdiscondetectIrqR {
        HostdiscondetectIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&self) -> EndevplugindetR {
        EndevplugindetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&self) -> DevpluginPolarityR {
        DevpluginPolarityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&self) -> ResumeirqstickyR {
        ResumeirqstickyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&self) -> EnirqresumedetectR {
        EnirqresumedetectR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&self) -> ResumeIrqR {
        ResumeIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DevpluginIrqR {
        DevpluginIrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> Enutmilevel2R {
        Enutmilevel2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> Enutmilevel3R {
        Enutmilevel3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub fn enirqwakeup(&self) -> EnirqwakeupR {
        EnirqwakeupR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WakeupIrqR {
        WakeupIrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AutoresumeEnR {
        AutoresumeEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> EnautoclrClkgateR {
        EnautoclrClkgateR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> EnautoclrPhyPwdR {
        EnautoclrPhyPwdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&self) -> EndpdmchgWkupR {
        EndpdmchgWkupR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub fn envbuschg_wkup(&self) -> EnvbuschgWkupR {
        EnvbuschgWkupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoclr_usbclkgate(&self) -> EnautoclrUsbclkgateR {
        EnautoclrUsbclkgateR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoset_usbclks(&self) -> EnautosetUsbclksR {
        EnautosetUsbclksR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HostForceLsSe0R {
        HostForceLsSe0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UtmiSuspendmR {
        UtmiSuspendmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SftrstR {
        SftrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&mut self) -> EnhostdiscondetectW<'_, CtrlTogSpec> {
        EnhostdiscondetectW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&mut self) -> EnirqhostdisconW<'_, CtrlTogSpec> {
        EnirqhostdisconW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&mut self) -> HostdiscondetectIrqW<'_, CtrlTogSpec> {
        HostdiscondetectIrqW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&mut self) -> EndevplugindetW<'_, CtrlTogSpec> {
        EndevplugindetW::new(self, 4)
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&mut self) -> DevpluginPolarityW<'_, CtrlTogSpec> {
        DevpluginPolarityW::new(self, 5)
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&mut self) -> ResumeirqstickyW<'_, CtrlTogSpec> {
        ResumeirqstickyW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&mut self) -> EnirqresumedetectW<'_, CtrlTogSpec> {
        EnirqresumedetectW::new(self, 9)
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&mut self) -> ResumeIrqW<'_, CtrlTogSpec> {
        ResumeIrqW::new(self, 10)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&mut self) -> DevpluginIrqW<'_, CtrlTogSpec> {
        DevpluginIrqW::new(self, 12)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&mut self) -> Enutmilevel2W<'_, CtrlTogSpec> {
        Enutmilevel2W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&mut self) -> Enutmilevel3W<'_, CtrlTogSpec> {
        Enutmilevel3W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub fn enirqwakeup(&mut self) -> EnirqwakeupW<'_, CtrlTogSpec> {
        EnirqwakeupW::new(self, 16)
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub fn wakeup_irq(&mut self) -> WakeupIrqW<'_, CtrlTogSpec> {
        WakeupIrqW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&mut self) -> AutoresumeEnW<'_, CtrlTogSpec> {
        AutoresumeEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&mut self) -> EnautoclrClkgateW<'_, CtrlTogSpec> {
        EnautoclrClkgateW::new(self, 19)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&mut self) -> EnautoclrPhyPwdW<'_, CtrlTogSpec> {
        EnautoclrPhyPwdW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&mut self) -> EndpdmchgWkupW<'_, CtrlTogSpec> {
        EndpdmchgWkupW::new(self, 21)
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub fn envbuschg_wkup(&mut self) -> EnvbuschgWkupW<'_, CtrlTogSpec> {
        EnvbuschgWkupW::new(self, 23)
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoclr_usbclkgate(&mut self) -> EnautoclrUsbclkgateW<'_, CtrlTogSpec> {
        EnautoclrUsbclkgateW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoset_usbclks(&mut self) -> EnautosetUsbclksW<'_, CtrlTogSpec> {
        EnautosetUsbclksW::new(self, 26)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&mut self) -> HostForceLsSe0W<'_, CtrlTogSpec> {
        HostForceLsSe0W::new(self, 28)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&mut self) -> UtmiSuspendmW<'_, CtrlTogSpec> {
        UtmiSuspendmW::new(self, 29)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> ClkgateW<'_, CtrlTogSpec> {
        ClkgateW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SftrstW<'_, CtrlTogSpec> {
        SftrstW::new(self, 31)
    }
}
#[doc = "USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_tog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_tog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlTogSpec;
impl crate::RegisterSpec for CtrlTogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_tog::R`](R) reader structure"]
impl crate::Readable for CtrlTogSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_tog::W`](W) writer structure"]
impl crate::Writable for CtrlTogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_TOG to value 0xc000_0000"]
impl crate::Resettable for CtrlTogSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}

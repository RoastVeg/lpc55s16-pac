#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `OK_STATUS_3V` reader - Indicates the USB 3v power rails are in range."]
pub type OkStatus3vR = crate::BitReader;
#[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostdiscondetectStatus {
    #[doc = "0: USB cable disconnect has not been detected at the local host"]
    Value0 = 0,
    #[doc = "1: USB cable disconnect has been detected at the local host"]
    Value1 = 1,
}
impl From<HostdiscondetectStatus> for bool {
    #[inline(always)]
    fn from(variant: HostdiscondetectStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTDISCONDETECT_STATUS` reader - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
pub type HostdiscondetectStatusR = crate::BitReader<HostdiscondetectStatus>;
impl HostdiscondetectStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostdiscondetectStatus {
        match self.bits {
            false => HostdiscondetectStatus::Value0,
            true => HostdiscondetectStatus::Value1,
        }
    }
    #[doc = "USB cable disconnect has not been detected at the local host"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == HostdiscondetectStatus::Value0
    }
    #[doc = "USB cable disconnect has been detected at the local host"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HostdiscondetectStatus::Value1
    }
}
#[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevpluginStatus {
    #[doc = "0: No attachment to a USB host is detected"]
    Value0 = 0,
    #[doc = "1: Cable attachment to a USB host is detected"]
    Value1 = 1,
}
impl From<DevpluginStatus> for bool {
    #[inline(always)]
    fn from(variant: DevpluginStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVPLUGIN_STATUS` reader - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
pub type DevpluginStatusR = crate::BitReader<DevpluginStatus>;
impl DevpluginStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevpluginStatus {
        match self.bits {
            false => DevpluginStatus::Value0,
            true => DevpluginStatus::Value1,
        }
    }
    #[doc = "No attachment to a USB host is detected"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DevpluginStatus::Value0
    }
    #[doc = "Cable attachment to a USB host is detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DevpluginStatus::Value1
    }
}
#[doc = "Field `RESUME_STATUS` reader - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
pub type ResumeStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the USB 3v power rails are in range."]
    #[inline(always)]
    pub fn ok_status_3v(&self) -> OkStatus3vR {
        OkStatus3vR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HostdiscondetectStatusR {
        HostdiscondetectStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DevpluginStatusR {
        DevpluginStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> ResumeStatusR {
        ResumeStatusR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {}
#[doc = "USB PHY Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

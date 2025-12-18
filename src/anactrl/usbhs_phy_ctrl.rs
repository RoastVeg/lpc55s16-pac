#[doc = "Register `USBHS_PHY_CTRL` reader"]
pub type R = crate::R<UsbhsPhyCtrlSpec>;
#[doc = "Register `USBHS_PHY_CTRL` writer"]
pub type W = crate::W<UsbhsPhyCtrlSpec>;
#[doc = "Field `usb_vbusvalid_ext` reader - Override value for Vbus if using external detectors."]
pub type UsbVbusvalidExtR = crate::BitReader;
#[doc = "Field `usb_vbusvalid_ext` writer - Override value for Vbus if using external detectors."]
pub type UsbVbusvalidExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb_id_ext` reader - Override value for ID if using external detectors."]
pub type UsbIdExtR = crate::BitReader;
#[doc = "Field `usb_id_ext` writer - Override value for ID if using external detectors."]
pub type UsbIdExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_atx` reader - ."]
pub type IsoAtxR = crate::BitReader;
#[doc = "Field `iso_atx` writer - ."]
pub type IsoAtxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&self) -> UsbVbusvalidExtR {
        UsbVbusvalidExtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&self) -> UsbIdExtR {
        UsbIdExtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn iso_atx(&self) -> IsoAtxR {
        IsoAtxR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBHS_PHY_CTRL")
            .field("usb_vbusvalid_ext", &self.usb_vbusvalid_ext())
            .field("usb_id_ext", &self.usb_id_ext())
            .field("iso_atx", &self.iso_atx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&mut self) -> UsbVbusvalidExtW<'_, UsbhsPhyCtrlSpec> {
        UsbVbusvalidExtW::new(self, 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&mut self) -> UsbIdExtW<'_, UsbhsPhyCtrlSpec> {
        UsbIdExtW::new(self, 1)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn iso_atx(&mut self) -> IsoAtxW<'_, UsbhsPhyCtrlSpec> {
        IsoAtxW::new(self, 3)
    }
}
#[doc = "USB High Speed Phy Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhs_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhs_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbhsPhyCtrlSpec;
impl crate::RegisterSpec for UsbhsPhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbhs_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for UsbhsPhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbhs_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for UsbhsPhyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBHS_PHY_CTRL to value 0x08"]
impl crate::Resettable for UsbhsPhyCtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}

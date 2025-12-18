#[doc = "Register `USBCFG` reader"]
pub type R = crate::R<UsbcfgSpec>;
#[doc = "Register `USBCFG` writer"]
pub type W = crate::W<UsbcfgSpec>;
#[doc = "Field `XO32M_READY_TIME_OUT_MS` reader - no description available"]
pub type Xo32mReadyTimeOutMsR = crate::FieldReader;
#[doc = "Field `XO32M_READY_TIME_OUT_MS` writer - no description available"]
pub type Xo32mReadyTimeOutMsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB_SPEED` reader - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
pub type UsbSpeedR = crate::FieldReader;
#[doc = "Field `USB_SPEED` writer - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
pub type UsbSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB_USE_XO32M_CAPA_BANKS` reader - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
pub type UsbUseXo32mCapaBanksR = crate::BitReader;
#[doc = "Field `USB_USE_XO32M_CAPA_BANKS` writer - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
pub type UsbUseXo32mCapaBanksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn xo32m_ready_time_out_ms(&self) -> Xo32mReadyTimeOutMsR {
        Xo32mReadyTimeOutMsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
    #[inline(always)]
    pub fn usb_speed(&self) -> UsbSpeedR {
        UsbSpeedR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub fn usb_use_xo32m_capa_banks(&self) -> UsbUseXo32mCapaBanksR {
        UsbUseXo32mCapaBanksR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn xo32m_ready_time_out_ms(&mut self) -> Xo32mReadyTimeOutMsW<'_, UsbcfgSpec> {
        Xo32mReadyTimeOutMsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
    #[inline(always)]
    pub fn usb_speed(&mut self) -> UsbSpeedW<'_, UsbcfgSpec> {
        UsbSpeedW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub fn usb_use_xo32m_capa_banks(&mut self) -> UsbUseXo32mCapaBanksW<'_, UsbcfgSpec> {
        UsbUseXo32mCapaBanksW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbcfgSpec;
impl crate::RegisterSpec for UsbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcfg::R`](R) reader structure"]
impl crate::Readable for UsbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`usbcfg::W`](W) writer structure"]
impl crate::Writable for UsbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCFG to value 0"]
impl crate::Resettable for UsbcfgSpec {}

#[doc = "Register `USB_ID` reader"]
pub type R = crate::R<UsbIdSpec>;
#[doc = "Register `USB_ID` writer"]
pub type W = crate::W<UsbIdSpec>;
#[doc = "Field `USB_VENDOR_ID` reader - no description available"]
pub type UsbVendorIdR = crate::FieldReader<u16>;
#[doc = "Field `USB_VENDOR_ID` writer - no description available"]
pub type UsbVendorIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USB_PRODUCT_ID` reader - no description available"]
pub type UsbProductIdR = crate::FieldReader<u16>;
#[doc = "Field `USB_PRODUCT_ID` writer - no description available"]
pub type UsbProductIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn usb_vendor_id(&self) -> UsbVendorIdR {
        UsbVendorIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn usb_product_id(&self) -> UsbProductIdR {
        UsbProductIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_ID")
            .field("usb_vendor_id", &self.usb_vendor_id())
            .field("usb_product_id", &self.usb_product_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn usb_vendor_id(&mut self) -> UsbVendorIdW<'_, UsbIdSpec> {
        UsbVendorIdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn usb_product_id(&mut self) -> UsbProductIdW<'_, UsbIdSpec> {
        UsbProductIdW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbIdSpec;
impl crate::RegisterSpec for UsbIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_id::R`](R) reader structure"]
impl crate::Readable for UsbIdSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_id::W`](W) writer structure"]
impl crate::Writable for UsbIdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_ID to value 0"]
impl crate::Resettable for UsbIdSpec {}

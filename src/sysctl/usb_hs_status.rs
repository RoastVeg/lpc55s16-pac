#[doc = "Register `USB_HS_STATUS` reader"]
pub type R = crate::R<UsbHsStatusSpec>;
#[doc = "USB_HS: Low voltage detection on 3.3V supply.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbhs3vNok {
    #[doc = "0: 3v3 supply is good."]
    Supply3vOk = 0,
    #[doc = "1: 3v3 supply is too low."]
    Supply3vLow = 1,
}
impl From<Usbhs3vNok> for bool {
    #[inline(always)]
    fn from(variant: Usbhs3vNok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_3V_NOK` reader - USB_HS: Low voltage detection on 3.3V supply."]
pub type Usbhs3vNokR = crate::BitReader<Usbhs3vNok>;
impl Usbhs3vNokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbhs3vNok {
        match self.bits {
            false => Usbhs3vNok::Supply3vOk,
            true => Usbhs3vNok::Supply3vLow,
        }
    }
    #[doc = "3v3 supply is good."]
    #[inline(always)]
    pub fn is_supply_3v_ok(&self) -> bool {
        *self == Usbhs3vNok::Supply3vOk
    }
    #[doc = "3v3 supply is too low."]
    #[inline(always)]
    pub fn is_supply_3v_low(&self) -> bool {
        *self == Usbhs3vNok::Supply3vLow
    }
}
impl R {
    #[doc = "Bit 0 - USB_HS: Low voltage detection on 3.3V supply."]
    #[inline(always)]
    pub fn usbhs_3v_nok(&self) -> Usbhs3vNokR {
        Usbhs3vNokR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_HS_STATUS")
            .field("usbhs_3v_nok", &self.usbhs_3v_nok())
            .finish()
    }
}
#[doc = "Status register for USB HS\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_hs_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbHsStatusSpec;
impl crate::RegisterSpec for UsbHsStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_hs_status::R`](R) reader structure"]
impl crate::Readable for UsbHsStatusSpec {}
#[doc = "`reset()` method sets USB_HS_STATUS to value 0"]
impl crate::Resettable for UsbHsStatusSpec {}

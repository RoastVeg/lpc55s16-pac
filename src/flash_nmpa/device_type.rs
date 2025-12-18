#[doc = "Register `DEVICE_TYPE` reader"]
pub type R = crate::R<DeviceTypeSpec>;
#[doc = "Register `DEVICE_TYPE` writer"]
pub type W = crate::W<DeviceTypeSpec>;
#[doc = "Field `DEVICE_TYPE_NUM` reader - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
pub type DeviceTypeNumR = crate::FieldReader<u16>;
#[doc = "Field `DEVICE_TYPE_NUM` writer - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
pub type DeviceTypeNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEVICE_TYPE_SEC` reader - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DeviceTypeSecR = crate::BitReader;
#[doc = "Field `DEVICE_TYPE_SEC` writer - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DeviceTypeSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_TYPE_PKG` reader - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
pub type DeviceTypePkgR = crate::FieldReader;
#[doc = "Field `DEVICE_TYPE_PKG` writer - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
pub type DeviceTypePkgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEVICE_TYPE_PIN` reader - Number of pins on the package."]
pub type DeviceTypePinR = crate::FieldReader;
#[doc = "Field `DEVICE_TYPE_PIN` writer - Number of pins on the package."]
pub type DeviceTypePinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[inline(always)]
    pub fn device_type_num(&self) -> DeviceTypeNumR {
        DeviceTypeNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&self) -> DeviceTypeSecR {
        DeviceTypeSecR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[inline(always)]
    pub fn device_type_pkg(&self) -> DeviceTypePkgR {
        DeviceTypePkgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Number of pins on the package."]
    #[inline(always)]
    pub fn device_type_pin(&self) -> DeviceTypePinR {
        DeviceTypePinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_TYPE")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[inline(always)]
    pub fn device_type_num(&mut self) -> DeviceTypeNumW<'_, DeviceTypeSpec> {
        DeviceTypeNumW::new(self, 0)
    }
    #[doc = "Bit 16 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&mut self) -> DeviceTypeSecW<'_, DeviceTypeSpec> {
        DeviceTypeSecW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[inline(always)]
    pub fn device_type_pkg(&mut self) -> DeviceTypePkgW<'_, DeviceTypeSpec> {
        DeviceTypePkgW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Number of pins on the package."]
    #[inline(always)]
    pub fn device_type_pin(&mut self) -> DeviceTypePinW<'_, DeviceTypeSpec> {
        DeviceTypePinW::new(self, 24)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`device_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceTypeSpec;
impl crate::RegisterSpec for DeviceTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_type::R`](R) reader structure"]
impl crate::Readable for DeviceTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`device_type::W`](W) writer structure"]
impl crate::Writable for DeviceTypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVICE_TYPE to value 0"]
impl crate::Resettable for DeviceTypeSpec {}

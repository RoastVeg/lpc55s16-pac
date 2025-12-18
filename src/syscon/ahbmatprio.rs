#[doc = "Register `AHBMATPRIO` reader"]
pub type R = crate::R<AhbmatprioSpec>;
#[doc = "Register `AHBMATPRIO` writer"]
pub type W = crate::W<AhbmatprioSpec>;
#[doc = "Field `PRI_CPU0_CBUS` reader - CPU0 C-AHB bus."]
pub type PriCpu0CbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU0_CBUS` writer - CPU0 C-AHB bus."]
pub type PriCpu0CbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CPU0_SBUS` reader - CPU0 S-AHB bus."]
pub type PriCpu0SbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU0_SBUS` writer - CPU0 S-AHB bus."]
pub type PriCpu0SbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SDMA0` reader - DMA0 controller priority."]
pub type PriSdma0R = crate::FieldReader;
#[doc = "Field `PRI_SDMA0` writer - DMA0 controller priority."]
pub type PriSdma0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SDMA1` reader - DMA1 controller priority."]
pub type PriSdma1R = crate::FieldReader;
#[doc = "Field `PRI_SDMA1` writer - DMA1 controller priority."]
pub type PriSdma1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_USB_FSD` reader - USB0-FS Device.(USB0)"]
pub type PriUsbFsdR = crate::FieldReader;
#[doc = "Field `PRI_USB_FSD` writer - USB0-FS Device.(USB0)"]
pub type PriUsbFsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_USB_FSH` reader - USB0-FS host.(USB0)"]
pub type PriUsbFshR = crate::FieldReader;
#[doc = "Field `PRI_USB_FSH` writer - USB0-FS host.(USB0)"]
pub type PriUsbFshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_HASH_AES` reader - HASH_AES."]
pub type PriHashAesR = crate::FieldReader;
#[doc = "Field `PRI_HASH_AES` writer - HASH_AES."]
pub type PriHashAesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CANFD` reader - CANFD."]
pub type PriCanfdR = crate::FieldReader;
#[doc = "Field `PRI_CANFD` writer - CANFD."]
pub type PriCanfdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&self) -> PriCpu0CbusR {
        PriCpu0CbusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&self) -> PriCpu0SbusR {
        PriCpu0SbusR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&self) -> PriSdma0R {
        PriSdma0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&self) -> PriSdma1R {
        PriSdma1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - USB0-FS Device.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fsd(&self) -> PriUsbFsdR {
        PriUsbFsdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - USB0-FS host.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fsh(&self) -> PriUsbFshR {
        PriUsbFshR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&self) -> PriHashAesR {
        PriHashAesR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CANFD."]
    #[inline(always)]
    pub fn pri_canfd(&self) -> PriCanfdR {
        PriCanfdR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&mut self) -> PriCpu0CbusW<'_, AhbmatprioSpec> {
        PriCpu0CbusW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&mut self) -> PriCpu0SbusW<'_, AhbmatprioSpec> {
        PriCpu0SbusW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&mut self) -> PriSdma0W<'_, AhbmatprioSpec> {
        PriSdma0W::new(self, 4)
    }
    #[doc = "Bits 6:7 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&mut self) -> PriSdma1W<'_, AhbmatprioSpec> {
        PriSdma1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - USB0-FS Device.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fsd(&mut self) -> PriUsbFsdW<'_, AhbmatprioSpec> {
        PriUsbFsdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - USB0-FS host.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fsh(&mut self) -> PriUsbFshW<'_, AhbmatprioSpec> {
        PriUsbFshW::new(self, 10)
    }
    #[doc = "Bits 16:17 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&mut self) -> PriHashAesW<'_, AhbmatprioSpec> {
        PriHashAesW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CANFD."]
    #[inline(always)]
    pub fn pri_canfd(&mut self) -> PriCanfdW<'_, AhbmatprioSpec> {
        PriCanfdW::new(self, 18)
    }
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmatprioSpec;
impl crate::RegisterSpec for AhbmatprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmatprio::R`](R) reader structure"]
impl crate::Readable for AhbmatprioSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmatprio::W`](W) writer structure"]
impl crate::Writable for AhbmatprioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBMATPRIO to value 0"]
impl crate::Resettable for AhbmatprioSpec {}

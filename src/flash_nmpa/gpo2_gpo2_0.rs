#[doc = "Register `GPO2_0` reader"]
pub type R = crate::R<Gpo2Gpo2_0Spec>;
#[doc = "Register `GPO2_0` writer"]
pub type W = crate::W<Gpo2Gpo2_0Spec>;
#[doc = "Field `USBHS_PHY_TRIM_VALID` reader - no description available"]
pub type UsbhsPhyTrimValidR = crate::BitReader;
#[doc = "Field `USBHS_PHY_TRIM_VALID` writer - no description available"]
pub type UsbhsPhyTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` reader - no description available"]
pub type TrimUsbRegEnvTailAdjVdR = crate::FieldReader;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` writer - no description available"]
pub type TrimUsbRegEnvTailAdjVdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` reader - no description available"]
pub type TrimUsbphyTxDCalR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` writer - no description available"]
pub type TrimUsbphyTxDCalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` reader - no description available"]
pub type TrimUsbphyTxCal45dpR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` writer - no description available"]
pub type TrimUsbphyTxCal45dpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DN` reader - no description available"]
pub type TrimUsbphyTxCal45dnR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DN` writer - no description available"]
pub type TrimUsbphyTxCal45dnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` reader - no description available"]
pub type TrimUsb2RefbiasTstR = crate::FieldReader;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` writer - no description available"]
pub type TrimUsb2RefbiasTstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` reader - no description available"]
pub type TrimUsb2RefbiasVbgadjR = crate::FieldReader;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` writer - no description available"]
pub type TrimUsb2RefbiasVbgadjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` reader - no description available"]
pub type TrimPllCtrl0DivSelR = crate::FieldReader;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` writer - no description available"]
pub type TrimPllCtrl0DivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLASH_SIZE` reader - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
pub type FlashSizeR = crate::FieldReader;
#[doc = "Field `FLASH_SIZE` writer - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
pub type FlashSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` reader - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type Cpu0SecurityExtensionDisableR = crate::FieldReader;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` writer - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type Cpu0SecurityExtensionDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn usbhs_phy_trim_valid(&self) -> UsbhsPhyTrimValidR {
        UsbhsPhyTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - no description available"]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TrimUsbRegEnvTailAdjVdR {
        TrimUsbRegEnvTailAdjVdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TrimUsbphyTxDCalR {
        TrimUsbphyTxDCalR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:11 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TrimUsbphyTxCal45dpR {
        TrimUsbphyTxCal45dpR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dn(&self) -> TrimUsbphyTxCal45dnR {
        TrimUsbphyTxCal45dnR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - no description available"]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&self) -> TrimUsb2RefbiasTstR {
        TrimUsb2RefbiasTstR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - no description available"]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TrimUsb2RefbiasVbgadjR {
        TrimUsb2RefbiasVbgadjR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - no description available"]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TrimPllCtrl0DivSelR {
        TrimPllCtrl0DivSelR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&self) -> Cpu0SecurityExtensionDisableR {
        Cpu0SecurityExtensionDisableR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn usbhs_phy_trim_valid(&mut self) -> UsbhsPhyTrimValidW<'_, Gpo2Gpo2_0Spec> {
        UsbhsPhyTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:2 - no description available"]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&mut self) -> TrimUsbRegEnvTailAdjVdW<'_, Gpo2Gpo2_0Spec> {
        TrimUsbRegEnvTailAdjVdW::new(self, 1)
    }
    #[doc = "Bits 3:6 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> TrimUsbphyTxDCalW<'_, Gpo2Gpo2_0Spec> {
        TrimUsbphyTxDCalW::new(self, 3)
    }
    #[doc = "Bits 7:11 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> TrimUsbphyTxCal45dpW<'_, Gpo2Gpo2_0Spec> {
        TrimUsbphyTxCal45dpW::new(self, 7)
    }
    #[doc = "Bits 12:16 - no description available"]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dn(&mut self) -> TrimUsbphyTxCal45dnW<'_, Gpo2Gpo2_0Spec> {
        TrimUsbphyTxCal45dnW::new(self, 12)
    }
    #[doc = "Bits 17:18 - no description available"]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&mut self) -> TrimUsb2RefbiasTstW<'_, Gpo2Gpo2_0Spec> {
        TrimUsb2RefbiasTstW::new(self, 17)
    }
    #[doc = "Bits 19:21 - no description available"]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> TrimUsb2RefbiasVbgadjW<'_, Gpo2Gpo2_0Spec> {
        TrimUsb2RefbiasVbgadjW::new(self, 19)
    }
    #[doc = "Bits 22:24 - no description available"]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> TrimPllCtrl0DivSelW<'_, Gpo2Gpo2_0Spec> {
        TrimPllCtrl0DivSelW::new(self, 22)
    }
    #[doc = "Bits 25:27 - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FlashSizeW<'_, Gpo2Gpo2_0Spec> {
        FlashSizeW::new(self, 25)
    }
    #[doc = "Bits 28:31 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(
        &mut self,
    ) -> Cpu0SecurityExtensionDisableW<'_, Gpo2Gpo2_0Spec> {
        Cpu0SecurityExtensionDisableW::new(self, 28)
    }
}
#[doc = "GPO2 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo2Gpo2_0Spec;
impl crate::RegisterSpec for Gpo2Gpo2_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo2_gpo2_0::R`](R) reader structure"]
impl crate::Readable for Gpo2Gpo2_0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo2_gpo2_0::W`](W) writer structure"]
impl crate::Writable for Gpo2Gpo2_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO2_0 to value 0"]
impl crate::Resettable for Gpo2Gpo2_0Spec {}

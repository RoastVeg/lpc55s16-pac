#[doc = "Register `USBHS_PHY_TRIM` reader"]
pub type R = crate::R<UsbhsPhyTrimSpec>;
#[doc = "Register `USBHS_PHY_TRIM` writer"]
pub type W = crate::W<UsbhsPhyTrimSpec>;
#[doc = "Field `trim_usb_reg_env_tail_adj_vd` reader - Adjusts time constant of HS RX squelch (envelope) comparator."]
pub type TrimUsbRegEnvTailAdjVdR = crate::FieldReader;
#[doc = "Field `trim_usb_reg_env_tail_adj_vd` writer - Adjusts time constant of HS RX squelch (envelope) comparator."]
pub type TrimUsbRegEnvTailAdjVdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trim_usbphy_tx_d_cal` reader - ."]
pub type TrimUsbphyTxDCalR = crate::FieldReader;
#[doc = "Field `trim_usbphy_tx_d_cal` writer - ."]
pub type TrimUsbphyTxDCalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `trim_usbphy_tx_cal45dp` reader - ."]
pub type TrimUsbphyTxCal45dpR = crate::FieldReader;
#[doc = "Field `trim_usbphy_tx_cal45dp` writer - ."]
pub type TrimUsbphyTxCal45dpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `trim_usbphy_tx_cal45dm` reader - ."]
pub type TrimUsbphyTxCal45dmR = crate::FieldReader;
#[doc = "Field `trim_usbphy_tx_cal45dm` writer - ."]
pub type TrimUsbphyTxCal45dmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `trim_usb2_refbias_tst` reader - ."]
pub type TrimUsb2RefbiasTstR = crate::FieldReader;
#[doc = "Field `trim_usb2_refbias_tst` writer - ."]
pub type TrimUsb2RefbiasTstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trim_usb2_refbias_vbgadj` reader - ."]
pub type TrimUsb2RefbiasVbgadjR = crate::FieldReader;
#[doc = "Field `trim_usb2_refbias_vbgadj` writer - ."]
pub type TrimUsb2RefbiasVbgadjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `trim_pll_ctrl0_div_sel` reader - ."]
pub type TrimPllCtrl0DivSelR = crate::FieldReader;
#[doc = "Field `trim_pll_ctrl0_div_sel` writer - ."]
pub type TrimPllCtrl0DivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TrimUsbRegEnvTailAdjVdR {
        TrimUsbRegEnvTailAdjVdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TrimUsbphyTxDCalR {
        TrimUsbphyTxDCalR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TrimUsbphyTxCal45dpR {
        TrimUsbphyTxCal45dpR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TrimUsbphyTxCal45dmR {
        TrimUsbphyTxCal45dmR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&self) -> TrimUsb2RefbiasTstR {
        TrimUsb2RefbiasTstR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TrimUsb2RefbiasVbgadjR {
        TrimUsb2RefbiasVbgadjR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - ."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TrimPllCtrl0DivSelR {
        TrimPllCtrl0DivSelR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(
        &mut self,
    ) -> TrimUsbRegEnvTailAdjVdW<'_, UsbhsPhyTrimSpec> {
        TrimUsbRegEnvTailAdjVdW::new(self, 0)
    }
    #[doc = "Bits 2:5 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> TrimUsbphyTxDCalW<'_, UsbhsPhyTrimSpec> {
        TrimUsbphyTxDCalW::new(self, 2)
    }
    #[doc = "Bits 6:10 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> TrimUsbphyTxCal45dpW<'_, UsbhsPhyTrimSpec> {
        TrimUsbphyTxCal45dpW::new(self, 6)
    }
    #[doc = "Bits 11:15 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&mut self) -> TrimUsbphyTxCal45dmW<'_, UsbhsPhyTrimSpec> {
        TrimUsbphyTxCal45dmW::new(self, 11)
    }
    #[doc = "Bits 16:17 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&mut self) -> TrimUsb2RefbiasTstW<'_, UsbhsPhyTrimSpec> {
        TrimUsb2RefbiasTstW::new(self, 16)
    }
    #[doc = "Bits 18:20 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> TrimUsb2RefbiasVbgadjW<'_, UsbhsPhyTrimSpec> {
        TrimUsb2RefbiasVbgadjW::new(self, 18)
    }
    #[doc = "Bits 21:23 - ."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> TrimPllCtrl0DivSelW<'_, UsbhsPhyTrimSpec> {
        TrimPllCtrl0DivSelW::new(self, 21)
    }
}
#[doc = "USB High Speed Phy Trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhs_phy_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhs_phy_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbhsPhyTrimSpec;
impl crate::RegisterSpec for UsbhsPhyTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbhs_phy_trim::R`](R) reader structure"]
impl crate::Readable for UsbhsPhyTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`usbhs_phy_trim::W`](W) writer structure"]
impl crate::Writable for UsbhsPhyTrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBHS_PHY_TRIM to value 0"]
impl crate::Resettable for UsbhsPhyTrimSpec {}

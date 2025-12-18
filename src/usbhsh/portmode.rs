#[doc = "Register `PORTMODE` reader"]
pub type R = crate::R<PortmodeSpec>;
#[doc = "Register `PORTMODE` writer"]
pub type W = crate::W<PortmodeSpec>;
#[doc = "Field `DEV_ENABLE` reader - If this bit is set to one, one of the ports will behave as a USB device."]
pub type DevEnableR = crate::BitReader;
#[doc = "Field `DEV_ENABLE` writer - If this bit is set to one, one of the ports will behave as a USB device."]
pub type DevEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CTRL_PDCOM` reader - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
pub type SwCtrlPdcomR = crate::BitReader;
#[doc = "Field `SW_CTRL_PDCOM` writer - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
pub type SwCtrlPdcomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PDCOM` reader - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
pub type SwPdcomR = crate::BitReader;
#[doc = "Field `SW_PDCOM` writer - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
pub type SwPdcomW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DevEnableR {
        DevEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&self) -> SwCtrlPdcomR {
        SwCtrlPdcomR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&self) -> SwPdcomR {
        SwPdcomR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMODE")
            .field("dev_enable", &self.dev_enable())
            .field("sw_ctrl_pdcom", &self.sw_ctrl_pdcom())
            .field("sw_pdcom", &self.sw_pdcom())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DevEnableW<'_, PortmodeSpec> {
        DevEnableW::new(self, 16)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&mut self) -> SwCtrlPdcomW<'_, PortmodeSpec> {
        SwCtrlPdcomW::new(self, 18)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&mut self) -> SwPdcomW<'_, PortmodeSpec> {
        SwPdcomW::new(self, 19)
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nYou can [`read`](crate::Reg::read) this register and get [`portmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortmodeSpec;
impl crate::RegisterSpec for PortmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portmode::R`](R) reader structure"]
impl crate::Readable for PortmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`portmode::W`](W) writer structure"]
impl crate::Writable for PortmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTMODE to value 0x0004_0000"]
impl crate::Resettable for PortmodeSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}

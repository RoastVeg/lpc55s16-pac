#[doc = "Register `BOOT_CFG` reader"]
pub type R = crate::R<BootCfgSpec>;
#[doc = "Register `BOOT_CFG` writer"]
pub type W = crate::W<BootCfgSpec>;
#[doc = "Default ISP mode:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefaultIspMode {
    #[doc = "0: Auto ISP"]
    AutoIsp = 0,
    #[doc = "1: USB_HID_ISP"]
    UsbHidIsp = 1,
    #[doc = "2: UART ISP"]
    UartIsp = 2,
    #[doc = "3: SPI Slave ISP"]
    SpiIsp = 3,
    #[doc = "4: I2C Slave ISP"]
    I2cIsp = 4,
    #[doc = "7: Disable ISP fall through"]
    Disable = 7,
}
impl From<DefaultIspMode> for u8 {
    #[inline(always)]
    fn from(variant: DefaultIspMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DefaultIspMode {
    type Ux = u8;
}
impl crate::IsEnum for DefaultIspMode {}
#[doc = "Field `DEFAULT_ISP_MODE` reader - Default ISP mode:"]
pub type DefaultIspModeR = crate::FieldReader<DefaultIspMode>;
impl DefaultIspModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DefaultIspMode> {
        match self.bits {
            0 => Some(DefaultIspMode::AutoIsp),
            1 => Some(DefaultIspMode::UsbHidIsp),
            2 => Some(DefaultIspMode::UartIsp),
            3 => Some(DefaultIspMode::SpiIsp),
            4 => Some(DefaultIspMode::I2cIsp),
            7 => Some(DefaultIspMode::Disable),
            _ => None,
        }
    }
    #[doc = "Auto ISP"]
    #[inline(always)]
    pub fn is_auto_isp(&self) -> bool {
        *self == DefaultIspMode::AutoIsp
    }
    #[doc = "USB_HID_ISP"]
    #[inline(always)]
    pub fn is_usb_hid_isp(&self) -> bool {
        *self == DefaultIspMode::UsbHidIsp
    }
    #[doc = "UART ISP"]
    #[inline(always)]
    pub fn is_uart_isp(&self) -> bool {
        *self == DefaultIspMode::UartIsp
    }
    #[doc = "SPI Slave ISP"]
    #[inline(always)]
    pub fn is_spi_isp(&self) -> bool {
        *self == DefaultIspMode::SpiIsp
    }
    #[doc = "I2C Slave ISP"]
    #[inline(always)]
    pub fn is_i2c_isp(&self) -> bool {
        *self == DefaultIspMode::I2cIsp
    }
    #[doc = "Disable ISP fall through"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DefaultIspMode::Disable
    }
}
#[doc = "Field `DEFAULT_ISP_MODE` writer - Default ISP mode:"]
pub type DefaultIspModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, DefaultIspMode>;
impl<'a, REG> DefaultIspModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto ISP"]
    #[inline(always)]
    pub fn auto_isp(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::AutoIsp)
    }
    #[doc = "USB_HID_ISP"]
    #[inline(always)]
    pub fn usb_hid_isp(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::UsbHidIsp)
    }
    #[doc = "UART ISP"]
    #[inline(always)]
    pub fn uart_isp(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::UartIsp)
    }
    #[doc = "SPI Slave ISP"]
    #[inline(always)]
    pub fn spi_isp(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::SpiIsp)
    }
    #[doc = "I2C Slave ISP"]
    #[inline(always)]
    pub fn i2c_isp(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::I2cIsp)
    }
    #[doc = "Disable ISP fall through"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DefaultIspMode::Disable)
    }
}
#[doc = "Core clock:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootSpeed {
    #[doc = "0: Defined by NMPA.SYSTEM_SPEED_CODE"]
    Value0 = 0,
    #[doc = "2: 48MHz FRO"]
    Value2 = 2,
}
impl From<BootSpeed> for u8 {
    #[inline(always)]
    fn from(variant: BootSpeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BootSpeed {
    type Ux = u8;
}
impl crate::IsEnum for BootSpeed {}
#[doc = "Field `BOOT_SPEED` reader - Core clock:"]
pub type BootSpeedR = crate::FieldReader<BootSpeed>;
impl BootSpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BootSpeed> {
        match self.bits {
            0 => Some(BootSpeed::Value0),
            2 => Some(BootSpeed::Value2),
            _ => None,
        }
    }
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == BootSpeed::Value0
    }
    #[doc = "48MHz FRO"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BootSpeed::Value2
    }
}
#[doc = "Field `BOOT_SPEED` writer - Core clock:"]
pub type BootSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, BootSpeed>;
impl<'a, REG> BootSpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(BootSpeed::Value0)
    }
    #[doc = "48MHz FRO"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(BootSpeed::Value2)
    }
}
#[doc = "Field `BOOT_FAILURE_PIN` reader - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
pub type BootFailurePinR = crate::FieldReader;
#[doc = "Field `BOOT_FAILURE_PIN` writer - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
pub type BootFailurePinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&self) -> DefaultIspModeR {
        DefaultIspModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&self) -> BootSpeedR {
        BootSpeedR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&self) -> BootFailurePinR {
        BootFailurePinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_CFG")
            .field("default_isp_mode", &self.default_isp_mode())
            .field("boot_speed", &self.boot_speed())
            .field("boot_failure_pin", &self.boot_failure_pin())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&mut self) -> DefaultIspModeW<'_, BootCfgSpec> {
        DefaultIspModeW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&mut self) -> BootSpeedW<'_, BootCfgSpec> {
        BootSpeedW::new(self, 7)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&mut self) -> BootFailurePinW<'_, BootCfgSpec> {
        BootFailurePinW::new(self, 24)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootCfgSpec;
impl crate::RegisterSpec for BootCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_cfg::R`](R) reader structure"]
impl crate::Readable for BootCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_cfg::W`](W) writer structure"]
impl crate::Writable for BootCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_CFG to value 0"]
impl crate::Resettable for BootCfgSpec {}

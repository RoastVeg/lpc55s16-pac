#[doc = "Register `ANALOG_CTRL_STATUS` reader"]
pub type R = crate::R<AnalogCtrlStatusSpec>;
#[doc = "Flash Power Down status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashPwrdwn {
    #[doc = "0: Flash is not in power down mode."]
    Pwrup = 0,
    #[doc = "1: Flash is in power down mode."]
    Pwrdwn = 1,
}
impl From<FlashPwrdwn> for bool {
    #[inline(always)]
    fn from(variant: FlashPwrdwn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_PWRDWN` reader - Flash Power Down status."]
pub type FlashPwrdwnR = crate::BitReader<FlashPwrdwn>;
impl FlashPwrdwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashPwrdwn {
        match self.bits {
            false => FlashPwrdwn::Pwrup,
            true => FlashPwrdwn::Pwrdwn,
        }
    }
    #[doc = "Flash is not in power down mode."]
    #[inline(always)]
    pub fn is_pwrup(&self) -> bool {
        *self == FlashPwrdwn::Pwrup
    }
    #[doc = "Flash is in power down mode."]
    #[inline(always)]
    pub fn is_pwrdwn(&self) -> bool {
        *self == FlashPwrdwn::Pwrdwn
    }
}
#[doc = "Flash initialization error status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashInitError {
    #[doc = "0: No error."]
    Noerror = 0,
    #[doc = "1: At least one error occured during flash initialization.."]
    Error = 1,
}
impl From<FlashInitError> for bool {
    #[inline(always)]
    fn from(variant: FlashInitError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_INIT_ERROR` reader - Flash initialization error status."]
pub type FlashInitErrorR = crate::BitReader<FlashInitError>;
impl FlashInitErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashInitError {
        match self.bits {
            false => FlashInitError::Noerror,
            true => FlashInitError::Error,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == FlashInitError::Noerror
    }
    #[doc = "At least one error occured during flash initialization.."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FlashInitError::Error
    }
}
impl R {
    #[doc = "Bit 12 - Flash Power Down status."]
    #[inline(always)]
    pub fn flash_pwrdwn(&self) -> FlashPwrdwnR {
        FlashPwrdwnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash initialization error status."]
    #[inline(always)]
    pub fn flash_init_error(&self) -> FlashInitErrorR {
        FlashInitErrorR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Analog Macroblock Identity registers, Flash Status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_ctrl_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtrlStatusSpec;
impl crate::RegisterSpec for AnalogCtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctrl_status::R`](R) reader structure"]
impl crate::Readable for AnalogCtrlStatusSpec {}
#[doc = "`reset()` method sets ANALOG_CTRL_STATUS to value 0x5000_0000"]
impl crate::Resettable for AnalogCtrlStatusSpec {
    const RESET_VALUE: u32 = 0x5000_0000;
}

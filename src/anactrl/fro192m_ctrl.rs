#[doc = "Register `FRO192M_CTRL` reader"]
pub type R = crate::R<Fro192mCtrlSpec>;
#[doc = "Register `FRO192M_CTRL` writer"]
pub type W = crate::W<Fro192mCtrlSpec>;
#[doc = "12 MHz clock control.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena12mhzclk {
    #[doc = "0: 12 MHz clock is disabled."]
    Disable = 0,
    #[doc = "1: 12 MHz clock is enabled."]
    Enable = 1,
}
impl From<Ena12mhzclk> for bool {
    #[inline(always)]
    fn from(variant: Ena12mhzclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_12MHZCLK` reader - 12 MHz clock control."]
pub type Ena12mhzclkR = crate::BitReader<Ena12mhzclk>;
impl Ena12mhzclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena12mhzclk {
        match self.bits {
            false => Ena12mhzclk::Disable,
            true => Ena12mhzclk::Enable,
        }
    }
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ena12mhzclk::Disable
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ena12mhzclk::Enable
    }
}
#[doc = "Field `ENA_12MHZCLK` writer - 12 MHz clock control."]
pub type Ena12mhzclkW<'a, REG> = crate::BitWriter<'a, REG, Ena12mhzclk>;
impl<'a, REG> Ena12mhzclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ena12mhzclk::Disable)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ena12mhzclk::Enable)
    }
}
#[doc = "Field `DAC_TRIM` reader - Frequency trim."]
pub type DacTrimR = crate::FieldReader;
#[doc = "Field `DAC_TRIM` writer - Frequency trim."]
pub type DacTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USBCLKADJ` reader - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub type UsbclkadjR = crate::BitReader;
#[doc = "Field `USBCLKADJ` writer - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub type UsbclkadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBMODCHG` reader - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
pub type UsbmodchgR = crate::BitReader;
#[doc = "96 MHz clock control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena96mhzclk {
    #[doc = "0: 96 MHz clock is disabled."]
    Disable = 0,
    #[doc = "1: 96 MHz clock is enabled."]
    Enable = 1,
}
impl From<Ena96mhzclk> for bool {
    #[inline(always)]
    fn from(variant: Ena96mhzclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_96MHZCLK` reader - 96 MHz clock control."]
pub type Ena96mhzclkR = crate::BitReader<Ena96mhzclk>;
impl Ena96mhzclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena96mhzclk {
        match self.bits {
            false => Ena96mhzclk::Disable,
            true => Ena96mhzclk::Enable,
        }
    }
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ena96mhzclk::Disable
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ena96mhzclk::Enable
    }
}
#[doc = "Field `ENA_96MHZCLK` writer - 96 MHz clock control."]
pub type Ena96mhzclkW<'a, REG> = crate::BitWriter<'a, REG, Ena96mhzclk>;
impl<'a, REG> Ena96mhzclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ena96mhzclk::Disable)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ena96mhzclk::Enable)
    }
}
#[doc = "Field `WRTRIM` writer - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
pub type WrtrimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&self) -> Ena12mhzclkR {
        Ena12mhzclkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DacTrimR {
        DacTrimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> UsbclkadjR {
        UsbclkadjR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> UsbmodchgR {
        UsbmodchgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> Ena96mhzclkR {
        Ena96mhzclkR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO192M_CTRL")
            .field("ena_12mhzclk", &self.ena_12mhzclk())
            .field("dac_trim", &self.dac_trim())
            .field("usbclkadj", &self.usbclkadj())
            .field("usbmodchg", &self.usbmodchg())
            .field("ena_96mhzclk", &self.ena_96mhzclk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&mut self) -> Ena12mhzclkW<'_, Fro192mCtrlSpec> {
        Ena12mhzclkW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&mut self) -> DacTrimW<'_, Fro192mCtrlSpec> {
        DacTrimW::new(self, 16)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> UsbclkadjW<'_, Fro192mCtrlSpec> {
        UsbclkadjW::new(self, 24)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&mut self) -> Ena96mhzclkW<'_, Fro192mCtrlSpec> {
        Ena96mhzclkW::new(self, 30)
    }
    #[doc = "Bit 31 - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WrtrimW<'_, Fro192mCtrlSpec> {
        WrtrimW::new(self, 31)
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fro192m_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro192m_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fro192mCtrlSpec;
impl crate::RegisterSpec for Fro192mCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro192m_ctrl::R`](R) reader structure"]
impl crate::Readable for Fro192mCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fro192m_ctrl::W`](W) writer structure"]
impl crate::Writable for Fro192mCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRO192M_CTRL to value 0x0080_d01a"]
impl crate::Resettable for Fro192mCtrlSpec {
    const RESET_VALUE: u32 = 0x0080_d01a;
}

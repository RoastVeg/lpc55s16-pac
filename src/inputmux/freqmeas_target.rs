#[doc = "Register `FREQMEAS_TARGET` reader"]
pub type R = crate::R<FreqmeasTargetSpec>;
#[doc = "Register `FREQMEAS_TARGET` writer"]
pub type W = crate::W<FreqmeasTargetSpec>;
#[doc = "Clock source number (decimal value) for frequency measure function target clock:\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkin {
    #[doc = "0: External main crystal oscilator (Clock_in)."]
    Value0 = 0,
    #[doc = "1: FRO 12MHz clock."]
    Value1 = 1,
    #[doc = "2: FRO 96MHz clock."]
    Value2 = 2,
    #[doc = "3: Watchdog oscillator / FRO1MHz clock."]
    Value3 = 3,
    #[doc = "4: 32 kHz oscillator (32k_clk) clock."]
    Value4 = 4,
    #[doc = "5: main clock (main_clock)."]
    Value5 = 5,
    #[doc = "6: FREQME_GPIO_CLK_A."]
    Value6 = 6,
    #[doc = "7: FREQME_GPIO_CLK_B."]
    Value7 = 7,
}
impl From<Clkin> for u8 {
    #[inline(always)]
    fn from(variant: Clkin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkin {
    type Ux = u8;
}
impl crate::IsEnum for Clkin {}
#[doc = "Field `CLKIN` reader - Clock source number (decimal value) for frequency measure function target clock:"]
pub type ClkinR = crate::FieldReader<Clkin>;
impl ClkinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkin> {
        match self.bits {
            0 => Some(Clkin::Value0),
            1 => Some(Clkin::Value1),
            2 => Some(Clkin::Value2),
            3 => Some(Clkin::Value3),
            4 => Some(Clkin::Value4),
            5 => Some(Clkin::Value5),
            6 => Some(Clkin::Value6),
            7 => Some(Clkin::Value7),
            _ => None,
        }
    }
    #[doc = "External main crystal oscilator (Clock_in)."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Clkin::Value0
    }
    #[doc = "FRO 12MHz clock."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clkin::Value1
    }
    #[doc = "FRO 96MHz clock."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clkin::Value2
    }
    #[doc = "Watchdog oscillator / FRO1MHz clock."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clkin::Value3
    }
    #[doc = "32 kHz oscillator (32k_clk) clock."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clkin::Value4
    }
    #[doc = "main clock (main_clock)."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Clkin::Value5
    }
    #[doc = "FREQME_GPIO_CLK_A."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Clkin::Value6
    }
    #[doc = "FREQME_GPIO_CLK_B."]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Clkin::Value7
    }
}
#[doc = "Field `CLKIN` writer - Clock source number (decimal value) for frequency measure function target clock:"]
pub type ClkinW<'a, REG> = crate::FieldWriter<'a, REG, 5, Clkin>;
impl<'a, REG> ClkinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External main crystal oscilator (Clock_in)."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value0)
    }
    #[doc = "FRO 12MHz clock."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value1)
    }
    #[doc = "FRO 96MHz clock."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value2)
    }
    #[doc = "Watchdog oscillator / FRO1MHz clock."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value3)
    }
    #[doc = "32 kHz oscillator (32k_clk) clock."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value4)
    }
    #[doc = "main clock (main_clock)."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value5)
    }
    #[doc = "FREQME_GPIO_CLK_A."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value6)
    }
    #[doc = "FREQME_GPIO_CLK_B."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Clkin::Value7)
    }
}
impl R {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock:"]
    #[inline(always)]
    pub fn clkin(&self) -> ClkinR {
        ClkinR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQMEAS_TARGET")
            .field("clkin", &self.clkin())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock:"]
    #[inline(always)]
    pub fn clkin(&mut self) -> ClkinW<'_, FreqmeasTargetSpec> {
        ClkinW::new(self, 0)
    }
}
#[doc = "Selection for frequency measurement target clock\n\nYou can [`read`](crate::Reg::read) this register and get [`freqmeas_target::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqmeas_target::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqmeasTargetSpec;
impl crate::RegisterSpec for FreqmeasTargetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqmeas_target::R`](R) reader structure"]
impl crate::Readable for FreqmeasTargetSpec {}
#[doc = "`write(|w| ..)` method takes [`freqmeas_target::W`](W) writer structure"]
impl crate::Writable for FreqmeasTargetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREQMEAS_TARGET to value 0x1f"]
impl crate::Resettable for FreqmeasTargetSpec {
    const RESET_VALUE: u32 = 0x1f;
}

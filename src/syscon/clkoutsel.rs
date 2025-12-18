#[doc = "Register `CLKOUTSEL` reader"]
pub type R = crate::R<ClkoutselSpec>;
#[doc = "Register `CLKOUTSEL` writer"]
pub type W = crate::W<ClkoutselSpec>;
#[doc = "CLKOUT clock source select.\n\nValue on reset: 15"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main clock."]
    Enum0x0 = 0,
    #[doc = "1: PLL0 clock."]
    Enum0x1 = 1,
    #[doc = "2: CLKIN clock."]
    Enum0x2 = 2,
    #[doc = "3: FRO 96 MHz clock."]
    Enum0x3 = 3,
    #[doc = "4: FRO 1MHz clock."]
    Enum0x4 = 4,
    #[doc = "5: PLL1 clock."]
    Enum0x5 = 5,
    #[doc = "6: Oscillator 32kHz clock."]
    Enum0x6 = 6,
    #[doc = "7: No clock."]
    Enum0x7 = 7,
    #[doc = "12: No clock."]
    Enum0xC = 12,
    #[doc = "13: No clock."]
    Enum0xD = 13,
    #[doc = "14: No clock."]
    Enum0xE = 14,
    #[doc = "15: No clock."]
    Enum0xF = 15,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - CLKOUT clock source select."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Enum0x0),
            1 => Some(Sel::Enum0x1),
            2 => Some(Sel::Enum0x2),
            3 => Some(Sel::Enum0x3),
            4 => Some(Sel::Enum0x4),
            5 => Some(Sel::Enum0x5),
            6 => Some(Sel::Enum0x6),
            7 => Some(Sel::Enum0x7),
            12 => Some(Sel::Enum0xC),
            13 => Some(Sel::Enum0xD),
            14 => Some(Sel::Enum0xE),
            15 => Some(Sel::Enum0xF),
            _ => None,
        }
    }
    #[doc = "Main clock."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Sel::Enum0x0
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
    #[doc = "CLKIN clock."]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == Sel::Enum0x2
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == Sel::Enum0x3
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x4(&self) -> bool {
        *self == Sel::Enum0x4
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn is_enum_0x5(&self) -> bool {
        *self == Sel::Enum0x5
    }
    #[doc = "Oscillator 32kHz clock."]
    #[inline(always)]
    pub fn is_enum_0x6(&self) -> bool {
        *self == Sel::Enum0x6
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x7(&self) -> bool {
        *self == Sel::Enum0x7
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x_c(&self) -> bool {
        *self == Sel::Enum0xC
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x_d(&self) -> bool {
        *self == Sel::Enum0xD
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x_e(&self) -> bool {
        *self == Sel::Enum0xE
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x_f(&self) -> bool {
        *self == Sel::Enum0xF
    }
}
#[doc = "Field `SEL` writer - CLKOUT clock source select."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x0)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
    #[doc = "CLKIN clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x2)
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x3)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn enum_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x4)
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn enum_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x5)
    }
    #[doc = "Oscillator 32kHz clock."]
    #[inline(always)]
    pub fn enum_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x6)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x7)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0xC)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0xD)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0xE)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - CLKOUT clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CLKOUT clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, ClkoutselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "CLKOUT clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutselSpec;
impl crate::RegisterSpec for ClkoutselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutsel::R`](R) reader structure"]
impl crate::Readable for ClkoutselSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutsel::W`](W) writer structure"]
impl crate::Writable for ClkoutselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKOUTSEL to value 0x0f"]
impl crate::Resettable for ClkoutselSpec {
    const RESET_VALUE: u32 = 0x0f;
}

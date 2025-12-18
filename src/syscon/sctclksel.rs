#[doc = "Register `SCTCLKSEL` reader"]
pub type R = crate::R<SctclkselSpec>;
#[doc = "Register `SCTCLKSEL` writer"]
pub type W = crate::W<SctclkselSpec>;
#[doc = "SCTimer/PWM clock source select.\n\nValue on reset: 7"]
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
    #[doc = "4: No clock."]
    Enum0x4 = 4,
    #[doc = "5: MCLK clock."]
    Enum0x5 = 5,
    #[doc = "6: No clock."]
    Enum0x6 = 6,
    #[doc = "7: No clock."]
    Enum0x7 = 7,
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
#[doc = "Field `SEL` reader - SCTimer/PWM clock source select."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::Enum0x0,
            1 => Sel::Enum0x1,
            2 => Sel::Enum0x2,
            3 => Sel::Enum0x3,
            4 => Sel::Enum0x4,
            5 => Sel::Enum0x5,
            6 => Sel::Enum0x6,
            7 => Sel::Enum0x7,
            _ => unreachable!(),
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
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x4(&self) -> bool {
        *self == Sel::Enum0x4
    }
    #[doc = "MCLK clock."]
    #[inline(always)]
    pub fn is_enum_0x5(&self) -> bool {
        *self == Sel::Enum0x5
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x6(&self) -> bool {
        *self == Sel::Enum0x6
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x7(&self) -> bool {
        *self == Sel::Enum0x7
    }
}
#[doc = "Field `SEL` writer - SCTimer/PWM clock source select."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel, crate::Safe>;
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
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x4)
    }
    #[doc = "MCLK clock."]
    #[inline(always)]
    pub fn enum_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x5)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x6)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x7)
    }
}
impl R {
    #[doc = "Bits 0:2 - SCTimer/PWM clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SCTimer/PWM clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, SctclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "SCTimer/PWM clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`sctclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctclkselSpec;
impl crate::RegisterSpec for SctclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctclksel::R`](R) reader structure"]
impl crate::Readable for SctclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`sctclksel::W`](W) writer structure"]
impl crate::Writable for SctclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCTCLKSEL to value 0x07"]
impl crate::Resettable for SctclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}

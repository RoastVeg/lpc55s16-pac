#[doc = "Register `MCLKCLKSEL` reader"]
pub type R = crate::R<MclkclkselSpec>;
#[doc = "Register `MCLKCLKSEL` writer"]
pub type W = crate::W<MclkclkselSpec>;
#[doc = "MCLK clock source select.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: FRO 96 MHz clock."]
    Enum0x0 = 0,
    #[doc = "1: PLL0 clock."]
    Enum0x1 = 1,
    #[doc = "4: No clock."]
    Enum0x4 = 4,
    #[doc = "5: No clock."]
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
#[doc = "Field `SEL` reader - MCLK clock source select."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Enum0x0),
            1 => Some(Sel::Enum0x1),
            4 => Some(Sel::Enum0x4),
            5 => Some(Sel::Enum0x5),
            6 => Some(Sel::Enum0x6),
            7 => Some(Sel::Enum0x7),
            _ => None,
        }
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Sel::Enum0x0
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x4(&self) -> bool {
        *self == Sel::Enum0x4
    }
    #[doc = "No clock."]
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
#[doc = "Field `SEL` writer - MCLK clock source select."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x0)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x4)
    }
    #[doc = "No clock."]
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
    #[doc = "Bits 0:2 - MCLK clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, MclkclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "MCLK clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkclkselSpec;
impl crate::RegisterSpec for MclkclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkclksel::R`](R) reader structure"]
impl crate::Readable for MclkclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`mclkclksel::W`](W) writer structure"]
impl crate::Writable for MclkclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCLKCLKSEL to value 0x07"]
impl crate::Resettable for MclkclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}

#[doc = "Register `FCCLKSEL3` reader"]
pub type R = crate::R<FcclkselFcclksel3Spec>;
#[doc = "Register `FCCLKSEL3` writer"]
pub type W = crate::W<FcclkselFcclksel3Spec>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main clock."]
    Enum0x0 = 0,
    #[doc = "1: system PLL divided clock."]
    Enum0x1 = 1,
    #[doc = "2: FRO 12 MHz clock."]
    Enum0x2 = 2,
    #[doc = "3: FRO 96 MHz clock."]
    Enum0x3 = 3,
    #[doc = "4: FRO 1MHz clock."]
    Enum0x4 = 4,
    #[doc = "5: MCLK clock."]
    Enum0x5 = 5,
    #[doc = "6: Oscillator 32 kHz clock."]
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
#[doc = "Field `SEL` reader - Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
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
    #[doc = "system PLL divided clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
    #[doc = "FRO 12 MHz clock."]
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
    #[doc = "MCLK clock."]
    #[inline(always)]
    pub fn is_enum_0x5(&self) -> bool {
        *self == Sel::Enum0x5
    }
    #[doc = "Oscillator 32 kHz clock."]
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
#[doc = "Field `SEL` writer - Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
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
    #[doc = "system PLL divided clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
    #[doc = "FRO 12 MHz clock."]
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
    #[doc = "MCLK clock."]
    #[inline(always)]
    pub fn enum_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x5)
    }
    #[doc = "Oscillator 32 kHz clock."]
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
    #[doc = "Bits 0:2 - Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, FcclkselFcclksel3Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcclkselFcclksel3Spec;
impl crate::RegisterSpec for FcclkselFcclksel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcclksel_fcclksel3::R`](R) reader structure"]
impl crate::Readable for FcclkselFcclksel3Spec {}
#[doc = "`write(|w| ..)` method takes [`fcclksel_fcclksel3::W`](W) writer structure"]
impl crate::Writable for FcclkselFcclksel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCLKSEL3 to value 0x07"]
impl crate::Resettable for FcclkselFcclksel3Spec {
    const RESET_VALUE: u32 = 0x07;
}

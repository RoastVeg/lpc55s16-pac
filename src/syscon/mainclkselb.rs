#[doc = "Register `MAINCLKSELB` reader"]
pub type R = crate::R<MainclkselbSpec>;
#[doc = "Register `MAINCLKSELB` writer"]
pub type W = crate::W<MainclkselbSpec>;
#[doc = "Main clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main Clock A."]
    Enum0x0 = 0,
    #[doc = "1: PLL0 clock."]
    Enum0x1 = 1,
    #[doc = "2: PLL1 clock."]
    Enum0x2 = 2,
    #[doc = "3: Oscillator 32 kHz clock."]
    Enum0x3 = 3,
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
#[doc = "Field `SEL` reader - Main clock source select."]
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
            _ => None,
        }
    }
    #[doc = "Main Clock A."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Sel::Enum0x0
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == Sel::Enum0x2
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == Sel::Enum0x3
    }
}
#[doc = "Field `SEL` writer - Main clock source select."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Clock A."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x0)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x2)
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Main clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, MainclkselbSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "Main clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclkselb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclkselb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclkselbSpec;
impl crate::RegisterSpec for MainclkselbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclkselb::R`](R) reader structure"]
impl crate::Readable for MainclkselbSpec {}
#[doc = "`write(|w| ..)` method takes [`mainclkselb::W`](W) writer structure"]
impl crate::Writable for MainclkselbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAINCLKSELB to value 0"]
impl crate::Resettable for MainclkselbSpec {}

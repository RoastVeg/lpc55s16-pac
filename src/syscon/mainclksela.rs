#[doc = "Register `MAINCLKSELA` reader"]
pub type R = crate::R<MainclkselaSpec>;
#[doc = "Register `MAINCLKSELA` writer"]
pub type W = crate::W<MainclkselaSpec>;
#[doc = "Main clock A source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: FRO 12 MHz clock."]
    Enum0x0 = 0,
    #[doc = "1: CLKIN clock."]
    Enum0x1 = 1,
    #[doc = "2: FRO 1MHz clock."]
    Enum0x2 = 2,
    #[doc = "3: FRO 96 MHz clock."]
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
#[doc = "Field `SEL` reader - Main clock A source select."]
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
    #[doc = "FRO 12 MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Sel::Enum0x0
    }
    #[doc = "CLKIN clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == Sel::Enum0x2
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == Sel::Enum0x3
    }
}
#[doc = "Field `SEL` writer - Main clock A source select."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRO 12 MHz clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x0)
    }
    #[doc = "CLKIN clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x2)
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Main clock A source select."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main clock A source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, MainclkselaSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "Main clock A source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksela::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksela::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclkselaSpec;
impl crate::RegisterSpec for MainclkselaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclksela::R`](R) reader structure"]
impl crate::Readable for MainclkselaSpec {}
#[doc = "`write(|w| ..)` method takes [`mainclksela::W`](W) writer structure"]
impl crate::Writable for MainclkselaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAINCLKSELA to value 0"]
impl crate::Resettable for MainclkselaSpec {}

#[doc = "Register `OSTIMER` reader"]
pub type R = crate::R<OstimerSpec>;
#[doc = "Register `OSTIMER` writer"]
pub type W = crate::W<OstimerSpec>;
#[doc = "Field `SOFTRESET` reader - Active high reset."]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - Active high reset."]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCKENABLE` reader - Enable OS event timer clock."]
pub type ClockenableR = crate::BitReader;
#[doc = "Field `CLOCKENABLE` writer - Enable OS event timer clock."]
pub type ClockenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDWAKEUPENABLE` reader - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DpdwakeupenableR = crate::BitReader;
#[doc = "Field `DPDWAKEUPENABLE` writer - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DpdwakeupenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KPD` reader - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type Osc32kpdR = crate::BitReader;
#[doc = "Field `OSC32KPD` writer - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type Osc32kpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "OS event timer clock select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ostimerclksel {
    #[doc = "0: Oscillator 32 kHz clock."]
    Enum0x0 = 0,
    #[doc = "1: FRO 1MHz clock."]
    Enum0x1 = 1,
    #[doc = "2: Main clock for OS timer."]
    Enum0x2 = 2,
    #[doc = "3: No clock."]
    Enum0x3 = 3,
}
impl From<Ostimerclksel> for u8 {
    #[inline(always)]
    fn from(variant: Ostimerclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ostimerclksel {
    type Ux = u8;
}
impl crate::IsEnum for Ostimerclksel {}
#[doc = "Field `OSTIMERCLKSEL` reader - OS event timer clock select."]
pub type OstimerclkselR = crate::FieldReader<Ostimerclksel>;
impl OstimerclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostimerclksel {
        match self.bits {
            0 => Ostimerclksel::Enum0x0,
            1 => Ostimerclksel::Enum0x1,
            2 => Ostimerclksel::Enum0x2,
            3 => Ostimerclksel::Enum0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Ostimerclksel::Enum0x0
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Ostimerclksel::Enum0x1
    }
    #[doc = "Main clock for OS timer."]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == Ostimerclksel::Enum0x2
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == Ostimerclksel::Enum0x3
    }
}
#[doc = "Field `OSTIMERCLKSEL` writer - OS event timer clock select."]
pub type OstimerclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ostimerclksel, crate::Safe>;
impl<'a, REG> OstimerclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimerclksel::Enum0x0)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimerclksel::Enum0x1)
    }
    #[doc = "Main clock for OS timer."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimerclksel::Enum0x2)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimerclksel::Enum0x3)
    }
}
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OS event timer clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> ClockenableR {
        ClockenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DpdwakeupenableR {
        DpdwakeupenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> Osc32kpdR {
        Osc32kpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - OS event timer clock select."]
    #[inline(always)]
    pub fn ostimerclksel(&self) -> OstimerclkselR {
        OstimerclkselR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SoftresetW<'_, OstimerSpec> {
        SoftresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable OS event timer clock."]
    #[inline(always)]
    pub fn clockenable(&mut self) -> ClockenableW<'_, OstimerSpec> {
        ClockenableW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&mut self) -> DpdwakeupenableW<'_, OstimerSpec> {
        DpdwakeupenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&mut self) -> Osc32kpdW<'_, OstimerSpec> {
        Osc32kpdW::new(self, 3)
    }
    #[doc = "Bits 4:5 - OS event timer clock select."]
    #[inline(always)]
    pub fn ostimerclksel(&mut self) -> OstimerclkselW<'_, OstimerSpec> {
        OstimerclkselW::new(self, 4)
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ostimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OstimerSpec;
impl crate::RegisterSpec for OstimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ostimer::R`](R) reader structure"]
impl crate::Readable for OstimerSpec {}
#[doc = "`write(|w| ..)` method takes [`ostimer::W`](W) writer structure"]
impl crate::Writable for OstimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSTIMER to value 0x08"]
impl crate::Resettable for OstimerSpec {
    const RESET_VALUE: u32 = 0x08;
}

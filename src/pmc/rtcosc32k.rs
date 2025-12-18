#[doc = "Register `RTCOSC32K` reader"]
pub type R = crate::R<Rtcosc32kSpec>;
#[doc = "Register `RTCOSC32K` writer"]
pub type W = crate::W<Rtcosc32kSpec>;
#[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: FRO 32 KHz."]
    Fro32k = 0,
    #[doc = "1: XTAL 32KHz."]
    Xtal32k = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Fro32k,
            true => Sel::Xtal32k,
        }
    }
    #[doc = "FRO 32 KHz."]
    #[inline(always)]
    pub fn is_fro32k(&self) -> bool {
        *self == Sel::Fro32k
    }
    #[doc = "XTAL 32KHz."]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        *self == Sel::Xtal32k
    }
}
#[doc = "Field `SEL` writer - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRO 32 KHz."]
    #[inline(always)]
    pub fn fro32k(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Fro32k)
    }
    #[doc = "XTAL 32KHz."]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Xtal32k)
    }
}
#[doc = "Field `CLK1KHZDIV` reader - Actual division ratio is : 28 + CLK1KHZDIV."]
pub type Clk1khzdivR = crate::FieldReader;
#[doc = "Field `CLK1KHZDIV` writer - Actual division ratio is : 28 + CLK1KHZDIV."]
pub type Clk1khzdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLK1KHZDIVUPDATEREQ` reader - RTC 1KHz clock Divider status flag."]
pub type Clk1khzdivupdatereqR = crate::BitReader;
#[doc = "Field `CLK1KHZDIVUPDATEREQ` writer - RTC 1KHz clock Divider status flag."]
pub type Clk1khzdivupdatereqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK1HZDIV` reader - Actual division ratio is : 31744 + CLK1HZDIV."]
pub type Clk1hzdivR = crate::FieldReader<u16>;
#[doc = "Field `CLK1HZDIV` writer - Actual division ratio is : 31744 + CLK1HZDIV."]
pub type Clk1hzdivW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CLK1HZDIVHALT` reader - Halts the divider counter."]
pub type Clk1hzdivhaltR = crate::BitReader;
#[doc = "Field `CLK1HZDIVHALT` writer - Halts the divider counter."]
pub type Clk1hzdivhaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK1HZDIVUPDATEREQ` reader - RTC 1Hz Divider status flag."]
pub type Clk1hzdivupdatereqR = crate::BitReader;
#[doc = "Field `CLK1HZDIVUPDATEREQ` writer - RTC 1Hz Divider status flag."]
pub type Clk1hzdivupdatereqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&self) -> Clk1khzdivR {
        Clk1khzdivR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&self) -> Clk1khzdivupdatereqR {
        Clk1khzdivupdatereqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&self) -> Clk1hzdivR {
        Clk1hzdivR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&self) -> Clk1hzdivhaltR {
        Clk1hzdivhaltR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&self) -> Clk1hzdivupdatereqR {
        Clk1hzdivupdatereqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCOSC32K")
            .field("sel", &self.sel())
            .field("clk1khzdiv", &self.clk1khzdiv())
            .field("clk1khzdivupdatereq", &self.clk1khzdivupdatereq())
            .field("clk1hzdiv", &self.clk1hzdiv())
            .field("clk1hzdivhalt", &self.clk1hzdivhalt())
            .field("clk1hzdivupdatereq", &self.clk1hzdivupdatereq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, Rtcosc32kSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&mut self) -> Clk1khzdivW<'_, Rtcosc32kSpec> {
        Clk1khzdivW::new(self, 1)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&mut self) -> Clk1khzdivupdatereqW<'_, Rtcosc32kSpec> {
        Clk1khzdivupdatereqW::new(self, 15)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&mut self) -> Clk1hzdivW<'_, Rtcosc32kSpec> {
        Clk1hzdivW::new(self, 16)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&mut self) -> Clk1hzdivhaltW<'_, Rtcosc32kSpec> {
        Clk1hzdivhaltW::new(self, 30)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&mut self) -> Clk1hzdivupdatereqW<'_, Rtcosc32kSpec> {
        Clk1hzdivupdatereqW::new(self, 31)
    }
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcosc32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcosc32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcosc32kSpec;
impl crate::RegisterSpec for Rtcosc32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcosc32k::R`](R) reader structure"]
impl crate::Readable for Rtcosc32kSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcosc32k::W`](W) writer structure"]
impl crate::Writable for Rtcosc32kSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCOSC32K to value 0x03ff_0008"]
impl crate::Resettable for Rtcosc32kSpec {
    const RESET_VALUE: u32 = 0x03ff_0008;
}

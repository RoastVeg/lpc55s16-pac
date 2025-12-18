#[doc = "Register `XTAL32K` reader"]
pub type R = crate::R<Xtal32kSpec>;
#[doc = "Register `XTAL32K` writer"]
pub type W = crate::W<Xtal32kSpec>;
#[doc = "Field `IREF` reader - reference output current selection inputs."]
pub type IrefR = crate::FieldReader;
#[doc = "Field `IREF` writer - reference output current selection inputs."]
pub type IrefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TEST` reader - Oscillator Test Mode."]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - Oscillator Test Mode."]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS` reader - bias current selection inputs."]
pub type IbiasR = crate::FieldReader;
#[doc = "Field `IBIAS` writer - bias current selection inputs."]
pub type IbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPL` reader - oscillator amplitude selection inputs."]
pub type AmplR = crate::FieldReader;
#[doc = "Field `AMPL` writer - oscillator amplitude selection inputs."]
pub type AmplW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAPBANKIN` reader - Capa bank setting input."]
pub type CapbankinR = crate::FieldReader;
#[doc = "Field `CAPBANKIN` writer - Capa bank setting input."]
pub type CapbankinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CAPBANKOUT` reader - Capa bank setting output."]
pub type CapbankoutR = crate::FieldReader;
#[doc = "Field `CAPBANKOUT` writer - Capa bank setting output."]
pub type CapbankoutW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Source selection for xo32k_captest_start_ao_set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capteststartsrcsel {
    #[doc = "0: Sourced from CAPTESTSTART."]
    Capstart = 0,
    #[doc = "1: Sourced from calibration."]
    Calib = 1,
}
impl From<Capteststartsrcsel> for bool {
    #[inline(always)]
    fn from(variant: Capteststartsrcsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTESTSTARTSRCSEL` reader - Source selection for xo32k_captest_start_ao_set."]
pub type CapteststartsrcselR = crate::BitReader<Capteststartsrcsel>;
impl CapteststartsrcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capteststartsrcsel {
        match self.bits {
            false => Capteststartsrcsel::Capstart,
            true => Capteststartsrcsel::Calib,
        }
    }
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline(always)]
    pub fn is_capstart(&self) -> bool {
        *self == Capteststartsrcsel::Capstart
    }
    #[doc = "Sourced from calibration."]
    #[inline(always)]
    pub fn is_calib(&self) -> bool {
        *self == Capteststartsrcsel::Calib
    }
}
#[doc = "Field `CAPTESTSTARTSRCSEL` writer - Source selection for xo32k_captest_start_ao_set."]
pub type CapteststartsrcselW<'a, REG> = crate::BitWriter<'a, REG, Capteststartsrcsel>;
impl<'a, REG> CapteststartsrcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline(always)]
    pub fn capstart(self) -> &'a mut crate::W<REG> {
        self.variant(Capteststartsrcsel::Capstart)
    }
    #[doc = "Sourced from calibration."]
    #[inline(always)]
    pub fn calib(self) -> &'a mut crate::W<REG> {
        self.variant(Capteststartsrcsel::Calib)
    }
}
#[doc = "Field `CAPTESTSTART` reader - Start test."]
pub type CapteststartR = crate::BitReader;
#[doc = "Field `CAPTESTSTART` writer - Start test."]
pub type CapteststartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTESTENABLE` reader - Enable signal for cap test."]
pub type CaptestenableR = crate::BitReader;
#[doc = "Field `CAPTESTENABLE` writer - Enable signal for cap test."]
pub type CaptestenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select the input for test.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captestoscinsel {
    #[doc = "0: Oscillator output pin (osc_out)."]
    Oscout = 0,
    #[doc = "1: Oscillator input pin (osc_in)."]
    Oscin = 1,
}
impl From<Captestoscinsel> for bool {
    #[inline(always)]
    fn from(variant: Captestoscinsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTESTOSCINSEL` reader - Select the input for test."]
pub type CaptestoscinselR = crate::BitReader<Captestoscinsel>;
impl CaptestoscinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captestoscinsel {
        match self.bits {
            false => Captestoscinsel::Oscout,
            true => Captestoscinsel::Oscin,
        }
    }
    #[doc = "Oscillator output pin (osc_out)."]
    #[inline(always)]
    pub fn is_oscout(&self) -> bool {
        *self == Captestoscinsel::Oscout
    }
    #[doc = "Oscillator input pin (osc_in)."]
    #[inline(always)]
    pub fn is_oscin(&self) -> bool {
        *self == Captestoscinsel::Oscin
    }
}
#[doc = "Field `CAPTESTOSCINSEL` writer - Select the input for test."]
pub type CaptestoscinselW<'a, REG> = crate::BitWriter<'a, REG, Captestoscinsel>;
impl<'a, REG> CaptestoscinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator output pin (osc_out)."]
    #[inline(always)]
    pub fn oscout(self) -> &'a mut crate::W<REG> {
        self.variant(Captestoscinsel::Oscout)
    }
    #[doc = "Oscillator input pin (osc_in)."]
    #[inline(always)]
    pub fn oscin(self) -> &'a mut crate::W<REG> {
        self.variant(Captestoscinsel::Oscin)
    }
}
impl R {
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline(always)]
    pub fn iref(&self) -> IrefR {
        IrefR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline(always)]
    pub fn ibias(&self) -> IbiasR {
        IbiasR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline(always)]
    pub fn ampl(&self) -> AmplR {
        AmplR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&self) -> CapbankinR {
        CapbankinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&self) -> CapbankoutR {
        CapbankoutR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub fn capteststartsrcsel(&self) -> CapteststartsrcselR {
        CapteststartsrcselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Start test."]
    #[inline(always)]
    pub fn capteststart(&self) -> CapteststartR {
        CapteststartR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline(always)]
    pub fn captestenable(&self) -> CaptestenableR {
        CaptestenableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&self) -> CaptestoscinselR {
        CaptestoscinselR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K")
            .field("iref", &self.iref())
            .field("test", &self.test())
            .field("ibias", &self.ibias())
            .field("ampl", &self.ampl())
            .field("capbankin", &self.capbankin())
            .field("capbankout", &self.capbankout())
            .field("capteststartsrcsel", &self.capteststartsrcsel())
            .field("capteststart", &self.capteststart())
            .field("captestenable", &self.captestenable())
            .field("captestoscinsel", &self.captestoscinsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline(always)]
    pub fn iref(&mut self) -> IrefW<'_, Xtal32kSpec> {
        IrefW::new(self, 1)
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<'_, Xtal32kSpec> {
        TestW::new(self, 3)
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline(always)]
    pub fn ibias(&mut self) -> IbiasW<'_, Xtal32kSpec> {
        IbiasW::new(self, 4)
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline(always)]
    pub fn ampl(&mut self) -> AmplW<'_, Xtal32kSpec> {
        AmplW::new(self, 6)
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&mut self) -> CapbankinW<'_, Xtal32kSpec> {
        CapbankinW::new(self, 8)
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&mut self) -> CapbankoutW<'_, Xtal32kSpec> {
        CapbankoutW::new(self, 15)
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub fn capteststartsrcsel(&mut self) -> CapteststartsrcselW<'_, Xtal32kSpec> {
        CapteststartsrcselW::new(self, 22)
    }
    #[doc = "Bit 23 - Start test."]
    #[inline(always)]
    pub fn capteststart(&mut self) -> CapteststartW<'_, Xtal32kSpec> {
        CapteststartW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline(always)]
    pub fn captestenable(&mut self) -> CaptestenableW<'_, Xtal32kSpec> {
        CaptestenableW::new(self, 24)
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&mut self) -> CaptestoscinselW<'_, Xtal32kSpec> {
        CaptestoscinselW::new(self, 25)
    }
}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32kSpec;
impl crate::RegisterSpec for Xtal32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k::R`](R) reader structure"]
impl crate::Readable for Xtal32kSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32k::W`](W) writer structure"]
impl crate::Writable for Xtal32kSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32K to value 0x0020_4052"]
impl crate::Resettable for Xtal32kSpec {
    const RESET_VALUE: u32 = 0x0020_4052;
}

#[doc = "Register `DCDC1` reader"]
pub type R = crate::R<Dcdc1Spec>;
#[doc = "Register `DCDC1` writer"]
pub type W = crate::W<Dcdc1Spec>;
#[doc = "Field `RTRIMOFFET` reader - Adjust the offset voltage of BJT based comparator."]
pub type RtrimoffetR = crate::FieldReader;
#[doc = "Field `RTRIMOFFET` writer - Adjust the offset voltage of BJT based comparator."]
pub type RtrimoffetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSENSETRIM` reader - Adjust Max inductor peak current limiting."]
pub type RsensetrimR = crate::FieldReader;
#[doc = "Field `RSENSETRIM` writer - Adjust Max inductor peak current limiting."]
pub type RsensetrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTESTENABLE` reader - Enable Digital test signals."]
pub type DtestenableR = crate::BitReader;
#[doc = "Field `DTESTENABLE` writer - Enable Digital test signals."]
pub type DtestenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETCURVE` reader - Bandgap calibration parameter."]
pub type SetcurveR = crate::FieldReader;
#[doc = "Field `SETCURVE` writer - Bandgap calibration parameter."]
pub type SetcurveW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETDC` reader - Bandgap calibration parameter."]
pub type SetdcR = crate::FieldReader;
#[doc = "Field `SETDC` writer - Bandgap calibration parameter."]
pub type SetdcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTESTSEL` reader - Select the output signal for test."]
pub type DtestselR = crate::FieldReader;
#[doc = "Field `DTESTSEL` writer - Select the output signal for test."]
pub type DtestselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ISCALEENABLE` reader - Modify COT behavior."]
pub type IscaleenableR = crate::BitReader;
#[doc = "Field `ISCALEENABLE` writer - Modify COT behavior."]
pub type IscaleenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEBYPASS` reader - Force bypass mode."]
pub type ForcebypassR = crate::BitReader;
#[doc = "Field `FORCEBYPASS` writer - Force bypass mode."]
pub type ForcebypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMAUTOCOT` reader - Change the scaling ratio of the feedforward compensation."]
pub type TrimautocotR = crate::FieldReader;
#[doc = "Field `TRIMAUTOCOT` writer - Change the scaling ratio of the feedforward compensation."]
pub type TrimautocotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FORCEFULLCYCLE` reader - Force full PFM PMOS and NMOS cycle."]
pub type ForcefullcycleR = crate::BitReader;
#[doc = "Field `FORCEFULLCYCLE` writer - Force full PFM PMOS and NMOS cycle."]
pub type ForcefullcycleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCENABLE` reader - Change the range of the peak detector of current inside the inductor."]
pub type LcenableR = crate::BitReader;
#[doc = "Field `LCENABLE` writer - Change the range of the peak detector of current inside the inductor."]
pub type LcenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOFF` reader - Constant Off-Time calibration input."]
pub type ToffR = crate::FieldReader;
#[doc = "Field `TOFF` writer - Constant Off-Time calibration input."]
pub type ToffW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOFFENABLE` reader - Enable Constant Off-Time feature."]
pub type ToffenableR = crate::BitReader;
#[doc = "Field `TOFFENABLE` writer - Enable Constant Off-Time feature."]
pub type ToffenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub fn rtrimoffet(&self) -> RtrimoffetR {
        RtrimoffetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub fn rsensetrim(&self) -> RsensetrimR {
        RsensetrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable Digital test signals."]
    #[inline(always)]
    pub fn dtestenable(&self) -> DtestenableR {
        DtestenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setcurve(&self) -> SetcurveR {
        SetcurveR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setdc(&self) -> SetdcR {
        SetdcR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - Select the output signal for test."]
    #[inline(always)]
    pub fn dtestsel(&self) -> DtestselR {
        DtestselR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Modify COT behavior."]
    #[inline(always)]
    pub fn iscaleenable(&self) -> IscaleenableR {
        IscaleenableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force bypass mode."]
    #[inline(always)]
    pub fn forcebypass(&self) -> ForcebypassR {
        ForcebypassR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub fn trimautocot(&self) -> TrimautocotR {
        TrimautocotR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub fn forcefullcycle(&self) -> ForcefullcycleR {
        ForcefullcycleR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub fn lcenable(&self) -> LcenableR {
        LcenableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Constant Off-Time calibration input."]
    #[inline(always)]
    pub fn toff(&self) -> ToffR {
        ToffR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable Constant Off-Time feature."]
    #[inline(always)]
    pub fn toffenable(&self) -> ToffenableR {
        ToffenableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC1")
            .field("rtrimoffet", &self.rtrimoffet())
            .field("rsensetrim", &self.rsensetrim())
            .field("dtestenable", &self.dtestenable())
            .field("setcurve", &self.setcurve())
            .field("setdc", &self.setdc())
            .field("dtestsel", &self.dtestsel())
            .field("iscaleenable", &self.iscaleenable())
            .field("forcebypass", &self.forcebypass())
            .field("trimautocot", &self.trimautocot())
            .field("forcefullcycle", &self.forcefullcycle())
            .field("lcenable", &self.lcenable())
            .field("toff", &self.toff())
            .field("toffenable", &self.toffenable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub fn rtrimoffet(&mut self) -> RtrimoffetW<'_, Dcdc1Spec> {
        RtrimoffetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub fn rsensetrim(&mut self) -> RsensetrimW<'_, Dcdc1Spec> {
        RsensetrimW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable Digital test signals."]
    #[inline(always)]
    pub fn dtestenable(&mut self) -> DtestenableW<'_, Dcdc1Spec> {
        DtestenableW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setcurve(&mut self) -> SetcurveW<'_, Dcdc1Spec> {
        SetcurveW::new(self, 9)
    }
    #[doc = "Bits 11:14 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setdc(&mut self) -> SetdcW<'_, Dcdc1Spec> {
        SetdcW::new(self, 11)
    }
    #[doc = "Bits 15:17 - Select the output signal for test."]
    #[inline(always)]
    pub fn dtestsel(&mut self) -> DtestselW<'_, Dcdc1Spec> {
        DtestselW::new(self, 15)
    }
    #[doc = "Bit 18 - Modify COT behavior."]
    #[inline(always)]
    pub fn iscaleenable(&mut self) -> IscaleenableW<'_, Dcdc1Spec> {
        IscaleenableW::new(self, 18)
    }
    #[doc = "Bit 19 - Force bypass mode."]
    #[inline(always)]
    pub fn forcebypass(&mut self) -> ForcebypassW<'_, Dcdc1Spec> {
        ForcebypassW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub fn trimautocot(&mut self) -> TrimautocotW<'_, Dcdc1Spec> {
        TrimautocotW::new(self, 20)
    }
    #[doc = "Bit 24 - Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub fn forcefullcycle(&mut self) -> ForcefullcycleW<'_, Dcdc1Spec> {
        ForcefullcycleW::new(self, 24)
    }
    #[doc = "Bit 25 - Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub fn lcenable(&mut self) -> LcenableW<'_, Dcdc1Spec> {
        LcenableW::new(self, 25)
    }
    #[doc = "Bits 26:30 - Constant Off-Time calibration input."]
    #[inline(always)]
    pub fn toff(&mut self) -> ToffW<'_, Dcdc1Spec> {
        ToffW::new(self, 26)
    }
    #[doc = "Bit 31 - Enable Constant Off-Time feature."]
    #[inline(always)]
    pub fn toffenable(&mut self) -> ToffenableW<'_, Dcdc1Spec> {
        ToffenableW::new(self, 31)
    }
}
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc1Spec;
impl crate::RegisterSpec for Dcdc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc1::R`](R) reader structure"]
impl crate::Readable for Dcdc1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc1::W`](W) writer structure"]
impl crate::Writable for Dcdc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC1 to value 0x0180_3a98"]
impl crate::Resettable for Dcdc1Spec {
    const RESET_VALUE: u32 = 0x0180_3a98;
}

#[doc = "Register `DCDC_POWER_PROFILE_LOW_0` reader"]
pub type R = crate::R<DcdcPowerProfileLowDcdcPowerProfileLow0Spec>;
#[doc = "Register `DCDC_POWER_PROFILE_LOW_0` writer"]
pub type W = crate::W<DcdcPowerProfileLowDcdcPowerProfileLow0Spec>;
#[doc = "Field `DCDC_TRIM_VALID` reader - DCDC is trimed."]
pub type DcdcTrimValidR = crate::BitReader;
#[doc = "Field `DCDC_TRIM_VALID` writer - DCDC is trimed."]
pub type DcdcTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - Constant On-Time calibration."]
pub type RcR = crate::FieldReader;
#[doc = "Field `RC` writer - Constant On-Time calibration."]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ICOMP` reader - Select the type of ZCD comparator."]
pub type IcompR = crate::FieldReader;
#[doc = "Field `ICOMP` writer - Select the type of ZCD comparator."]
pub type IcompW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ISEL` reader - Alter Internal biasing currents."]
pub type IselR = crate::FieldReader;
#[doc = "Field `ISEL` writer - Alter Internal biasing currents."]
pub type IselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICENABLE` reader - Selection of auto scaling of COT period with variations in VDD."]
pub type IcenableR = crate::BitReader;
#[doc = "Field `ICENABLE` writer - Selection of auto scaling of COT period with variations in VDD."]
pub type IcenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOS` reader - One-shot generator reference current trimming signal."]
pub type TmosR = crate::FieldReader;
#[doc = "Field `TMOS` writer - One-shot generator reference current trimming signal."]
pub type TmosW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISABLEISENSE` reader - Disable Current sensing."]
pub type DisableisenseR = crate::BitReader;
#[doc = "Field `DISABLEISENSE` writer - Disable Current sensing."]
pub type DisableisenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOUT` reader - Set output regulation voltage."]
pub type VoutR = crate::FieldReader;
#[doc = "Field `VOUT` writer - Set output regulation voltage."]
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLICINGENABLE` reader - Enable staggered switching of power switches."]
pub type SlicingenableR = crate::BitReader;
#[doc = "Field `SLICINGENABLE` writer - Enable staggered switching of power switches."]
pub type SlicingenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDUCTORCLAMPENABLE` reader - Enable shorting of Inductor during PFM idle time."]
pub type InductorclampenableR = crate::BitReader;
#[doc = "Field `INDUCTORCLAMPENABLE` writer - Enable shorting of Inductor during PFM idle time."]
pub type InductorclampenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOUT_PWD` reader - Set output regulation voltage during Deep Sleep."]
pub type VoutPwdR = crate::FieldReader;
#[doc = "Field `VOUT_PWD` writer - Set output regulation voltage during Deep Sleep."]
pub type VoutPwdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DCDC is trimed."]
    #[inline(always)]
    pub fn dcdc_trim_valid(&self) -> DcdcTrimValidR {
        DcdcTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:8 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&self) -> IcompR {
        IcompR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&self) -> IcenableR {
        IcenableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&self) -> TmosR {
        TmosR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(&self) -> DisableisenseR {
        DisableisenseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(&self) -> SlicingenableR {
        SlicingenableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(&self) -> InductorclampenableR {
        InductorclampenableR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&self) -> VoutPwdR {
        VoutPwdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0")
            .field("dcdc_trim_valid", &self.dcdc_trim_valid())
            .field("rc", &self.rc())
            .field("icomp", &self.icomp())
            .field("isel", &self.isel())
            .field("icenable", &self.icenable())
            .field("tmos", &self.tmos())
            .field("disableisense", &self.disableisense())
            .field("vout", &self.vout())
            .field("slicingenable", &self.slicingenable())
            .field("inductorclampenable", &self.inductorclampenable())
            .field("vout_pwd", &self.vout_pwd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DCDC is trimed."]
    #[inline(always)]
    pub fn dcdc_trim_valid(
        &mut self,
    ) -> DcdcTrimValidW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        DcdcTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:6 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        RcW::new(self, 1)
    }
    #[doc = "Bits 7:8 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&mut self) -> IcompW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        IcompW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        IselW::new(self, 9)
    }
    #[doc = "Bit 11 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&mut self) -> IcenableW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        IcenableW::new(self, 11)
    }
    #[doc = "Bits 12:16 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&mut self) -> TmosW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        TmosW::new(self, 12)
    }
    #[doc = "Bit 17 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(
        &mut self,
    ) -> DisableisenseW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        DisableisenseW::new(self, 17)
    }
    #[doc = "Bits 18:21 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        VoutW::new(self, 18)
    }
    #[doc = "Bit 22 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(
        &mut self,
    ) -> SlicingenableW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        SlicingenableW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(
        &mut self,
    ) -> InductorclampenableW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        InductorclampenableW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&mut self) -> VoutPwdW<'_, DcdcPowerProfileLowDcdcPowerProfileLow0Spec> {
        VoutPwdW::new(self, 24)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_low_dcdc_power_profile_low_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_low_dcdc_power_profile_low_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcPowerProfileLowDcdcPowerProfileLow0Spec;
impl crate::RegisterSpec for DcdcPowerProfileLowDcdcPowerProfileLow0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_power_profile_low_dcdc_power_profile_low_0::R`](R) reader structure"]
impl crate::Readable for DcdcPowerProfileLowDcdcPowerProfileLow0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc_power_profile_low_dcdc_power_profile_low_0::W`](W) writer structure"]
impl crate::Writable for DcdcPowerProfileLowDcdcPowerProfileLow0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_POWER_PROFILE_LOW_0 to value 0"]
impl crate::Resettable for DcdcPowerProfileLowDcdcPowerProfileLow0Spec {}

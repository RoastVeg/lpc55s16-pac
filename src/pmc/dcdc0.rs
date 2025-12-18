#[doc = "Register `DCDC0` reader"]
pub type R = crate::R<Dcdc0Spec>;
#[doc = "Register `DCDC0` writer"]
pub type W = crate::W<Dcdc0Spec>;
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
#[doc = "Set output regulation voltage.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vout {
    #[doc = "0: 0.95 V."]
    VDcdc0p950 = 0,
    #[doc = "1: 0.975 V."]
    VDcdc0p975 = 1,
    #[doc = "2: 1 V."]
    VDcdc1p000 = 2,
    #[doc = "3: 1.025 V."]
    VDcdc1p025 = 3,
    #[doc = "4: 1.05 V."]
    VDcdc1p050 = 4,
    #[doc = "5: 1.075 V."]
    VDcdc1p075 = 5,
    #[doc = "6: 1.1 V."]
    VDcdc1p100 = 6,
    #[doc = "7: 1.125 V."]
    VDcdc1p125 = 7,
    #[doc = "8: 1.15 V."]
    VDcdc1p150 = 8,
    #[doc = "9: 1.175 V."]
    VDcdc1p175 = 9,
    #[doc = "10: 1.2 V."]
    VDcdc1p200 = 10,
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(variant: Vout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vout {
    type Ux = u8;
}
impl crate::IsEnum for Vout {}
#[doc = "Field `VOUT` reader - Set output regulation voltage."]
pub type VoutR = crate::FieldReader<Vout>;
impl VoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vout> {
        match self.bits {
            0 => Some(Vout::VDcdc0p950),
            1 => Some(Vout::VDcdc0p975),
            2 => Some(Vout::VDcdc1p000),
            3 => Some(Vout::VDcdc1p025),
            4 => Some(Vout::VDcdc1p050),
            5 => Some(Vout::VDcdc1p075),
            6 => Some(Vout::VDcdc1p100),
            7 => Some(Vout::VDcdc1p125),
            8 => Some(Vout::VDcdc1p150),
            9 => Some(Vout::VDcdc1p175),
            10 => Some(Vout::VDcdc1p200),
            _ => None,
        }
    }
    #[doc = "0.95 V."]
    #[inline(always)]
    pub fn is_v_dcdc_0p950(&self) -> bool {
        *self == Vout::VDcdc0p950
    }
    #[doc = "0.975 V."]
    #[inline(always)]
    pub fn is_v_dcdc_0p975(&self) -> bool {
        *self == Vout::VDcdc0p975
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p000(&self) -> bool {
        *self == Vout::VDcdc1p000
    }
    #[doc = "1.025 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p025(&self) -> bool {
        *self == Vout::VDcdc1p025
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p050(&self) -> bool {
        *self == Vout::VDcdc1p050
    }
    #[doc = "1.075 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p075(&self) -> bool {
        *self == Vout::VDcdc1p075
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p100(&self) -> bool {
        *self == Vout::VDcdc1p100
    }
    #[doc = "1.125 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p125(&self) -> bool {
        *self == Vout::VDcdc1p125
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p150(&self) -> bool {
        *self == Vout::VDcdc1p150
    }
    #[doc = "1.175 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p175(&self) -> bool {
        *self == Vout::VDcdc1p175
    }
    #[doc = "1.2 V."]
    #[inline(always)]
    pub fn is_v_dcdc_1p200(&self) -> bool {
        *self == Vout::VDcdc1p200
    }
}
#[doc = "Field `VOUT` writer - Set output regulation voltage."]
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vout>;
impl<'a, REG> VoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.95 V."]
    #[inline(always)]
    pub fn v_dcdc_0p950(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc0p950)
    }
    #[doc = "0.975 V."]
    #[inline(always)]
    pub fn v_dcdc_0p975(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc0p975)
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn v_dcdc_1p000(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p000)
    }
    #[doc = "1.025 V."]
    #[inline(always)]
    pub fn v_dcdc_1p025(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p025)
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn v_dcdc_1p050(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p050)
    }
    #[doc = "1.075 V."]
    #[inline(always)]
    pub fn v_dcdc_1p075(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p075)
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn v_dcdc_1p100(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p100)
    }
    #[doc = "1.125 V."]
    #[inline(always)]
    pub fn v_dcdc_1p125(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p125)
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn v_dcdc_1p150(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p150)
    }
    #[doc = "1.175 V."]
    #[inline(always)]
    pub fn v_dcdc_1p175(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p175)
    }
    #[doc = "1.2 V."]
    #[inline(always)]
    pub fn v_dcdc_1p200(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::VDcdc1p200)
    }
}
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
    #[doc = "Bits 0:5 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&self) -> IcompR {
        IcompR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&self) -> IcenableR {
        IcenableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&self) -> TmosR {
        TmosR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(&self) -> DisableisenseR {
        DisableisenseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(&self) -> SlicingenableR {
        SlicingenableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(&self) -> InductorclampenableR {
        InductorclampenableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&self) -> VoutPwdR {
        VoutPwdR::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<'_, Dcdc0Spec> {
        RcW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&mut self) -> IcompW<'_, Dcdc0Spec> {
        IcompW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<'_, Dcdc0Spec> {
        IselW::new(self, 8)
    }
    #[doc = "Bit 10 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&mut self) -> IcenableW<'_, Dcdc0Spec> {
        IcenableW::new(self, 10)
    }
    #[doc = "Bits 11:15 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&mut self) -> TmosW<'_, Dcdc0Spec> {
        TmosW::new(self, 11)
    }
    #[doc = "Bit 16 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(&mut self) -> DisableisenseW<'_, Dcdc0Spec> {
        DisableisenseW::new(self, 16)
    }
    #[doc = "Bits 17:20 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<'_, Dcdc0Spec> {
        VoutW::new(self, 17)
    }
    #[doc = "Bit 21 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(&mut self) -> SlicingenableW<'_, Dcdc0Spec> {
        SlicingenableW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(&mut self) -> InductorclampenableW<'_, Dcdc0Spec> {
        InductorclampenableW::new(self, 22)
    }
    #[doc = "Bits 23:26 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&mut self) -> VoutPwdW<'_, Dcdc0Spec> {
        VoutPwdW::new(self, 23)
    }
}
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc0Spec;
impl crate::RegisterSpec for Dcdc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc0::R`](R) reader structure"]
impl crate::Readable for Dcdc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc0::W`](W) writer structure"]
impl crate::Writable for Dcdc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC0 to value 0x010c_4e68"]
impl crate::Resettable for Dcdc0Spec {
    const RESET_VALUE: u32 = 0x010c_4e68;
}

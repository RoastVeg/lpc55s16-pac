#[doc = "Register `GCC[%s]` reader"]
pub type R = crate::R<GccSpec>;
#[doc = "Field `GAIN_CAL` reader - Gain Calibration Value"]
pub type GainCalR = crate::FieldReader<u16>;
#[doc = "Gain Calibration Value Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    Rdy0 = 0,
    #[doc = "1: The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\] register field."]
    Rdy1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Gain Calibration Value Valid"]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::Rdy0,
            true => Rdy::Rdy1,
        }
    }
    #[doc = "The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == Rdy::Rdy0
    }
    #[doc = "The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\] register field."]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == Rdy::Rdy1
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain_cal(&self) -> GainCalR {
        GainCalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calibration Value Valid"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Gain Calibration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gcc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GccSpec;
impl crate::RegisterSpec for GccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcc::R`](R) reader structure"]
impl crate::Readable for GccSpec {}
#[doc = "`reset()` method sets GCC[%s] to value 0"]
impl crate::Resettable for GccSpec {}

#[doc = "Register `GCR[%s]` reader"]
pub type R = crate::R<GcrSpec>;
#[doc = "Register `GCR[%s]` writer"]
pub type W = crate::W<GcrSpec>;
#[doc = "Field `GCALR` reader - Gain Calculation Result"]
pub type GcalrR = crate::FieldReader<u16>;
#[doc = "Field `GCALR` writer - Gain Calculation Result"]
pub type GcalrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Gain Calculation Ready\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: The gain offset calculation value is invalid."]
    Rdy0 = 0,
    #[doc = "1: The gain calibration value is valid."]
    Rdy1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Gain Calculation Ready"]
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
    #[doc = "The gain offset calculation value is invalid."]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == Rdy::Rdy0
    }
    #[doc = "The gain calibration value is valid."]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == Rdy::Rdy1
    }
}
#[doc = "Field `RDY` writer - Gain Calculation Ready"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG, Rdy>;
impl<'a, REG> RdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The gain offset calculation value is invalid."]
    #[inline(always)]
    pub fn rdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Rdy0)
    }
    #[doc = "The gain calibration value is valid."]
    #[inline(always)]
    pub fn rdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Rdy1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&self) -> GcalrR {
        GcalrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("gcalr", &self.gcalr())
            .field("rdy", &self.rdy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&mut self) -> GcalrW<'_, GcrSpec> {
        GcalrW::new(self, 0)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<'_, GcrSpec> {
        RdyW::new(self, 24)
    }
}
#[doc = "Gain Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCR[%s] to value 0"]
impl crate::Resettable for GcrSpec {}

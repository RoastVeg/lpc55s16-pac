#[doc = "Register `TEMP_SENS_SLOPE` reader"]
pub type R = crate::R<TempSensSlopeSpec>;
#[doc = "Register `TEMP_SENS_SLOPE` writer"]
pub type W = crate::W<TempSensSlopeSpec>;
#[doc = "Field `VALID` reader - no description available"]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - no description available"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOPE_x1024` reader - SLOPE_x1024\\[30:0\\]"]
pub type SlopeX1024R = crate::FieldReader<u32>;
#[doc = "Field `SLOPE_x1024` writer - SLOPE_x1024\\[30:0\\]"]
pub type SlopeX1024W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - SLOPE_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn slope_x1024(&self) -> SlopeX1024R {
        SlopeX1024R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<'_, TempSensSlopeSpec> {
        ValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - SLOPE_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn slope_x1024(&mut self) -> SlopeX1024W<'_, TempSensSlopeSpec> {
        SlopeX1024W::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_slope::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_slope::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSensSlopeSpec;
impl crate::RegisterSpec for TempSensSlopeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp_sens_slope::R`](R) reader structure"]
impl crate::Readable for TempSensSlopeSpec {}
#[doc = "`write(|w| ..)` method takes [`temp_sens_slope::W`](W) writer structure"]
impl crate::Writable for TempSensSlopeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMP_SENS_SLOPE to value 0"]
impl crate::Resettable for TempSensSlopeSpec {}

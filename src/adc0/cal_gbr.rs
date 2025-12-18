#[doc = "Register `CAL_GBR[%s]` reader"]
pub type R = crate::R<CalGbrSpec>;
#[doc = "Register `CAL_GBR[%s]` writer"]
pub type W = crate::W<CalGbrSpec>;
#[doc = "Field `CAL_GBR_VAL` reader - Calibration General B Side Register Element"]
pub type CalGbrValR = crate::FieldReader<u16>;
#[doc = "Field `CAL_GBR_VAL` writer - Calibration General B Side Register Element"]
pub type CalGbrValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&self) -> CalGbrValR {
        CalGbrValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&mut self) -> CalGbrValW<'_, CalGbrSpec> {
        CalGbrValW::new(self, 0)
    }
}
#[doc = "Calibration General B-Side Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_gbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_gbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalGbrSpec;
impl crate::RegisterSpec for CalGbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_gbr::R`](R) reader structure"]
impl crate::Readable for CalGbrSpec {}
#[doc = "`write(|w| ..)` method takes [`cal_gbr::W`](W) writer structure"]
impl crate::Writable for CalGbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL_GBR[%s] to value 0"]
impl crate::Resettable for CalGbrSpec {}

#[doc = "Register `TEMP_SENS_OFFSET` reader"]
pub type R = crate::R<TempSensOffsetSpec>;
#[doc = "Register `TEMP_SENS_OFFSET` writer"]
pub type W = crate::W<TempSensOffsetSpec>;
#[doc = "Field `VALID` reader - no description available"]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - no description available"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET_x1024` reader - OFFSET_x1024\\[30:0\\]"]
pub type OffsetX1024R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET_x1024` writer - OFFSET_x1024\\[30:0\\]"]
pub type OffsetX1024W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - OFFSET_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn offset_x1024(&self) -> OffsetX1024R {
        OffsetX1024R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<'_, TempSensOffsetSpec> {
        ValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - OFFSET_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn offset_x1024(&mut self) -> OffsetX1024W<'_, TempSensOffsetSpec> {
        OffsetX1024W::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSensOffsetSpec;
impl crate::RegisterSpec for TempSensOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp_sens_offset::R`](R) reader structure"]
impl crate::Readable for TempSensOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`temp_sens_offset::W`](W) writer structure"]
impl crate::Writable for TempSensOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMP_SENS_OFFSET to value 0"]
impl crate::Resettable for TempSensOffsetSpec {}

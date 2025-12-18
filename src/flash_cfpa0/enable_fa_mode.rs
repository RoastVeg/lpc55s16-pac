#[doc = "Register `ENABLE_FA_MODE` reader"]
pub type R = crate::R<EnableFaModeSpec>;
#[doc = "Register `ENABLE_FA_MODE` writer"]
pub type W = crate::W<EnableFaModeSpec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, EnableFaModeSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_fa_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_fa_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableFaModeSpec;
impl crate::RegisterSpec for EnableFaModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_fa_mode::R`](R) reader structure"]
impl crate::Readable for EnableFaModeSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_fa_mode::W`](W) writer structure"]
impl crate::Writable for EnableFaModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_FA_MODE to value 0"]
impl crate::Resettable for EnableFaModeSpec {}

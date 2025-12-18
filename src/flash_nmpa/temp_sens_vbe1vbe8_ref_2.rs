#[doc = "Register `TEMP_SENS_VBE1VBE8_REF_2` reader"]
pub type R = crate::R<TempSensVbe1vbe8Ref2Spec>;
#[doc = "Register `TEMP_SENS_VBE1VBE8_REF_2` writer"]
pub type W = crate::W<TempSensVbe1vbe8Ref2Spec>;
#[doc = "Field `VBE1` reader - no description available"]
pub type Vbe1R = crate::FieldReader<u16>;
#[doc = "Field `VBE1` writer - no description available"]
pub type Vbe1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VBE8` reader - no description available"]
pub type Vbe8R = crate::FieldReader<u16>;
#[doc = "Field `VBE8` writer - no description available"]
pub type Vbe8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn vbe1(&self) -> Vbe1R {
        Vbe1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn vbe8(&self) -> Vbe8R {
        Vbe8R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn vbe1(&mut self) -> Vbe1W<'_, TempSensVbe1vbe8Ref2Spec> {
        Vbe1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn vbe8(&mut self) -> Vbe8W<'_, TempSensVbe1vbe8Ref2Spec> {
        Vbe8W::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_vbe1vbe8_ref_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_vbe1vbe8_ref_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSensVbe1vbe8Ref2Spec;
impl crate::RegisterSpec for TempSensVbe1vbe8Ref2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp_sens_vbe1vbe8_ref_2::R`](R) reader structure"]
impl crate::Readable for TempSensVbe1vbe8Ref2Spec {}
#[doc = "`write(|w| ..)` method takes [`temp_sens_vbe1vbe8_ref_2::W`](W) writer structure"]
impl crate::Writable for TempSensVbe1vbe8Ref2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMP_SENS_VBE1VBE8_REF_2 to value 0"]
impl crate::Resettable for TempSensVbe1vbe8Ref2Spec {}

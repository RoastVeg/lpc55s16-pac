#[doc = "Register `GPO2_ARRAY2` reader"]
pub type R = crate::R<Gpo2Gpo2Array2Spec>;
#[doc = "Register `GPO2_ARRAY2` writer"]
pub type W = crate::W<Gpo2Gpo2Array2Spec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_GPO2_ARRAY2")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo2Gpo2Array2Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "GPO2 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_array2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_array2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo2Gpo2Array2Spec;
impl crate::RegisterSpec for Gpo2Gpo2Array2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo2_gpo2_array2::R`](R) reader structure"]
impl crate::Readable for Gpo2Gpo2Array2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo2_gpo2_array2::W`](W) writer structure"]
impl crate::Writable for Gpo2Gpo2Array2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO2_ARRAY2 to value 0"]
impl crate::Resettable for Gpo2Gpo2Array2Spec {}

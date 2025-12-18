#[doc = "Register `UUID_ARRAY1` reader"]
pub type R = crate::R<UuidUuidArray1Spec>;
#[doc = "Register `UUID_ARRAY1` writer"]
pub type W = crate::W<UuidUuidArray1Spec>;
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
    pub fn field(&mut self) -> FieldW<'_, UuidUuidArray1Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_array1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_array1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UuidUuidArray1Spec;
impl crate::RegisterSpec for UuidUuidArray1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uuid_uuid_array1::R`](R) reader structure"]
impl crate::Readable for UuidUuidArray1Spec {}
#[doc = "`write(|w| ..)` method takes [`uuid_uuid_array1::W`](W) writer structure"]
impl crate::Writable for UuidUuidArray1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUID_ARRAY1 to value 0"]
impl crate::Resettable for UuidUuidArray1Spec {}

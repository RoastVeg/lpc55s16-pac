#[doc = "Register `ROTKH[%s]` reader"]
pub type R = crate::R<RotkhSpec>;
#[doc = "Register `ROTKH[%s]` writer"]
pub type W = crate::W<RotkhSpec>;
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
        f.debug_struct("ROTKH")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, RotkhSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`rotkh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotkh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RotkhSpec;
impl crate::RegisterSpec for RotkhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rotkh::R`](R) reader structure"]
impl crate::Readable for RotkhSpec {}
#[doc = "`write(|w| ..)` method takes [`rotkh::W`](W) writer structure"]
impl crate::Writable for RotkhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROTKH[%s] to value 0"]
impl crate::Resettable for RotkhSpec {}

#[doc = "Register `GPO_CHECKSUM_ARRAY0` reader"]
pub type R = crate::R<GpoChecksumGpoChecksumArray0Spec>;
#[doc = "Register `GPO_CHECKSUM_ARRAY0` writer"]
pub type W = crate::W<GpoChecksumGpoChecksumArray0Spec>;
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
        f.debug_struct("GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, GpoChecksumGpoChecksumArray0Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_array0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_array0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoChecksumGpoChecksumArray0Spec;
impl crate::RegisterSpec for GpoChecksumGpoChecksumArray0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_checksum_gpo_checksum_array0::R`](R) reader structure"]
impl crate::Readable for GpoChecksumGpoChecksumArray0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_checksum_gpo_checksum_array0::W`](W) writer structure"]
impl crate::Writable for GpoChecksumGpoChecksumArray0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO_CHECKSUM_ARRAY0 to value 0"]
impl crate::Resettable for GpoChecksumGpoChecksumArray0Spec {}

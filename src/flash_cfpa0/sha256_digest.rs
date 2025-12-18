#[doc = "Register `SHA256_DIGEST[%s]` reader"]
pub type R = crate::R<Sha256DigestSpec>;
#[doc = "Register `SHA256_DIGEST[%s]` writer"]
pub type W = crate::W<Sha256DigestSpec>;
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
        f.debug_struct("SHA256_DIGEST")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Sha256DigestSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sha256DigestSpec;
impl crate::RegisterSpec for Sha256DigestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_digest::R`](R) reader structure"]
impl crate::Readable for Sha256DigestSpec {}
#[doc = "`write(|w| ..)` method takes [`sha256_digest::W`](W) writer structure"]
impl crate::Writable for Sha256DigestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_DIGEST[%s] to value 0"]
impl crate::Resettable for Sha256DigestSpec {}

#[doc = "Register `IMAGE_KEY_REVOKE` reader"]
pub type R = crate::R<ImageKeyRevokeSpec>;
#[doc = "Register `IMAGE_KEY_REVOKE` writer"]
pub type W = crate::W<ImageKeyRevokeSpec>;
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
        f.debug_struct("IMAGE_KEY_REVOKE")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, ImageKeyRevokeSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Image key revocation ID (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`image_key_revoke::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`image_key_revoke::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImageKeyRevokeSpec;
impl crate::RegisterSpec for ImageKeyRevokeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`image_key_revoke::R`](R) reader structure"]
impl crate::Readable for ImageKeyRevokeSpec {}
#[doc = "`write(|w| ..)` method takes [`image_key_revoke::W`](W) writer structure"]
impl crate::Writable for ImageKeyRevokeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMAGE_KEY_REVOKE to value 0"]
impl crate::Resettable for ImageKeyRevokeSpec {}

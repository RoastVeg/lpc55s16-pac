#[doc = "Register `RELOAD[%s]` writer"]
pub type W = crate::W<ReloadSpec>;
#[doc = "Field `DIGEST` writer - SHA Digest word to reload."]
pub type DigestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<ReloadSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA Digest word to reload."]
    #[inline(always)]
    pub fn digest(&mut self) -> DigestW<'_, ReloadSpec> {
        DigestW::new(self, 0)
    }
}
#[doc = "The WO digest-reload registers may be written with a saved Hash digest, to allow continuation from where left off. These registers may only be written if the Reload field in CTRL is 1. If SHA1, only the 1st 5 are used.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReloadSpec;
impl crate::RegisterSpec for ReloadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reload::W`](W) writer structure"]
impl crate::Writable for ReloadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RELOAD[%s] to value 0"]
impl crate::Resettable for ReloadSpec {}

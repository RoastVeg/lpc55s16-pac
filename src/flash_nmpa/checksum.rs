#[doc = "Register `CHECKSUM[%s]` reader"]
pub type R = crate::R<ChecksumSpec>;
#[doc = "Register `CHECKSUM[%s]` writer"]
pub type W = crate::W<ChecksumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Checksum of the whole page\n\nYou can [`read`](crate::Reg::read) this register and get [`checksum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`checksum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChecksumSpec;
impl crate::RegisterSpec for ChecksumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`checksum::R`](R) reader structure"]
impl crate::Readable for ChecksumSpec {}
#[doc = "`write(|w| ..)` method takes [`checksum::W`](W) writer structure"]
impl crate::Writable for ChecksumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHECKSUM[%s] to value 0"]
impl crate::Resettable for ChecksumSpec {}

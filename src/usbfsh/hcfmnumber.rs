#[doc = "Register `HCFMNUMBER` reader"]
pub type R = crate::R<HcfmnumberSpec>;
#[doc = "Register `HCFMNUMBER` writer"]
pub type W = crate::W<HcfmnumberSpec>;
#[doc = "Field `FN` reader - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
pub type FnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmnumber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfmnumber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmnumberSpec;
impl crate::RegisterSpec for HcfmnumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfmnumber::R`](R) reader structure"]
impl crate::Readable for HcfmnumberSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfmnumber::W`](W) writer structure"]
impl crate::Writable for HcfmnumberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFMNUMBER to value 0"]
impl crate::Resettable for HcfmnumberSpec {}

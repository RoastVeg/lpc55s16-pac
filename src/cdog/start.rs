#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `STRT` writer - Address of start command access"]
pub type StrtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Address of start command access"]
    #[inline(always)]
    pub fn strt(&mut self) -> StrtW<'_, StartSpec> {
        StrtW::new(self, 0)
    }
}
#[doc = "Write address for issuing the START command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}

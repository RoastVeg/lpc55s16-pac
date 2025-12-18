#[doc = "Register `SUB1` writer"]
pub type W = crate::W<Sub1Spec>;
#[doc = "Field `S1B` writer - Address of SUB1 command."]
pub type S1bW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Address of SUB1 command."]
    #[inline(always)]
    pub fn s1b(&mut self) -> S1bW<'_, Sub1Spec> {
        S1bW::new(self, 0)
    }
}
#[doc = "Write address for issuing the SUB1 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub1Spec;
impl crate::RegisterSpec for Sub1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sub1::W`](W) writer structure"]
impl crate::Writable for Sub1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUB1 to value 0"]
impl crate::Resettable for Sub1Spec {}

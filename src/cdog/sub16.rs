#[doc = "Register `SUB16` writer"]
pub type W = crate::W<Sub16Spec>;
#[doc = "Field `SB16` writer - Address of SUB16 command."]
pub type Sb16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Address of SUB16 command."]
    #[inline(always)]
    pub fn sb16(&mut self) -> Sb16W<'_, Sub16Spec> {
        Sb16W::new(self, 0)
    }
}
#[doc = "Write address for issuing the SUB16 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub16Spec;
impl crate::RegisterSpec for Sub16Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sub16::W`](W) writer structure"]
impl crate::Writable for Sub16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUB16 to value 0"]
impl crate::Resettable for Sub16Spec {}

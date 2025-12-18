#[doc = "Register `SUB256` writer"]
pub type W = crate::W<Sub256Spec>;
#[doc = "Field `SB256` writer - Address of (you guessed it) SUB256 command."]
pub type Sb256W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Sub256Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of (you guessed it) SUB256 command."]
    #[inline(always)]
    pub fn sb256(&mut self) -> Sb256W<'_, Sub256Spec> {
        Sb256W::new(self, 0)
    }
}
#[doc = "Write address for issuing the SUB256 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub256::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub256Spec;
impl crate::RegisterSpec for Sub256Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sub256::W`](W) writer structure"]
impl crate::Writable for Sub256Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUB256 to value 0"]
impl crate::Resettable for Sub256Spec {}

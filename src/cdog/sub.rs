#[doc = "Register `SUB` writer"]
pub type W = crate::W<SubSpec>;
#[doc = "Field `S0B` writer - Address of SUB command."]
pub type S0bW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SubSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of SUB command."]
    #[inline(always)]
    pub fn s0b(&mut self) -> S0bW<'_, SubSpec> {
        S0bW::new(self, 0)
    }
}
#[doc = "Write address for issuing the SUB command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubSpec;
impl crate::RegisterSpec for SubSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sub::W`](W) writer structure"]
impl crate::Writable for SubSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUB to value 0"]
impl crate::Resettable for SubSpec {}

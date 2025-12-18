#[doc = "Register `MASK[%s]` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `MASK` writer - A random word."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<MaskSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - A random word."]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, MaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Allows Application to write a random mask for ICB use. Normally only a new one on each system reset (including power up).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK[%s] to value 0"]
impl crate::Resettable for MaskSpec {}

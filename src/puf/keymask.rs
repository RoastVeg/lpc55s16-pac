#[doc = "Register `KEYMASK[%s]` writer"]
pub type W = crate::W<KeymaskSpec>;
#[doc = "Field `KEYMASK` writer - no description available"]
pub type KeymaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn keymask(&mut self) -> KeymaskW<'_, KeymaskSpec> {
        KeymaskW::new(self, 0)
    }
}
#[doc = "Only reset in case of full IC reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keymask::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeymaskSpec;
impl crate::RegisterSpec for KeymaskSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keymask::W`](W) writer structure"]
impl crate::Writable for KeymaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYMASK[%s] to value 0"]
impl crate::Resettable for KeymaskSpec {}

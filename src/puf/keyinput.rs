#[doc = "Register `KEYINPUT` writer"]
pub type W = crate::W<KeyinputSpec>;
#[doc = "Field `KEYIN` writer - Key input data"]
pub type KeyinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key input data"]
    #[inline(always)]
    pub fn keyin(&mut self) -> KeyinW<'_, KeyinputSpec> {
        KeyinW::new(self, 0)
    }
}
#[doc = "PUF Key Input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyinput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyinputSpec;
impl crate::RegisterSpec for KeyinputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyinput::W`](W) writer structure"]
impl crate::Writable for KeyinputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYINPUT to value 0"]
impl crate::Resettable for KeyinputSpec {}

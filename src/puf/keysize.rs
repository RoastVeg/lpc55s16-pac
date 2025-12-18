#[doc = "Register `KEYSIZE` reader"]
pub type R = crate::R<KeysizeSpec>;
#[doc = "Register `KEYSIZE` writer"]
pub type W = crate::W<KeysizeSpec>;
#[doc = "Field `KEYSIZE` reader - Key size for Set Key operations"]
pub type KeysizeR = crate::FieldReader;
#[doc = "Field `KEYSIZE` writer - Key size for Set Key operations"]
pub type KeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYSIZE")
            .field("keysize", &self.keysize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Key size for Set Key operations"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KeysizeW<'_, KeysizeSpec> {
        KeysizeW::new(self, 0)
    }
}
#[doc = "PUF Key Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`keysize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keysize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeysizeSpec;
impl crate::RegisterSpec for KeysizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keysize::R`](R) reader structure"]
impl crate::Readable for KeysizeSpec {}
#[doc = "`write(|w| ..)` method takes [`keysize::W`](W) writer structure"]
impl crate::Writable for KeysizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSIZE to value 0"]
impl crate::Resettable for KeysizeSpec {}

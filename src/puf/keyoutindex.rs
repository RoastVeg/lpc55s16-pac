#[doc = "Register `KEYOUTINDEX` reader"]
pub type R = crate::R<KeyoutindexSpec>;
#[doc = "Field `KEYOUTIDX` reader - Key index for the key that is currently output via the Key Output register"]
pub type KeyoutidxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Key index for the key that is currently output via the Key Output register"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KeyoutidxR {
        KeyoutidxR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "PUF Key Output Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutindex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyoutindexSpec;
impl crate::RegisterSpec for KeyoutindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyoutindex::R`](R) reader structure"]
impl crate::Readable for KeyoutindexSpec {}
#[doc = "`reset()` method sets KEYOUTINDEX to value 0"]
impl crate::Resettable for KeyoutindexSpec {}

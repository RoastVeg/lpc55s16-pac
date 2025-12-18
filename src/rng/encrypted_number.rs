#[doc = "Register `ENCRYPTED_NUMBER` reader"]
pub type R = crate::R<EncryptedNumberSpec>;
#[doc = "Register `ENCRYPTED_NUMBER` writer"]
pub type W = crate::W<EncryptedNumberSpec>;
#[doc = "Field `ENCRYPTED_NUMBER` reader - This register contains a random 32 bit number which is pre-computed."]
pub type EncryptedNumberR = crate::FieldReader<u32>;
#[doc = "Field `ENCRYPTED_NUMBER` writer - This register contains a random 32 bit number which is pre-computed."]
pub type EncryptedNumberW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub fn encrypted_number(&self) -> EncryptedNumberR {
        EncryptedNumberR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub fn encrypted_number(&mut self) -> EncryptedNumberW<'_, EncryptedNumberSpec> {
        EncryptedNumberW::new(self, 0)
    }
}
#[doc = "This register contains a random 32 bit number which is pre-computed\n\nYou can [`read`](crate::Reg::read) this register and get [`encrypted_number::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encrypted_number::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EncryptedNumberSpec;
impl crate::RegisterSpec for EncryptedNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`encrypted_number::R`](R) reader structure"]
impl crate::Readable for EncryptedNumberSpec {}
#[doc = "`write(|w| ..)` method takes [`encrypted_number::W`](W) writer structure"]
impl crate::Writable for EncryptedNumberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENCRYPTED_NUMBER to value 0"]
impl crate::Resettable for EncryptedNumberSpec {}

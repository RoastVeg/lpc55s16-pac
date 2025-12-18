#[doc = "Register `ACTIVATION_CODE[%s]` reader"]
pub type R = crate::R<ActivationCodeSpec>;
#[doc = "Register `ACTIVATION_CODE[%s]` writer"]
pub type W = crate::W<ActivationCodeSpec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVATION_CODE")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, ActivationCodeSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`activation_code::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`activation_code::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivationCodeSpec;
impl crate::RegisterSpec for ActivationCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`activation_code::R`](R) reader structure"]
impl crate::Readable for ActivationCodeSpec {}
#[doc = "`write(|w| ..)` method takes [`activation_code::W`](W) writer structure"]
impl crate::Writable for ActivationCodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACTIVATION_CODE[%s] to value 0"]
impl crate::Resettable for ActivationCodeSpec {}

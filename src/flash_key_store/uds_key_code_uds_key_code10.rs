#[doc = "Register `UDS_KEY_CODE10` reader"]
pub type R = crate::R<UdsKeyCodeUdsKeyCode10Spec>;
#[doc = "Register `UDS_KEY_CODE10` writer"]
pub type W = crate::W<UdsKeyCodeUdsKeyCode10Spec>;
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
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, UdsKeyCodeUdsKeyCode10Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdsKeyCodeUdsKeyCode10Spec;
impl crate::RegisterSpec for UdsKeyCodeUdsKeyCode10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uds_key_code_uds_key_code10::R`](R) reader structure"]
impl crate::Readable for UdsKeyCodeUdsKeyCode10Spec {}
#[doc = "`write(|w| ..)` method takes [`uds_key_code_uds_key_code10::W`](W) writer structure"]
impl crate::Writable for UdsKeyCodeUdsKeyCode10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDS_KEY_CODE10 to value 0"]
impl crate::Resettable for UdsKeyCodeUdsKeyCode10Spec {}

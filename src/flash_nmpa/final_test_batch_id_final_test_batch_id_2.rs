#[doc = "Register `FINAL_TEST_BATCH_ID_2` reader"]
pub type R = crate::R<FinalTestBatchIdFinalTestBatchId2Spec>;
#[doc = "Register `FINAL_TEST_BATCH_ID_2` writer"]
pub type W = crate::W<FinalTestBatchIdFinalTestBatchId2Spec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, FinalTestBatchIdFinalTestBatchId2Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinalTestBatchIdFinalTestBatchId2Spec;
impl crate::RegisterSpec for FinalTestBatchIdFinalTestBatchId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`final_test_batch_id_final_test_batch_id_2::R`](R) reader structure"]
impl crate::Readable for FinalTestBatchIdFinalTestBatchId2Spec {}
#[doc = "`write(|w| ..)` method takes [`final_test_batch_id_final_test_batch_id_2::W`](W) writer structure"]
impl crate::Writable for FinalTestBatchIdFinalTestBatchId2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FINAL_TEST_BATCH_ID_2 to value 0"]
impl crate::Resettable for FinalTestBatchIdFinalTestBatchId2Spec {}

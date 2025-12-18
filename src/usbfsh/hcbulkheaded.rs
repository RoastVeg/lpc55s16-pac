#[doc = "Register `HCBULKHEADED` reader"]
pub type R = crate::R<HcbulkheadedSpec>;
#[doc = "Register `HCBULKHEADED` writer"]
pub type W = crate::W<HcbulkheadedSpec>;
#[doc = "Field `BHED` reader - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
pub type BhedR = crate::FieldReader<u32>;
#[doc = "Field `BHED` writer - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
pub type BhedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&self) -> BhedR {
        BhedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&mut self) -> BhedW<'_, HcbulkheadedSpec> {
        BhedW::new(self, 4)
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkheaded::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkheaded::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbulkheadedSpec;
impl crate::RegisterSpec for HcbulkheadedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcbulkheaded::R`](R) reader structure"]
impl crate::Readable for HcbulkheadedSpec {}
#[doc = "`write(|w| ..)` method takes [`hcbulkheaded::W`](W) writer structure"]
impl crate::Writable for HcbulkheadedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCBULKHEADED to value 0"]
impl crate::Resettable for HcbulkheadedSpec {}

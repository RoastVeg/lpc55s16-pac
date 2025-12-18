#[doc = "Register `MASK_MSB` writer"]
pub type W = crate::W<MaskMsbSpec>;
#[doc = "Field `MASKVAL` writer - Value of the 32 Most Significant Bits of the 64-bit data mask."]
pub type MaskvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub fn maskval(&mut self) -> MaskvalW<'_, MaskMsbSpec> {
        MaskvalW::new(self, 0)
    }
}
#[doc = "Data Mask register, 32 Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_msb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskMsbSpec;
impl crate::RegisterSpec for MaskMsbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mask_msb::W`](W) writer structure"]
impl crate::Writable for MaskMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK_MSB to value 0"]
impl crate::Resettable for MaskMsbSpec {}

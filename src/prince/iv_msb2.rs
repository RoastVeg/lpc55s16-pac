#[doc = "Register `IV_MSB2` writer"]
pub type W = crate::W<IvMsb2Spec>;
#[doc = "Field `IVVAL` writer - Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
pub type IvvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub fn ivval(&mut self) -> IvvalW<'_, IvMsb2Spec> {
        IvvalW::new(self, 0)
    }
}
#[doc = "Initial Vector register for region 2, Most Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_msb2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvMsb2Spec;
impl crate::RegisterSpec for IvMsb2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iv_msb2::W`](W) writer structure"]
impl crate::Writable for IvMsb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IV_MSB2 to value 0"]
impl crate::Resettable for IvMsb2Spec {}

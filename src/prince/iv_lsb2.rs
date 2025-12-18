#[doc = "Register `IV_LSB2` writer"]
pub type W = crate::W<IvLsb2Spec>;
#[doc = "Field `IVVAL` writer - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
pub type IvvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<IvLsb2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub fn ivval(&mut self) -> IvvalW<'_, IvLsb2Spec> {
        IvvalW::new(self, 0)
    }
}
#[doc = "Initial Vector register for region 2, Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_lsb2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvLsb2Spec;
impl crate::RegisterSpec for IvLsb2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iv_lsb2::W`](W) writer structure"]
impl crate::Writable for IvLsb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IV_LSB2 to value 0"]
impl crate::Resettable for IvLsb2Spec {}

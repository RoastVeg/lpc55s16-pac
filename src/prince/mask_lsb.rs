#[doc = "Register `MASK_LSB` writer"]
pub type W = crate::W<MaskLsbSpec>;
#[doc = "Field `MASKVAL` writer - Value of the 32 Least Significant Bits of the 64-bit data mask."]
pub type MaskvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<MaskLsbSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub fn maskval(&mut self) -> MaskvalW<'_, MaskLsbSpec> {
        MaskvalW::new(self, 0)
    }
}
#[doc = "Data Mask register, 32 Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_lsb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskLsbSpec;
impl crate::RegisterSpec for MaskLsbSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mask_lsb::W`](W) writer structure"]
impl crate::Writable for MaskLsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK_LSB to value 0"]
impl crate::Resettable for MaskLsbSpec {}

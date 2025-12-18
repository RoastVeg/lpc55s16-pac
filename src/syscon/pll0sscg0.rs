#[doc = "Register `PLL0SSCG0` reader"]
pub type R = crate::R<Pll0sscg0Spec>;
#[doc = "Register `PLL0SSCG0` writer"]
pub type W = crate::W<Pll0sscg0Spec>;
#[doc = "Field `MD_LBS` reader - input word of the wrapper bit 31 to 0."]
pub type MdLbsR = crate::FieldReader<u32>;
#[doc = "Field `MD_LBS` writer - input word of the wrapper bit 31 to 0."]
pub type MdLbsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub fn md_lbs(&self) -> MdLbsR {
        MdLbsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub fn md_lbs(&mut self) -> MdLbsW<'_, Pll0sscg0Spec> {
        MdLbsW::new(self, 0)
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0sscg0Spec;
impl crate::RegisterSpec for Pll0sscg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0sscg0::R`](R) reader structure"]
impl crate::Readable for Pll0sscg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pll0sscg0::W`](W) writer structure"]
impl crate::Writable for Pll0sscg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0SSCG0 to value 0"]
impl crate::Resettable for Pll0sscg0Spec {}

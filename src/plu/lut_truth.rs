#[doc = "Register `LUT_TRUTH[%s]` reader"]
pub type R = crate::R<LutTruthSpec>;
#[doc = "Register `LUT_TRUTH[%s]` writer"]
pub type W = crate::W<LutTruthSpec>;
#[doc = "Field `LUTn_TRUTH` reader - Specifies the Truth Table contents for LUT0.."]
pub type LutnTruthR = crate::FieldReader<u32>;
#[doc = "Field `LUTn_TRUTH` writer - Specifies the Truth Table contents for LUT0.."]
pub type LutnTruthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    pub fn lutn_truth(&self) -> LutnTruthR {
        LutnTruthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    pub fn lutn_truth(&mut self) -> LutnTruthW<'_, LutTruthSpec> {
        LutnTruthW::new(self, 0)
    }
}
#[doc = "Specifies the Truth Table contents for LUTLUTn\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_truth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_truth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutTruthSpec;
impl crate::RegisterSpec for LutTruthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_truth::R`](R) reader structure"]
impl crate::Readable for LutTruthSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_truth::W`](W) writer structure"]
impl crate::Writable for LutTruthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_TRUTH[%s] to value 0"]
impl crate::Resettable for LutTruthSpec {}

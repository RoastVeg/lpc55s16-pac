#[doc = "Register `OUTPUTS` reader"]
pub type R = crate::R<OutputsSpec>;
#[doc = "Register `OUTPUTS` writer"]
pub type W = crate::W<OutputsSpec>;
#[doc = "Field `OUTPUT_STATE` reader - Provides the current state of the 8 designated PLU Outputs.."]
pub type OutputStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Provides the current state of the 8 designated PLU Outputs.."]
    #[inline(always)]
    pub fn output_state(&self) -> OutputStateR {
        OutputStateR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides the current state of the 8 designated PLU Outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`outputs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputsSpec;
impl crate::RegisterSpec for OutputsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputs::R`](R) reader structure"]
impl crate::Readable for OutputsSpec {}
#[doc = "`write(|w| ..)` method takes [`outputs::W`](W) writer structure"]
impl crate::Writable for OutputsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPUTS to value 0"]
impl crate::Resettable for OutputsSpec {}

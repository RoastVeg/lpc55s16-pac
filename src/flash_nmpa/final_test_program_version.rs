#[doc = "Register `FINAL_TEST_PROGRAM_VERSION` reader"]
pub type R = crate::R<FinalTestProgramVersionSpec>;
#[doc = "Register `FINAL_TEST_PROGRAM_VERSION` writer"]
pub type W = crate::W<FinalTestProgramVersionSpec>;
#[doc = "Field `PROGRAM_VERSION` reader - PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type ProgramVersionR = crate::FieldReader<u32>;
#[doc = "Field `PROGRAM_VERSION` writer - PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type ProgramVersionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn program_version(&self) -> ProgramVersionR {
        ProgramVersionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn program_version(&mut self) -> ProgramVersionW<'_, FinalTestProgramVersionSpec> {
        ProgramVersionW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_program_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_program_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinalTestProgramVersionSpec;
impl crate::RegisterSpec for FinalTestProgramVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`final_test_program_version::R`](R) reader structure"]
impl crate::Readable for FinalTestProgramVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`final_test_program_version::W`](W) writer structure"]
impl crate::Writable for FinalTestProgramVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FINAL_TEST_PROGRAM_VERSION to value 0"]
impl crate::Resettable for FinalTestProgramVersionSpec {}

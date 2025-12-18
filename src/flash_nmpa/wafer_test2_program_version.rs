#[doc = "Register `WAFER_TEST2_PROGRAM_VERSION` reader"]
pub type R = crate::R<WaferTest2ProgramVersionSpec>;
#[doc = "Register `WAFER_TEST2_PROGRAM_VERSION` writer"]
pub type W = crate::W<WaferTest2ProgramVersionSpec>;
#[doc = "Field `WT2_PROGRAM_VERSION` reader - WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type Wt2ProgramVersionR = crate::FieldReader<u32>;
#[doc = "Field `WT2_PROGRAM_VERSION` writer - WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type Wt2ProgramVersionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt2_program_version(&self) -> Wt2ProgramVersionR {
        Wt2ProgramVersionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt2_program_version(&mut self) -> Wt2ProgramVersionW<'_, WaferTest2ProgramVersionSpec> {
        Wt2ProgramVersionW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_program_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_program_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest2ProgramVersionSpec;
impl crate::RegisterSpec for WaferTest2ProgramVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test2_program_version::R`](R) reader structure"]
impl crate::Readable for WaferTest2ProgramVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test2_program_version::W`](W) writer structure"]
impl crate::Writable for WaferTest2ProgramVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST2_PROGRAM_VERSION to value 0"]
impl crate::Resettable for WaferTest2ProgramVersionSpec {}

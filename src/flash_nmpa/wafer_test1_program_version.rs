#[doc = "Register `WAFER_TEST1_PROGRAM_VERSION` reader"]
pub type R = crate::R<WaferTest1ProgramVersionSpec>;
#[doc = "Register `WAFER_TEST1_PROGRAM_VERSION` writer"]
pub type W = crate::W<WaferTest1ProgramVersionSpec>;
#[doc = "Field `WT1_PROGRAM_VERSION` reader - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type Wt1ProgramVersionR = crate::FieldReader<u32>;
#[doc = "Field `WT1_PROGRAM_VERSION` writer - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type Wt1ProgramVersionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt1_program_version(&self) -> Wt1ProgramVersionR {
        Wt1ProgramVersionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt1_program_version(&mut self) -> Wt1ProgramVersionW<'_, WaferTest1ProgramVersionSpec> {
        Wt1ProgramVersionW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_program_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_program_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferTest1ProgramVersionSpec;
impl crate::RegisterSpec for WaferTest1ProgramVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_test1_program_version::R`](R) reader structure"]
impl crate::Readable for WaferTest1ProgramVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`wafer_test1_program_version::W`](W) writer structure"]
impl crate::Writable for WaferTest1ProgramVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAFER_TEST1_PROGRAM_VERSION to value 0"]
impl crate::Resettable for WaferTest1ProgramVersionSpec {}

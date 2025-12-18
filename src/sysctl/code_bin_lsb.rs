#[doc = "Register `CODE_BIN_LSB` reader"]
pub type R = crate::R<CodeBinLsbSpec>;
#[doc = "Field `CODE_BIN_LSB` reader - Binary converted code (42bits)"]
pub type CodeBinLsbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Binary converted code (42bits)"]
    #[inline(always)]
    pub fn code_bin_lsb(&self) -> CodeBinLsbR {
        CodeBinLsbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODE_BIN_LSB")
            .field("code_bin_lsb", &self.code_bin_lsb())
            .finish()
    }
}
#[doc = "CODE_BIN LSB output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_bin_lsb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeBinLsbSpec;
impl crate::RegisterSpec for CodeBinLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`code_bin_lsb::R`](R) reader structure"]
impl crate::Readable for CodeBinLsbSpec {}
#[doc = "`reset()` method sets CODE_BIN_LSB to value 0"]
impl crate::Resettable for CodeBinLsbSpec {}

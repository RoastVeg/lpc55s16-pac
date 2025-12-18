#[doc = "Register `CODE_BIN_MSB` reader"]
pub type R = crate::R<CodeBinMsbSpec>;
#[doc = "Field `CODE_BIN_MSB` reader - Binary converted code (42bits)"]
pub type CodeBinMsbR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Binary converted code (42bits)"]
    #[inline(always)]
    pub fn code_bin_msb(&self) -> CodeBinMsbR {
        CodeBinMsbR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODE_BIN_MSB")
            .field("code_bin_msb", &self.code_bin_msb())
            .finish()
    }
}
#[doc = "CODE_BIN MSB output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_bin_msb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeBinMsbSpec;
impl crate::RegisterSpec for CodeBinMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`code_bin_msb::R`](R) reader structure"]
impl crate::Readable for CodeBinMsbSpec {}
#[doc = "`reset()` method sets CODE_BIN_MSB to value 0"]
impl crate::Resettable for CodeBinMsbSpec {}

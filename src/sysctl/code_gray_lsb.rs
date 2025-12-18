#[doc = "Register `CODE_GRAY_LSB` reader"]
pub type R = crate::R<CodeGrayLsbSpec>;
#[doc = "Register `CODE_GRAY_LSB` writer"]
pub type W = crate::W<CodeGrayLsbSpec>;
#[doc = "Field `CODE_GRAY_LSB` reader - Gray code (42bits) to be converted back to binary"]
pub type CodeGrayLsbR = crate::FieldReader<u32>;
#[doc = "Field `CODE_GRAY_LSB` writer - Gray code (42bits) to be converted back to binary"]
pub type CodeGrayLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_lsb(&self) -> CodeGrayLsbR {
        CodeGrayLsbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODE_GRAY_LSB")
            .field("code_gray_lsb", &self.code_gray_lsb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_lsb(&mut self) -> CodeGrayLsbW<'_, CodeGrayLsbSpec> {
        CodeGrayLsbW::new(self, 0)
    }
}
#[doc = "CODE_GRAY LSB input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_gray_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`code_gray_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeGrayLsbSpec;
impl crate::RegisterSpec for CodeGrayLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`code_gray_lsb::R`](R) reader structure"]
impl crate::Readable for CodeGrayLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`code_gray_lsb::W`](W) writer structure"]
impl crate::Writable for CodeGrayLsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODE_GRAY_LSB to value 0"]
impl crate::Resettable for CodeGrayLsbSpec {}

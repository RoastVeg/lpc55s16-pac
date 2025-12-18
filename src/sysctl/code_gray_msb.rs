#[doc = "Register `CODE_GRAY_MSB` reader"]
pub type R = crate::R<CodeGrayMsbSpec>;
#[doc = "Register `CODE_GRAY_MSB` writer"]
pub type W = crate::W<CodeGrayMsbSpec>;
#[doc = "Field `CODE_GRAY_MSB` reader - Gray code (42bits) to be converted back to binary"]
pub type CodeGrayMsbR = crate::FieldReader<u16>;
#[doc = "Field `CODE_GRAY_MSB` writer - Gray code (42bits) to be converted back to binary"]
pub type CodeGrayMsbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_msb(&self) -> CodeGrayMsbR {
        CodeGrayMsbR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_msb(&mut self) -> CodeGrayMsbW<'_, CodeGrayMsbSpec> {
        CodeGrayMsbW::new(self, 0)
    }
}
#[doc = "CODE_GRAY MSB input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`code_gray_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`code_gray_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeGrayMsbSpec;
impl crate::RegisterSpec for CodeGrayMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`code_gray_msb::R`](R) reader structure"]
impl crate::Readable for CodeGrayMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`code_gray_msb::W`](W) writer structure"]
impl crate::Writable for CodeGrayMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODE_GRAY_MSB to value 0"]
impl crate::Resettable for CodeGrayMsbSpec {}

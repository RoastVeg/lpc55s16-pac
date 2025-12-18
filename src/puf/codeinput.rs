#[doc = "Register `CODEINPUT` writer"]
pub type W = crate::W<CodeinputSpec>;
#[doc = "Field `CODEIN` writer - AC/KC input data"]
pub type CodeinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AC/KC input data"]
    #[inline(always)]
    pub fn codein(&mut self) -> CodeinW<'_, CodeinputSpec> {
        CodeinW::new(self, 0)
    }
}
#[doc = "PUF Code Input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codeinput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeinputSpec;
impl crate::RegisterSpec for CodeinputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`codeinput::W`](W) writer structure"]
impl crate::Writable for CodeinputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEINPUT to value 0"]
impl crate::Resettable for CodeinputSpec {}

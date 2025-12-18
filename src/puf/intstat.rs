#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `READY` reader - Triggers on falling edge of busy, write 1 to clear"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - Triggers on falling edge of busy, write 1 to clear"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUCCESS` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type SuccessR = crate::BitReader;
#[doc = "Field `SUCCESS` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type SuccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYINREQ` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type KeyinreqR = crate::BitReader;
#[doc = "Field `KEYINREQ` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type KeyinreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYOUTAVAIL` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type KeyoutavailR = crate::BitReader;
#[doc = "Field `KEYOUTAVAIL` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type KeyoutavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEINREQ` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type CodeinreqR = crate::BitReader;
#[doc = "Field `CODEINREQ` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type CodeinreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEOUTAVAIL` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub type CodeoutavailR = crate::BitReader;
#[doc = "Field `CODEOUTAVAIL` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub type CodeoutavailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&self) -> SuccessR {
        SuccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KeyinreqR {
        KeyinreqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KeyoutavailR {
        KeyoutavailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CodeinreqR {
        CodeinreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CodeoutavailR {
        CodeoutavailR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntstatSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&mut self) -> SuccessW<'_, IntstatSpec> {
        SuccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntstatSpec> {
        ErrorW::new(self, 2)
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&mut self) -> KeyinreqW<'_, IntstatSpec> {
        KeyinreqW::new(self, 4)
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&mut self) -> KeyoutavailW<'_, IntstatSpec> {
        KeyoutavailW::new(self, 5)
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&mut self) -> CodeinreqW<'_, IntstatSpec> {
        CodeinreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&mut self) -> CodeoutavailW<'_, IntstatSpec> {
        CodeoutavailW::new(self, 7)
    }
}
#[doc = "PUF interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}

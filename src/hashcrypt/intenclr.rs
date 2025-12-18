#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `WAITING` reader - Write 1 to clear mask."]
pub type WaitingR = crate::BitReader;
#[doc = "Field `WAITING` writer - Write 1 to clear mask."]
pub type WaitingW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DIGEST` reader - Write 1 to clear mask."]
pub type DigestR = crate::BitReader;
#[doc = "Field `DIGEST` writer - Write 1 to clear mask."]
pub type DigestW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Write 1 to clear mask."]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Write 1 to clear mask."]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT` reader - Write 1 to clear mask."]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - Write 1 to clear mask."]
pub type FaultW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn waiting(&self) -> WaitingR {
        WaitingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn digest(&self) -> DigestR {
        DigestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .field("fault", &self.fault())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn waiting(&mut self) -> WaitingW<'_, IntenclrSpec> {
        WaitingW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn digest(&mut self) -> DigestW<'_, IntenclrSpec> {
        DigestW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn fault(&mut self) -> FaultW<'_, IntenclrSpec> {
        FaultW::new(self, 3)
    }
}
#[doc = "Write 1 to clear interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

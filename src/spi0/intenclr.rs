#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SSAEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTIDLE` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type MstidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn ssaen(&mut self) -> SsaenW<'_, IntenclrSpec> {
        SsaenW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn ssden(&mut self) -> SsdenW<'_, IntenclrSpec> {
        SsdenW::new(self, 5)
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn mstidle(&mut self) -> MstidleW<'_, IntenclrSpec> {
        MstidleW::new(self, 8)
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

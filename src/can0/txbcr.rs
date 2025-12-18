#[doc = "Register `TXBCR` reader"]
pub type R = crate::R<TxbcrSpec>;
#[doc = "Register `TXBCR` writer"]
pub type W = crate::W<TxbcrSpec>;
#[doc = "Field `CR` reader - Cancellation request."]
pub type CrR = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - Cancellation request."]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation request."]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCR").field("cr", &self.cr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation request."]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, TxbcrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "Tx Buffer Cancellation Request\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcrSpec;
impl crate::RegisterSpec for TxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcr::R`](R) reader structure"]
impl crate::Readable for TxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcr::W`](W) writer structure"]
impl crate::Writable for TxbcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TxbcrSpec {}

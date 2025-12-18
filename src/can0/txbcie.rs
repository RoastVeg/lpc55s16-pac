#[doc = "Register `TXBCIE` reader"]
pub type R = crate::R<TxbcieSpec>;
#[doc = "Register `TXBCIE` writer"]
pub type W = crate::W<TxbcieSpec>;
#[doc = "Field `CFIE` reader - Cancellation finished interrupt enable."]
pub type CfieR = crate::FieldReader<u32>;
#[doc = "Field `CFIE` writer - Cancellation finished interrupt enable."]
pub type CfieW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn cfie(&self) -> CfieR {
        CfieR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCIE")
            .field("cfie", &self.cfie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn cfie(&mut self) -> CfieW<'_, TxbcieSpec> {
        CfieW::new(self, 0)
    }
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcieSpec;
impl crate::RegisterSpec for TxbcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcie::R`](R) reader structure"]
impl crate::Readable for TxbcieSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcie::W`](W) writer structure"]
impl crate::Writable for TxbcieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TxbcieSpec {}

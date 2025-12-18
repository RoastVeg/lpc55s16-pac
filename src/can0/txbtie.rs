#[doc = "Register `TXBTIE` reader"]
pub type R = crate::R<TxbtieSpec>;
#[doc = "Register `TXBTIE` writer"]
pub type W = crate::W<TxbtieSpec>;
#[doc = "Field `TIE` reader - Transmission interrupt enable."]
pub type TieR = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - Transmission interrupt enable."]
pub type TieW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission interrupt enable."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTIE").field("tie", &self.tie()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission interrupt enable."]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, TxbtieSpec> {
        TieW::new(self, 0)
    }
}
#[doc = "Tx Buffer Transmission Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtieSpec;
impl crate::RegisterSpec for TxbtieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbtie::R`](R) reader structure"]
impl crate::Readable for TxbtieSpec {}
#[doc = "`write(|w| ..)` method takes [`txbtie::W`](W) writer structure"]
impl crate::Writable for TxbtieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TxbtieSpec {}

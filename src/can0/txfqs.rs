#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TxfqsSpec>;
#[doc = "Register `TXFQS` writer"]
pub type W = crate::W<TxfqsSpec>;
#[doc = "Field `TFGI` reader - Tx FIFO get index."]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFGI` writer - Tx FIFO get index."]
pub type TfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index."]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQPI` writer - Tx FIFO/queue put index."]
pub type TfqpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFQF` reader - Tx FIFO/queue full."]
pub type TfqfR = crate::BitReader;
#[doc = "Field `TFQF` writer - Tx FIFO/queue full."]
pub type TfqfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFQS")
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&mut self) -> TfgiW<'_, TxfqsSpec> {
        TfgiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&mut self) -> TfqpiW<'_, TxfqsSpec> {
        TfqpiW::new(self, 16)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&mut self) -> TfqfW<'_, TxfqsSpec> {
        TfqfW::new(self, 21)
    }
}
#[doc = "Tx FIFO/Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfqs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfqsSpec;
impl crate::RegisterSpec for TxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TxfqsSpec {}
#[doc = "`write(|w| ..)` method takes [`txfqs::W`](W) writer structure"]
impl crate::Writable for TxfqsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TxfqsSpec {}

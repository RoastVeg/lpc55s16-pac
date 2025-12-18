#[doc = "Register `ETSCC` reader"]
pub type R = crate::R<EtsccSpec>;
#[doc = "Register `ETSCC` writer"]
pub type W = crate::W<EtsccSpec>;
#[doc = "Field `ETCP` reader - External timestamp prescaler value."]
pub type EtcpR = crate::FieldReader<u16>;
#[doc = "Field `ETCP` writer - External timestamp prescaler value."]
pub type EtcpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ETCE` reader - External timestamp counter enable."]
pub type EtceR = crate::BitReader;
#[doc = "Field `ETCE` writer - External timestamp counter enable."]
pub type EtceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&self) -> EtcpR {
        EtcpR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&self) -> EtceR {
        EtceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETSCC")
            .field("etcp", &self.etcp())
            .field("etce", &self.etce())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&mut self) -> EtcpW<'_, EtsccSpec> {
        EtcpW::new(self, 0)
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&mut self) -> EtceW<'_, EtsccSpec> {
        EtceW::new(self, 31)
    }
}
#[doc = "External Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`etscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtsccSpec;
impl crate::RegisterSpec for EtsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etscc::R`](R) reader structure"]
impl crate::Readable for EtsccSpec {}
#[doc = "`write(|w| ..)` method takes [`etscc::W`](W) writer structure"]
impl crate::Writable for EtsccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETSCC to value 0"]
impl crate::Resettable for EtsccSpec {}

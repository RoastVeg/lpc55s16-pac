#[doc = "Register `EPSKIP` reader"]
pub type R = crate::R<EpskipSpec>;
#[doc = "Register `EPSKIP` writer"]
pub type W = crate::W<EpskipSpec>;
#[doc = "Field `SKIP` reader - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub type SkipR = crate::FieldReader<u16>;
#[doc = "Field `SKIP` writer - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub type SkipW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    pub fn skip(&self) -> SkipR {
        SkipR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPSKIP")
            .field("skip", &self.skip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    pub fn skip(&mut self) -> SkipW<'_, EpskipSpec> {
        SkipW::new(self, 0)
    }
}
#[doc = "USB Endpoint skip\n\nYou can [`read`](crate::Reg::read) this register and get [`epskip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epskip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpskipSpec;
impl crate::RegisterSpec for EpskipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epskip::R`](R) reader structure"]
impl crate::Readable for EpskipSpec {}
#[doc = "`write(|w| ..)` method takes [`epskip::W`](W) writer structure"]
impl crate::Writable for EpskipSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPSKIP to value 0"]
impl crate::Resettable for EpskipSpec {}

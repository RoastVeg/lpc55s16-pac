#[doc = "Register `HCPERIODCURRENTED` reader"]
pub type R = crate::R<HcperiodcurrentedSpec>;
#[doc = "Register `HCPERIODCURRENTED` writer"]
pub type W = crate::W<HcperiodcurrentedSpec>;
#[doc = "Field `PCED` reader - The content of this register is updated by HC after a periodic ED is processed."]
pub type PcedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&self) -> PcedR {
        PcedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCPERIODCURRENTED")
            .field("pced", &self.pced())
            .finish()
    }
}
impl W {}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodcurrented::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcperiodcurrented::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcperiodcurrentedSpec;
impl crate::RegisterSpec for HcperiodcurrentedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcperiodcurrented::R`](R) reader structure"]
impl crate::Readable for HcperiodcurrentedSpec {}
#[doc = "`write(|w| ..)` method takes [`hcperiodcurrented::W`](W) writer structure"]
impl crate::Writable for HcperiodcurrentedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPERIODCURRENTED to value 0"]
impl crate::Resettable for HcperiodcurrentedSpec {}

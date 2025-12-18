#[doc = "Register `HCFMREMAINING` reader"]
pub type R = crate::R<HcfmremainingSpec>;
#[doc = "Register `HCFMREMAINING` writer"]
pub type W = crate::W<HcfmremainingSpec>;
#[doc = "Field `FR` reader - FrameRemaining This counter is decremented at each bit time."]
pub type FrR = crate::FieldReader<u16>;
#[doc = "Field `FRT` reader - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
pub type FrtR = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&self) -> FrtR {
        FrtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFMREMAINING")
            .field("fr", &self.fr())
            .field("frt", &self.frt())
            .finish()
    }
}
impl W {}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmremaining::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfmremaining::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmremainingSpec;
impl crate::RegisterSpec for HcfmremainingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfmremaining::R`](R) reader structure"]
impl crate::Readable for HcfmremainingSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfmremaining::W`](W) writer structure"]
impl crate::Writable for HcfmremainingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFMREMAINING to value 0"]
impl crate::Resettable for HcfmremainingSpec {}

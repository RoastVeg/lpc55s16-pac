#[doc = "Register `HCDONEHEAD` reader"]
pub type R = crate::R<HcdoneheadSpec>;
#[doc = "Register `HCDONEHEAD` writer"]
pub type W = crate::W<HcdoneheadSpec>;
#[doc = "Field `DH` reader - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
pub type DhR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub fn dh(&self) -> DhR {
        DhR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDONEHEAD")
            .field("dh", &self.dh())
            .finish()
    }
}
impl W {}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdonehead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdonehead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdoneheadSpec;
impl crate::RegisterSpec for HcdoneheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdonehead::R`](R) reader structure"]
impl crate::Readable for HcdoneheadSpec {}
#[doc = "`write(|w| ..)` method takes [`hcdonehead::W`](W) writer structure"]
impl crate::Writable for HcdoneheadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDONEHEAD to value 0"]
impl crate::Resettable for HcdoneheadSpec {}

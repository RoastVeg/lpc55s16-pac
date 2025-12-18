#[doc = "Register `HCLSTHRESHOLD` reader"]
pub type R = crate::R<HclsthresholdSpec>;
#[doc = "Register `HCLSTHRESHOLD` writer"]
pub type W = crate::W<HclsthresholdSpec>;
#[doc = "Field `LST` reader - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub type LstR = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub type LstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCLSTHRESHOLD")
            .field("lst", &self.lst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&mut self) -> LstW<'_, HclsthresholdSpec> {
        LstW::new(self, 0)
    }
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF\n\nYou can [`read`](crate::Reg::read) this register and get [`hclsthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclsthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HclsthresholdSpec;
impl crate::RegisterSpec for HclsthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hclsthreshold::R`](R) reader structure"]
impl crate::Readable for HclsthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`hclsthreshold::W`](W) writer structure"]
impl crate::Writable for HclsthresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCLSTHRESHOLD to value 0x0628"]
impl crate::Resettable for HclsthresholdSpec {
    const RESET_VALUE: u32 = 0x0628;
}

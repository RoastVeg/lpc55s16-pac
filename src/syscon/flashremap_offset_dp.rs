#[doc = "Register `FLASHREMAP_OFFSET_DP` reader"]
pub type R = crate::R<FlashremapOffsetDpSpec>;
#[doc = "Register `FLASHREMAP_OFFSET_DP` writer"]
pub type W = crate::W<FlashremapOffsetDpSpec>;
#[doc = "Field `FLASHREMAP_OFFSET` reader - no description available"]
pub type FlashremapOffsetR = crate::FieldReader<u32>;
#[doc = "Field `FLASHREMAP_OFFSET` writer - no description available"]
pub type FlashremapOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_offset(&self) -> FlashremapOffsetR {
        FlashremapOffsetR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_OFFSET_DP")
            .field("flashremap_offset", &self.flashremap_offset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_offset(&mut self) -> FlashremapOffsetW<'_, FlashremapOffsetDpSpec> {
        FlashremapOffsetW::new(self, 0)
    }
}
#[doc = "This 32-bit register is a duplicate of FLASHREMAPOFFSET for increased security.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashremap_offset_dp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashremap_offset_dp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashremapOffsetDpSpec;
impl crate::RegisterSpec for FlashremapOffsetDpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashremap_offset_dp::R`](R) reader structure"]
impl crate::Readable for FlashremapOffsetDpSpec {}
#[doc = "`write(|w| ..)` method takes [`flashremap_offset_dp::W`](W) writer structure"]
impl crate::Writable for FlashremapOffsetDpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHREMAP_OFFSET_DP to value 0"]
impl crate::Resettable for FlashremapOffsetDpSpec {}

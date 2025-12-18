#[doc = "Register `FLASH_REMAP_OFFSET` reader"]
pub type R = crate::R<FlashRemapOffsetSpec>;
#[doc = "Register `FLASH_REMAP_OFFSET` writer"]
pub type W = crate::W<FlashRemapOffsetSpec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, FlashRemapOffsetSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB.\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_remap_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_remap_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRemapOffsetSpec;
impl crate::RegisterSpec for FlashRemapOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_remap_offset::R`](R) reader structure"]
impl crate::Readable for FlashRemapOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_remap_offset::W`](W) writer structure"]
impl crate::Writable for FlashRemapOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_REMAP_OFFSET to value 0"]
impl crate::Resettable for FlashRemapOffsetSpec {}

#[doc = "Register `FLASH_REMAP_SIZE` reader"]
pub type R = crate::R<FlashRemapSizeSpec>;
#[doc = "Register `FLASH_REMAP_SIZE` writer"]
pub type W = crate::W<FlashRemapSizeSpec>;
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
    pub fn field(&mut self) -> FieldW<'_, FlashRemapSizeSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB.\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_remap_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_remap_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRemapSizeSpec;
impl crate::RegisterSpec for FlashRemapSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_remap_size::R`](R) reader structure"]
impl crate::Readable for FlashRemapSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_remap_size::W`](W) writer structure"]
impl crate::Writable for FlashRemapSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_REMAP_SIZE to value 0"]
impl crate::Resettable for FlashRemapSizeSpec {}

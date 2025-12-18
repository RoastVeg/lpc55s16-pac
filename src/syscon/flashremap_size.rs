#[doc = "Register `FLASHREMAP_SIZE` reader"]
pub type R = crate::R<FlashremapSizeSpec>;
#[doc = "Register `FLASHREMAP_SIZE` writer"]
pub type W = crate::W<FlashremapSizeSpec>;
#[doc = "Field `FLASHREMAP_SIZE` reader - no description available"]
pub type FlashremapSizeR = crate::FieldReader<u32>;
#[doc = "Field `FLASHREMAP_SIZE` writer - no description available"]
pub type FlashremapSizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_size(&self) -> FlashremapSizeR {
        FlashremapSizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_size(&mut self) -> FlashremapSizeW<'_, FlashremapSizeSpec> {
        FlashremapSizeW::new(self, 0)
    }
}
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashremap_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashremap_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashremapSizeSpec;
impl crate::RegisterSpec for FlashremapSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashremap_size::R`](R) reader structure"]
impl crate::Readable for FlashremapSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`flashremap_size::W`](W) writer structure"]
impl crate::Writable for FlashremapSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHREMAP_SIZE to value 0"]
impl crate::Resettable for FlashremapSizeSpec {}

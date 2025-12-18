#[doc = "Register `SDIO_DELAY` reader"]
pub type R = crate::R<SdioDelaySpec>;
#[doc = "Register `SDIO_DELAY` writer"]
pub type W = crate::W<SdioDelaySpec>;
#[doc = "Field `SDIO_0_VALID` reader - no description available"]
pub type Sdio0ValidR = crate::BitReader;
#[doc = "Field `SDIO_0_VALID` writer - no description available"]
pub type Sdio0ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_0_DELAY` reader - SDIO_0_DELAY (unit: 100 ps)."]
pub type Sdio0DelayR = crate::FieldReader<u16>;
#[doc = "Field `SDIO_0_DELAY` writer - SDIO_0_DELAY (unit: 100 ps)."]
pub type Sdio0DelayW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sdio_0_valid(&self) -> Sdio0ValidR {
        Sdio0ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub fn sdio_0_delay(&self) -> Sdio0DelayR {
        Sdio0DelayR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sdio_0_valid(&mut self) -> Sdio0ValidW<'_, SdioDelaySpec> {
        Sdio0ValidW::new(self, 0)
    }
    #[doc = "Bits 1:10 - SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub fn sdio_0_delay(&mut self) -> Sdio0DelayW<'_, SdioDelaySpec> {
        Sdio0DelayW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioDelaySpec;
impl crate::RegisterSpec for SdioDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_delay::R`](R) reader structure"]
impl crate::Readable for SdioDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`sdio_delay::W`](W) writer structure"]
impl crate::Writable for SdioDelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_DELAY to value 0"]
impl crate::Resettable for SdioDelaySpec {}

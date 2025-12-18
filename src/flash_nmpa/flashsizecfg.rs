#[doc = "Register `FLASHSIZECFG` reader"]
pub type R = crate::R<FlashsizecfgSpec>;
#[doc = "Register `FLASHSIZECFG` writer"]
pub type W = crate::W<FlashsizecfgSpec>;
#[doc = "Field `FLASH_CONFIGURATION` reader - no description available"]
pub type FlashConfigurationR = crate::FieldReader<u32>;
#[doc = "Field `FLASH_CONFIGURATION` writer - no description available"]
pub type FlashConfigurationW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flash_configuration(&self) -> FlashConfigurationR {
        FlashConfigurationR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHSIZECFG")
            .field("flash_configuration", &self.flash_configuration())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flash_configuration(&mut self) -> FlashConfigurationW<'_, FlashsizecfgSpec> {
        FlashConfigurationW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`flashsizecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashsizecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashsizecfgSpec;
impl crate::RegisterSpec for FlashsizecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashsizecfg::R`](R) reader structure"]
impl crate::Readable for FlashsizecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashsizecfg::W`](W) writer structure"]
impl crate::Writable for FlashsizecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHSIZECFG to value 0"]
impl crate::Resettable for FlashsizecfgSpec {}

#[doc = "Register `SPI_FLASH_CFG` reader"]
pub type R = crate::R<SpiFlashCfgSpec>;
#[doc = "Register `SPI_FLASH_CFG` writer"]
pub type W = crate::W<SpiFlashCfgSpec>;
#[doc = "Field `SPI_RECOVERY_BOOT_EN` reader - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub type SpiRecoveryBootEnR = crate::FieldReader;
#[doc = "Field `SPI_RECOVERY_BOOT_EN` writer - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub type SpiRecoveryBootEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub fn spi_recovery_boot_en(&self) -> SpiRecoveryBootEnR {
        SpiRecoveryBootEnR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub fn spi_recovery_boot_en(&mut self) -> SpiRecoveryBootEnW<'_, SpiFlashCfgSpec> {
        SpiRecoveryBootEnW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_flash_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_flash_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFlashCfgSpec;
impl crate::RegisterSpec for SpiFlashCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_flash_cfg::R`](R) reader structure"]
impl crate::Readable for SpiFlashCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_flash_cfg::W`](W) writer structure"]
impl crate::Writable for SpiFlashCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_FLASH_CFG to value 0"]
impl crate::Resettable for SpiFlashCfgSpec {}

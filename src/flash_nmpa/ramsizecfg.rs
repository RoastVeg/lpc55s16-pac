#[doc = "Register `RAMSIZECFG` reader"]
pub type R = crate::R<RamsizecfgSpec>;
#[doc = "Register `RAMSIZECFG` writer"]
pub type W = crate::W<RamsizecfgSpec>;
#[doc = "Field `SRAM_CONFIGURATION` reader - no description available"]
pub type SramConfigurationR = crate::FieldReader<u32>;
#[doc = "Field `SRAM_CONFIGURATION` writer - no description available"]
pub type SramConfigurationW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sram_configuration(&self) -> SramConfigurationR {
        SramConfigurationR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMSIZECFG")
            .field("sram_configuration", &self.sram_configuration())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sram_configuration(&mut self) -> SramConfigurationW<'_, RamsizecfgSpec> {
        SramConfigurationW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ramsizecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramsizecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamsizecfgSpec;
impl crate::RegisterSpec for RamsizecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramsizecfg::R`](R) reader structure"]
impl crate::Readable for RamsizecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ramsizecfg::W`](W) writer structure"]
impl crate::Writable for RamsizecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMSIZECFG to value 0"]
impl crate::Resettable for RamsizecfgSpec {}

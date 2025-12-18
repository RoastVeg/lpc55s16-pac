#[doc = "Register `SDIO_CFG` reader"]
pub type R = crate::R<SdioCfgSpec>;
#[doc = "Register `SDIO_CFG` writer"]
pub type W = crate::W<SdioCfgSpec>;
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
    pub fn field(&mut self) -> FieldW<'_, SdioCfgSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioCfgSpec;
impl crate::RegisterSpec for SdioCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_cfg::R`](R) reader structure"]
impl crate::Readable for SdioCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sdio_cfg::W`](W) writer structure"]
impl crate::Writable for SdioCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_CFG to value 0"]
impl crate::Resettable for SdioCfgSpec {}

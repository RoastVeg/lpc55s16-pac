#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `enable` reader - PUF SRAM Controller activation"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - PUF SRAM Controller activation"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ckgating` reader - PUF SRAM Clock Gating control"]
pub type CkgatingR = crate::BitReader;
#[doc = "Field `ckgating` writer - PUF SRAM Clock Gating control"]
pub type CkgatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PUF SRAM Controller activation"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&self) -> CkgatingR {
        CkgatingR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("enable", &self.enable())
            .field("ckgating", &self.ckgating())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PUF SRAM Controller activation"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&mut self) -> CkgatingW<'_, CfgSpec> {
        CkgatingW::new(self, 2)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}

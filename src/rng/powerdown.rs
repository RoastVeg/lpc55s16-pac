#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<PowerdownSpec>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<PowerdownSpec>;
#[doc = "Field `SOFT_RESET` reader - Request softreset that will go low automaticaly after acknowledge from CORE."]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - Request softreset that will go low automaticaly after acknowledge from CORE."]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SOFT_RESET` reader - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
pub type ForceSoftResetR = crate::BitReader;
#[doc = "Field `FORCE_SOFT_RESET` writer - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
pub type ForceSoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWERDOWN` reader - When set all accesses to standard registers are blocked."]
pub type PowerdownR = crate::BitReader;
#[doc = "Field `POWERDOWN` writer - When set all accesses to standard registers are blocked."]
pub type PowerdownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&self) -> ForceSoftResetR {
        ForceSoftResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWERDOWN")
            .field("soft_reset", &self.soft_reset())
            .field("force_soft_reset", &self.force_soft_reset())
            .field("powerdown", &self.powerdown())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SoftResetW<'_, PowerdownSpec> {
        SoftResetW::new(self, 0)
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&mut self) -> ForceSoftResetW<'_, PowerdownSpec> {
        ForceSoftResetW::new(self, 1)
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&mut self) -> PowerdownW<'_, PowerdownSpec> {
        PowerdownW::new(self, 31)
    }
}
#[doc = "Powerdown mode (standard but certainly useless here)\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerdownSpec;
impl crate::RegisterSpec for PowerdownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for PowerdownSpec {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for PowerdownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for PowerdownSpec {}

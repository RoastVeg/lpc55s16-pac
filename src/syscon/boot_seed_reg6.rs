#[doc = "Register `BOOT_SEED_REG6` reader"]
pub type R = crate::R<BootSeedReg6Spec>;
#[doc = "Register `BOOT_SEED_REG6` writer"]
pub type W = crate::W<BootSeedReg6Spec>;
#[doc = "Field `BOOT_SEED_REG6` reader - no description available"]
pub type BootSeedReg6R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_SEED_REG6` writer - no description available"]
pub type BootSeedReg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg6(&self) -> BootSeedReg6R {
        BootSeedReg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg6(&mut self) -> BootSeedReg6W<'_, BootSeedReg6Spec> {
        BootSeedReg6W::new(self, 0)
    }
}
#[doc = "boot seed (256-bit random value)\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_seed_reg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_seed_reg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootSeedReg6Spec;
impl crate::RegisterSpec for BootSeedReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_seed_reg6::R`](R) reader structure"]
impl crate::Readable for BootSeedReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`boot_seed_reg6::W`](W) writer structure"]
impl crate::Writable for BootSeedReg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_SEED_REG6 to value 0"]
impl crate::Resettable for BootSeedReg6Spec {}

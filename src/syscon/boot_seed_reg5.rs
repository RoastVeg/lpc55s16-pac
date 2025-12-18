#[doc = "Register `BOOT_SEED_REG5` reader"]
pub type R = crate::R<BootSeedReg5Spec>;
#[doc = "Register `BOOT_SEED_REG5` writer"]
pub type W = crate::W<BootSeedReg5Spec>;
#[doc = "Field `BOOT_SEED_REG5` reader - no description available"]
pub type BootSeedReg5R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_SEED_REG5` writer - no description available"]
pub type BootSeedReg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg5(&self) -> BootSeedReg5R {
        BootSeedReg5R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG5")
            .field("boot_seed_reg5", &self.boot_seed_reg5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg5(&mut self) -> BootSeedReg5W<'_, BootSeedReg5Spec> {
        BootSeedReg5W::new(self, 0)
    }
}
#[doc = "boot seed (256-bit random value)\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_seed_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_seed_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootSeedReg5Spec;
impl crate::RegisterSpec for BootSeedReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_seed_reg5::R`](R) reader structure"]
impl crate::Readable for BootSeedReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`boot_seed_reg5::W`](W) writer structure"]
impl crate::Writable for BootSeedReg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_SEED_REG5 to value 0"]
impl crate::Resettable for BootSeedReg5Spec {}

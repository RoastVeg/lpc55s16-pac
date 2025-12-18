#[doc = "Register `BOOT_SEED_REG0` reader"]
pub type R = crate::R<BootSeedReg0Spec>;
#[doc = "Register `BOOT_SEED_REG0` writer"]
pub type W = crate::W<BootSeedReg0Spec>;
#[doc = "Field `BOOT_SEED_REG0` reader - no description available"]
pub type BootSeedReg0R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_SEED_REG0` writer - no description available"]
pub type BootSeedReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg0(&self) -> BootSeedReg0R {
        BootSeedReg0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG0")
            .field("boot_seed_reg0", &self.boot_seed_reg0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn boot_seed_reg0(&mut self) -> BootSeedReg0W<'_, BootSeedReg0Spec> {
        BootSeedReg0W::new(self, 0)
    }
}
#[doc = "boot seed (256-bit random value)\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_seed_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_seed_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootSeedReg0Spec;
impl crate::RegisterSpec for BootSeedReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_seed_reg0::R`](R) reader structure"]
impl crate::Readable for BootSeedReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`boot_seed_reg0::W`](W) writer structure"]
impl crate::Writable for BootSeedReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_SEED_REG0 to value 0"]
impl crate::Resettable for BootSeedReg0Spec {}

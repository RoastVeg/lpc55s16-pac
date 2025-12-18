#[doc = "Register `BOOT_LOCK` reader"]
pub type R = crate::R<BootLockSpec>;
#[doc = "Register `BOOT_LOCK` writer"]
pub type W = crate::W<BootLockSpec>;
#[doc = "Control write access to BOOT_SEED_REG registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockBootSeed {
    #[doc = "1: write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    Lock = 1,
}
impl From<LockBootSeed> for bool {
    #[inline(always)]
    fn from(variant: LockBootSeed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_BOOT_SEED` reader - Control write access to BOOT_SEED_REG registers."]
pub type LockBootSeedR = crate::BitReader<LockBootSeed>;
impl LockBootSeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockBootSeed> {
        match self.bits {
            true => Some(LockBootSeed::Lock),
            _ => None,
        }
    }
    #[doc = "write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LockBootSeed::Lock
    }
}
#[doc = "Field `LOCK_BOOT_SEED` writer - Control write access to BOOT_SEED_REG registers."]
pub type LockBootSeedW<'a, REG> = crate::BitWriter<'a, REG, LockBootSeed>;
impl<'a, REG> LockBootSeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LockBootSeed::Lock)
    }
}
#[doc = "Control write access to HMAC_REG registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockHmac {
    #[doc = "1: write access to all 8 registers HMAC_REG is locked. This register is write once."]
    Lock = 1,
}
impl From<LockHmac> for bool {
    #[inline(always)]
    fn from(variant: LockHmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_HMAC` reader - Control write access to HMAC_REG registers."]
pub type LockHmacR = crate::BitReader<LockHmac>;
impl LockHmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockHmac> {
        match self.bits {
            true => Some(LockHmac::Lock),
            _ => None,
        }
    }
    #[doc = "write access to all 8 registers HMAC_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LockHmac::Lock
    }
}
#[doc = "Field `LOCK_HMAC` writer - Control write access to HMAC_REG registers."]
pub type LockHmacW<'a, REG> = crate::BitWriter<'a, REG, LockHmac>;
impl<'a, REG> LockHmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write access to all 8 registers HMAC_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LockHmac::Lock)
    }
}
impl R {
    #[doc = "Bit 0 - Control write access to BOOT_SEED_REG registers."]
    #[inline(always)]
    pub fn lock_boot_seed(&self) -> LockBootSeedR {
        LockBootSeedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control write access to HMAC_REG registers."]
    #[inline(always)]
    pub fn lock_hmac(&self) -> LockHmacR {
        LockHmacR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control write access to BOOT_SEED_REG registers."]
    #[inline(always)]
    pub fn lock_boot_seed(&mut self) -> LockBootSeedW<'_, BootLockSpec> {
        LockBootSeedW::new(self, 0)
    }
    #[doc = "Bit 1 - Control write access to HMAC_REG registers."]
    #[inline(always)]
    pub fn lock_hmac(&mut self) -> LockHmacW<'_, BootLockSpec> {
        LockHmacW::new(self, 1)
    }
}
#[doc = "Control write access to boot seed security registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootLockSpec;
impl crate::RegisterSpec for BootLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_lock::R`](R) reader structure"]
impl crate::Readable for BootLockSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_lock::W`](W) writer structure"]
impl crate::Writable for BootLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_LOCK to value 0"]
impl crate::Resettable for BootLockSpec {}

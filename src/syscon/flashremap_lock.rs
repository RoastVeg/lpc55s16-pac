#[doc = "Register `FLASHREMAP_LOCK` reader"]
pub type R = crate::R<FlashremapLockSpec>;
#[doc = "Register `FLASHREMAP_LOCK` writer"]
pub type W = crate::W<FlashremapLockSpec>;
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state.\n\nValue on reset: 3275531610"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Lock {
    #[doc = "1019435685: Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    Unlock = 1019435685,
    #[doc = "3275531610: Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    Lock = 3275531610,
}
impl From<Lock> for u32 {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lock {
    type Ux = u32;
}
impl crate::IsEnum for Lock {}
#[doc = "Field `LOCK` reader - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
pub type LockR = crate::FieldReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lock> {
        match self.bits {
            1019435685 => Some(Lock::Unlock),
            3275531610 => Some(Lock::Lock),
            _ => None,
        }
    }
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Lock::Unlock
    }
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Lock::Lock
    }
}
#[doc = "Field `LOCK` writer - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 32, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlock)
    }
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Lock)
    }
}
impl R {
    #[doc = "Bits 0:31 - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, FlashremapLockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashremap_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashremap_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashremapLockSpec;
impl crate::RegisterSpec for FlashremapLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashremap_lock::R`](R) reader structure"]
impl crate::Readable for FlashremapLockSpec {}
#[doc = "`write(|w| ..)` method takes [`flashremap_lock::W`](W) writer structure"]
impl crate::Writable for FlashremapLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHREMAP_LOCK to value 0xc33c_a55a"]
impl crate::Resettable for FlashremapLockSpec {
    const RESET_VALUE: u32 = 0xc33c_a55a;
}

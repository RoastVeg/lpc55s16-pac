#[doc = "Register `DEBUG_LOCK_EN` reader"]
pub type R = crate::R<DebugLockEnSpec>;
#[doc = "Register `DEBUG_LOCK_EN` writer"]
pub type W = crate::W<DebugLockEnSpec>;
#[doc = "Control write access to security registers.\n\nValue on reset: 5"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockAll {
    #[doc = "0: Any other value than b1010: disable write access to all registers."]
    Disable = 0,
    #[doc = "10: 1010: Enable write access to all registers."]
    Enable = 10,
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(variant: LockAll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockAll {
    type Ux = u8;
}
impl crate::IsEnum for LockAll {}
#[doc = "Field `LOCK_ALL` reader - Control write access to security registers."]
pub type LockAllR = crate::FieldReader<LockAll>;
impl LockAllR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockAll> {
        match self.bits {
            0 => Some(LockAll::Disable),
            10 => Some(LockAll::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b1010: disable write access to all registers."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LockAll::Disable
    }
    #[doc = "1010: Enable write access to all registers."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LockAll::Enable
    }
}
#[doc = "Field `LOCK_ALL` writer - Control write access to security registers."]
pub type LockAllW<'a, REG> = crate::FieldWriter<'a, REG, 4, LockAll>;
impl<'a, REG> LockAllW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b1010: disable write access to all registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LockAll::Disable)
    }
    #[doc = "1010: Enable write access to all registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LockAll::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Control write access to security registers."]
    #[inline(always)]
    pub fn lock_all(&self) -> LockAllR {
        LockAllR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_LOCK_EN")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Control write access to security registers."]
    #[inline(always)]
    pub fn lock_all(&mut self) -> LockAllW<'_, DebugLockEnSpec> {
        LockAllW::new(self, 0)
    }
}
#[doc = "Control write access to security registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_lock_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_lock_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugLockEnSpec;
impl crate::RegisterSpec for DebugLockEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_lock_en::R`](R) reader structure"]
impl crate::Readable for DebugLockEnSpec {}
#[doc = "`write(|w| ..)` method takes [`debug_lock_en::W`](W) writer structure"]
impl crate::Writable for DebugLockEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_LOCK_EN to value 0x05"]
impl crate::Resettable for DebugLockEnSpec {
    const RESET_VALUE: u32 = 0x05;
}

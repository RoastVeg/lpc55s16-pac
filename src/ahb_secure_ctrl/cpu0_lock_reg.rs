#[doc = "Register `CPU0_LOCK_REG` reader"]
pub type R = crate::R<Cpu0LockRegSpec>;
#[doc = "Register `CPU0_LOCK_REG` writer"]
pub type W = crate::W<Cpu0LockRegSpec>;
#[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsVtor {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(variant: LockNsVtor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsVtor {
    type Ux = u8;
}
impl crate::IsEnum for LockNsVtor {}
#[doc = "Field `LOCK_NS_VTOR` reader - Cortex M33 (CPU0) VTOR_NS register write-lock."]
pub type LockNsVtorR = crate::FieldReader<LockNsVtor>;
impl LockNsVtorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsVtor> {
        match self.bits {
            1 => Some(LockNsVtor::Blocked),
            2 => Some(LockNsVtor::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsVtor::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsVtor::Writable
    }
}
#[doc = "Field `LOCK_NS_VTOR` writer - Cortex M33 (CPU0) VTOR_NS register write-lock."]
pub type LockNsVtorW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsVtor>;
impl<'a, REG> LockNsVtorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Writable)
    }
}
#[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsMpu {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(variant: LockNsMpu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsMpu {
    type Ux = u8;
}
impl crate::IsEnum for LockNsMpu {}
#[doc = "Field `LOCK_NS_MPU` reader - Cortex M33 (CPU0) non-secure MPU register write-lock."]
pub type LockNsMpuR = crate::FieldReader<LockNsMpu>;
impl LockNsMpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsMpu> {
        match self.bits {
            1 => Some(LockNsMpu::Blocked),
            2 => Some(LockNsMpu::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsMpu::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsMpu::Writable
    }
}
#[doc = "Field `LOCK_NS_MPU` writer - Cortex M33 (CPU0) non-secure MPU register write-lock."]
pub type LockNsMpuW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsMpu>;
impl<'a, REG> LockNsMpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Writable)
    }
}
#[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSVtaircr {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(variant: LockSVtaircr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSVtaircr {
    type Ux = u8;
}
impl crate::IsEnum for LockSVtaircr {}
#[doc = "Field `LOCK_S_VTAIRCR` reader - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
pub type LockSVtaircrR = crate::FieldReader<LockSVtaircr>;
impl LockSVtaircrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSVtaircr> {
        match self.bits {
            1 => Some(LockSVtaircr::Blocked),
            2 => Some(LockSVtaircr::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSVtaircr::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSVtaircr::Writable
    }
}
#[doc = "Field `LOCK_S_VTAIRCR` writer - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
pub type LockSVtaircrW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSVtaircr>;
impl<'a, REG> LockSVtaircrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSVtaircr::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSVtaircr::Writable)
    }
}
#[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSMpu {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(variant: LockSMpu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSMpu {
    type Ux = u8;
}
impl crate::IsEnum for LockSMpu {}
#[doc = "Field `LOCK_S_MPU` reader - Cortex M33 (CPU0) Secure MPU registers write-lock."]
pub type LockSMpuR = crate::FieldReader<LockSMpu>;
impl LockSMpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSMpu> {
        match self.bits {
            1 => Some(LockSMpu::Blocked),
            2 => Some(LockSMpu::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSMpu::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSMpu::Writable
    }
}
#[doc = "Field `LOCK_S_MPU` writer - Cortex M33 (CPU0) Secure MPU registers write-lock."]
pub type LockSMpuW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSMpu>;
impl<'a, REG> LockSMpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSMpu::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSMpu::Writable)
    }
}
#[doc = "Cortex M33 (CPU0) SAU registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSau {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(variant: LockSau) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSau {
    type Ux = u8;
}
impl crate::IsEnum for LockSau {}
#[doc = "Field `LOCK_SAU` reader - Cortex M33 (CPU0) SAU registers write-lock."]
pub type LockSauR = crate::FieldReader<LockSau>;
impl LockSauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSau> {
        match self.bits {
            1 => Some(LockSau::Blocked),
            2 => Some(LockSau::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSau::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSau::Writable
    }
}
#[doc = "Field `LOCK_SAU` writer - Cortex M33 (CPU0) SAU registers write-lock."]
pub type LockSauW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSau>;
impl<'a, REG> LockSauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSau::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSau::Writable)
    }
}
#[doc = "CPU0_LOCK_REG write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0LockRegLock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<Cpu0LockRegLock> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0LockRegLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0LockRegLock {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0LockRegLock {}
#[doc = "Field `CPU0_LOCK_REG_LOCK` reader - CPU0_LOCK_REG write-lock."]
pub type Cpu0LockRegLockR = crate::FieldReader<Cpu0LockRegLock>;
impl Cpu0LockRegLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0LockRegLock> {
        match self.bits {
            1 => Some(Cpu0LockRegLock::Blocked),
            2 => Some(Cpu0LockRegLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Cpu0LockRegLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == Cpu0LockRegLock::Writable
    }
}
#[doc = "Field `CPU0_LOCK_REG_LOCK` writer - CPU0_LOCK_REG write-lock."]
pub type Cpu0LockRegLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0LockRegLock>;
impl<'a, REG> Cpu0LockRegLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0LockRegLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0LockRegLock::Writable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LockNsVtorR {
        LockNsVtorR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LockNsMpuR {
        LockNsMpuR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub fn lock_s_vtaircr(&self) -> LockSVtaircrR {
        LockSVtaircrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub fn lock_s_mpu(&self) -> LockSMpuR {
        LockSMpuR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub fn lock_sau(&self) -> LockSauR {
        LockSauR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu0_lock_reg_lock(&self) -> Cpu0LockRegLockR {
        Cpu0LockRegLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&mut self) -> LockNsVtorW<'_, Cpu0LockRegSpec> {
        LockNsVtorW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&mut self) -> LockNsMpuW<'_, Cpu0LockRegSpec> {
        LockNsMpuW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub fn lock_s_vtaircr(&mut self) -> LockSVtaircrW<'_, Cpu0LockRegSpec> {
        LockSVtaircrW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub fn lock_s_mpu(&mut self) -> LockSMpuW<'_, Cpu0LockRegSpec> {
        LockSMpuW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub fn lock_sau(&mut self) -> LockSauW<'_, Cpu0LockRegSpec> {
        LockSauW::new(self, 8)
    }
    #[doc = "Bits 30:31 - CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu0_lock_reg_lock(&mut self) -> Cpu0LockRegLockW<'_, Cpu0LockRegSpec> {
        Cpu0LockRegLockW::new(self, 30)
    }
}
#[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_lock_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_lock_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0LockRegSpec;
impl crate::RegisterSpec for Cpu0LockRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_lock_reg::R`](R) reader structure"]
impl crate::Readable for Cpu0LockRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_lock_reg::W`](W) writer structure"]
impl crate::Writable for Cpu0LockRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_LOCK_REG to value 0x8000_02aa"]
impl crate::Resettable for Cpu0LockRegSpec {
    const RESET_VALUE: u32 = 0x8000_02aa;
}

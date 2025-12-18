#[doc = "Register `PRINCE_BASE_ADDR` reader"]
pub type R = crate::R<PrinceBaseAddrSpec>;
#[doc = "Register `PRINCE_BASE_ADDR` writer"]
pub type W = crate::W<PrinceBaseAddrSpec>;
#[doc = "Field `ADDR0_PRG` reader - Programmable portion of the base address of region 0"]
pub type Addr0PrgR = crate::FieldReader;
#[doc = "Field `ADDR0_PRG` writer - Programmable portion of the base address of region 0"]
pub type Addr0PrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDR1_PRG` reader - Programmable portion of the base address of region 1"]
pub type Addr1PrgR = crate::FieldReader;
#[doc = "Field `ADDR1_PRG` writer - Programmable portion of the base address of region 1"]
pub type Addr1PrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDR2_PRG` reader - Programmable portion of the base address of region 2"]
pub type Addr2PrgR = crate::FieldReader;
#[doc = "Field `ADDR2_PRG` writer - Programmable portion of the base address of region 2"]
pub type Addr2PrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Lock PRINCE region0 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockReg0 {
    #[doc = "0: Region is not locked"]
    Unlock = 0,
    #[doc = "1: Region is locked"]
    Lock = 1,
    #[doc = "2: Region is locked"]
    Value2 = 2,
    #[doc = "3: Region is locked"]
    Value3 = 3,
}
impl From<LockReg0> for u8 {
    #[inline(always)]
    fn from(variant: LockReg0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockReg0 {
    type Ux = u8;
}
impl crate::IsEnum for LockReg0 {}
#[doc = "Field `LOCK_REG0` reader - Lock PRINCE region0 settings"]
pub type LockReg0R = crate::FieldReader<LockReg0>;
impl LockReg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockReg0 {
        match self.bits {
            0 => LockReg0::Unlock,
            1 => LockReg0::Lock,
            2 => LockReg0::Value2,
            3 => LockReg0::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Region is not locked"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LockReg0::Unlock
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LockReg0::Lock
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == LockReg0::Value2
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == LockReg0::Value3
    }
}
#[doc = "Field `LOCK_REG0` writer - Lock PRINCE region0 settings"]
pub type LockReg0W<'a, REG> = crate::FieldWriter<'a, REG, 2, LockReg0, crate::Safe>;
impl<'a, REG> LockReg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Region is not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg0::Unlock)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg0::Lock)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg0::Value2)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg0::Value3)
    }
}
#[doc = "Lock PRINCE region1 settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockReg1 {
    #[doc = "0: Region is not locked"]
    Unlock = 0,
    #[doc = "1: Region is locked"]
    Lock = 1,
    #[doc = "2: Region is locked"]
    Value2 = 2,
    #[doc = "3: Region is locked"]
    Value3 = 3,
}
impl From<LockReg1> for u8 {
    #[inline(always)]
    fn from(variant: LockReg1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockReg1 {
    type Ux = u8;
}
impl crate::IsEnum for LockReg1 {}
#[doc = "Field `LOCK_REG1` reader - Lock PRINCE region1 settings"]
pub type LockReg1R = crate::FieldReader<LockReg1>;
impl LockReg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockReg1 {
        match self.bits {
            0 => LockReg1::Unlock,
            1 => LockReg1::Lock,
            2 => LockReg1::Value2,
            3 => LockReg1::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Region is not locked"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LockReg1::Unlock
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LockReg1::Lock
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == LockReg1::Value2
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == LockReg1::Value3
    }
}
#[doc = "Field `LOCK_REG1` writer - Lock PRINCE region1 settings"]
pub type LockReg1W<'a, REG> = crate::FieldWriter<'a, REG, 2, LockReg1, crate::Safe>;
impl<'a, REG> LockReg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Region is not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg1::Unlock)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg1::Lock)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg1::Value2)
    }
    #[doc = "Region is locked"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(LockReg1::Value3)
    }
}
#[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg0EraseCheckEn {
    #[doc = "0: Region is disabled"]
    Disable = 0,
    #[doc = "1: Region is enabled"]
    Enable = 1,
    #[doc = "2: Region is enabled"]
    Value2 = 2,
    #[doc = "3: Region is enabled"]
    Value3 = 3,
}
impl From<Reg0EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(variant: Reg0EraseCheckEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg0EraseCheckEn {
    type Ux = u8;
}
impl crate::IsEnum for Reg0EraseCheckEn {}
#[doc = "Field `REG0_ERASE_CHECK_EN` reader - For PRINCE region0 enable checking whether all encrypted pages are erased together"]
pub type Reg0EraseCheckEnR = crate::FieldReader<Reg0EraseCheckEn>;
impl Reg0EraseCheckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg0EraseCheckEn {
        match self.bits {
            0 => Reg0EraseCheckEn::Disable,
            1 => Reg0EraseCheckEn::Enable,
            2 => Reg0EraseCheckEn::Value2,
            3 => Reg0EraseCheckEn::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reg0EraseCheckEn::Disable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Reg0EraseCheckEn::Enable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Reg0EraseCheckEn::Value2
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == Reg0EraseCheckEn::Value3
    }
}
#[doc = "Field `REG0_ERASE_CHECK_EN` writer - For PRINCE region0 enable checking whether all encrypted pages are erased together"]
pub type Reg0EraseCheckEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg0EraseCheckEn, crate::Safe>;
impl<'a, REG> Reg0EraseCheckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0EraseCheckEn::Disable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0EraseCheckEn::Enable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0EraseCheckEn::Value2)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0EraseCheckEn::Value3)
    }
}
#[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg1EraseCheckEn {
    #[doc = "0: Region is disabled"]
    Disable = 0,
    #[doc = "1: Region is enabled"]
    Enable = 1,
    #[doc = "2: Region is enabled"]
    Value2 = 2,
    #[doc = "3: Region is enabled"]
    Value3 = 3,
}
impl From<Reg1EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(variant: Reg1EraseCheckEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg1EraseCheckEn {
    type Ux = u8;
}
impl crate::IsEnum for Reg1EraseCheckEn {}
#[doc = "Field `REG1_ERASE_CHECK_EN` reader - For PRINCE region1 enable checking whether all encrypted pages are erased together"]
pub type Reg1EraseCheckEnR = crate::FieldReader<Reg1EraseCheckEn>;
impl Reg1EraseCheckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg1EraseCheckEn {
        match self.bits {
            0 => Reg1EraseCheckEn::Disable,
            1 => Reg1EraseCheckEn::Enable,
            2 => Reg1EraseCheckEn::Value2,
            3 => Reg1EraseCheckEn::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reg1EraseCheckEn::Disable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Reg1EraseCheckEn::Enable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Reg1EraseCheckEn::Value2
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == Reg1EraseCheckEn::Value3
    }
}
#[doc = "Field `REG1_ERASE_CHECK_EN` writer - For PRINCE region1 enable checking whether all encrypted pages are erased together"]
pub type Reg1EraseCheckEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg1EraseCheckEn, crate::Safe>;
impl<'a, REG> Reg1EraseCheckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1EraseCheckEn::Disable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1EraseCheckEn::Enable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1EraseCheckEn::Value2)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1EraseCheckEn::Value3)
    }
}
#[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg2EraseCheckEn {
    #[doc = "0: Region is disabled"]
    Disable = 0,
    #[doc = "1: Region is enabled"]
    Enable = 1,
    #[doc = "2: Region is enabled"]
    Value2 = 2,
    #[doc = "3: Region is enabled"]
    Value3 = 3,
}
impl From<Reg2EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(variant: Reg2EraseCheckEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg2EraseCheckEn {
    type Ux = u8;
}
impl crate::IsEnum for Reg2EraseCheckEn {}
#[doc = "Field `REG2_ERASE_CHECK_EN` reader - For PRINCE region2 enable checking whether all encrypted pages are erased together"]
pub type Reg2EraseCheckEnR = crate::FieldReader<Reg2EraseCheckEn>;
impl Reg2EraseCheckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg2EraseCheckEn {
        match self.bits {
            0 => Reg2EraseCheckEn::Disable,
            1 => Reg2EraseCheckEn::Enable,
            2 => Reg2EraseCheckEn::Value2,
            3 => Reg2EraseCheckEn::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reg2EraseCheckEn::Disable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Reg2EraseCheckEn::Enable
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Reg2EraseCheckEn::Value2
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == Reg2EraseCheckEn::Value3
    }
}
#[doc = "Field `REG2_ERASE_CHECK_EN` writer - For PRINCE region2 enable checking whether all encrypted pages are erased together"]
pub type Reg2EraseCheckEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg2EraseCheckEn, crate::Safe>;
impl<'a, REG> Reg2EraseCheckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Region is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg2EraseCheckEn::Disable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reg2EraseCheckEn::Enable)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Reg2EraseCheckEn::Value2)
    }
    #[doc = "Region is enabled"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(Reg2EraseCheckEn::Value3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0"]
    #[inline(always)]
    pub fn addr0_prg(&self) -> Addr0PrgR {
        Addr0PrgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1"]
    #[inline(always)]
    pub fn addr1_prg(&self) -> Addr1PrgR {
        Addr1PrgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2"]
    #[inline(always)]
    pub fn addr2_prg(&self) -> Addr2PrgR {
        Addr2PrgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region0 settings"]
    #[inline(always)]
    pub fn lock_reg0(&self) -> LockReg0R {
        LockReg0R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region1 settings"]
    #[inline(always)]
    pub fn lock_reg1(&self) -> LockReg1R {
        LockReg1R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg0_erase_check_en(&self) -> Reg0EraseCheckEnR {
        Reg0EraseCheckEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg1_erase_check_en(&self) -> Reg1EraseCheckEnR {
        Reg1EraseCheckEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg2_erase_check_en(&self) -> Reg2EraseCheckEnR {
        Reg2EraseCheckEnR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0"]
    #[inline(always)]
    pub fn addr0_prg(&mut self) -> Addr0PrgW<'_, PrinceBaseAddrSpec> {
        Addr0PrgW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1"]
    #[inline(always)]
    pub fn addr1_prg(&mut self) -> Addr1PrgW<'_, PrinceBaseAddrSpec> {
        Addr1PrgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2"]
    #[inline(always)]
    pub fn addr2_prg(&mut self) -> Addr2PrgW<'_, PrinceBaseAddrSpec> {
        Addr2PrgW::new(self, 8)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region0 settings"]
    #[inline(always)]
    pub fn lock_reg0(&mut self) -> LockReg0W<'_, PrinceBaseAddrSpec> {
        LockReg0W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region1 settings"]
    #[inline(always)]
    pub fn lock_reg1(&mut self) -> LockReg1W<'_, PrinceBaseAddrSpec> {
        LockReg1W::new(self, 20)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg0_erase_check_en(&mut self) -> Reg0EraseCheckEnW<'_, PrinceBaseAddrSpec> {
        Reg0EraseCheckEnW::new(self, 24)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg1_erase_check_en(&mut self) -> Reg1EraseCheckEnW<'_, PrinceBaseAddrSpec> {
        Reg1EraseCheckEnW::new(self, 26)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub fn reg2_erase_check_en(&mut self) -> Reg2EraseCheckEnW<'_, PrinceBaseAddrSpec> {
        Reg2EraseCheckEnW::new(self, 28)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceBaseAddrSpec;
impl crate::RegisterSpec for PrinceBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_base_addr::R`](R) reader structure"]
impl crate::Readable for PrinceBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`prince_base_addr::W`](W) writer structure"]
impl crate::Writable for PrinceBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRINCE_BASE_ADDR to value 0"]
impl crate::Resettable for PrinceBaseAddrSpec {}

#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Lock Region 0 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockreg0 {
    #[doc = "0: Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    Disabled = 0,
    #[doc = "1: Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    Enabled = 1,
}
impl From<Lockreg0> for bool {
    #[inline(always)]
    fn from(variant: Lockreg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG0` reader - Lock Region 0 registers."]
pub type Lockreg0R = crate::BitReader<Lockreg0>;
impl Lockreg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockreg0 {
        match self.bits {
            false => Lockreg0::Disabled,
            true => Lockreg0::Enabled,
        }
    }
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockreg0::Disabled
    }
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lockreg0::Enabled
    }
}
#[doc = "Field `LOCKREG0` writer - Lock Region 0 registers."]
pub type Lockreg0W<'a, REG> = crate::BitWriter<'a, REG, Lockreg0>;
impl<'a, REG> Lockreg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg0::Disabled)
    }
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg0::Enabled)
    }
}
#[doc = "Lock Region 1 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockreg1 {
    #[doc = "0: Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    Disabled = 0,
    #[doc = "1: Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    Enabled = 1,
}
impl From<Lockreg1> for bool {
    #[inline(always)]
    fn from(variant: Lockreg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG1` reader - Lock Region 1 registers."]
pub type Lockreg1R = crate::BitReader<Lockreg1>;
impl Lockreg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockreg1 {
        match self.bits {
            false => Lockreg1::Disabled,
            true => Lockreg1::Enabled,
        }
    }
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockreg1::Disabled
    }
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lockreg1::Enabled
    }
}
#[doc = "Field `LOCKREG1` writer - Lock Region 1 registers."]
pub type Lockreg1W<'a, REG> = crate::BitWriter<'a, REG, Lockreg1>;
impl<'a, REG> Lockreg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg1::Disabled)
    }
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg1::Enabled)
    }
}
#[doc = "Lock Region 2 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockreg2 {
    #[doc = "0: Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    Disabled = 0,
    #[doc = "1: Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    Enabled = 1,
}
impl From<Lockreg2> for bool {
    #[inline(always)]
    fn from(variant: Lockreg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG2` reader - Lock Region 2 registers."]
pub type Lockreg2R = crate::BitReader<Lockreg2>;
impl Lockreg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockreg2 {
        match self.bits {
            false => Lockreg2::Disabled,
            true => Lockreg2::Enabled,
        }
    }
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockreg2::Disabled
    }
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lockreg2::Enabled
    }
}
#[doc = "Field `LOCKREG2` writer - Lock Region 2 registers."]
pub type Lockreg2W<'a, REG> = crate::BitWriter<'a, REG, Lockreg2>;
impl<'a, REG> Lockreg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg2::Disabled)
    }
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockreg2::Enabled)
    }
}
#[doc = "Lock the Mask registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockmask {
    #[doc = "0: Disabled. MASK_LSB, and MASK_MSB are writable.."]
    Disabled = 0,
    #[doc = "1: Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    Enabled = 1,
}
impl From<Lockmask> for bool {
    #[inline(always)]
    fn from(variant: Lockmask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKMASK` reader - Lock the Mask registers."]
pub type LockmaskR = crate::BitReader<Lockmask>;
impl LockmaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockmask {
        match self.bits {
            false => Lockmask::Disabled,
            true => Lockmask::Enabled,
        }
    }
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockmask::Disabled
    }
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lockmask::Enabled
    }
}
#[doc = "Field `LOCKMASK` writer - Lock the Mask registers."]
pub type LockmaskW<'a, REG> = crate::BitWriter<'a, REG, Lockmask>;
impl<'a, REG> LockmaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmask::Disabled)
    }
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockmask::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline(always)]
    pub fn lockreg0(&self) -> Lockreg0R {
        Lockreg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline(always)]
    pub fn lockreg1(&self) -> Lockreg1R {
        Lockreg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline(always)]
    pub fn lockreg2(&self) -> Lockreg2R {
        Lockreg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline(always)]
    pub fn lockmask(&self) -> LockmaskR {
        LockmaskR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline(always)]
    pub fn lockreg0(&mut self) -> Lockreg0W<'_, LockSpec> {
        Lockreg0W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline(always)]
    pub fn lockreg1(&mut self) -> Lockreg1W<'_, LockSpec> {
        Lockreg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline(always)]
    pub fn lockreg2(&mut self) -> Lockreg2W<'_, LockSpec> {
        Lockreg2W::new(self, 2)
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline(always)]
    pub fn lockmask(&mut self) -> LockmaskW<'_, LockSpec> {
        LockmaskW::new(self, 8)
    }
}
#[doc = "Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}

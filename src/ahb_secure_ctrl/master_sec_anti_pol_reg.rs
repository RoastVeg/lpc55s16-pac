#[doc = "Register `MASTER_SEC_ANTI_POL_REG` reader"]
pub type R = crate::R<MasterSecAntiPolRegSpec>;
#[doc = "Register `MASTER_SEC_ANTI_POL_REG` writer"]
pub type W = crate::W<MasterSecAntiPolRegSpec>;
#[doc = "USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbfsd {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Usbfsd> for u8 {
    #[inline(always)]
    fn from(variant: Usbfsd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbfsd {
    type Ux = u8;
}
impl crate::IsEnum for Usbfsd {}
#[doc = "Field `USBFSD` reader - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
pub type UsbfsdR = crate::FieldReader<Usbfsd>;
impl UsbfsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfsd {
        match self.bits {
            0 => Usbfsd::EnumSP,
            1 => Usbfsd::EnumSNp,
            2 => Usbfsd::EnumNsP,
            3 => Usbfsd::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Usbfsd::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Usbfsd::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Usbfsd::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Usbfsd::EnumNsNp
    }
}
#[doc = "Field `USBFSD` writer - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
pub type UsbfsdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbfsd, crate::Safe>;
impl<'a, REG> UsbfsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumNsNp)
    }
}
#[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdma0 {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Sdma0> for u8 {
    #[inline(always)]
    fn from(variant: Sdma0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdma0 {
    type Ux = u8;
}
impl crate::IsEnum for Sdma0 {}
#[doc = "Field `SDMA0` reader - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub type Sdma0R = crate::FieldReader<Sdma0>;
impl Sdma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma0 {
        match self.bits {
            0 => Sdma0::EnumSP,
            1 => Sdma0::EnumSNp,
            2 => Sdma0::EnumNsP,
            3 => Sdma0::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdma0::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdma0::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdma0::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdma0::EnumNsNp
    }
}
#[doc = "Field `SDMA0` writer - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub type Sdma0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdma0, crate::Safe>;
impl<'a, REG> Sdma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumNsNp)
    }
}
#[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hash {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Hash> for u8 {
    #[inline(always)]
    fn from(variant: Hash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hash {
    type Ux = u8;
}
impl crate::IsEnum for Hash {}
#[doc = "Field `HASH` reader - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub type HashR = crate::FieldReader<Hash>;
impl HashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hash {
        match self.bits {
            0 => Hash::EnumSP,
            1 => Hash::EnumSNp,
            2 => Hash::EnumNsP,
            3 => Hash::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Hash::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Hash::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Hash::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Hash::EnumNsNp
    }
}
#[doc = "Field `HASH` writer - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub type HashW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hash, crate::Safe>;
impl<'a, REG> HashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumNsNp)
    }
}
#[doc = "USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbfsh {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Usbfsh> for u8 {
    #[inline(always)]
    fn from(variant: Usbfsh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbfsh {
    type Ux = u8;
}
impl crate::IsEnum for Usbfsh {}
#[doc = "Field `USBFSH` reader - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
pub type UsbfshR = crate::FieldReader<Usbfsh>;
impl UsbfshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfsh {
        match self.bits {
            0 => Usbfsh::EnumSP,
            1 => Usbfsh::EnumSNp,
            2 => Usbfsh::EnumNsP,
            3 => Usbfsh::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Usbfsh::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Usbfsh::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Usbfsh::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Usbfsh::EnumNsNp
    }
}
#[doc = "Field `USBFSH` writer - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
pub type UsbfshW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbfsh, crate::Safe>;
impl<'a, REG> UsbfshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumNsNp)
    }
}
#[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdma1 {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Sdma1> for u8 {
    #[inline(always)]
    fn from(variant: Sdma1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdma1 {
    type Ux = u8;
}
impl crate::IsEnum for Sdma1 {}
#[doc = "Field `SDMA1` reader - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub type Sdma1R = crate::FieldReader<Sdma1>;
impl Sdma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma1 {
        match self.bits {
            0 => Sdma1::EnumSP,
            1 => Sdma1::EnumSNp,
            2 => Sdma1::EnumNsP,
            3 => Sdma1::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdma1::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdma1::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdma1::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdma1::EnumNsNp
    }
}
#[doc = "Field `SDMA1` writer - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub type Sdma1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdma1, crate::Safe>;
impl<'a, REG> Sdma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumNsNp)
    }
}
#[doc = "CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Canfd {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Canfd> for u8 {
    #[inline(always)]
    fn from(variant: Canfd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Canfd {
    type Ux = u8;
}
impl crate::IsEnum for Canfd {}
#[doc = "Field `CANFD` reader - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
pub type CanfdR = crate::FieldReader<Canfd>;
impl CanfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Canfd {
        match self.bits {
            0 => Canfd::EnumSP,
            1 => Canfd::EnumSNp,
            2 => Canfd::EnumNsP,
            3 => Canfd::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Canfd::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Canfd::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Canfd::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Canfd::EnumNsNp
    }
}
#[doc = "Field `CANFD` writer - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
pub type CanfdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Canfd, crate::Safe>;
impl<'a, REG> CanfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Canfd::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Canfd::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Canfd::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Canfd::EnumNsNp)
    }
}
#[doc = "MASTER_SEC_ANTI_POL_REG register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MasterSecLevelAntipolLock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<MasterSecLevelAntipolLock> for u8 {
    #[inline(always)]
    fn from(variant: MasterSecLevelAntipolLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MasterSecLevelAntipolLock {
    type Ux = u8;
}
impl crate::IsEnum for MasterSecLevelAntipolLock {}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` reader - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub type MasterSecLevelAntipolLockR = crate::FieldReader<MasterSecLevelAntipolLock>;
impl MasterSecLevelAntipolLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MasterSecLevelAntipolLock> {
        match self.bits {
            1 => Some(MasterSecLevelAntipolLock::Blocked),
            2 => Some(MasterSecLevelAntipolLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MasterSecLevelAntipolLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == MasterSecLevelAntipolLock::Writable
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` writer - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub type MasterSecLevelAntipolLockW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, MasterSecLevelAntipolLock>;
impl<'a, REG> MasterSecLevelAntipolLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelAntipolLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelAntipolLock::Writable)
    }
}
impl R {
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline(always)]
    pub fn usbfsd(&self) -> UsbfsdR {
        UsbfsdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&self) -> Sdma0R {
        Sdma0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&self) -> HashR {
        HashR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline(always)]
    pub fn usbfsh(&self) -> UsbfshR {
        UsbfshR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&self) -> Sdma1R {
        Sdma1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
    #[inline(always)]
    pub fn canfd(&self) -> CanfdR {
        CanfdR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(&self) -> MasterSecLevelAntipolLockR {
        MasterSecLevelAntipolLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_SEC_ANTI_POL_REG")
            .field("usbfsd", &self.usbfsd())
            .field("sdma0", &self.sdma0())
            .field("hash", &self.hash())
            .field("usbfsh", &self.usbfsh())
            .field("sdma1", &self.sdma1())
            .field("canfd", &self.canfd())
            .field(
                "master_sec_level_antipol_lock",
                &self.master_sec_level_antipol_lock(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline(always)]
    pub fn usbfsd(&mut self) -> UsbfsdW<'_, MasterSecAntiPolRegSpec> {
        UsbfsdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&mut self) -> Sdma0W<'_, MasterSecAntiPolRegSpec> {
        Sdma0W::new(self, 10)
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&mut self) -> HashW<'_, MasterSecAntiPolRegSpec> {
        HashW::new(self, 20)
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline(always)]
    pub fn usbfsh(&mut self) -> UsbfshW<'_, MasterSecAntiPolRegSpec> {
        UsbfshW::new(self, 22)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&mut self) -> Sdma1W<'_, MasterSecAntiPolRegSpec> {
        Sdma1W::new(self, 24)
    }
    #[doc = "Bits 26:27 - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
    #[inline(always)]
    pub fn canfd(&mut self) -> CanfdW<'_, MasterSecAntiPolRegSpec> {
        CanfdW::new(self, 26)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(
        &mut self,
    ) -> MasterSecLevelAntipolLockW<'_, MasterSecAntiPolRegSpec> {
        MasterSecLevelAntipolLockW::new(self, 30)
    }
}
#[doc = "master secure level anti-pole register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_anti_pol_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_anti_pol_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterSecAntiPolRegSpec;
impl crate::RegisterSpec for MasterSecAntiPolRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_sec_anti_pol_reg::R`](R) reader structure"]
impl crate::Readable for MasterSecAntiPolRegSpec {}
#[doc = "`write(|w| ..)` method takes [`master_sec_anti_pol_reg::W`](W) writer structure"]
impl crate::Writable for MasterSecAntiPolRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASTER_SEC_ANTI_POL_REG to value 0xbfff_ffff"]
impl crate::Resettable for MasterSecAntiPolRegSpec {
    const RESET_VALUE: u32 = 0xbfff_ffff;
}

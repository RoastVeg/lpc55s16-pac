#[doc = "Register `SEC_VIO_INFO_VALID` reader"]
pub type R = crate::R<SecVioInfoValidSpec>;
#[doc = "Register `SEC_VIO_INFO_VALID` writer"]
pub type W = crate::W<SecVioInfoValidSpec>;
#[doc = "violation information valid flag for AHB port 0. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid0 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid0> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID0` reader - violation information valid flag for AHB port 0. Write 1 to clear."]
pub type VioInfoValid0R = crate::BitReader<VioInfoValid0>;
impl VioInfoValid0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid0 {
        match self.bits {
            false => VioInfoValid0::NotValid,
            true => VioInfoValid0::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid0::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid0::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID0` writer - violation information valid flag for AHB port 0. Write 1 to clear."]
pub type VioInfoValid0W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid0>;
impl<'a, REG> VioInfoValid0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid0::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid0::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 1. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid1 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid1> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID1` reader - violation information valid flag for AHB port 1. Write 1 to clear."]
pub type VioInfoValid1R = crate::BitReader<VioInfoValid1>;
impl VioInfoValid1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid1 {
        match self.bits {
            false => VioInfoValid1::NotValid,
            true => VioInfoValid1::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid1::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid1::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID1` writer - violation information valid flag for AHB port 1. Write 1 to clear."]
pub type VioInfoValid1W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid1>;
impl<'a, REG> VioInfoValid1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid1::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid1::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 2. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid2 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid2> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID2` reader - violation information valid flag for AHB port 2. Write 1 to clear."]
pub type VioInfoValid2R = crate::BitReader<VioInfoValid2>;
impl VioInfoValid2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid2 {
        match self.bits {
            false => VioInfoValid2::NotValid,
            true => VioInfoValid2::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid2::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid2::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID2` writer - violation information valid flag for AHB port 2. Write 1 to clear."]
pub type VioInfoValid2W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid2>;
impl<'a, REG> VioInfoValid2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid2::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid2::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 3. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid3 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid3> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID3` reader - violation information valid flag for AHB port 3. Write 1 to clear."]
pub type VioInfoValid3R = crate::BitReader<VioInfoValid3>;
impl VioInfoValid3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid3 {
        match self.bits {
            false => VioInfoValid3::NotValid,
            true => VioInfoValid3::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid3::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid3::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID3` writer - violation information valid flag for AHB port 3. Write 1 to clear."]
pub type VioInfoValid3W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid3>;
impl<'a, REG> VioInfoValid3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid3::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid3::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 4. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid4 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid4> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID4` reader - violation information valid flag for AHB port 4. Write 1 to clear."]
pub type VioInfoValid4R = crate::BitReader<VioInfoValid4>;
impl VioInfoValid4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid4 {
        match self.bits {
            false => VioInfoValid4::NotValid,
            true => VioInfoValid4::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid4::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid4::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID4` writer - violation information valid flag for AHB port 4. Write 1 to clear."]
pub type VioInfoValid4W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid4>;
impl<'a, REG> VioInfoValid4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid4::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid4::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 5. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid5 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid5> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID5` reader - violation information valid flag for AHB port 5. Write 1 to clear."]
pub type VioInfoValid5R = crate::BitReader<VioInfoValid5>;
impl VioInfoValid5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid5 {
        match self.bits {
            false => VioInfoValid5::NotValid,
            true => VioInfoValid5::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid5::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid5::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID5` writer - violation information valid flag for AHB port 5. Write 1 to clear."]
pub type VioInfoValid5W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid5>;
impl<'a, REG> VioInfoValid5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid5::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid5::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 6. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid6 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid6> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID6` reader - violation information valid flag for AHB port 6. Write 1 to clear."]
pub type VioInfoValid6R = crate::BitReader<VioInfoValid6>;
impl VioInfoValid6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid6 {
        match self.bits {
            false => VioInfoValid6::NotValid,
            true => VioInfoValid6::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid6::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid6::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID6` writer - violation information valid flag for AHB port 6. Write 1 to clear."]
pub type VioInfoValid6W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid6>;
impl<'a, REG> VioInfoValid6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid6::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid6::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 7. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid7 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid7> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID7` reader - violation information valid flag for AHB port 7. Write 1 to clear."]
pub type VioInfoValid7R = crate::BitReader<VioInfoValid7>;
impl VioInfoValid7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid7 {
        match self.bits {
            false => VioInfoValid7::NotValid,
            true => VioInfoValid7::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid7::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid7::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID7` writer - violation information valid flag for AHB port 7. Write 1 to clear."]
pub type VioInfoValid7W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid7>;
impl<'a, REG> VioInfoValid7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid7::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid7::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 8. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid8 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid8> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID8` reader - violation information valid flag for AHB port 8. Write 1 to clear."]
pub type VioInfoValid8R = crate::BitReader<VioInfoValid8>;
impl VioInfoValid8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid8 {
        match self.bits {
            false => VioInfoValid8::NotValid,
            true => VioInfoValid8::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid8::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid8::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID8` writer - violation information valid flag for AHB port 8. Write 1 to clear."]
pub type VioInfoValid8W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid8>;
impl<'a, REG> VioInfoValid8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid8::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid8::Valid)
    }
}
#[doc = "violation information valid flag for AHB port 9. Write 1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioInfoValid9 {
    #[doc = "0: Not valid."]
    NotValid = 0,
    #[doc = "1: Valid (violation occurred)."]
    Valid = 1,
}
impl From<VioInfoValid9> for bool {
    #[inline(always)]
    fn from(variant: VioInfoValid9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_INFO_VALID9` reader - violation information valid flag for AHB port 9. Write 1 to clear."]
pub type VioInfoValid9R = crate::BitReader<VioInfoValid9>;
impl VioInfoValid9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioInfoValid9 {
        match self.bits {
            false => VioInfoValid9::NotValid,
            true => VioInfoValid9::Valid,
        }
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VioInfoValid9::NotValid
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VioInfoValid9::Valid
    }
}
#[doc = "Field `VIO_INFO_VALID9` writer - violation information valid flag for AHB port 9. Write 1 to clear."]
pub type VioInfoValid9W<'a, REG> = crate::BitWriter<'a, REG, VioInfoValid9>;
impl<'a, REG> VioInfoValid9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid9::NotValid)
    }
    #[doc = "Valid (violation occurred)."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(VioInfoValid9::Valid)
    }
}
impl R {
    #[doc = "Bit 0 - violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid0(&self) -> VioInfoValid0R {
        VioInfoValid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid1(&self) -> VioInfoValid1R {
        VioInfoValid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid2(&self) -> VioInfoValid2R {
        VioInfoValid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid3(&self) -> VioInfoValid3R {
        VioInfoValid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid4(&self) -> VioInfoValid4R {
        VioInfoValid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid5(&self) -> VioInfoValid5R {
        VioInfoValid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid6(&self) -> VioInfoValid6R {
        VioInfoValid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid7(&self) -> VioInfoValid7R {
        VioInfoValid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid8(&self) -> VioInfoValid8R {
        VioInfoValid8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid9(&self) -> VioInfoValid9R {
        VioInfoValid9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_VIO_INFO_VALID")
            .field("vio_info_valid0", &self.vio_info_valid0())
            .field("vio_info_valid1", &self.vio_info_valid1())
            .field("vio_info_valid2", &self.vio_info_valid2())
            .field("vio_info_valid3", &self.vio_info_valid3())
            .field("vio_info_valid4", &self.vio_info_valid4())
            .field("vio_info_valid5", &self.vio_info_valid5())
            .field("vio_info_valid6", &self.vio_info_valid6())
            .field("vio_info_valid7", &self.vio_info_valid7())
            .field("vio_info_valid8", &self.vio_info_valid8())
            .field("vio_info_valid9", &self.vio_info_valid9())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - violation information valid flag for AHB port 0. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid0(&mut self) -> VioInfoValid0W<'_, SecVioInfoValidSpec> {
        VioInfoValid0W::new(self, 0)
    }
    #[doc = "Bit 1 - violation information valid flag for AHB port 1. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid1(&mut self) -> VioInfoValid1W<'_, SecVioInfoValidSpec> {
        VioInfoValid1W::new(self, 1)
    }
    #[doc = "Bit 2 - violation information valid flag for AHB port 2. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid2(&mut self) -> VioInfoValid2W<'_, SecVioInfoValidSpec> {
        VioInfoValid2W::new(self, 2)
    }
    #[doc = "Bit 3 - violation information valid flag for AHB port 3. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid3(&mut self) -> VioInfoValid3W<'_, SecVioInfoValidSpec> {
        VioInfoValid3W::new(self, 3)
    }
    #[doc = "Bit 4 - violation information valid flag for AHB port 4. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid4(&mut self) -> VioInfoValid4W<'_, SecVioInfoValidSpec> {
        VioInfoValid4W::new(self, 4)
    }
    #[doc = "Bit 5 - violation information valid flag for AHB port 5. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid5(&mut self) -> VioInfoValid5W<'_, SecVioInfoValidSpec> {
        VioInfoValid5W::new(self, 5)
    }
    #[doc = "Bit 6 - violation information valid flag for AHB port 6. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid6(&mut self) -> VioInfoValid6W<'_, SecVioInfoValidSpec> {
        VioInfoValid6W::new(self, 6)
    }
    #[doc = "Bit 7 - violation information valid flag for AHB port 7. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid7(&mut self) -> VioInfoValid7W<'_, SecVioInfoValidSpec> {
        VioInfoValid7W::new(self, 7)
    }
    #[doc = "Bit 8 - violation information valid flag for AHB port 8. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid8(&mut self) -> VioInfoValid8W<'_, SecVioInfoValidSpec> {
        VioInfoValid8W::new(self, 8)
    }
    #[doc = "Bit 9 - violation information valid flag for AHB port 9. Write 1 to clear."]
    #[inline(always)]
    pub fn vio_info_valid9(&mut self) -> VioInfoValid9W<'_, SecVioInfoValidSpec> {
        VioInfoValid9W::new(self, 9)
    }
}
#[doc = "security violation address/information registers valid flags\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_info_valid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_vio_info_valid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecVioInfoValidSpec;
impl crate::RegisterSpec for SecVioInfoValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_vio_info_valid::R`](R) reader structure"]
impl crate::Readable for SecVioInfoValidSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_vio_info_valid::W`](W) writer structure"]
impl crate::Writable for SecVioInfoValidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_VIO_INFO_VALID to value 0"]
impl crate::Resettable for SecVioInfoValidSpec {}

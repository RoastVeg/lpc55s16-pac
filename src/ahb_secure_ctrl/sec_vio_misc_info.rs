#[doc = "Register `sec_vio_misc_info[%s]` reader"]
pub type R = crate::R<SecVioMiscInfoSpec>;
#[doc = "security violation access read/write indicator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecVioInfoWrite {
    #[doc = "0: Read access."]
    Read = 0,
    #[doc = "1: Write access."]
    Write = 1,
}
impl From<SecVioInfoWrite> for bool {
    #[inline(always)]
    fn from(variant: SecVioInfoWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_VIO_INFO_WRITE` reader - security violation access read/write indicator."]
pub type SecVioInfoWriteR = crate::BitReader<SecVioInfoWrite>;
impl SecVioInfoWriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecVioInfoWrite {
        match self.bits {
            false => SecVioInfoWrite::Read,
            true => SecVioInfoWrite::Write,
        }
    }
    #[doc = "Read access."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == SecVioInfoWrite::Read
    }
    #[doc = "Write access."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == SecVioInfoWrite::Write
    }
}
#[doc = "security violation access data/code indicator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecVioInfoDataAccess {
    #[doc = "0: Code access."]
    Code = 0,
    #[doc = "1: Data access."]
    Data = 1,
}
impl From<SecVioInfoDataAccess> for bool {
    #[inline(always)]
    fn from(variant: SecVioInfoDataAccess) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_VIO_INFO_DATA_ACCESS` reader - security violation access data/code indicator."]
pub type SecVioInfoDataAccessR = crate::BitReader<SecVioInfoDataAccess>;
impl SecVioInfoDataAccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecVioInfoDataAccess {
        match self.bits {
            false => SecVioInfoDataAccess::Code,
            true => SecVioInfoDataAccess::Data,
        }
    }
    #[doc = "Code access."]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        *self == SecVioInfoDataAccess::Code
    }
    #[doc = "Data access."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == SecVioInfoDataAccess::Data
    }
}
#[doc = "Field `SEC_VIO_INFO_MASTER_SEC_LEVEL` reader - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
pub type SecVioInfoMasterSecLevelR = crate::FieldReader;
#[doc = "security violation master number\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecVioInfoMaster {
    #[doc = "0: CPU0 Code."]
    Value0 = 0,
    #[doc = "1: CPU0 System."]
    Value1 = 1,
    #[doc = "4: USB-HS Device."]
    Value4 = 4,
    #[doc = "5: SDMA0."]
    Value5 = 5,
    #[doc = "10: HASH."]
    Value10 = 10,
    #[doc = "11: USB-FS Host."]
    Value11 = 11,
    #[doc = "12: SDMA1."]
    Value12 = 12,
    #[doc = "13: CAN-FD."]
    Value13 = 13,
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(variant: SecVioInfoMaster) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecVioInfoMaster {
    type Ux = u8;
}
impl crate::IsEnum for SecVioInfoMaster {}
#[doc = "Field `SEC_VIO_INFO_MASTER` reader - security violation master number"]
pub type SecVioInfoMasterR = crate::FieldReader<SecVioInfoMaster>;
impl SecVioInfoMasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecVioInfoMaster> {
        match self.bits {
            0 => Some(SecVioInfoMaster::Value0),
            1 => Some(SecVioInfoMaster::Value1),
            4 => Some(SecVioInfoMaster::Value4),
            5 => Some(SecVioInfoMaster::Value5),
            10 => Some(SecVioInfoMaster::Value10),
            11 => Some(SecVioInfoMaster::Value11),
            12 => Some(SecVioInfoMaster::Value12),
            13 => Some(SecVioInfoMaster::Value13),
            _ => None,
        }
    }
    #[doc = "CPU0 Code."]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SecVioInfoMaster::Value0
    }
    #[doc = "CPU0 System."]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SecVioInfoMaster::Value1
    }
    #[doc = "USB-HS Device."]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == SecVioInfoMaster::Value4
    }
    #[doc = "SDMA0."]
    #[inline(always)]
    pub fn is_value_5(&self) -> bool {
        *self == SecVioInfoMaster::Value5
    }
    #[doc = "HASH."]
    #[inline(always)]
    pub fn is_value_10(&self) -> bool {
        *self == SecVioInfoMaster::Value10
    }
    #[doc = "USB-FS Host."]
    #[inline(always)]
    pub fn is_value_11(&self) -> bool {
        *self == SecVioInfoMaster::Value11
    }
    #[doc = "SDMA1."]
    #[inline(always)]
    pub fn is_value_12(&self) -> bool {
        *self == SecVioInfoMaster::Value12
    }
    #[doc = "CAN-FD."]
    #[inline(always)]
    pub fn is_value_13(&self) -> bool {
        *self == SecVioInfoMaster::Value13
    }
}
impl R {
    #[doc = "Bit 0 - security violation access read/write indicator."]
    #[inline(always)]
    pub fn sec_vio_info_write(&self) -> SecVioInfoWriteR {
        SecVioInfoWriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - security violation access data/code indicator."]
    #[inline(always)]
    pub fn sec_vio_info_data_access(&self) -> SecVioInfoDataAccessR {
        SecVioInfoDataAccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[inline(always)]
    pub fn sec_vio_info_master_sec_level(&self) -> SecVioInfoMasterSecLevelR {
        SecVioInfoMasterSecLevelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - security violation master number"]
    #[inline(always)]
    pub fn sec_vio_info_master(&self) -> SecVioInfoMasterR {
        SecVioInfoMasterR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sec_vio_misc_info")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[doc = "most recent security violation miscellaneous information for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_misc_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecVioMiscInfoSpec;
impl crate::RegisterSpec for SecVioMiscInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_vio_misc_info::R`](R) reader structure"]
impl crate::Readable for SecVioMiscInfoSpec {}
#[doc = "`reset()` method sets sec_vio_misc_info[%s] to value 0"]
impl crate::Resettable for SecVioMiscInfoSpec {}

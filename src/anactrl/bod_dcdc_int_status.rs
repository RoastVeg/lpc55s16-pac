#[doc = "Register `BOD_DCDC_INT_STATUS` reader"]
pub type R = crate::R<BodDcdcIntStatusSpec>;
#[doc = "BOD VBAT Interrupt status before Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodvbatStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<BodvbatStatus> for bool {
    #[inline(always)]
    fn from(variant: BodvbatStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBAT_STATUS` reader - BOD VBAT Interrupt status before Interrupt Enable."]
pub type BodvbatStatusR = crate::BitReader<BodvbatStatus>;
impl BodvbatStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodvbatStatus {
        match self.bits {
            false => BodvbatStatus::NotPending,
            true => BodvbatStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BodvbatStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BodvbatStatus::Pending
    }
}
#[doc = "BOD VBAT Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodvbatIntStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<BodvbatIntStatus> for bool {
    #[inline(always)]
    fn from(variant: BodvbatIntStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBAT_INT_STATUS` reader - BOD VBAT Interrupt status after Interrupt Enable."]
pub type BodvbatIntStatusR = crate::BitReader<BodvbatIntStatus>;
impl BodvbatIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodvbatIntStatus {
        match self.bits {
            false => BodvbatIntStatus::NotPending,
            true => BodvbatIntStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BodvbatIntStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BodvbatIntStatus::Pending
    }
}
#[doc = "Current value of BOD VBAT power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodvbatVal {
    #[doc = "0: VBAT voltage level is below the threshold."]
    NotOk = 0,
    #[doc = "1: VBAT voltage level is above the threshold."]
    Ok = 1,
}
impl From<BodvbatVal> for bool {
    #[inline(always)]
    fn from(variant: BodvbatVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBAT_VAL` reader - Current value of BOD VBAT power status output."]
pub type BodvbatValR = crate::BitReader<BodvbatVal>;
impl BodvbatValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodvbatVal {
        match self.bits {
            false => BodvbatVal::NotOk,
            true => BodvbatVal::Ok,
        }
    }
    #[doc = "VBAT voltage level is below the threshold."]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BodvbatVal::NotOk
    }
    #[doc = "VBAT voltage level is above the threshold."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BodvbatVal::Ok
    }
}
#[doc = "BOD CORE Interrupt status before Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodcoreStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<BodcoreStatus> for bool {
    #[inline(always)]
    fn from(variant: BodcoreStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORE_STATUS` reader - BOD CORE Interrupt status before Interrupt Enable."]
pub type BodcoreStatusR = crate::BitReader<BodcoreStatus>;
impl BodcoreStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodcoreStatus {
        match self.bits {
            false => BodcoreStatus::NotPending,
            true => BodcoreStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BodcoreStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BodcoreStatus::Pending
    }
}
#[doc = "BOD CORE Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodcoreIntStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<BodcoreIntStatus> for bool {
    #[inline(always)]
    fn from(variant: BodcoreIntStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORE_INT_STATUS` reader - BOD CORE Interrupt status after Interrupt Enable."]
pub type BodcoreIntStatusR = crate::BitReader<BodcoreIntStatus>;
impl BodcoreIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodcoreIntStatus {
        match self.bits {
            false => BodcoreIntStatus::NotPending,
            true => BodcoreIntStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BodcoreIntStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BodcoreIntStatus::Pending
    }
}
#[doc = "Current value of BOD CORE power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodcoreVal {
    #[doc = "0: CORE voltage level is below the threshold."]
    NotOk = 0,
    #[doc = "1: CORE voltage level is above the threshold."]
    Ok = 1,
}
impl From<BodcoreVal> for bool {
    #[inline(always)]
    fn from(variant: BodcoreVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORE_VAL` reader - Current value of BOD CORE power status output."]
pub type BodcoreValR = crate::BitReader<BodcoreVal>;
impl BodcoreValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BodcoreVal {
        match self.bits {
            false => BodcoreVal::NotOk,
            true => BodcoreVal::Ok,
        }
    }
    #[doc = "CORE voltage level is below the threshold."]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BodcoreVal::NotOk
    }
    #[doc = "CORE voltage level is above the threshold."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BodcoreVal::Ok
    }
}
#[doc = "DCDC Interrupt status before Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<DcdcStatus> for bool {
    #[inline(always)]
    fn from(variant: DcdcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_STATUS` reader - DCDC Interrupt status before Interrupt Enable."]
pub type DcdcStatusR = crate::BitReader<DcdcStatus>;
impl DcdcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcStatus {
        match self.bits {
            false => DcdcStatus::NotPending,
            true => DcdcStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DcdcStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DcdcStatus::Pending
    }
}
#[doc = "DCDC Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcIntStatus {
    #[doc = "0: No interrupt pending.."]
    NotPending = 0,
    #[doc = "1: Interrupt pending.."]
    Pending = 1,
}
impl From<DcdcIntStatus> for bool {
    #[inline(always)]
    fn from(variant: DcdcIntStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_INT_STATUS` reader - DCDC Interrupt status after Interrupt Enable."]
pub type DcdcIntStatusR = crate::BitReader<DcdcIntStatus>;
impl DcdcIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcIntStatus {
        match self.bits {
            false => DcdcIntStatus::NotPending,
            true => DcdcIntStatus::Pending,
        }
    }
    #[doc = "No interrupt pending.."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DcdcIntStatus::NotPending
    }
    #[doc = "Interrupt pending.."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DcdcIntStatus::Pending
    }
}
#[doc = "Current value of DCDC power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcVal {
    #[doc = "0: DCDC output Voltage is below the targeted regulation level."]
    NotOk = 0,
    #[doc = "1: DCDC output Voltage is above the targeted regulation level."]
    Ok = 1,
}
impl From<DcdcVal> for bool {
    #[inline(always)]
    fn from(variant: DcdcVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_VAL` reader - Current value of DCDC power status output."]
pub type DcdcValR = crate::BitReader<DcdcVal>;
impl DcdcValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcVal {
        match self.bits {
            false => DcdcVal::NotOk,
            true => DcdcVal::Ok,
        }
    }
    #[doc = "DCDC output Voltage is below the targeted regulation level."]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == DcdcVal::NotOk
    }
    #[doc = "DCDC output Voltage is above the targeted regulation level."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == DcdcVal::Ok
    }
}
impl R {
    #[doc = "Bit 0 - BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_status(&self) -> BodvbatStatusR {
        BodvbatStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_int_status(&self) -> BodvbatIntStatusR {
        BodvbatIntStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current value of BOD VBAT power status output."]
    #[inline(always)]
    pub fn bodvbat_val(&self) -> BodvbatValR {
        BodvbatValR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD CORE Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_status(&self) -> BodcoreStatusR {
        BodcoreStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD CORE Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_int_status(&self) -> BodcoreIntStatusR {
        BodcoreIntStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Current value of BOD CORE power status output."]
    #[inline(always)]
    pub fn bodcore_val(&self) -> BodcoreValR {
        BodcoreValR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCDC Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_status(&self) -> DcdcStatusR {
        DcdcStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDC Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_int_status(&self) -> DcdcIntStatusR {
        DcdcIntStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current value of DCDC power status output."]
    #[inline(always)]
    pub fn dcdc_val(&self) -> DcdcValR {
        DcdcValR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "BoDs & DCDC interrupts status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bod_dcdc_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodDcdcIntStatusSpec;
impl crate::RegisterSpec for BodDcdcIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod_dcdc_int_status::R`](R) reader structure"]
impl crate::Readable for BodDcdcIntStatusSpec {}
#[doc = "`reset()` method sets BOD_DCDC_INT_STATUS to value 0x012d"]
impl crate::Resettable for BodDcdcIntStatusSpec {
    const RESET_VALUE: u32 = 0x012d;
}

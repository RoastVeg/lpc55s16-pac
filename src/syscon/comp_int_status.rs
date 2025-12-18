#[doc = "Register `COMP_INT_STATUS` reader"]
pub type R = crate::R<CompIntStatusSpec>;
#[doc = "Register `COMP_INT_STATUS` writer"]
pub type W = crate::W<CompIntStatusSpec>;
#[doc = "Interrupt status BEFORE Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: no interrupt pending."]
    NoInt = 0,
    #[doc = "1: interrupt pending."]
    Pending = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Interrupt status BEFORE Interrupt Enable."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::NoInt,
            true => Status::Pending,
        }
    }
    #[doc = "no interrupt pending."]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        *self == Status::NoInt
    }
    #[doc = "interrupt pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Status::Pending
    }
}
#[doc = "Interrupt status AFTER Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntStatus {
    #[doc = "0: no interrupt pending."]
    NoInt = 0,
    #[doc = "1: interrupt pending."]
    Pending = 1,
}
impl From<IntStatus> for bool {
    #[inline(always)]
    fn from(variant: IntStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_STATUS` reader - Interrupt status AFTER Interrupt Enable."]
pub type IntStatusR = crate::BitReader<IntStatus>;
impl IntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntStatus {
        match self.bits {
            false => IntStatus::NoInt,
            true => IntStatus::Pending,
        }
    }
    #[doc = "no interrupt pending."]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        *self == IntStatus::NoInt
    }
    #[doc = "interrupt pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IntStatus::Pending
    }
}
#[doc = "comparator analog output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "0: P+ is smaller than P-."]
    Smaller = 0,
    #[doc = "1: P+ is greater than P-."]
    Greater = 1,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - comparator analog output."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            false => Val::Smaller,
            true => Val::Greater,
        }
    }
    #[doc = "P+ is smaller than P-."]
    #[inline(always)]
    pub fn is_smaller(&self) -> bool {
        *self == Val::Smaller
    }
    #[doc = "P+ is greater than P-."]
    #[inline(always)]
    pub fn is_greater(&self) -> bool {
        *self == Val::Greater
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status BEFORE Interrupt Enable."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status AFTER Interrupt Enable."]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - comparator analog output."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "Comparator Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompIntStatusSpec;
impl crate::RegisterSpec for CompIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_int_status::R`](R) reader structure"]
impl crate::Readable for CompIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_int_status::W`](W) writer structure"]
impl crate::Writable for CompIntStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_INT_STATUS to value 0"]
impl crate::Resettable for CompIntStatusSpec {}

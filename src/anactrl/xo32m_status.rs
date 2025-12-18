#[doc = "Register `XO32M_STATUS` reader"]
pub type R = crate::R<Xo32mStatusSpec>;
#[doc = "Indicates XO out frequency statibilty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XoReady {
    #[doc = "0: XO output frequency is not yet stable."]
    NotStable = 0,
    #[doc = "1: XO output frequency is stable."]
    Stable = 1,
}
impl From<XoReady> for bool {
    #[inline(always)]
    fn from(variant: XoReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XO_READY` reader - Indicates XO out frequency statibilty."]
pub type XoReadyR = crate::BitReader<XoReady>;
impl XoReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XoReady {
        match self.bits {
            false => XoReady::NotStable,
            true => XoReady::Stable,
        }
    }
    #[doc = "XO output frequency is not yet stable."]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == XoReady::NotStable
    }
    #[doc = "XO output frequency is stable."]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == XoReady::Stable
    }
}
impl R {
    #[doc = "Bit 0 - Indicates XO out frequency statibilty."]
    #[inline(always)]
    pub fn xo_ready(&self) -> XoReadyR {
        XoReadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "High speed Crystal Oscillator Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32m_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xo32mStatusSpec;
impl crate::RegisterSpec for Xo32mStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xo32m_status::R`](R) reader structure"]
impl crate::Readable for Xo32mStatusSpec {}
#[doc = "`reset()` method sets XO32M_STATUS to value 0"]
impl crate::Resettable for Xo32mStatusSpec {}

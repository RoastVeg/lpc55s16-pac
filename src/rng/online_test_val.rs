#[doc = "Register `ONLINE_TEST_VAL` reader"]
pub type R = crate::R<OnlineTestValSpec>;
#[doc = "Register `ONLINE_TEST_VAL` writer"]
pub type W = crate::W<OnlineTestValSpec>;
#[doc = "Field `LIVE_CHI_SQUARED` reader - This value is updated as described in field 'activate'."]
pub type LiveChiSquaredR = crate::FieldReader;
#[doc = "Field `MIN_CHI_SQUARED` reader - This field is reset when 'activate'==0."]
pub type MinChiSquaredR = crate::FieldReader;
#[doc = "Field `MAX_CHI_SQUARED` reader - This field is reset when 'activate'==0."]
pub type MaxChiSquaredR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - This value is updated as described in field 'activate'."]
    #[inline(always)]
    pub fn live_chi_squared(&self) -> LiveChiSquaredR {
        LiveChiSquaredR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn min_chi_squared(&self) -> MinChiSquaredR {
        MinChiSquaredR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn max_chi_squared(&self) -> MaxChiSquaredR {
        MaxChiSquaredR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnlineTestValSpec;
impl crate::RegisterSpec for OnlineTestValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`online_test_val::R`](R) reader structure"]
impl crate::Readable for OnlineTestValSpec {}
#[doc = "`write(|w| ..)` method takes [`online_test_val::W`](W) writer structure"]
impl crate::Writable for OnlineTestValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ONLINE_TEST_VAL to value 0"]
impl crate::Resettable for OnlineTestValSpec {}

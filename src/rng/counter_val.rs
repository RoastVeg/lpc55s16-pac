#[doc = "Register `COUNTER_VAL` reader"]
pub type R = crate::R<CounterValSpec>;
#[doc = "Register `COUNTER_VAL` writer"]
pub type W = crate::W<CounterValSpec>;
#[doc = "Field `CLK_RATIO` reader - Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
pub type ClkRatioR = crate::FieldReader;
#[doc = "Field `REFRESH_CNT` reader - Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
pub type RefreshCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[inline(always)]
    pub fn clk_ratio(&self) -> ClkRatioR {
        ClkRatioR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:21 - Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[inline(always)]
    pub fn refresh_cnt(&self) -> RefreshCntR {
        RefreshCntR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterValSpec;
impl crate::RegisterSpec for CounterValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter_val::R`](R) reader structure"]
impl crate::Readable for CounterValSpec {}
#[doc = "`write(|w| ..)` method takes [`counter_val::W`](W) writer structure"]
impl crate::Writable for CounterValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNTER_VAL to value 0"]
impl crate::Resettable for CounterValSpec {}

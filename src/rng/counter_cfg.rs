#[doc = "Register `COUNTER_CFG` reader"]
pub type R = crate::R<CounterCfgSpec>;
#[doc = "Register `COUNTER_CFG` writer"]
pub type W = crate::W<CounterCfgSpec>;
#[doc = "Field `MODE` reader - 00: disabled 01: update once."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 00: disabled 01: update once."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLOCK_SEL` reader - Selects the internal clock on which to compute statistics."]
pub type ClockSelR = crate::FieldReader;
#[doc = "Field `CLOCK_SEL` writer - Selects the internal clock on which to compute statistics."]
pub type ClockSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SHIFT4X` reader - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub type Shift4xR = crate::FieldReader;
#[doc = "Field `SHIFT4X` writer - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub type Shift4xW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&self) -> ClockSelR {
        ClockSelR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&self) -> Shift4xR {
        Shift4xR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CounterCfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> ClockSelW<'_, CounterCfgSpec> {
        ClockSelW::new(self, 2)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&mut self) -> Shift4xW<'_, CounterCfgSpec> {
        Shift4xW::new(self, 5)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterCfgSpec;
impl crate::RegisterSpec for CounterCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter_cfg::R`](R) reader structure"]
impl crate::Readable for CounterCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`counter_cfg::W`](W) writer structure"]
impl crate::Writable for CounterCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNTER_CFG to value 0"]
impl crate::Resettable for CounterCfgSpec {}

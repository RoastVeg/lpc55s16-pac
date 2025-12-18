#[doc = "Register `ONLINE_TEST_CFG` reader"]
pub type R = crate::R<OnlineTestCfgSpec>;
#[doc = "Register `ONLINE_TEST_CFG` writer"]
pub type W = crate::W<OnlineTestCfgSpec>;
#[doc = "Field `ACTIVATE` reader - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
pub type ActivateR = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
pub type ActivateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SEL` reader - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
pub type DataSelR = crate::FieldReader;
#[doc = "Field `DATA_SEL` writer - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
pub type DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub fn activate(&self) -> ActivateR {
        ActivateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub fn data_sel(&self) -> DataSelR {
        DataSelR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub fn activate(&mut self) -> ActivateW<'_, OnlineTestCfgSpec> {
        ActivateW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub fn data_sel(&mut self) -> DataSelW<'_, OnlineTestCfgSpec> {
        DataSelW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnlineTestCfgSpec;
impl crate::RegisterSpec for OnlineTestCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`online_test_cfg::R`](R) reader structure"]
impl crate::Readable for OnlineTestCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`online_test_cfg::W`](W) writer structure"]
impl crate::Writable for OnlineTestCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ONLINE_TEST_CFG to value 0"]
impl crate::Resettable for OnlineTestCfgSpec {}

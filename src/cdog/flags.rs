#[doc = "Register `FLAGS` reader"]
pub type R = crate::R<FlagsSpec>;
#[doc = "Register `FLAGS` writer"]
pub type W = crate::W<FlagsSpec>;
#[doc = "Field `TO_FLAG` reader - Timeout flag"]
pub type ToFlagR = crate::BitReader;
#[doc = "Field `TO_FLAG` writer - Timeout flag"]
pub type ToFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISCOM_FLAG` reader - Miscompare flag"]
pub type MiscomFlagR = crate::BitReader;
#[doc = "Field `MISCOM_FLAG` writer - Miscompare flag"]
pub type MiscomFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_FLAG` reader - Sequence flag"]
pub type SeqFlagR = crate::BitReader;
#[doc = "Field `SEQ_FLAG` writer - Sequence flag"]
pub type SeqFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_FLAG` reader - Control (fault) flag"]
pub type CntFlagR = crate::BitReader;
#[doc = "Field `CNT_FLAG` writer - Control (fault) flag"]
pub type CntFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATE_FLAG` reader - State flag"]
pub type StateFlagR = crate::BitReader;
#[doc = "Field `STATE_FLAG` writer - State flag"]
pub type StateFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_FLAG` reader - Address flag"]
pub type AddrFlagR = crate::BitReader;
#[doc = "Field `ADDR_FLAG` writer - Address flag"]
pub type AddrFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_FLAG` reader - Power-on reset flag"]
pub type PorFlagR = crate::BitReader;
#[doc = "Field `POR_FLAG` writer - Power-on reset flag"]
pub type PorFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timeout flag"]
    #[inline(always)]
    pub fn to_flag(&self) -> ToFlagR {
        ToFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miscompare flag"]
    #[inline(always)]
    pub fn miscom_flag(&self) -> MiscomFlagR {
        MiscomFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequence flag"]
    #[inline(always)]
    pub fn seq_flag(&self) -> SeqFlagR {
        SeqFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control (fault) flag"]
    #[inline(always)]
    pub fn cnt_flag(&self) -> CntFlagR {
        CntFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - State flag"]
    #[inline(always)]
    pub fn state_flag(&self) -> StateFlagR {
        StateFlagR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address flag"]
    #[inline(always)]
    pub fn addr_flag(&self) -> AddrFlagR {
        AddrFlagR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Power-on reset flag"]
    #[inline(always)]
    pub fn por_flag(&self) -> PorFlagR {
        PorFlagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout flag"]
    #[inline(always)]
    pub fn to_flag(&mut self) -> ToFlagW<'_, FlagsSpec> {
        ToFlagW::new(self, 0)
    }
    #[doc = "Bit 1 - Miscompare flag"]
    #[inline(always)]
    pub fn miscom_flag(&mut self) -> MiscomFlagW<'_, FlagsSpec> {
        MiscomFlagW::new(self, 1)
    }
    #[doc = "Bit 2 - Sequence flag"]
    #[inline(always)]
    pub fn seq_flag(&mut self) -> SeqFlagW<'_, FlagsSpec> {
        SeqFlagW::new(self, 2)
    }
    #[doc = "Bit 3 - Control (fault) flag"]
    #[inline(always)]
    pub fn cnt_flag(&mut self) -> CntFlagW<'_, FlagsSpec> {
        CntFlagW::new(self, 3)
    }
    #[doc = "Bit 4 - State flag"]
    #[inline(always)]
    pub fn state_flag(&mut self) -> StateFlagW<'_, FlagsSpec> {
        StateFlagW::new(self, 4)
    }
    #[doc = "Bit 5 - Address flag"]
    #[inline(always)]
    pub fn addr_flag(&mut self) -> AddrFlagW<'_, FlagsSpec> {
        AddrFlagW::new(self, 5)
    }
    #[doc = "Bit 16 - Power-on reset flag"]
    #[inline(always)]
    pub fn por_flag(&mut self) -> PorFlagW<'_, FlagsSpec> {
        PorFlagW::new(self, 16)
    }
}
#[doc = "Hardware flags\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagsSpec;
impl crate::RegisterSpec for FlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flags::R`](R) reader structure"]
impl crate::Readable for FlagsSpec {}
#[doc = "`write(|w| ..)` method takes [`flags::W`](W) writer structure"]
impl crate::Writable for FlagsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FlagsSpec {}

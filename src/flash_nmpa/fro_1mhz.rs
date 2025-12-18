#[doc = "Register `FRO_1MHZ` reader"]
pub type R = crate::R<Fro1mhzSpec>;
#[doc = "Register `FRO_1MHZ` writer"]
pub type W = crate::W<Fro1mhzSpec>;
#[doc = "Field `FRO1M_TRIM_VALID` reader - no description available"]
pub type Fro1mTrimValidR = crate::BitReader;
#[doc = "Field `FRO1M_TRIM_VALID` writer - no description available"]
pub type Fro1mTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRO1M_FREQSEL` reader - Frequency trimming bits."]
pub type Fro1mFreqselR = crate::FieldReader;
#[doc = "Field `FRO1M_FREQSEL` writer - Frequency trimming bits."]
pub type Fro1mFreqselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro1m_trim_valid(&self) -> Fro1mTrimValidR {
        Fro1mTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Frequency trimming bits."]
    #[inline(always)]
    pub fn fro1m_freqsel(&self) -> Fro1mFreqselR {
        Fro1mFreqselR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro1m_trim_valid(&mut self) -> Fro1mTrimValidW<'_, Fro1mhzSpec> {
        Fro1mTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Frequency trimming bits."]
    #[inline(always)]
    pub fn fro1m_freqsel(&mut self) -> Fro1mFreqselW<'_, Fro1mhzSpec> {
        Fro1mFreqselW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`fro_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fro1mhzSpec;
impl crate::RegisterSpec for Fro1mhzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro_1mhz::R`](R) reader structure"]
impl crate::Readable for Fro1mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`fro_1mhz::W`](W) writer structure"]
impl crate::Writable for Fro1mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRO_1MHZ to value 0"]
impl crate::Resettable for Fro1mhzSpec {}

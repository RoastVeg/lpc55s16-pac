#[doc = "Register `FREQ_ME_CTRL` reader"]
pub type R = crate::R<FreqMeCtrlSpec>;
#[doc = "Register `FREQ_ME_CTRL` writer"]
pub type W = crate::W<FreqMeCtrlSpec>;
#[doc = "Field `CAPVAL_SCALE` reader - Frequency measure result /Frequency measur scale"]
pub type CapvalScaleR = crate::FieldReader<u32>;
#[doc = "Field `CAPVAL_SCALE` writer - Frequency measure result /Frequency measur scale"]
pub type CapvalScaleW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `PROG` reader - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub type ProgR = crate::BitReader;
#[doc = "Field `PROG` writer - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub type ProgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub fn capval_scale(&self) -> CapvalScaleR {
        CapvalScaleR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub fn capval_scale(&mut self) -> CapvalScaleW<'_, FreqMeCtrlSpec> {
        CapvalScaleW::new(self, 0)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub fn prog(&mut self) -> ProgW<'_, FreqMeCtrlSpec> {
        ProgW::new(self, 31)
    }
}
#[doc = "Frequency Measure function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_me_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_me_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqMeCtrlSpec;
impl crate::RegisterSpec for FreqMeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq_me_ctrl::R`](R) reader structure"]
impl crate::Readable for FreqMeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`freq_me_ctrl::W`](W) writer structure"]
impl crate::Writable for FreqMeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREQ_ME_CTRL to value 0"]
impl crate::Resettable for FreqMeCtrlSpec {}

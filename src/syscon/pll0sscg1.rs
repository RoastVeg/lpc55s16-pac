#[doc = "Register `PLL0SSCG1` reader"]
pub type R = crate::R<Pll0sscg1Spec>;
#[doc = "Register `PLL0SSCG1` writer"]
pub type W = crate::W<Pll0sscg1Spec>;
#[doc = "Field `MD_MBS` reader - input word of the wrapper bit 32."]
pub type MdMbsR = crate::BitReader;
#[doc = "Field `MD_MBS` writer - input word of the wrapper bit 32."]
pub type MdMbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD_REQ` reader - md change request."]
pub type MdReqR = crate::BitReader;
#[doc = "Field `MD_REQ` writer - md change request."]
pub type MdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MF` reader - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
pub type MfR = crate::FieldReader;
#[doc = "Field `MF` writer - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
pub type MfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MR` reader - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
pub type MrR = crate::FieldReader;
#[doc = "Field `MR` writer - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
pub type MrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MC` reader - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub type McR = crate::FieldReader;
#[doc = "Field `MC` writer - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MDIV_EXT` reader - to select an external mdiv value."]
pub type MdivExtR = crate::FieldReader<u16>;
#[doc = "Field `MDIV_EXT` writer - to select an external mdiv value."]
pub type MdivExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MREQ` reader - to select an external mreq value."]
pub type MreqR = crate::BitReader;
#[doc = "Field `MREQ` writer - to select an external mreq value."]
pub type MreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHER` reader - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub type DitherR = crate::BitReader;
#[doc = "Field `DITHER` writer - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub type DitherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_EXT` reader - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub type SelExtR = crate::BitReader;
#[doc = "Field `SEL_EXT` writer - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub type SelExtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    pub fn md_mbs(&self) -> MdMbsR {
        MdMbsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    pub fn md_req(&self) -> MdReqR {
        MdReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&self) -> MfR {
        MfR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline(always)]
    pub fn mr(&self) -> MrR {
        MrR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    pub fn mdiv_ext(&self) -> MdivExtR {
        MdivExtR::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    pub fn mreq(&self) -> MreqR {
        MreqR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SelExtR {
        SelExtR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    pub fn md_mbs(&mut self) -> MdMbsW<'_, Pll0sscg1Spec> {
        MdMbsW::new(self, 0)
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    pub fn md_req(&mut self) -> MdReqW<'_, Pll0sscg1Spec> {
        MdReqW::new(self, 1)
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&mut self) -> MfW<'_, Pll0sscg1Spec> {
        MfW::new(self, 2)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline(always)]
    pub fn mr(&mut self) -> MrW<'_, Pll0sscg1Spec> {
        MrW::new(self, 5)
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Pll0sscg1Spec> {
        McW::new(self, 8)
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    pub fn mdiv_ext(&mut self) -> MdivExtW<'_, Pll0sscg1Spec> {
        MdivExtW::new(self, 10)
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MreqW<'_, Pll0sscg1Spec> {
        MreqW::new(self, 26)
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub fn dither(&mut self) -> DitherW<'_, Pll0sscg1Spec> {
        DitherW::new(self, 27)
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> SelExtW<'_, Pll0sscg1Spec> {
        SelExtW::new(self, 28)
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0sscg1Spec;
impl crate::RegisterSpec for Pll0sscg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0sscg1::R`](R) reader structure"]
impl crate::Readable for Pll0sscg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pll0sscg1::W`](W) writer structure"]
impl crate::Writable for Pll0sscg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0SSCG1 to value 0"]
impl crate::Resettable for Pll0sscg1Spec {}

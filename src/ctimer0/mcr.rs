#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `MR0I` reader - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type Mr0iR = crate::BitReader;
#[doc = "Field `MR0I` writer - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type Mr0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR0R` reader - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type Mr0rR = crate::BitReader;
#[doc = "Field `MR0R` writer - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type Mr0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR0S` reader - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
pub type Mr0sR = crate::BitReader;
#[doc = "Field `MR0S` writer - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
pub type Mr0sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1I` reader - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type Mr1iR = crate::BitReader;
#[doc = "Field `MR1I` writer - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type Mr1iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1R` reader - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type Mr1rR = crate::BitReader;
#[doc = "Field `MR1R` writer - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type Mr1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1S` reader - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
pub type Mr1sR = crate::BitReader;
#[doc = "Field `MR1S` writer - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
pub type Mr1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2I` reader - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type Mr2iR = crate::BitReader;
#[doc = "Field `MR2I` writer - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type Mr2iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2R` reader - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type Mr2rR = crate::BitReader;
#[doc = "Field `MR2R` writer - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type Mr2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2S` reader - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
pub type Mr2sR = crate::BitReader;
#[doc = "Field `MR2S` writer - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
pub type Mr2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3I` reader - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type Mr3iR = crate::BitReader;
#[doc = "Field `MR3I` writer - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type Mr3iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3R` reader - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type Mr3rR = crate::BitReader;
#[doc = "Field `MR3R` writer - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type Mr3rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3S` reader - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
pub type Mr3sR = crate::BitReader;
#[doc = "Field `MR3S` writer - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
pub type Mr3sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR0RL` reader - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr0rlR = crate::BitReader;
#[doc = "Field `MR0RL` writer - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr0rlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1RL` reader - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr1rlR = crate::BitReader;
#[doc = "Field `MR1RL` writer - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr1rlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2RL` reader - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr2rlR = crate::BitReader;
#[doc = "Field `MR2RL` writer - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr2rlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3RL` reader - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr3rlR = crate::BitReader;
#[doc = "Field `MR3RL` writer - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type Mr3rlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> Mr0iR {
        Mr0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> Mr0rR {
        Mr0rR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> Mr0sR {
        Mr0sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> Mr1iR {
        Mr1iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> Mr1rR {
        Mr1rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> Mr1sR {
        Mr1sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> Mr2iR {
        Mr2iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> Mr2rR {
        Mr2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> Mr2sR {
        Mr2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> Mr3iR {
        Mr3iR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> Mr3rR {
        Mr3rR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> Mr3sR {
        Mr3sR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr0rl(&self) -> Mr0rlR {
        Mr0rlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr1rl(&self) -> Mr1rlR {
        Mr1rlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr2rl(&self) -> Mr2rlR {
        Mr2rlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr3rl(&self) -> Mr3rlR {
        Mr3rlR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&mut self) -> Mr0iW<'_, McrSpec> {
        Mr0iW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&mut self) -> Mr0rW<'_, McrSpec> {
        Mr0rW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&mut self) -> Mr0sW<'_, McrSpec> {
        Mr0sW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&mut self) -> Mr1iW<'_, McrSpec> {
        Mr1iW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&mut self) -> Mr1rW<'_, McrSpec> {
        Mr1rW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&mut self) -> Mr1sW<'_, McrSpec> {
        Mr1sW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&mut self) -> Mr2iW<'_, McrSpec> {
        Mr2iW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&mut self) -> Mr2rW<'_, McrSpec> {
        Mr2rW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> Mr2sW<'_, McrSpec> {
        Mr2sW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&mut self) -> Mr3iW<'_, McrSpec> {
        Mr3iW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&mut self) -> Mr3rW<'_, McrSpec> {
        Mr3rW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&mut self) -> Mr3sW<'_, McrSpec> {
        Mr3sW::new(self, 11)
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr0rl(&mut self) -> Mr0rlW<'_, McrSpec> {
        Mr0rlW::new(self, 24)
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr1rl(&mut self) -> Mr1rlW<'_, McrSpec> {
        Mr1rlW::new(self, 25)
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr2rl(&mut self) -> Mr2rlW<'_, McrSpec> {
        Mr2rlW::new(self, 26)
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr3rl(&mut self) -> Mr3rlW<'_, McrSpec> {
        Mr3rlW::new(self, 27)
    }
}
#[doc = "Match Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}

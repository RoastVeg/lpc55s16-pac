#[doc = "Register `PLL0PDEC` reader"]
pub type R = crate::R<Pll0pdecSpec>;
#[doc = "Register `PLL0PDEC` writer"]
pub type W = crate::W<Pll0pdecSpec>;
#[doc = "Field `PDIV` reader - post-divider divider ratio (P-divider)"]
pub type PdivR = crate::FieldReader;
#[doc = "Field `PDIV` writer - post-divider divider ratio (P-divider)"]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PREQ` reader - feedback ratio change request."]
pub type PreqR = crate::BitReader;
#[doc = "Field `PREQ` writer - feedback ratio change request."]
pub type PreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&self) -> PreqR {
        PreqR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PdivW<'_, Pll0pdecSpec> {
        PdivW::new(self, 0)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&mut self) -> PreqW<'_, Pll0pdecSpec> {
        PreqW::new(self, 5)
    }
}
#[doc = "PLL0 550m P divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0pdec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0pdec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0pdecSpec;
impl crate::RegisterSpec for Pll0pdecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0pdec::R`](R) reader structure"]
impl crate::Readable for Pll0pdecSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0pdec::W`](W) writer structure"]
impl crate::Writable for Pll0pdecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0PDEC to value 0"]
impl crate::Resettable for Pll0pdecSpec {}

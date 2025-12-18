#[doc = "Register `PLL1MDEC` reader"]
pub type R = crate::R<Pll1mdecSpec>;
#[doc = "Register `PLL1MDEC` writer"]
pub type W = crate::W<Pll1mdecSpec>;
#[doc = "Field `MDIV` reader - feedback divider divider ratio (M-divider)."]
pub type MdivR = crate::FieldReader<u16>;
#[doc = "Field `MDIV` writer - feedback divider divider ratio (M-divider)."]
pub type MdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MREQ` reader - feedback ratio change request."]
pub type MreqR = crate::BitReader;
#[doc = "Field `MREQ` writer - feedback ratio change request."]
pub type MreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&self) -> MdivR {
        MdivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&self) -> MreqR {
        MreqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MdivW<'_, Pll1mdecSpec> {
        MdivW::new(self, 0)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MreqW<'_, Pll1mdecSpec> {
        MreqW::new(self, 16)
    }
}
#[doc = "PLL1 550m M divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1mdec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1mdec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1mdecSpec;
impl crate::RegisterSpec for Pll1mdecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1mdec::R`](R) reader structure"]
impl crate::Readable for Pll1mdecSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1mdec::W`](W) writer structure"]
impl crate::Writable for Pll1mdecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL1MDEC to value 0"]
impl crate::Resettable for Pll1mdecSpec {}

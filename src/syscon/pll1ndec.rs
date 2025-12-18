#[doc = "Register `PLL1NDEC` reader"]
pub type R = crate::R<Pll1ndecSpec>;
#[doc = "Register `PLL1NDEC` writer"]
pub type W = crate::W<Pll1ndecSpec>;
#[doc = "Field `NDIV` reader - pre-divider divider ratio (N-divider)."]
pub type NdivR = crate::FieldReader;
#[doc = "Field `NDIV` writer - pre-divider divider ratio (N-divider)."]
pub type NdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NREQ` reader - pre-divider ratio change request."]
pub type NreqR = crate::BitReader;
#[doc = "Field `NREQ` writer - pre-divider ratio change request."]
pub type NreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&self) -> NdivR {
        NdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&self) -> NreqR {
        NreqR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1NDEC")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NdivW<'_, Pll1ndecSpec> {
        NdivW::new(self, 0)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NreqW<'_, Pll1ndecSpec> {
        NreqW::new(self, 8)
    }
}
#[doc = "PLL1 550m N divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1ndec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1ndec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1ndecSpec;
impl crate::RegisterSpec for Pll1ndecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1ndec::R`](R) reader structure"]
impl crate::Readable for Pll1ndecSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1ndec::W`](W) writer structure"]
impl crate::Writable for Pll1ndecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL1NDEC to value 0"]
impl crate::Resettable for Pll1ndecSpec {}

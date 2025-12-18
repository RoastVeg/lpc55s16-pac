#[doc = "Register `CASPER_CTRL` reader"]
pub type R = crate::R<CasperCtrlSpec>;
#[doc = "Register `CASPER_CTRL` writer"]
pub type W = crate::W<CasperCtrlSpec>;
#[doc = "Control RAM access for RAMX0 and RAMX1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interleave {
    #[doc = "0: RAM access to RAMX0 and RAMX1 is consecutive."]
    Normal = 0,
    #[doc = "1: RAM access to RAMX0 and RAMX1 is interleaved."]
    Interleave = 1,
}
impl From<Interleave> for bool {
    #[inline(always)]
    fn from(variant: Interleave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERLEAVE` reader - Control RAM access for RAMX0 and RAMX1."]
pub type InterleaveR = crate::BitReader<Interleave>;
impl InterleaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Interleave {
        match self.bits {
            false => Interleave::Normal,
            true => Interleave::Interleave,
        }
    }
    #[doc = "RAM access to RAMX0 and RAMX1 is consecutive."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Interleave::Normal
    }
    #[doc = "RAM access to RAMX0 and RAMX1 is interleaved."]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == Interleave::Interleave
    }
}
#[doc = "Field `INTERLEAVE` writer - Control RAM access for RAMX0 and RAMX1."]
pub type InterleaveW<'a, REG> = crate::BitWriter<'a, REG, Interleave>;
impl<'a, REG> InterleaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM access to RAMX0 and RAMX1 is consecutive."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Interleave::Normal)
    }
    #[doc = "RAM access to RAMX0 and RAMX1 is interleaved."]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut crate::W<REG> {
        self.variant(Interleave::Interleave)
    }
}
impl R {
    #[doc = "Bit 0 - Control RAM access for RAMX0 and RAMX1."]
    #[inline(always)]
    pub fn interleave(&self) -> InterleaveR {
        InterleaveR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CASPER_CTRL")
            .field("interleave", &self.interleave())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Control RAM access for RAMX0 and RAMX1."]
    #[inline(always)]
    pub fn interleave(&mut self) -> InterleaveW<'_, CasperCtrlSpec> {
        InterleaveW::new(self, 0)
    }
}
#[doc = "Control CASPER integration.\n\nYou can [`read`](crate::Reg::read) this register and get [`casper_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`casper_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CasperCtrlSpec;
impl crate::RegisterSpec for CasperCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`casper_ctrl::R`](R) reader structure"]
impl crate::Readable for CasperCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`casper_ctrl::W`](W) writer structure"]
impl crate::Writable for CasperCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CASPER_CTRL to value 0"]
impl crate::Resettable for CasperCtrlSpec {}

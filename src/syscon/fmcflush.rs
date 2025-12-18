#[doc = "Register `FMCFLUSH` writer"]
pub type W = crate::W<FmcflushSpec>;
#[doc = "Flush control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: No action is performed."]
    NoFlush = 0,
    #[doc = "1: Flush the FMC buffer contents."]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Flush control"]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action is performed."]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::NoFlush)
    }
    #[doc = "Flush the FMC buffer contents."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
impl core::fmt::Debug for crate::generic::Reg<FmcflushSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Flush control"]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<'_, FmcflushSpec> {
        FlushW::new(self, 0)
    }
}
#[doc = "FMCflush control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcflush::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcflushSpec;
impl crate::RegisterSpec for FmcflushSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmcflush::W`](W) writer structure"]
impl crate::Writable for FmcflushSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMCFLUSH to value 0"]
impl crate::Resettable for FmcflushSpec {}

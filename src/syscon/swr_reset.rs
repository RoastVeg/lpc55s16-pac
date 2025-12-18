#[doc = "Register `SWR_RESET` writer"]
pub type W = crate::W<SwrResetSpec>;
#[doc = "Write 0x5A00_0001 to generate a software_reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SwrReset {
    #[doc = "0: Bloc is not reset."]
    Released = 0,
    #[doc = "1509949441: Generate a software reset."]
    Asserted = 1509949441,
}
impl From<SwrReset> for u32 {
    #[inline(always)]
    fn from(variant: SwrReset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwrReset {
    type Ux = u32;
}
impl crate::IsEnum for SwrReset {}
#[doc = "Field `SWR_RESET` writer - Write 0x5A00_0001 to generate a software_reset."]
pub type SwrResetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SwrReset>;
impl<'a, REG> SwrResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(SwrReset::Released)
    }
    #[doc = "Generate a software reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SwrReset::Asserted)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write 0x5A00_0001 to generate a software_reset."]
    #[inline(always)]
    pub fn swr_reset(&mut self) -> SwrResetW<'_, SwrResetSpec> {
        SwrResetW::new(self, 0)
    }
}
#[doc = "generate a software_reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrResetSpec;
impl crate::RegisterSpec for SwrResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swr_reset::W`](W) writer structure"]
impl crate::Writable for SwrResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWR_RESET to value 0"]
impl crate::Resettable for SwrResetSpec {}

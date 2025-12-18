#[doc = "Register `INT_CLR_ENABLE` writer"]
pub type W = crate::W<IntClrEnableSpec>;
#[doc = "Field `READY` writer - READY Interrupt Enable clear"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_ERR` writer - APB_ERR Interrupt Enable clear"]
pub type ApbErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IntClrEnableSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - READY Interrupt Enable clear"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntClrEnableSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - APB_ERR Interrupt Enable clear"]
    #[inline(always)]
    pub fn apb_err(&mut self) -> ApbErrW<'_, IntClrEnableSpec> {
        ApbErrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_enable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrEnableSpec;
impl crate::RegisterSpec for IntClrEnableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_enable::W`](W) writer structure"]
impl crate::Writable for IntClrEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR_ENABLE to value 0"]
impl crate::Resettable for IntClrEnableSpec {}

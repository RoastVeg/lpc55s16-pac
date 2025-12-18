#[doc = "Register `INT_SET_ENABLE` writer"]
pub type W = crate::W<IntSetEnableSpec>;
#[doc = "Field `READY` writer - READY Interrupt Enable set"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_ERR` writer - APB_ERR Interrupt Enable set"]
pub type ApbErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IntSetEnableSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - READY Interrupt Enable set"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntSetEnableSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - APB_ERR Interrupt Enable set"]
    #[inline(always)]
    pub fn apb_err(&mut self) -> ApbErrW<'_, IntSetEnableSpec> {
        ApbErrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_enable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSetEnableSpec;
impl crate::RegisterSpec for IntSetEnableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_set_enable::W`](W) writer structure"]
impl crate::Writable for IntSetEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_SET_ENABLE to value 0"]
impl crate::Resettable for IntSetEnableSpec {}

#[doc = "Register `INT_SET_STATUS` writer"]
pub type W = crate::W<IntSetStatusSpec>;
#[doc = "Field `READY` writer - READY Interrupt Status set"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_ERR` writer - APB_ERR Interrupt Status Set"]
pub type ApbErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IntSetStatusSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - READY Interrupt Status set"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntSetStatusSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - APB_ERR Interrupt Status Set"]
    #[inline(always)]
    pub fn apb_err(&mut self) -> ApbErrW<'_, IntSetStatusSpec> {
        ApbErrW::new(self, 1)
    }
}
#[doc = "Interrupt Status set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSetStatusSpec;
impl crate::RegisterSpec for IntSetStatusSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_set_status::W`](W) writer structure"]
impl crate::Writable for IntSetStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_SET_STATUS to value 0"]
impl crate::Resettable for IntSetStatusSpec {}

#[doc = "Register `INT_CLR_STATUS` writer"]
pub type W = crate::W<IntClrStatusSpec>;
#[doc = "Field `READY` writer - READY Interrupt Status clear"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_ERR` writer - APB_ERR Interrupt Status Clear"]
pub type ApbErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - READY Interrupt Status clear"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntClrStatusSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - APB_ERR Interrupt Status Clear"]
    #[inline(always)]
    pub fn apb_err(&mut self) -> ApbErrW<'_, IntClrStatusSpec> {
        ApbErrW::new(self, 1)
    }
}
#[doc = "Interrupt Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrStatusSpec;
impl crate::RegisterSpec for IntClrStatusSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_status::W`](W) writer structure"]
impl crate::Writable for IntClrStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR_STATUS to value 0"]
impl crate::Resettable for IntClrStatusSpec {}

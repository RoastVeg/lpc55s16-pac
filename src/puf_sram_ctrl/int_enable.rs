#[doc = "Register `INT_ENABLE` reader"]
pub type R = crate::R<IntEnableSpec>;
#[doc = "Register `INT_ENABLE` writer"]
pub type W = crate::W<IntEnableSpec>;
#[doc = "Field `READY` reader - READY Interrupt Enable"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `APB_ERR` reader - APB_ERR Interrupt Enable"]
pub type ApbErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - READY Interrupt Enable"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APB_ERR Interrupt Enable"]
    #[inline(always)]
    pub fn apb_err(&self) -> ApbErrR {
        ApbErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENABLE")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
impl W {}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnableSpec;
impl crate::RegisterSpec for IntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_enable::R`](R) reader structure"]
impl crate::Readable for IntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`int_enable::W`](W) writer structure"]
impl crate::Writable for IntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for IntEnableSpec {}

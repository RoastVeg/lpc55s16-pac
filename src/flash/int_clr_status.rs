#[doc = "Register `INT_CLR_STATUS` writer"]
pub type W = crate::W<IntClrStatusSpec>;
#[doc = "Field `FAIL` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` writer - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
pub type EccErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IntClrStatusSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<'_, IntClrStatusSpec> {
        FailW::new(self, 0)
    }
    #[doc = "Bit 1 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IntClrStatusSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 2 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntClrStatusSpec> {
        DoneW::new(self, 2)
    }
    #[doc = "Bit 3 - When a CLR_STATUS bit is written to 1, the corresponding INT_STATUS bit is cleared."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> EccErrW<'_, IntClrStatusSpec> {
        EccErrW::new(self, 3)
    }
}
#[doc = "Clear interrupt status bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

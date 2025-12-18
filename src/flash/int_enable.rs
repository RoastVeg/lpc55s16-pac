#[doc = "Register `INT_ENABLE` reader"]
pub type R = crate::R<IntEnableSpec>;
#[doc = "Register `INT_ENABLE` writer"]
pub type W = crate::W<IntEnableSpec>;
#[doc = "Field `FAIL` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub type FailR = crate::BitReader;
#[doc = "Field `ERR` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub type ErrR = crate::BitReader;
#[doc = "Field `DONE` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub type DoneR = crate::BitReader;
#[doc = "Field `ECC_ERR` reader - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
pub type EccErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn ecc_err(&self) -> EccErrR {
        EccErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENABLE")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
impl W {}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

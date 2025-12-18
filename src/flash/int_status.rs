#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `INT_STATUS` writer"]
pub type W = crate::W<IntStatusSpec>;
#[doc = "Field `FAIL` reader - This status bit is set if execution of a (legal) command failed."]
pub type FailR = crate::BitReader;
#[doc = "Field `ERR` reader - This status bit is set if execution of an illegal command is detected."]
pub type ErrR = crate::BitReader;
#[doc = "Field `DONE` reader - This status bit is set at the end of command execution."]
pub type DoneR = crate::BitReader;
#[doc = "Field `ECC_ERR` reader - This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
pub type EccErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status bit is set if execution of a (legal) command failed."]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This status bit is set if execution of an illegal command is detected."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This status bit is set at the end of command execution."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[inline(always)]
    pub fn ecc_err(&self) -> EccErrR {
        EccErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("fail", &self.fail())
            .field("err", &self.err())
            .field("done", &self.done())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
impl W {}
#[doc = "Interrupt status bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {}

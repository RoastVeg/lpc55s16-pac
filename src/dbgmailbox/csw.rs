#[doc = "Register `CSW` reader"]
pub type R = crate::R<CswSpec>;
#[doc = "Register `CSW` writer"]
pub type W = crate::W<CswSpec>;
#[doc = "Field `RESYNCH_REQ` reader - Debugger will set this bit to 1 to request a resynchronrisation"]
pub type ResynchReqR = crate::BitReader;
#[doc = "Field `RESYNCH_REQ` writer - Debugger will set this bit to 1 to request a resynchronrisation"]
pub type ResynchReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQ_PENDING` reader - Request is pending from debugger (i.e unread value in REQUEST)"]
pub type ReqPendingR = crate::BitReader;
#[doc = "Field `REQ_PENDING` writer - Request is pending from debugger (i.e unread value in REQUEST)"]
pub type ReqPendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_OR_ERR` reader - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
pub type DbgOrErrR = crate::BitReader;
#[doc = "Field `DBG_OR_ERR` writer - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
pub type DbgOrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_OR_ERR` reader - AHB overrun Error (Return value overwritten by ROM)"]
pub type AhbOrErrR = crate::BitReader;
#[doc = "Field `AHB_OR_ERR` writer - AHB overrun Error (Return value overwritten by ROM)"]
pub type AhbOrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT_RESET` reader - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIP_RESET_REQ` writer - Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)"]
pub type ChipResetReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debugger will set this bit to 1 to request a resynchronrisation"]
    #[inline(always)]
    pub fn resynch_req(&self) -> ResynchReqR {
        ResynchReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request is pending from debugger (i.e unread value in REQUEST)"]
    #[inline(always)]
    pub fn req_pending(&self) -> ReqPendingR {
        ReqPendingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
    #[inline(always)]
    pub fn dbg_or_err(&self) -> DbgOrErrR {
        DbgOrErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AHB overrun Error (Return value overwritten by ROM)"]
    #[inline(always)]
    pub fn ahb_or_err(&self) -> AhbOrErrR {
        AhbOrErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSW")
            .field("resynch_req", &self.resynch_req())
            .field("req_pending", &self.req_pending())
            .field("dbg_or_err", &self.dbg_or_err())
            .field("ahb_or_err", &self.ahb_or_err())
            .field("soft_reset", &self.soft_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debugger will set this bit to 1 to request a resynchronrisation"]
    #[inline(always)]
    pub fn resynch_req(&mut self) -> ResynchReqW<'_, CswSpec> {
        ResynchReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Request is pending from debugger (i.e unread value in REQUEST)"]
    #[inline(always)]
    pub fn req_pending(&mut self) -> ReqPendingW<'_, CswSpec> {
        ReqPendingW::new(self, 1)
    }
    #[doc = "Bit 2 - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
    #[inline(always)]
    pub fn dbg_or_err(&mut self) -> DbgOrErrW<'_, CswSpec> {
        DbgOrErrW::new(self, 2)
    }
    #[doc = "Bit 3 - AHB overrun Error (Return value overwritten by ROM)"]
    #[inline(always)]
    pub fn ahb_or_err(&mut self) -> AhbOrErrW<'_, CswSpec> {
        AhbOrErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SoftResetW<'_, CswSpec> {
        SoftResetW::new(self, 4)
    }
    #[doc = "Bit 5 - Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)"]
    #[inline(always)]
    pub fn chip_reset_req(&mut self) -> ChipResetReqW<'_, CswSpec> {
        ChipResetReqW::new(self, 5)
    }
}
#[doc = "CRC mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`csw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CswSpec;
impl crate::RegisterSpec for CswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csw::R`](R) reader structure"]
impl crate::Readable for CswSpec {}
#[doc = "`write(|w| ..)` method takes [`csw::W`](W) writer structure"]
impl crate::Writable for CswSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSW to value 0"]
impl crate::Resettable for CswSpec {}

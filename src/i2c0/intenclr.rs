#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `MSTPENDINGCLR` writer - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
pub type MstpendingclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTARBLOSSCLR` writer - Master Arbitration Loss interrupt clear."]
pub type MstarblossclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTSTSTPERRCLR` writer - Master Start/Stop Error interrupt clear."]
pub type MstststperrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVPENDINGCLR` writer - Slave Pending interrupt clear."]
pub type SlvpendingclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVNOTSTRCLR` writer - Slave Not Stretching interrupt clear."]
pub type SlvnotstrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVDESELCLR` writer - Slave Deselect interrupt clear."]
pub type SlvdeselclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDYCLR` writer - Monitor data Ready interrupt clear."]
pub type MonrdyclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONOVCLR` writer - Monitor Overrun interrupt clear."]
pub type MonovclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONIDLECLR` writer - Monitor Idle interrupt clear."]
pub type MonidleclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTTIMEOUTCLR` writer - Event time-out interrupt clear."]
pub type EventtimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLTIMEOUTCLR` writer - SCL time-out interrupt clear."]
pub type ScltimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IntenclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub fn mstpendingclr(&mut self) -> MstpendingclrW<'_, IntenclrSpec> {
        MstpendingclrW::new(self, 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub fn mstarblossclr(&mut self) -> MstarblossclrW<'_, IntenclrSpec> {
        MstarblossclrW::new(self, 4)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub fn mstststperrclr(&mut self) -> MstststperrclrW<'_, IntenclrSpec> {
        MstststperrclrW::new(self, 6)
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear."]
    #[inline(always)]
    pub fn slvpendingclr(&mut self) -> SlvpendingclrW<'_, IntenclrSpec> {
        SlvpendingclrW::new(self, 8)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub fn slvnotstrclr(&mut self) -> SlvnotstrclrW<'_, IntenclrSpec> {
        SlvnotstrclrW::new(self, 11)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear."]
    #[inline(always)]
    pub fn slvdeselclr(&mut self) -> SlvdeselclrW<'_, IntenclrSpec> {
        SlvdeselclrW::new(self, 15)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub fn monrdyclr(&mut self) -> MonrdyclrW<'_, IntenclrSpec> {
        MonrdyclrW::new(self, 16)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub fn monovclr(&mut self) -> MonovclrW<'_, IntenclrSpec> {
        MonovclrW::new(self, 17)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear."]
    #[inline(always)]
    pub fn monidleclr(&mut self) -> MonidleclrW<'_, IntenclrSpec> {
        MonidleclrW::new(self, 19)
    }
    #[doc = "Bit 24 - Event time-out interrupt clear."]
    #[inline(always)]
    pub fn eventtimeoutclr(&mut self) -> EventtimeoutclrW<'_, IntenclrSpec> {
        EventtimeoutclrW::new(self, 24)
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear."]
    #[inline(always)]
    pub fn scltimeoutclr(&mut self) -> ScltimeoutclrW<'_, IntenclrSpec> {
        ScltimeoutclrW::new(self, 25)
    }
}
#[doc = "Interrupt Enable Clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}

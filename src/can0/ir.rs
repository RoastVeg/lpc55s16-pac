#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message."]
pub type Rf0nR = crate::BitReader;
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message."]
pub type Rf0nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 watermark reached."]
pub type Rf0wR = crate::BitReader;
#[doc = "Field `RF0W` writer - Rx FIFO 0 watermark reached."]
pub type Rf0wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 full."]
pub type Rf0fR = crate::BitReader;
#[doc = "Field `RF0F` writer - Rx FIFO 0 full."]
pub type Rf0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost."]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost."]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message."]
pub type Rf1nR = crate::BitReader;
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message."]
pub type Rf1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 watermark reached."]
pub type Rf1wR = crate::BitReader;
#[doc = "Field `RF1W` writer - Rx FIFO 1 watermark reached."]
pub type Rf1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 full."]
pub type Rf1fR = crate::BitReader;
#[doc = "Field `RF1F` writer - Rx FIFO 1 full."]
pub type Rf1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost."]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost."]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM` reader - High priority message."]
pub type HpmR = crate::BitReader;
#[doc = "Field `HPM` writer - High priority message."]
pub type HpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmission completed."]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Transmission completed."]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - Transmission cancellation finished."]
pub type TcfR = crate::BitReader;
#[doc = "Field `TCF` writer - Transmission cancellation finished."]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Tx FIFO empty."]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - Tx FIFO empty."]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFN` reader - Tx event FIFO new entry."]
pub type TefnR = crate::BitReader;
#[doc = "Field `TEFN` writer - Tx event FIFO new entry."]
pub type TefnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFW` reader - Tx event FIFO watermark reached."]
pub type TefwR = crate::BitReader;
#[doc = "Field `TEFW` writer - Tx event FIFO watermark reached."]
pub type TefwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFF` reader - Tx event FIFO full."]
pub type TeffR = crate::BitReader;
#[doc = "Field `TEFF` writer - Tx event FIFO full."]
pub type TeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - Tx event FIFO element lost."]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx event FIFO element lost."]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSW` reader - Timestamp wraparound."]
pub type TswR = crate::BitReader;
#[doc = "Field `TSW` writer - Timestamp wraparound."]
pub type TswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAF` reader - Message RAM access failure."]
pub type MrafR = crate::BitReader;
#[doc = "Field `MRAF` writer - Message RAM access failure."]
pub type MrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOO` reader - Timeout occurred."]
pub type TooR = crate::BitReader;
#[doc = "Field `TOO` writer - Timeout occurred."]
pub type TooW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRX` reader - Message stored in dedicated Rx buffer."]
pub type DrxR = crate::BitReader;
#[doc = "Field `DRX` writer - Message stored in dedicated Rx buffer."]
pub type DrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEC` reader - Bit error corrected."]
pub type BecR = crate::BitReader;
#[doc = "Field `BEC` writer - Bit error corrected."]
pub type BecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEU` reader - Bit error uncorrected."]
pub type BeuR = crate::BitReader;
#[doc = "Field `BEU` writer - Bit error uncorrected."]
pub type BeuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELO` reader - Error logging overflow."]
pub type EloR = crate::BitReader;
#[doc = "Field `ELO` writer - Error logging overflow."]
pub type EloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - Error passive."]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - Error passive."]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - Warning status."]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - Warning status."]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - Bus_Off Status."]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status."]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDI` reader - Watchdog interrupt."]
pub type WdiR = crate::BitReader;
#[doc = "Field `WDI` writer - Watchdog interrupt."]
pub type WdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEA` reader - Protocol error in arbitration phase."]
pub type PeaR = crate::BitReader;
#[doc = "Field `PEA` writer - Protocol error in arbitration phase."]
pub type PeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - Protocol error in data phase."]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - Protocol error in data phase."]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARA` reader - Access to reserved address."]
pub type AraR = crate::BitReader;
#[doc = "Field `ARA` writer - Access to reserved address."]
pub type AraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&self) -> Rf0wR {
        Rf0wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&self) -> Rf1wR {
        Rf1wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&self) -> TefwR {
        TefwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&self) -> DrxR {
        DrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&self) -> BecR {
        BecR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&self) -> BeuR {
        BeuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&mut self) -> Rf0nW<'_, IrSpec> {
        Rf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&mut self) -> Rf0wW<'_, IrSpec> {
        Rf0wW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&mut self) -> Rf0fW<'_, IrSpec> {
        Rf0fW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&mut self) -> Rf0lW<'_, IrSpec> {
        Rf0lW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&mut self) -> Rf1nW<'_, IrSpec> {
        Rf1nW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&mut self) -> Rf1wW<'_, IrSpec> {
        Rf1wW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&mut self) -> Rf1fW<'_, IrSpec> {
        Rf1fW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&mut self) -> Rf1lW<'_, IrSpec> {
        Rf1lW::new(self, 7)
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&mut self) -> HpmW<'_, IrSpec> {
        HpmW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, IrSpec> {
        TcW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&mut self) -> TcfW<'_, IrSpec> {
        TcfW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TfeW<'_, IrSpec> {
        TfeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&mut self) -> TefnW<'_, IrSpec> {
        TefnW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&mut self) -> TefwW<'_, IrSpec> {
        TefwW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&mut self) -> TeffW<'_, IrSpec> {
        TeffW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&mut self) -> TeflW<'_, IrSpec> {
        TeflW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&mut self) -> TswW<'_, IrSpec> {
        TswW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&mut self) -> MrafW<'_, IrSpec> {
        MrafW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&mut self) -> TooW<'_, IrSpec> {
        TooW::new(self, 18)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&mut self) -> DrxW<'_, IrSpec> {
        DrxW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&mut self) -> BecW<'_, IrSpec> {
        BecW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&mut self) -> BeuW<'_, IrSpec> {
        BeuW::new(self, 21)
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&mut self) -> EloW<'_, IrSpec> {
        EloW::new(self, 22)
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&mut self) -> EpW<'_, IrSpec> {
        EpW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&mut self) -> EwW<'_, IrSpec> {
        EwW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&mut self) -> BoW<'_, IrSpec> {
        BoW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&mut self) -> WdiW<'_, IrSpec> {
        WdiW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&mut self) -> PeaW<'_, IrSpec> {
        PeaW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&mut self) -> PedW<'_, IrSpec> {
        PedW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&mut self) -> AraW<'_, IrSpec> {
        AraW::new(self, 29)
    }
}
#[doc = "Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {}

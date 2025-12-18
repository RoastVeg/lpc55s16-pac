#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Register `INFO` writer"]
pub type W = crate::W<InfoSpec>;
#[doc = "Field `FRAME_NR` reader - Frame number. This contains the frame number of the last successfully received SOF. In case no SOF was received by the device at the beginning of a frame, the frame number returned is that of the last successfully received SOF. In case the SOF frame number contained a CRC error, the frame number returned will be the corrupted frame number as received by the device."]
pub type FrameNrR = crate::FieldReader<u16>;
#[doc = "The error code which last occurred:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrCode {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: PID encoding error"]
    PidEncodingError = 1,
    #[doc = "2: PID unknown"]
    PidUnknown = 2,
    #[doc = "3: Packet unexpected"]
    PacketUnexpected = 3,
    #[doc = "4: Token CRC error"]
    TokenCrcError = 4,
    #[doc = "5: Data CRC error"]
    DataCrcError = 5,
    #[doc = "6: Time out"]
    Timeout = 6,
    #[doc = "7: Babble"]
    Babble = 7,
    #[doc = "8: Truncated EOP"]
    TruncatedEop = 8,
    #[doc = "9: Sent/Received NAK"]
    SentReceivedNak = 9,
    #[doc = "10: Sent Stall"]
    SentStall = 10,
    #[doc = "11: Overrun"]
    Overrun = 11,
    #[doc = "12: Sent empty packet"]
    SentEmptyPacket = 12,
    #[doc = "13: Bitstuff error"]
    BitstuffError = 13,
    #[doc = "14: Sync error"]
    SyncError = 14,
    #[doc = "15: Wrong data toggle"]
    WrongDataToggle = 15,
}
impl From<ErrCode> for u8 {
    #[inline(always)]
    fn from(variant: ErrCode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ErrCode {
    type Ux = u8;
}
impl crate::IsEnum for ErrCode {}
#[doc = "Field `ERR_CODE` reader - The error code which last occurred:"]
pub type ErrCodeR = crate::FieldReader<ErrCode>;
impl ErrCodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrCode {
        match self.bits {
            0 => ErrCode::NoError,
            1 => ErrCode::PidEncodingError,
            2 => ErrCode::PidUnknown,
            3 => ErrCode::PacketUnexpected,
            4 => ErrCode::TokenCrcError,
            5 => ErrCode::DataCrcError,
            6 => ErrCode::Timeout,
            7 => ErrCode::Babble,
            8 => ErrCode::TruncatedEop,
            9 => ErrCode::SentReceivedNak,
            10 => ErrCode::SentStall,
            11 => ErrCode::Overrun,
            12 => ErrCode::SentEmptyPacket,
            13 => ErrCode::BitstuffError,
            14 => ErrCode::SyncError,
            15 => ErrCode::WrongDataToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ErrCode::NoError
    }
    #[doc = "PID encoding error"]
    #[inline(always)]
    pub fn is_pid_encoding_error(&self) -> bool {
        *self == ErrCode::PidEncodingError
    }
    #[doc = "PID unknown"]
    #[inline(always)]
    pub fn is_pid_unknown(&self) -> bool {
        *self == ErrCode::PidUnknown
    }
    #[doc = "Packet unexpected"]
    #[inline(always)]
    pub fn is_packet_unexpected(&self) -> bool {
        *self == ErrCode::PacketUnexpected
    }
    #[doc = "Token CRC error"]
    #[inline(always)]
    pub fn is_token_crc_error(&self) -> bool {
        *self == ErrCode::TokenCrcError
    }
    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn is_data_crc_error(&self) -> bool {
        *self == ErrCode::DataCrcError
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == ErrCode::Timeout
    }
    #[doc = "Babble"]
    #[inline(always)]
    pub fn is_babble(&self) -> bool {
        *self == ErrCode::Babble
    }
    #[doc = "Truncated EOP"]
    #[inline(always)]
    pub fn is_truncated_eop(&self) -> bool {
        *self == ErrCode::TruncatedEop
    }
    #[doc = "Sent/Received NAK"]
    #[inline(always)]
    pub fn is_sent_received_nak(&self) -> bool {
        *self == ErrCode::SentReceivedNak
    }
    #[doc = "Sent Stall"]
    #[inline(always)]
    pub fn is_sent_stall(&self) -> bool {
        *self == ErrCode::SentStall
    }
    #[doc = "Overrun"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ErrCode::Overrun
    }
    #[doc = "Sent empty packet"]
    #[inline(always)]
    pub fn is_sent_empty_packet(&self) -> bool {
        *self == ErrCode::SentEmptyPacket
    }
    #[doc = "Bitstuff error"]
    #[inline(always)]
    pub fn is_bitstuff_error(&self) -> bool {
        *self == ErrCode::BitstuffError
    }
    #[doc = "Sync error"]
    #[inline(always)]
    pub fn is_sync_error(&self) -> bool {
        *self == ErrCode::SyncError
    }
    #[doc = "Wrong data toggle"]
    #[inline(always)]
    pub fn is_wrong_data_toggle(&self) -> bool {
        *self == ErrCode::WrongDataToggle
    }
}
#[doc = "Field `ERR_CODE` writer - The error code which last occurred:"]
pub type ErrCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, ErrCode, crate::Safe>;
impl<'a, REG> ErrCodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::NoError)
    }
    #[doc = "PID encoding error"]
    #[inline(always)]
    pub fn pid_encoding_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::PidEncodingError)
    }
    #[doc = "PID unknown"]
    #[inline(always)]
    pub fn pid_unknown(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::PidUnknown)
    }
    #[doc = "Packet unexpected"]
    #[inline(always)]
    pub fn packet_unexpected(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::PacketUnexpected)
    }
    #[doc = "Token CRC error"]
    #[inline(always)]
    pub fn token_crc_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::TokenCrcError)
    }
    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn data_crc_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::DataCrcError)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::Timeout)
    }
    #[doc = "Babble"]
    #[inline(always)]
    pub fn babble(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::Babble)
    }
    #[doc = "Truncated EOP"]
    #[inline(always)]
    pub fn truncated_eop(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::TruncatedEop)
    }
    #[doc = "Sent/Received NAK"]
    #[inline(always)]
    pub fn sent_received_nak(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::SentReceivedNak)
    }
    #[doc = "Sent Stall"]
    #[inline(always)]
    pub fn sent_stall(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::SentStall)
    }
    #[doc = "Overrun"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::Overrun)
    }
    #[doc = "Sent empty packet"]
    #[inline(always)]
    pub fn sent_empty_packet(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::SentEmptyPacket)
    }
    #[doc = "Bitstuff error"]
    #[inline(always)]
    pub fn bitstuff_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::BitstuffError)
    }
    #[doc = "Sync error"]
    #[inline(always)]
    pub fn sync_error(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::SyncError)
    }
    #[doc = "Wrong data toggle"]
    #[inline(always)]
    pub fn wrong_data_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(ErrCode::WrongDataToggle)
    }
}
#[doc = "Field `MINREV` reader - Minor Revision."]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major Revision."]
pub type MajrevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - Frame number. This contains the frame number of the last successfully received SOF. In case no SOF was received by the device at the beginning of a frame, the frame number returned is that of the last successfully received SOF. In case the SOF frame number contained a CRC error, the frame number returned will be the corrupted frame number as received by the device."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FrameNrR {
        FrameNrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline(always)]
    pub fn err_code(&self) -> ErrCodeR {
        ErrCodeR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor Revision."]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Revision."]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFO")
            .field("frame_nr", &self.frame_nr())
            .field("err_code", &self.err_code())
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
impl W {
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline(always)]
    pub fn err_code(&mut self) -> ErrCodeW<'_, InfoSpec> {
        ErrCodeW::new(self, 11)
    }
}
#[doc = "USB Info register\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`write(|w| ..)` method takes [`info::W`](W) writer structure"]
impl crate::Writable for InfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {}

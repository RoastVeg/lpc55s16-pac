#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "If 1, the block is waiting for more data to process.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waiting {
    #[doc = "0: Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    NotWaiting = 0,
    #[doc = "1: Waiting for data to be written in (16 words)"]
    Waiting = 1,
}
impl From<Waiting> for bool {
    #[inline(always)]
    fn from(variant: Waiting) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITING` reader - If 1, the block is waiting for more data to process."]
pub type WaitingR = crate::BitReader<Waiting>;
impl WaitingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waiting {
        match self.bits {
            false => Waiting::NotWaiting,
            true => Waiting::Waiting,
        }
    }
    #[doc = "Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    #[inline(always)]
    pub fn is_not_waiting(&self) -> bool {
        *self == Waiting::NotWaiting
    }
    #[doc = "Waiting for data to be written in (16 words)"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == Waiting::Waiting
    }
}
#[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digest {
    #[doc = "0: No Digest is ready"]
    NotReady = 0,
    #[doc = "1: Digest is ready. Application may read it or may write more data"]
    Ready = 1,
}
impl From<Digest> for bool {
    #[inline(always)]
    fn from(variant: Digest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGEST` reader - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
pub type DigestR = crate::BitReader<Digest>;
impl DigestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digest {
        match self.bits {
            false => Digest::NotReady,
            true => Digest::Ready,
        }
    }
    #[doc = "No Digest is ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Digest::NotReady
    }
    #[doc = "Digest is ready. Application may read it or may write more data"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Digest::Ready
    }
}
#[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: No error."]
    NoError = 0,
    #[doc = "1: An error occurred since last cleared (written 1 to clear)."]
    Error = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::NoError,
            true => Error::Error,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Error::NoError
    }
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Error::Error
    }
}
#[doc = "Field `ERROR` writer - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Error::NoError)
    }
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Error)
    }
}
#[doc = "Indicates if an AES or PRNG fault has occurred\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "0: No AES or PRNG fault has occurred."]
    NoFault = 0,
    #[doc = "1: An AES or PRNG fault has occurred."]
    Fault = 1,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT` reader - Indicates if an AES or PRNG fault has occurred"]
pub type FaultR = crate::BitReader<Fault>;
impl FaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault {
        match self.bits {
            false => Fault::NoFault,
            true => Fault::Fault,
        }
    }
    #[doc = "No AES or PRNG fault has occurred."]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == Fault::NoFault
    }
    #[doc = "An AES or PRNG fault has occurred."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Fault::Fault
    }
}
#[doc = "Indicates the block wants the key to be written in (set along with WAITING)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Needkey {
    #[doc = "0: No Key is needed and writes will not be treated as Key"]
    NotNeed = 0,
    #[doc = "1: Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    Need = 1,
}
impl From<Needkey> for bool {
    #[inline(always)]
    fn from(variant: Needkey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEEDKEY` reader - Indicates the block wants the key to be written in (set along with WAITING)"]
pub type NeedkeyR = crate::BitReader<Needkey>;
impl NeedkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Needkey {
        match self.bits {
            false => Needkey::NotNeed,
            true => Needkey::Need,
        }
    }
    #[doc = "No Key is needed and writes will not be treated as Key"]
    #[inline(always)]
    pub fn is_not_need(&self) -> bool {
        *self == Needkey::NotNeed
    }
    #[doc = "Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        *self == Needkey::Need
    }
}
#[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Neediv {
    #[doc = "0: No IV/Nonce is needed, either because written already or because not needed."]
    NotNeed = 0,
    #[doc = "1: IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    Need = 1,
}
impl From<Neediv> for bool {
    #[inline(always)]
    fn from(variant: Neediv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEEDIV` reader - Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
pub type NeedivR = crate::BitReader<Neediv>;
impl NeedivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Neediv {
        match self.bits {
            false => Neediv::NotNeed,
            true => Neediv::Need,
        }
    }
    #[doc = "No IV/Nonce is needed, either because written already or because not needed."]
    #[inline(always)]
    pub fn is_not_need(&self) -> bool {
        *self == Neediv::NotNeed
    }
    #[doc = "IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        *self == Neediv::Need
    }
}
#[doc = "AES fault status\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesfault {
    #[doc = "0: No AES fault has occurred."]
    NoFault = 0,
    #[doc = "1: An AES fault has occurred."]
    Fault = 1,
}
impl From<Aesfault> for bool {
    #[inline(always)]
    fn from(variant: Aesfault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESFAULT` reader - AES fault status"]
pub type AesfaultR = crate::BitReader<Aesfault>;
impl AesfaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesfault {
        match self.bits {
            false => Aesfault::NoFault,
            true => Aesfault::Fault,
        }
    }
    #[doc = "No AES fault has occurred."]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == Aesfault::NoFault
    }
    #[doc = "An AES fault has occurred."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Aesfault::Fault
    }
}
#[doc = "PRNG fault status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prngfault {
    #[doc = "0: No PRNG fault has occurred."]
    NoFault = 0,
    #[doc = "1: A PRNG fault has occurred."]
    Fault = 1,
}
impl From<Prngfault> for bool {
    #[inline(always)]
    fn from(variant: Prngfault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRNGFAULT` reader - PRNG fault status"]
pub type PrngfaultR = crate::BitReader<Prngfault>;
impl PrngfaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prngfault {
        match self.bits {
            false => Prngfault::NoFault,
            true => Prngfault::Fault,
        }
    }
    #[doc = "No PRNG fault has occurred."]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == Prngfault::NoFault
    }
    #[doc = "A PRNG fault has occurred."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Prngfault::Fault
    }
}
impl R {
    #[doc = "Bit 0 - If 1, the block is waiting for more data to process."]
    #[inline(always)]
    pub fn waiting(&self) -> WaitingR {
        WaitingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline(always)]
    pub fn digest(&self) -> DigestR {
        DigestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if an AES or PRNG fault has occurred"]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the block wants the key to be written in (set along with WAITING)"]
    #[inline(always)]
    pub fn needkey(&self) -> NeedkeyR {
        NeedkeyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
    #[inline(always)]
    pub fn neediv(&self) -> NeedivR {
        NeedivR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - AES fault status"]
    #[inline(always)]
    pub fn aesfault(&self) -> AesfaultR {
        AesfaultR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PRNG fault status"]
    #[inline(always)]
    pub fn prngfault(&self) -> PrngfaultR {
        PrngfaultR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("waiting", &self.waiting())
            .field("digest", &self.digest())
            .field("error", &self.error())
            .field("fault", &self.fault())
            .field("needkey", &self.needkey())
            .field("neediv", &self.neediv())
            .field("aesfault", &self.aesfault())
            .field("prngfault", &self.prngfault())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, StatusSpec> {
        ErrorW::new(self, 2)
    }
}
#[doc = "Indicates status of Hash peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets STATUS to value 0x1500"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x1500;
}

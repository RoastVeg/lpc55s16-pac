#[doc = "Register `ERR` reader"]
pub type R = crate::R<ErrSpec>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ErrSpec>;
#[doc = "PRINCE Error Status. This bit is write-1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errstat {
    #[doc = "0: No PRINCE error."]
    NoError = 0,
    #[doc = "1: Error. A read of a PRINCE-encrypted region was attempted while ENC_ENABLE.EN=1."]
    Error = 1,
}
impl From<Errstat> for bool {
    #[inline(always)]
    fn from(variant: Errstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSTAT` reader - PRINCE Error Status. This bit is write-1 to clear."]
pub type ErrstatR = crate::BitReader<Errstat>;
impl ErrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errstat {
        match self.bits {
            false => Errstat::NoError,
            true => Errstat::Error,
        }
    }
    #[doc = "No PRINCE error."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Errstat::NoError
    }
    #[doc = "Error. A read of a PRINCE-encrypted region was attempted while ENC_ENABLE.EN=1."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Errstat::Error
    }
}
#[doc = "Field `ERRSTAT` writer - PRINCE Error Status. This bit is write-1 to clear."]
pub type ErrstatW<'a, REG> = crate::BitWriter<'a, REG, Errstat>;
impl<'a, REG> ErrstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PRINCE error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Errstat::NoError)
    }
    #[doc = "Error. A read of a PRINCE-encrypted region was attempted while ENC_ENABLE.EN=1."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Errstat::Error)
    }
}
impl R {
    #[doc = "Bit 0 - PRINCE Error Status. This bit is write-1 to clear."]
    #[inline(always)]
    pub fn errstat(&self) -> ErrstatR {
        ErrstatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRINCE Error Status. This bit is write-1 to clear."]
    #[inline(always)]
    pub fn errstat(&mut self) -> ErrstatW<'_, ErrSpec> {
        ErrstatW::new(self, 0)
    }
}
#[doc = "Error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrSpec;
impl crate::RegisterSpec for ErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ErrSpec {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ErrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ErrSpec {}

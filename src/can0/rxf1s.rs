#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<Rxf1sSpec>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level."]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index."]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index."]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1F` reader - Rx FIFO 1 full."]
pub type F1fR = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost."]
pub type Rf1lR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 fill level."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 get index."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 put index."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1sSpec;
impl crate::RegisterSpec for Rxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for Rxf1sSpec {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for Rxf1sSpec {}

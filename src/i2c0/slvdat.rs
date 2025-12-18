#[doc = "Register `SLVDAT` reader"]
pub type R = crate::R<SlvdatSpec>;
#[doc = "Register `SLVDAT` writer"]
pub type W = crate::W<SlvdatSpec>;
#[doc = "Field `DATA` reader - Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVDAT")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SlvdatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Combined Slave receiver and transmitter data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvdatSpec;
impl crate::RegisterSpec for SlvdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvdat::R`](R) reader structure"]
impl crate::Readable for SlvdatSpec {}
#[doc = "`write(|w| ..)` method takes [`slvdat::W`](W) writer structure"]
impl crate::Writable for SlvdatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLVDAT to value 0"]
impl crate::Resettable for SlvdatSpec {}

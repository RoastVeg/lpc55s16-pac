#[doc = "Register `puf_discharge_time_in_ms` reader"]
pub type R = crate::R<PufDischargeTimeInMsSpec>;
#[doc = "Register `puf_discharge_time_in_ms` writer"]
pub type W = crate::W<PufDischargeTimeInMsSpec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("puf_discharge_time_in_ms")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, PufDischargeTimeInMsSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "puf discharge time in ms.\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_discharge_time_in_ms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_discharge_time_in_ms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PufDischargeTimeInMsSpec;
impl crate::RegisterSpec for PufDischargeTimeInMsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`puf_discharge_time_in_ms::R`](R) reader structure"]
impl crate::Readable for PufDischargeTimeInMsSpec {}
#[doc = "`write(|w| ..)` method takes [`puf_discharge_time_in_ms::W`](W) writer structure"]
impl crate::Writable for PufDischargeTimeInMsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets puf_discharge_time_in_ms to value 0"]
impl crate::Resettable for PufDischargeTimeInMsSpec {}

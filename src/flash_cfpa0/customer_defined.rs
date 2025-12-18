#[doc = "Register `CUSTOMER_DEFINED[%s]` reader"]
pub type R = crate::R<CustomerDefinedSpec>;
#[doc = "Register `CUSTOMER_DEFINED[%s]` writer"]
pub type W = crate::W<CustomerDefinedSpec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CUSTOMER_DEFINED")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, CustomerDefinedSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Customer Defined (Programable through ROM API)\n\nYou can [`read`](crate::Reg::read) this register and get [`customer_defined::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer_defined::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CustomerDefinedSpec;
impl crate::RegisterSpec for CustomerDefinedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`customer_defined::R`](R) reader structure"]
impl crate::Readable for CustomerDefinedSpec {}
#[doc = "`write(|w| ..)` method takes [`customer_defined::W`](W) writer structure"]
impl crate::Writable for CustomerDefinedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUSTOMER_DEFINED[%s] to value 0"]
impl crate::Resettable for CustomerDefinedSpec {}

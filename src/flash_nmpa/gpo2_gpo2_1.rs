#[doc = "Register `GPO2_1` reader"]
pub type R = crate::R<Gpo2Gpo2_1Spec>;
#[doc = "Register `GPO2_1` writer"]
pub type W = crate::W<Gpo2Gpo2_1Spec>;
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
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo2Gpo2_1Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "GPO2 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo2Gpo2_1Spec;
impl crate::RegisterSpec for Gpo2Gpo2_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo2_gpo2_1::R`](R) reader structure"]
impl crate::Readable for Gpo2Gpo2_1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo2_gpo2_1::W`](W) writer structure"]
impl crate::Writable for Gpo2Gpo2_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO2_1 to value 0"]
impl crate::Resettable for Gpo2Gpo2_1Spec {}

#[doc = "Register `PRINCE_SR_0` reader"]
pub type R = crate::R<PrinceSr0Spec>;
#[doc = "Register `PRINCE_SR_0` writer"]
pub type W = crate::W<PrinceSr0Spec>;
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
        f.debug_struct("PRINCE_SR_0")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, PrinceSr0Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Region 0, sub-region enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_sr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_sr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceSr0Spec;
impl crate::RegisterSpec for PrinceSr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_sr_0::R`](R) reader structure"]
impl crate::Readable for PrinceSr0Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_sr_0::W`](W) writer structure"]
impl crate::Writable for PrinceSr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRINCE_SR_0 to value 0"]
impl crate::Resettable for PrinceSr0Spec {}

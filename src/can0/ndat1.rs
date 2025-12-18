#[doc = "Register `NDAT1` reader"]
pub type R = crate::R<Ndat1Spec>;
#[doc = "Register `NDAT1` writer"]
pub type W = crate::W<Ndat1Spec>;
#[doc = "Field `ND` reader - New Data."]
pub type NdR = crate::FieldReader<u32>;
#[doc = "Field `ND` writer - New Data."]
pub type NdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&self) -> NdR {
        NdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&mut self) -> NdW<'_, Ndat1Spec> {
        NdW::new(self, 0)
    }
}
#[doc = "New Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndat1Spec;
impl crate::RegisterSpec for Ndat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat1::R`](R) reader structure"]
impl crate::Readable for Ndat1Spec {}
#[doc = "`write(|w| ..)` method takes [`ndat1::W`](W) writer structure"]
impl crate::Writable for Ndat1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDAT1 to value 0"]
impl crate::Resettable for Ndat1Spec {}

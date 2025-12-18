#[doc = "Register `NDAT2` reader"]
pub type R = crate::R<Ndat2Spec>;
#[doc = "Register `NDAT2` writer"]
pub type W = crate::W<Ndat2Spec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NDAT2").field("nd", &self.nd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&mut self) -> NdW<'_, Ndat2Spec> {
        NdW::new(self, 0)
    }
}
#[doc = "New Data 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndat2Spec;
impl crate::RegisterSpec for Ndat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat2::R`](R) reader structure"]
impl crate::Readable for Ndat2Spec {}
#[doc = "`write(|w| ..)` method takes [`ndat2::W`](W) writer structure"]
impl crate::Writable for Ndat2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDAT2 to value 0"]
impl crate::Resettable for Ndat2Spec {}

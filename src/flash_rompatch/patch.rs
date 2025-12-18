#[doc = "Register `PATCH[%s]` reader"]
pub type R = crate::R<PatchSpec>;
#[doc = "Register `PATCH[%s]` writer"]
pub type W = crate::W<PatchSpec>;
#[doc = "Field `PATCH` reader - ."]
pub type PatchR = crate::FieldReader<u32>;
#[doc = "Field `PATCH` writer - ."]
pub type PatchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn patch(&self) -> PatchR {
        PatchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn patch(&mut self) -> PatchW<'_, PatchSpec> {
        PatchW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`patch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PatchSpec;
impl crate::RegisterSpec for PatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patch::R`](R) reader structure"]
impl crate::Readable for PatchSpec {}
#[doc = "`write(|w| ..)` method takes [`patch::W`](W) writer structure"]
impl crate::Writable for PatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATCH[%s] to value 0"]
impl crate::Resettable for PatchSpec {}

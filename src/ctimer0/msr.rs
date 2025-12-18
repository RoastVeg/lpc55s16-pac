#[doc = "Register `MSR[%s]` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR[%s]` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `SHADOW` reader - Timer counter match shadow value."]
pub type ShadowR = crate::FieldReader<u32>;
#[doc = "Field `SHADOW` writer - Timer counter match shadow value."]
pub type ShadowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter match shadow value."]
    #[inline(always)]
    pub fn shadow(&self) -> ShadowR {
        ShadowR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSR")
            .field("shadow", &self.shadow())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter match shadow value."]
    #[inline(always)]
    pub fn shadow(&mut self) -> ShadowW<'_, MsrSpec> {
        ShadowW::new(self, 0)
    }
}
#[doc = "Match Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSR[%s] to value 0"]
impl crate::Resettable for MsrSpec {}

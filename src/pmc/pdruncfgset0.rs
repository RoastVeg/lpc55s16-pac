#[doc = "Register `PDRUNCFGSET0` writer"]
pub type W = crate::W<Pdruncfgset0Spec>;
#[doc = "Field `PDRUNCFGSET0` writer - Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
pub type Pdruncfgset0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Pdruncfgset0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub fn pdruncfgset0(&mut self) -> Pdruncfgset0W<'_, Pdruncfgset0Spec> {
        Pdruncfgset0W::new(self, 0)
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfgset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfgset0Spec;
impl crate::RegisterSpec for Pdruncfgset0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfgset0::W`](W) writer structure"]
impl crate::Writable for Pdruncfgset0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDRUNCFGSET0 to value 0"]
impl crate::Resettable for Pdruncfgset0Spec {}

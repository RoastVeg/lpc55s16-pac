#[doc = "Register `PDRUNCFGCLR0` writer"]
pub type W = crate::W<Pdruncfgclr0Spec>;
#[doc = "Field `PDRUNCFGCLR0` writer - Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
pub type Pdruncfgclr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub fn pdruncfgclr0(&mut self) -> Pdruncfgclr0W<'_, Pdruncfgclr0Spec> {
        Pdruncfgclr0W::new(self, 0)
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfgclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfgclr0Spec;
impl crate::RegisterSpec for Pdruncfgclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfgclr0::W`](W) writer structure"]
impl crate::Writable for Pdruncfgclr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDRUNCFGCLR0 to value 0"]
impl crate::Resettable for Pdruncfgclr0Spec {}

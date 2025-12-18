#[doc = "Register `ADD1` writer"]
pub type W = crate::W<Add1Spec>;
#[doc = "Field `AD1` writer - Address of ADD1 command."]
pub type Ad1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Add1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of ADD1 command."]
    #[inline(always)]
    pub fn ad1(&mut self) -> Ad1W<'_, Add1Spec> {
        Ad1W::new(self, 0)
    }
}
#[doc = "Write address for issuing the ADD1 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`add1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Add1Spec;
impl crate::RegisterSpec for Add1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`add1::W`](W) writer structure"]
impl crate::Writable for Add1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADD1 to value 0"]
impl crate::Resettable for Add1Spec {}

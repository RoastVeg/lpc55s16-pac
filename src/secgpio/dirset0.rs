#[doc = "Register `DIRSET0` writer"]
pub type W = crate::W<Dirset0Spec>;
#[doc = "Field `DIRSETP` writer - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
pub type DirsetpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Dirset0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub fn dirsetp(&mut self) -> DirsetpW<'_, Dirset0Spec> {
        DirsetpW::new(self, 0)
    }
}
#[doc = "Set pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dirset0Spec;
impl crate::RegisterSpec for Dirset0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirset0::W`](W) writer structure"]
impl crate::Writable for Dirset0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIRSET0 to value 0"]
impl crate::Resettable for Dirset0Spec {}

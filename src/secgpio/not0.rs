#[doc = "Register `NOT0` writer"]
pub type W = crate::W<Not0Spec>;
#[doc = "Field `NOTP` writer - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
pub type NotpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<Not0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp(&mut self) -> NotpW<'_, Not0Spec> {
        NotpW::new(self, 0)
    }
}
#[doc = "Toggle port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`not0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Not0Spec;
impl crate::RegisterSpec for Not0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`not0::W`](W) writer structure"]
impl crate::Writable for Not0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NOT0 to value 0"]
impl crate::Resettable for Not0Spec {}

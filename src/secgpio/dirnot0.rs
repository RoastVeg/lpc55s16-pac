#[doc = "Register `DIRNOT0` writer"]
pub type W = crate::W<Dirnot0Spec>;
#[doc = "Field `DIRNOTP` writer - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
pub type DirnotpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub fn dirnotp(&mut self) -> DirnotpW<'_, Dirnot0Spec> {
        DirnotpW::new(self, 0)
    }
}
#[doc = "Toggle pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirnot0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dirnot0Spec;
impl crate::RegisterSpec for Dirnot0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirnot0::W`](W) writer structure"]
impl crate::Writable for Dirnot0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIRNOT0 to value 0"]
impl crate::Resettable for Dirnot0Spec {}

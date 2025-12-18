#[doc = "Register `ENABLECLR0` writer"]
pub type W = crate::W<Enableclr0Spec>;
#[doc = "Field `CLR` writer - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, Enableclr0Spec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Channel Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enableclr0Spec;
impl crate::RegisterSpec for Enableclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enableclr0::W`](W) writer structure"]
impl crate::Writable for Enableclr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLECLR0 to value 0"]
impl crate::Resettable for Enableclr0Spec {}

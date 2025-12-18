#[doc = "Register `B0_28` reader"]
pub type R = crate::R<B0_28Spec>;
#[doc = "Register `B0_28` writer"]
pub type W = crate::W<B0_28Spec>;
#[doc = "Field `PBYTE` reader - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PbyteR = crate::BitReader;
#[doc = "Field `PBYTE` writer - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PbyteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&self) -> PbyteR {
        PbyteR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0_28")
            .field("pbyte", &self.pbyte())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&mut self) -> PbyteW<'_, B0_28Spec> {
        PbyteW::new(self, 0)
    }
}
#[doc = "Byte pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b0_28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0_28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B0_28Spec;
impl crate::RegisterSpec for B0_28Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`b0_28::R`](R) reader structure"]
impl crate::Readable for B0_28Spec {}
#[doc = "`write(|w| ..)` method takes [`b0_28::W`](W) writer structure"]
impl crate::Writable for B0_28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B0_28 to value 0"]
impl crate::Resettable for B0_28Spec {}

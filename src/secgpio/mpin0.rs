#[doc = "Register `MPIN0` reader"]
pub type R = crate::R<Mpin0Spec>;
#[doc = "Register `MPIN0` writer"]
pub type W = crate::W<Mpin0Spec>;
#[doc = "Field `MPORTP` reader - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MportpR = crate::FieldReader<u32>;
#[doc = "Field `MPORTP` writer - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MportpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&self) -> MportpR {
        MportpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPIN0")
            .field("mportp", &self.mportp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&mut self) -> MportpW<'_, Mpin0Spec> {
        MportpW::new(self, 0)
    }
}
#[doc = "Masked port register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mpin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpin0Spec;
impl crate::RegisterSpec for Mpin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpin0::R`](R) reader structure"]
impl crate::Readable for Mpin0Spec {}
#[doc = "`write(|w| ..)` method takes [`mpin0::W`](W) writer structure"]
impl crate::Writable for Mpin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPIN0 to value 0"]
impl crate::Resettable for Mpin0Spec {}

#[doc = "Register `PIN0` reader"]
pub type R = crate::R<Pin0Spec>;
#[doc = "Register `PIN0` writer"]
pub type W = crate::W<Pin0Spec>;
#[doc = "Field `PORT` reader - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PortR = crate::FieldReader<u32>;
#[doc = "Field `PORT` writer - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN0").field("port", &self.port()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Pin0Spec> {
        PortW::new(self, 0)
    }
}
#[doc = "Port pin register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`pin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pin0Spec;
impl crate::RegisterSpec for Pin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin0::R`](R) reader structure"]
impl crate::Readable for Pin0Spec {}
#[doc = "`write(|w| ..)` method takes [`pin0::W`](W) writer structure"]
impl crate::Writable for Pin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN0 to value 0"]
impl crate::Resettable for Pin0Spec {}

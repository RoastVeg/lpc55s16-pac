#[doc = "Register `SET0` reader"]
pub type R = crate::R<Set0Spec>;
#[doc = "Register `SET0` writer"]
pub type W = crate::W<Set0Spec>;
#[doc = "Field `SETP` reader - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SetpR = crate::FieldReader<u32>;
#[doc = "Field `SETP` writer - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub type SetpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&self) -> SetpR {
        SetpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET0").field("setp", &self.setp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp(&mut self) -> SetpW<'_, Set0Spec> {
        SetpW::new(self, 0)
    }
}
#[doc = "Write: Set register for port. Read: output bits for port\n\nYou can [`read`](crate::Reg::read) this register and get [`set0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Set0Spec;
impl crate::RegisterSpec for Set0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set0::R`](R) reader structure"]
impl crate::Readable for Set0Spec {}
#[doc = "`write(|w| ..)` method takes [`set0::W`](W) writer structure"]
impl crate::Writable for Set0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET0 to value 0"]
impl crate::Resettable for Set0Spec {}

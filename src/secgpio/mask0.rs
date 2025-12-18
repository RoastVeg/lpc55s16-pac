#[doc = "Register `MASK0` reader"]
pub type R = crate::R<Mask0Spec>;
#[doc = "Register `MASK0` writer"]
pub type W = crate::W<Mask0Spec>;
#[doc = "Field `MASKP` reader - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MaskpR = crate::FieldReader<u32>;
#[doc = "Field `MASKP` writer - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
pub type MaskpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp(&self) -> MaskpR {
        MaskpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK0")
            .field("maskp", &self.maskp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package.0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp(&mut self) -> MaskpW<'_, Mask0Spec> {
        MaskpW::new(self, 0)
    }
}
#[doc = "Mask register for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mask0Spec;
impl crate::RegisterSpec for Mask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask0::R`](R) reader structure"]
impl crate::Readable for Mask0Spec {}
#[doc = "`write(|w| ..)` method takes [`mask0::W`](W) writer structure"]
impl crate::Writable for Mask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for Mask0Spec {}

#[doc = "Register `DIR0` reader"]
pub type R = crate::R<Dir0Spec>;
#[doc = "Register `DIR0` writer"]
pub type W = crate::W<Dir0Spec>;
#[doc = "Field `DIRP` reader - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DirpR = crate::FieldReader<u32>;
#[doc = "Field `DIRP` writer - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DirpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&self) -> DirpR {
        DirpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR0").field("dirp", &self.dirp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&mut self) -> DirpW<'_, Dir0Spec> {
        DirpW::new(self, 0)
    }
}
#[doc = "Direction registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`dir0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dir0Spec;
impl crate::RegisterSpec for Dir0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir0::R`](R) reader structure"]
impl crate::Readable for Dir0Spec {}
#[doc = "`write(|w| ..)` method takes [`dir0::W`](W) writer structure"]
impl crate::Writable for Dir0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIR0 to value 0"]
impl crate::Resettable for Dir0Spec {}

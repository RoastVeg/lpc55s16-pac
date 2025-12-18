#[doc = "Register `FLEXFRGXCTRL3` reader"]
pub type R = crate::R<FlexfrgctrlFlexfrgxctrl3Spec>;
#[doc = "Register `FLEXFRGXCTRL3` writer"]
pub type W = crate::W<FlexfrgctrlFlexfrgxctrl3Spec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGCTRL_FLEXFRGXCTRL3")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, FlexfrgctrlFlexfrgxctrl3Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexfrgctrlFlexfrgxctrl3Spec;
impl crate::RegisterSpec for FlexfrgctrlFlexfrgxctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrgctrl_flexfrgxctrl3::R`](R) reader structure"]
impl crate::Readable for FlexfrgctrlFlexfrgxctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`flexfrgctrl_flexfrgxctrl3::W`](W) writer structure"]
impl crate::Writable for FlexfrgctrlFlexfrgxctrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEXFRGXCTRL3 to value 0"]
impl crate::Resettable for FlexfrgctrlFlexfrgxctrl3Spec {}

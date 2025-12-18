#[doc = "Register `HCCONTROLCURRENTED` reader"]
pub type R = crate::R<HccontrolcurrentedSpec>;
#[doc = "Register `HCCONTROLCURRENTED` writer"]
pub type W = crate::W<HccontrolcurrentedSpec>;
#[doc = "Field `CCED` reader - ControlCurrentED."]
pub type CcedR = crate::FieldReader<u32>;
#[doc = "Field `CCED` writer - ControlCurrentED."]
pub type CcedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&self) -> CcedR {
        CcedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCONTROLCURRENTED")
            .field("cced", &self.cced())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&mut self) -> CcedW<'_, HccontrolcurrentedSpec> {
        CcedW::new(self, 4)
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolcurrented::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolcurrented::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccontrolcurrentedSpec;
impl crate::RegisterSpec for HccontrolcurrentedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccontrolcurrented::R`](R) reader structure"]
impl crate::Readable for HccontrolcurrentedSpec {}
#[doc = "`write(|w| ..)` method takes [`hccontrolcurrented::W`](W) writer structure"]
impl crate::Writable for HccontrolcurrentedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCCONTROLCURRENTED to value 0"]
impl crate::Resettable for HccontrolcurrentedSpec {}

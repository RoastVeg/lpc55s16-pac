#[doc = "Register `HCCONTROLHEADED` reader"]
pub type R = crate::R<HccontrolheadedSpec>;
#[doc = "Register `HCCONTROLHEADED` writer"]
pub type W = crate::W<HccontrolheadedSpec>;
#[doc = "Field `CHED` reader - HC traverses the Control list starting with the HcControlHeadED pointer."]
pub type ChedR = crate::FieldReader<u32>;
#[doc = "Field `CHED` writer - HC traverses the Control list starting with the HcControlHeadED pointer."]
pub type ChedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub fn ched(&self) -> ChedR {
        ChedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub fn ched(&mut self) -> ChedW<'_, HccontrolheadedSpec> {
        ChedW::new(self, 4)
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolheaded::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolheaded::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccontrolheadedSpec;
impl crate::RegisterSpec for HccontrolheadedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccontrolheaded::R`](R) reader structure"]
impl crate::Readable for HccontrolheadedSpec {}
#[doc = "`write(|w| ..)` method takes [`hccontrolheaded::W`](W) writer structure"]
impl crate::Writable for HccontrolheadedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCCONTROLHEADED to value 0"]
impl crate::Resettable for HccontrolheadedSpec {}

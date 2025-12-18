#[doc = "Register `HCBULKCURRENTED` reader"]
pub type R = crate::R<HcbulkcurrentedSpec>;
#[doc = "Register `HCBULKCURRENTED` writer"]
pub type W = crate::W<HcbulkcurrentedSpec>;
#[doc = "Field `BCED` reader - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
pub type BcedR = crate::FieldReader<u32>;
#[doc = "Field `BCED` writer - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
pub type BcedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub fn bced(&self) -> BcedR {
        BcedR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCBULKCURRENTED")
            .field("bced", &self.bced())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub fn bced(&mut self) -> BcedW<'_, HcbulkcurrentedSpec> {
        BcedW::new(self, 4)
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkcurrented::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkcurrented::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbulkcurrentedSpec;
impl crate::RegisterSpec for HcbulkcurrentedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcbulkcurrented::R`](R) reader structure"]
impl crate::Readable for HcbulkcurrentedSpec {}
#[doc = "`write(|w| ..)` method takes [`hcbulkcurrented::W`](W) writer structure"]
impl crate::Writable for HcbulkcurrentedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCBULKCURRENTED to value 0"]
impl crate::Resettable for HcbulkcurrentedSpec {}

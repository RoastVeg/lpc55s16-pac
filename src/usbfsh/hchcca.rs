#[doc = "Register `HCHCCA` reader"]
pub type R = crate::R<HchccaSpec>;
#[doc = "Register `HCHCCA` writer"]
pub type W = crate::W<HchccaSpec>;
#[doc = "Field `HCCA` reader - Base address of the Host Controller Communication Area."]
pub type HccaR = crate::FieldReader<u32>;
#[doc = "Field `HCCA` writer - Base address of the Host Controller Communication Area."]
pub type HccaW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub fn hcca(&self) -> HccaR {
        HccaR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub fn hcca(&mut self) -> HccaW<'_, HchccaSpec> {
        HccaW::new(self, 8)
    }
}
#[doc = "Contains the physical address of the host controller communication area\n\nYou can [`read`](crate::Reg::read) this register and get [`hchcca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hchcca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HchccaSpec;
impl crate::RegisterSpec for HchccaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hchcca::R`](R) reader structure"]
impl crate::Readable for HchccaSpec {}
#[doc = "`write(|w| ..)` method takes [`hchcca::W`](W) writer structure"]
impl crate::Writable for HchccaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCHCCA to value 0"]
impl crate::Resettable for HchccaSpec {}

#[doc = "Register `SR_ENABLE0` reader"]
pub type R = crate::R<SrEnable0Spec>;
#[doc = "Register `SR_ENABLE0` writer"]
pub type W = crate::W<SrEnable0Spec>;
#[doc = "Field `EN` reader - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
pub type EnR = crate::FieldReader<u32>;
#[doc = "Field `EN` writer - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, SrEnable0Spec> {
        EnW::new(self, 0)
    }
}
#[doc = "Sub-Region Enable register for region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrEnable0Spec;
impl crate::RegisterSpec for SrEnable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_enable0::R`](R) reader structure"]
impl crate::Readable for SrEnable0Spec {}
#[doc = "`write(|w| ..)` method takes [`sr_enable0::W`](W) writer structure"]
impl crate::Writable for SrEnable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_ENABLE0 to value 0"]
impl crate::Resettable for SrEnable0Spec {}

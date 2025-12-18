#[doc = "Register `SR_ENABLE1` reader"]
pub type R = crate::R<SrEnable1Spec>;
#[doc = "Register `SR_ENABLE1` writer"]
pub type W = crate::W<SrEnable1Spec>;
#[doc = "Field `EN` reader - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
pub type EnR = crate::FieldReader<u32>;
#[doc = "Field `EN` writer - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_ENABLE1")
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, SrEnable1Spec> {
        EnW::new(self, 0)
    }
}
#[doc = "Sub-Region Enable register for region 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrEnable1Spec;
impl crate::RegisterSpec for SrEnable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_enable1::R`](R) reader structure"]
impl crate::Readable for SrEnable1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr_enable1::W`](W) writer structure"]
impl crate::Writable for SrEnable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_ENABLE1 to value 0"]
impl crate::Resettable for SrEnable1Spec {}

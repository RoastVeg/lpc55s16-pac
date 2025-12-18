#[doc = "Register `NS_FW_Version` reader"]
pub type R = crate::R<NsFwVersionSpec>;
#[doc = "Register `NS_FW_Version` writer"]
pub type W = crate::W<NsFwVersionSpec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, NsFwVersionSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Non-Secure firmware version (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`ns_fw_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ns_fw_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsFwVersionSpec;
impl crate::RegisterSpec for NsFwVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ns_fw_version::R`](R) reader structure"]
impl crate::Readable for NsFwVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`ns_fw_version::W`](W) writer structure"]
impl crate::Writable for NsFwVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NS_FW_Version to value 0"]
impl crate::Resettable for NsFwVersionSpec {}

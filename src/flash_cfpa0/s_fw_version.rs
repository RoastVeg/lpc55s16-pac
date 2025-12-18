#[doc = "Register `S_FW_Version` reader"]
pub type R = crate::R<SFwVersionSpec>;
#[doc = "Register `S_FW_Version` writer"]
pub type W = crate::W<SFwVersionSpec>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S_FW_Version")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, SFwVersionSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Secure firmware version (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_fw_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_fw_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFwVersionSpec;
impl crate::RegisterSpec for SFwVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_fw_version::R`](R) reader structure"]
impl crate::Readable for SFwVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`s_fw_version::W`](W) writer structure"]
impl crate::Writable for SFwVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S_FW_Version to value 0"]
impl crate::Resettable for SFwVersionSpec {}

#[doc = "Register `NXP_DEVICE_CERTIFICATE_0[%s]` reader"]
pub type R = crate::R<NxpDeviceCertificate0Spec>;
#[doc = "Register `NXP_DEVICE_CERTIFICATE_0[%s]` writer"]
pub type W = crate::W<NxpDeviceCertificate0Spec>;
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
    pub fn field(&mut self) -> FieldW<'_, NxpDeviceCertificate0Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_certificate_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_certificate_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NxpDeviceCertificate0Spec;
impl crate::RegisterSpec for NxpDeviceCertificate0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nxp_device_certificate_0::R`](R) reader structure"]
impl crate::Readable for NxpDeviceCertificate0Spec {}
#[doc = "`write(|w| ..)` method takes [`nxp_device_certificate_0::W`](W) writer structure"]
impl crate::Writable for NxpDeviceCertificate0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NXP_DEVICE_CERTIFICATE_0[%s] to value 0"]
impl crate::Resettable for NxpDeviceCertificate0Spec {}

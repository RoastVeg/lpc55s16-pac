#[doc = "Register `ECID_BACKUP_ARRAY2` reader"]
pub type R = crate::R<EcidBackupEcidBackupArray2Spec>;
#[doc = "Register `ECID_BACKUP_ARRAY2` writer"]
pub type W = crate::W<EcidBackupEcidBackupArray2Spec>;
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
    pub fn field(&mut self) -> FieldW<'_, EcidBackupEcidBackupArray2Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "ECID backup (the original is in page n-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_array2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_array2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcidBackupEcidBackupArray2Spec;
impl crate::RegisterSpec for EcidBackupEcidBackupArray2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecid_backup_ecid_backup_array2::R`](R) reader structure"]
impl crate::Readable for EcidBackupEcidBackupArray2Spec {}
#[doc = "`write(|w| ..)` method takes [`ecid_backup_ecid_backup_array2::W`](W) writer structure"]
impl crate::Writable for EcidBackupEcidBackupArray2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECID_BACKUP_ARRAY2 to value 0"]
impl crate::Resettable for EcidBackupEcidBackupArray2Spec {}

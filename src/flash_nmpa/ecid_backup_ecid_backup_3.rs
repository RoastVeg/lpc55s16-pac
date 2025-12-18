#[doc = "Register `ECID_BACKUP_3` reader"]
pub type R = crate::R<EcidBackupEcidBackup3Spec>;
#[doc = "Register `ECID_BACKUP_3` writer"]
pub type W = crate::W<EcidBackupEcidBackup3Spec>;
#[doc = "Field `LOTID_MSB` reader - no description available"]
pub type LotidMsbR = crate::FieldReader<u32>;
#[doc = "Field `LOTID_MSB` writer - no description available"]
pub type LotidMsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lotid_msb(&self) -> LotidMsbR {
        LotidMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lotid_msb(&mut self) -> LotidMsbW<'_, EcidBackupEcidBackup3Spec> {
        LotidMsbW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcidBackupEcidBackup3Spec;
impl crate::RegisterSpec for EcidBackupEcidBackup3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecid_backup_ecid_backup_3::R`](R) reader structure"]
impl crate::Readable for EcidBackupEcidBackup3Spec {}
#[doc = "`write(|w| ..)` method takes [`ecid_backup_ecid_backup_3::W`](W) writer structure"]
impl crate::Writable for EcidBackupEcidBackup3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECID_BACKUP_3 to value 0"]
impl crate::Resettable for EcidBackupEcidBackup3Spec {}

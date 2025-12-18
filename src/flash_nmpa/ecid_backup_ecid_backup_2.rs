#[doc = "Register `ECID_BACKUP_2` reader"]
pub type R = crate::R<EcidBackupEcidBackup2Spec>;
#[doc = "Register `ECID_BACKUP_2` writer"]
pub type W = crate::W<EcidBackupEcidBackup2Spec>;
#[doc = "Field `LOTID_LSB` reader - no description available"]
pub type LotidLsbR = crate::FieldReader<u32>;
#[doc = "Field `LOTID_LSB` writer - no description available"]
pub type LotidLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lotid_lsb(&self) -> LotidLsbR {
        LotidLsbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ECID_BACKUP_2")
            .field("lotid_lsb", &self.lotid_lsb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lotid_lsb(&mut self) -> LotidLsbW<'_, EcidBackupEcidBackup2Spec> {
        LotidLsbW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcidBackupEcidBackup2Spec;
impl crate::RegisterSpec for EcidBackupEcidBackup2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecid_backup_ecid_backup_2::R`](R) reader structure"]
impl crate::Readable for EcidBackupEcidBackup2Spec {}
#[doc = "`write(|w| ..)` method takes [`ecid_backup_ecid_backup_2::W`](W) writer structure"]
impl crate::Writable for EcidBackupEcidBackup2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECID_BACKUP_2 to value 0"]
impl crate::Resettable for EcidBackupEcidBackup2Spec {}

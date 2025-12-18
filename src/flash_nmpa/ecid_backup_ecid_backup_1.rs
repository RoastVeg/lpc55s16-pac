#[doc = "Register `ECID_BACKUP_1` reader"]
pub type R = crate::R<EcidBackupEcidBackup1Spec>;
#[doc = "Register `ECID_BACKUP_1` writer"]
pub type W = crate::W<EcidBackupEcidBackup1Spec>;
#[doc = "Field `WAFER` reader - no description available"]
pub type WaferR = crate::FieldReader;
#[doc = "Field `WAFER` writer - no description available"]
pub type WaferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn wafer(&self) -> WaferR {
        WaferR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ECID_BACKUP_1")
            .field("wafer", &self.wafer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn wafer(&mut self) -> WaferW<'_, EcidBackupEcidBackup1Spec> {
        WaferW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcidBackupEcidBackup1Spec;
impl crate::RegisterSpec for EcidBackupEcidBackup1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecid_backup_ecid_backup_1::R`](R) reader structure"]
impl crate::Readable for EcidBackupEcidBackup1Spec {}
#[doc = "`write(|w| ..)` method takes [`ecid_backup_ecid_backup_1::W`](W) writer structure"]
impl crate::Writable for EcidBackupEcidBackup1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECID_BACKUP_1 to value 0"]
impl crate::Resettable for EcidBackupEcidBackup1Spec {}

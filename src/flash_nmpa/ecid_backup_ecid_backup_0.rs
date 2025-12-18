#[doc = "Register `ECID_BACKUP_0` reader"]
pub type R = crate::R<EcidBackupEcidBackup0Spec>;
#[doc = "Register `ECID_BACKUP_0` writer"]
pub type W = crate::W<EcidBackupEcidBackup0Spec>;
#[doc = "Field `COORD_Y` reader - no description available"]
pub type CoordYR = crate::FieldReader<u16>;
#[doc = "Field `COORD_Y` writer - no description available"]
pub type CoordYW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COORD_X` reader - no description available"]
pub type CoordXR = crate::FieldReader<u16>;
#[doc = "Field `COORD_X` writer - no description available"]
pub type CoordXW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn coord_y(&self) -> CoordYR {
        CoordYR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn coord_x(&self) -> CoordXR {
        CoordXR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ECID_BACKUP_0")
            .field("coord_y", &self.coord_y())
            .field("coord_x", &self.coord_x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn coord_y(&mut self) -> CoordYW<'_, EcidBackupEcidBackup0Spec> {
        CoordYW::new(self, 0)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn coord_x(&mut self) -> CoordXW<'_, EcidBackupEcidBackup0Spec> {
        CoordXW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcidBackupEcidBackup0Spec;
impl crate::RegisterSpec for EcidBackupEcidBackup0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecid_backup_ecid_backup_0::R`](R) reader structure"]
impl crate::Readable for EcidBackupEcidBackup0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecid_backup_ecid_backup_0::W`](W) writer structure"]
impl crate::Writable for EcidBackupEcidBackup0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECID_BACKUP_0 to value 0"]
impl crate::Resettable for EcidBackupEcidBackup0Spec {}

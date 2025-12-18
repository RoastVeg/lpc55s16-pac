#[doc = "Register `ROTKH_REVOKE` reader"]
pub type R = crate::R<RotkhRevokeSpec>;
#[doc = "Register `ROTKH_REVOKE` writer"]
pub type W = crate::W<RotkhRevokeSpec>;
#[doc = "Field `RoTK0_EN` reader - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk0EnR = crate::FieldReader;
#[doc = "Field `RoTK0_EN` writer - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk0EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RoTK1_EN` reader - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk1EnR = crate::FieldReader;
#[doc = "Field `RoTK1_EN` writer - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk1EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RoTK2_EN` reader - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk2EnR = crate::FieldReader;
#[doc = "Field `RoTK2_EN` writer - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk2EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RoTK3_EN` reader - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk3EnR = crate::FieldReader;
#[doc = "Field `RoTK3_EN` writer - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RoTk3EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk0_en(&self) -> RoTk0EnR {
        RoTk0EnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk1_en(&self) -> RoTk1EnR {
        RoTk1EnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk2_en(&self) -> RoTk2EnR {
        RoTk2EnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk3_en(&self) -> RoTk3EnR {
        RoTk3EnR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk0_en(&mut self) -> RoTk0EnW<'_, RotkhRevokeSpec> {
        RoTk0EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk1_en(&mut self) -> RoTk1EnW<'_, RotkhRevokeSpec> {
        RoTk1EnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk2_en(&mut self) -> RoTk2EnW<'_, RotkhRevokeSpec> {
        RoTk2EnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk3_en(&mut self) -> RoTk3EnW<'_, RotkhRevokeSpec> {
        RoTk3EnW::new(self, 6)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`rotkh_revoke::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotkh_revoke::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RotkhRevokeSpec;
impl crate::RegisterSpec for RotkhRevokeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rotkh_revoke::R`](R) reader structure"]
impl crate::Readable for RotkhRevokeSpec {}
#[doc = "`write(|w| ..)` method takes [`rotkh_revoke::W`](W) writer structure"]
impl crate::Writable for RotkhRevokeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROTKH_REVOKE to value 0"]
impl crate::Resettable for RotkhRevokeSpec {}

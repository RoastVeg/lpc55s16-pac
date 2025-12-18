#[doc = "Register `GPO1_1` reader"]
pub type R = crate::R<Gpo1Gpo1_1Spec>;
#[doc = "Register `GPO1_1` writer"]
pub type W = crate::W<Gpo1Gpo1_1Spec>;
#[doc = "Field `ROM_PATCH_VERSION` reader - ROM Patch Version \\[3:0\\]"]
pub type RomPatchVersionR = crate::FieldReader;
#[doc = "Field `ROM_PATCH_VERSION` writer - ROM Patch Version \\[3:0\\]"]
pub type RomPatchVersionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUSTOMER_REVISION_ID` reader - CUSTOMER REVISION ID\\[3:0\\]"]
pub type CustomerRevisionIdR = crate::FieldReader;
#[doc = "Field `CUSTOMER_REVISION_ID` writer - CUSTOMER REVISION ID\\[3:0\\]"]
pub type CustomerRevisionIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - ROM Patch Version \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_patch_version(&self) -> RomPatchVersionR {
        RomPatchVersionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CUSTOMER REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn customer_revision_id(&self) -> CustomerRevisionIdR {
        CustomerRevisionIdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_GPO1_1")
            .field("rom_patch_version", &self.rom_patch_version())
            .field("customer_revision_id", &self.customer_revision_id())
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ROM Patch Version \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_patch_version(&mut self) -> RomPatchVersionW<'_, Gpo1Gpo1_1Spec> {
        RomPatchVersionW::new(self, 0)
    }
    #[doc = "Bits 4:7 - CUSTOMER REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn customer_revision_id(&mut self) -> CustomerRevisionIdW<'_, Gpo1Gpo1_1Spec> {
        CustomerRevisionIdW::new(self, 4)
    }
    #[doc = "Bits 8:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo1Gpo1_1Spec> {
        FieldW::new(self, 8)
    }
}
#[doc = "GPO1 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo1Gpo1_1Spec;
impl crate::RegisterSpec for Gpo1Gpo1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo1_gpo1_1::R`](R) reader structure"]
impl crate::Readable for Gpo1Gpo1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo1_gpo1_1::W`](W) writer structure"]
impl crate::Writable for Gpo1Gpo1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO1_1 to value 0"]
impl crate::Resettable for Gpo1Gpo1_1Spec {}

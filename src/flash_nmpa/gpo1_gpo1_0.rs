#[doc = "Register `GPO1_0` reader"]
pub type R = crate::R<Gpo1Gpo1_0Spec>;
#[doc = "Register `GPO1_0` writer"]
pub type W = crate::W<Gpo1Gpo1_0Spec>;
#[doc = "Field `FINAL_TEST_NOT_DONE` reader - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FinalTestNotDoneR = crate::FieldReader;
#[doc = "Field `FINAL_TEST_NOT_DONE` writer - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FinalTestNotDoneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PARTCONFIG` reader - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
pub type PartconfigR = crate::FieldReader;
#[doc = "Field `PARTCONFIG` writer - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
pub type PartconfigW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DEVICE_TYPE_SEC` reader - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DeviceTypeSecR = crate::BitReader;
#[doc = "Field `DEVICE_TYPE_SEC` writer - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DeviceTypeSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_SIZE` reader - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
pub type SramSizeR = crate::FieldReader;
#[doc = "Field `SRAM_SIZE` writer - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
pub type SramSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` reader - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type Cpu0SecurityExtensionDisableR = crate::FieldReader;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` writer - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type Cpu0SecurityExtensionDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROM_REVISION_MINOR` reader - ROM Revision-Minor \\[3:0\\]"]
pub type RomRevisionMinorR = crate::FieldReader;
#[doc = "Field `ROM_REVISION_MINOR` writer - ROM Revision-Minor \\[3:0\\]"]
pub type RomRevisionMinorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `METAL_REVISION_ID` reader - METAL REVISION ID\\[3:0\\]"]
pub type MetalRevisionIdR = crate::FieldReader;
#[doc = "Field `METAL_REVISION_ID` writer - METAL REVISION ID\\[3:0\\]"]
pub type MetalRevisionIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&self) -> FinalTestNotDoneR {
        FinalTestNotDoneR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:10 - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[inline(always)]
    pub fn partconfig(&self) -> PartconfigR {
        PartconfigR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&self) -> DeviceTypeSecR {
        DeviceTypeSecR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
    #[inline(always)]
    pub fn sram_size(&self) -> SramSizeR {
        SramSizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&self) -> Cpu0SecurityExtensionDisableR {
        Cpu0SecurityExtensionDisableR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ROM Revision-Minor \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_revision_minor(&self) -> RomRevisionMinorR {
        RomRevisionMinorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - METAL REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn metal_revision_id(&self) -> MetalRevisionIdR {
        MetalRevisionIdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_GPO1_0")
            .field("final_test_not_done", &self.final_test_not_done())
            .field("partconfig", &self.partconfig())
            .field("device_type_sec", &self.device_type_sec())
            .field("sram_size", &self.sram_size())
            .field(
                "cpu0_security_extension_disable",
                &self.cpu0_security_extension_disable(),
            )
            .field("field", &self.field())
            .field("rom_revision_minor", &self.rom_revision_minor())
            .field("metal_revision_id", &self.metal_revision_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&mut self) -> FinalTestNotDoneW<'_, Gpo1Gpo1_0Spec> {
        FinalTestNotDoneW::new(self, 0)
    }
    #[doc = "Bits 4:10 - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[inline(always)]
    pub fn partconfig(&mut self) -> PartconfigW<'_, Gpo1Gpo1_0Spec> {
        PartconfigW::new(self, 4)
    }
    #[doc = "Bit 11 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&mut self) -> DeviceTypeSecW<'_, Gpo1Gpo1_0Spec> {
        DeviceTypeSecW::new(self, 11)
    }
    #[doc = "Bits 12:15 - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
    #[inline(always)]
    pub fn sram_size(&mut self) -> SramSizeW<'_, Gpo1Gpo1_0Spec> {
        SramSizeW::new(self, 12)
    }
    #[doc = "Bits 16:19 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(
        &mut self,
    ) -> Cpu0SecurityExtensionDisableW<'_, Gpo1Gpo1_0Spec> {
        Cpu0SecurityExtensionDisableW::new(self, 16)
    }
    #[doc = "Bits 20:23 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo1Gpo1_0Spec> {
        FieldW::new(self, 20)
    }
    #[doc = "Bits 24:27 - ROM Revision-Minor \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_revision_minor(&mut self) -> RomRevisionMinorW<'_, Gpo1Gpo1_0Spec> {
        RomRevisionMinorW::new(self, 24)
    }
    #[doc = "Bits 28:31 - METAL REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn metal_revision_id(&mut self) -> MetalRevisionIdW<'_, Gpo1Gpo1_0Spec> {
        MetalRevisionIdW::new(self, 28)
    }
}
#[doc = "GPO1 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo1Gpo1_0Spec;
impl crate::RegisterSpec for Gpo1Gpo1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo1_gpo1_0::R`](R) reader structure"]
impl crate::Readable for Gpo1Gpo1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo1_gpo1_0::W`](W) writer structure"]
impl crate::Writable for Gpo1Gpo1_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO1_0 to value 0"]
impl crate::Resettable for Gpo1Gpo1_0Spec {}

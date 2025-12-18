#[doc = "Register `DEVCMDSTAT` reader"]
pub type R = crate::R<DevcmdstatSpec>;
#[doc = "Register `DEVCMDSTAT` writer"]
pub type W = crate::W<DevcmdstatSpec>;
#[doc = "Field `DEV_ADDR` reader - USB device address."]
pub type DevAddrR = crate::FieldReader;
#[doc = "Field `DEV_ADDR` writer - USB device address."]
pub type DevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DEV_EN` reader - USB device enable."]
pub type DevEnR = crate::BitReader;
#[doc = "Field `DEV_EN` writer - USB device enable."]
pub type DevEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP` reader - SETUP token received."]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP token received."]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NEEDCLK` reader - Forces the NEEDCLK output to always be on:."]
pub type ForceNeedclkR = crate::BitReader;
#[doc = "Field `FORCE_NEEDCLK` writer - Forces the NEEDCLK output to always be on:."]
pub type ForceNeedclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_SUP` reader - LPM Supported:."]
pub type LpmSupR = crate::BitReader;
#[doc = "Field `LPM_SUP` writer - LPM Supported:."]
pub type LpmSupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP:."]
pub type IntonnakAoR = crate::BitReader;
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP:."]
pub type IntonnakAoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP:."]
pub type IntonnakAiR = crate::BitReader;
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP:."]
pub type IntonnakAiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP:."]
pub type IntonnakCoR = crate::BitReader;
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP:."]
pub type IntonnakCoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP:."]
pub type IntonnakCiR = crate::BitReader;
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP:."]
pub type IntonnakCiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCON` reader - Device status - connect."]
pub type DconR = crate::BitReader;
#[doc = "Field `DCON` writer - Device status - connect."]
pub type DconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSUS` reader - Device status - suspend."]
pub type DsusR = crate::BitReader;
#[doc = "Field `DSUS` writer - Device status - suspend."]
pub type DsusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend."]
pub type LpmSusR = crate::BitReader;
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend."]
pub type LpmSusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host."]
pub type LpmRewpR = crate::BitReader;
#[doc = "Field `Speed` reader - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `DCON_C` reader - Device status - connect change."]
pub type DconCR = crate::BitReader;
#[doc = "Field `DCON_C` writer - Device status - connect change."]
pub type DconCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSUS_C` reader - Device status - suspend change."]
pub type DsusCR = crate::BitReader;
#[doc = "Field `DSUS_C` writer - Device status - suspend change."]
pub type DsusCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRES_C` reader - Device status - reset change."]
pub type DresCR = crate::BitReader;
#[doc = "Field `DRES_C` writer - Device status - reset change."]
pub type DresCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_DEBOUNCED` reader - This bit indicates if VBUS is detected or not."]
pub type VbusDebouncedR = crate::BitReader;
#[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PhyTestMode {
    #[doc = "0: Test mode disabled."]
    Disable = 0,
    #[doc = "1: Test_J."]
    TestJ = 1,
    #[doc = "2: Test_K."]
    TestK = 2,
    #[doc = "3: Test_SE0_NAK."]
    TestSe0Nak = 3,
    #[doc = "4: Test_Packet."]
    TestPacket = 4,
    #[doc = "5: Test_Force_Enable."]
    TestForceEnable = 5,
}
impl From<PhyTestMode> for u8 {
    #[inline(always)]
    fn from(variant: PhyTestMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PhyTestMode {
    type Ux = u8;
}
impl crate::IsEnum for PhyTestMode {}
#[doc = "Field `PHY_TEST_MODE` reader - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
pub type PhyTestModeR = crate::FieldReader<PhyTestMode>;
impl PhyTestModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PhyTestMode> {
        match self.bits {
            0 => Some(PhyTestMode::Disable),
            1 => Some(PhyTestMode::TestJ),
            2 => Some(PhyTestMode::TestK),
            3 => Some(PhyTestMode::TestSe0Nak),
            4 => Some(PhyTestMode::TestPacket),
            5 => Some(PhyTestMode::TestForceEnable),
            _ => None,
        }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PhyTestMode::Disable
    }
    #[doc = "Test_J."]
    #[inline(always)]
    pub fn is_test_j(&self) -> bool {
        *self == PhyTestMode::TestJ
    }
    #[doc = "Test_K."]
    #[inline(always)]
    pub fn is_test_k(&self) -> bool {
        *self == PhyTestMode::TestK
    }
    #[doc = "Test_SE0_NAK."]
    #[inline(always)]
    pub fn is_test_se0_nak(&self) -> bool {
        *self == PhyTestMode::TestSe0Nak
    }
    #[doc = "Test_Packet."]
    #[inline(always)]
    pub fn is_test_packet(&self) -> bool {
        *self == PhyTestMode::TestPacket
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn is_test_force_enable(&self) -> bool {
        *self == PhyTestMode::TestForceEnable
    }
}
#[doc = "Field `PHY_TEST_MODE` writer - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
pub type PhyTestModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, PhyTestMode>;
impl<'a, REG> PhyTestModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::Disable)
    }
    #[doc = "Test_J."]
    #[inline(always)]
    pub fn test_j(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::TestJ)
    }
    #[doc = "Test_K."]
    #[inline(always)]
    pub fn test_k(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::TestK)
    }
    #[doc = "Test_SE0_NAK."]
    #[inline(always)]
    pub fn test_se0_nak(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::TestSe0Nak)
    }
    #[doc = "Test_Packet."]
    #[inline(always)]
    pub fn test_packet(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::TestPacket)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn test_force_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PhyTestMode::TestForceEnable)
    }
}
impl R {
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DevAddrR {
        DevAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    pub fn dev_en(&self) -> DevEnR {
        DevEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn force_needclk(&self) -> ForceNeedclkR {
        ForceNeedclkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LpmSupR {
        LpmSupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> IntonnakAoR {
        IntonnakAoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> IntonnakAiR {
        IntonnakAiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn intonnak_co(&self) -> IntonnakCoR {
        IntonnakCoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> IntonnakCiR {
        IntonnakCiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&self) -> DconR {
        DconR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    pub fn dsus(&self) -> DsusR {
        DsusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LpmSusR {
        LpmSusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LpmRewpR {
        LpmRewpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DconCR {
        DconCR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DsusCR {
        DsusCR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&self) -> DresCR {
        DresCR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub fn vbus_debounced(&self) -> VbusDebouncedR {
        VbusDebouncedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub fn phy_test_mode(&self) -> PhyTestModeR {
        PhyTestModeR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVCMDSTAT")
            .field("dev_addr", &self.dev_addr())
            .field("dev_en", &self.dev_en())
            .field("setup", &self.setup())
            .field("force_needclk", &self.force_needclk())
            .field("lpm_sup", &self.lpm_sup())
            .field("intonnak_ao", &self.intonnak_ao())
            .field("intonnak_ai", &self.intonnak_ai())
            .field("intonnak_co", &self.intonnak_co())
            .field("intonnak_ci", &self.intonnak_ci())
            .field("dcon", &self.dcon())
            .field("dsus", &self.dsus())
            .field("lpm_sus", &self.lpm_sus())
            .field("lpm_rewp", &self.lpm_rewp())
            .field("speed", &self.speed())
            .field("dcon_c", &self.dcon_c())
            .field("dsus_c", &self.dsus_c())
            .field("dres_c", &self.dres_c())
            .field("vbus_debounced", &self.vbus_debounced())
            .field("phy_test_mode", &self.phy_test_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DevAddrW<'_, DevcmdstatSpec> {
        DevAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    pub fn dev_en(&mut self) -> DevEnW<'_, DevcmdstatSpec> {
        DevEnW::new(self, 7)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&mut self) -> SetupW<'_, DevcmdstatSpec> {
        SetupW::new(self, 8)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn force_needclk(&mut self) -> ForceNeedclkW<'_, DevcmdstatSpec> {
        ForceNeedclkW::new(self, 9)
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> LpmSupW<'_, DevcmdstatSpec> {
        LpmSupW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> IntonnakAoW<'_, DevcmdstatSpec> {
        IntonnakAoW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> IntonnakAiW<'_, DevcmdstatSpec> {
        IntonnakAiW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> IntonnakCoW<'_, DevcmdstatSpec> {
        IntonnakCoW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> IntonnakCiW<'_, DevcmdstatSpec> {
        IntonnakCiW::new(self, 15)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&mut self) -> DconW<'_, DevcmdstatSpec> {
        DconW::new(self, 16)
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    pub fn dsus(&mut self) -> DsusW<'_, DevcmdstatSpec> {
        DsusW::new(self, 17)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> LpmSusW<'_, DevcmdstatSpec> {
        LpmSusW::new(self, 19)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&mut self) -> DconCW<'_, DevcmdstatSpec> {
        DconCW::new(self, 24)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&mut self) -> DsusCW<'_, DevcmdstatSpec> {
        DsusCW::new(self, 25)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&mut self) -> DresCW<'_, DevcmdstatSpec> {
        DresCW::new(self, 26)
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub fn phy_test_mode(&mut self) -> PhyTestModeW<'_, DevcmdstatSpec> {
        PhyTestModeW::new(self, 29)
    }
}
#[doc = "USB Device Command/Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`devcmdstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcmdstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevcmdstatSpec;
impl crate::RegisterSpec for DevcmdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devcmdstat::R`](R) reader structure"]
impl crate::Readable for DevcmdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`devcmdstat::W`](W) writer structure"]
impl crate::Writable for DevcmdstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DevcmdstatSpec {
    const RESET_VALUE: u32 = 0x0800;
}

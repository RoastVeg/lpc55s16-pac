#[doc = "Register `PUF_SRAM` reader"]
pub type R = crate::R<PufSramSpec>;
#[doc = "Register `PUF_SRAM` writer"]
pub type W = crate::W<PufSramSpec>;
#[doc = "Field `PUF_SRAM_VALID` reader - 1: PUF_SRAM is valid."]
pub type PufSramValidR = crate::BitReader;
#[doc = "Field `PUF_SRAM_VALID` writer - 1: PUF_SRAM is valid."]
pub type PufSramValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mode` reader - PUF SRAM Controller operating mode"]
pub type ModeR = crate::BitReader;
#[doc = "Field `mode` writer - PUF SRAM Controller operating mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ckgating` reader - PUF SRAM Clock Gating control"]
pub type CkgatingR = crate::BitReader;
#[doc = "Field `ckgating` writer - PUF SRAM Clock Gating control"]
pub type CkgatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Source Biasing voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smb {
    #[doc = "0: Low leakage."]
    Low = 0,
    #[doc = "1: Medium leakage."]
    Medium = 1,
    #[doc = "2: Highest leakage."]
    Highest = 2,
    #[doc = "3: Disable."]
    Disable = 3,
}
impl From<Smb> for u8 {
    #[inline(always)]
    fn from(variant: Smb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smb {
    type Ux = u8;
}
impl crate::IsEnum for Smb {}
#[doc = "Field `SMB` reader - Source Biasing voltage."]
pub type SmbR = crate::FieldReader<Smb>;
impl SmbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smb {
        match self.bits {
            0 => Smb::Low,
            1 => Smb::Medium,
            2 => Smb::Highest,
            3 => Smb::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Smb::Low
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Smb::Medium
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == Smb::Highest
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Smb::Disable
    }
}
#[doc = "Field `SMB` writer - Source Biasing voltage."]
pub type SmbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Smb, crate::Safe>;
impl<'a, REG> SmbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Low)
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Medium)
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Highest)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Disable)
    }
}
#[doc = "Field `RM` reader - Read Margin control settings."]
pub type RmR = crate::FieldReader;
#[doc = "Field `RM` writer - Read Margin control settings."]
pub type RmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WM` reader - Write Margin control settings."]
pub type WmR = crate::FieldReader;
#[doc = "Field `WM` writer - Write Margin control settings."]
pub type WmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRME` reader - Write read margin enable."]
pub type WrmeR = crate::BitReader;
#[doc = "Field `WRME` writer - Write read margin enable."]
pub type WrmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAEN` reader - SRAM Read Assist Enable"]
pub type RaenR = crate::BitReader;
#[doc = "Field `RAEN` writer - SRAM Read Assist Enable"]
pub type RaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM` reader - SRAM Read Assist settings"]
pub type RamR = crate::FieldReader;
#[doc = "Field `RAM` writer - SRAM Read Assist settings"]
pub type RamW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAEN` reader - SRAM Write Assist Enable"]
pub type WaenR = crate::BitReader;
#[doc = "Field `WAEN` writer - SRAM Write Assist Enable"]
pub type WaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAM` reader - SRAM Write Assist settings"]
pub type WamR = crate::FieldReader;
#[doc = "Field `WAM` writer - SRAM Write Assist settings"]
pub type WamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STBP` reader - STBP"]
pub type StbpR = crate::BitReader;
#[doc = "Field `STBP` writer - STBP"]
pub type StbpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: PUF_SRAM is valid."]
    #[inline(always)]
    pub fn puf_sram_valid(&self) -> PufSramValidR {
        PufSramValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PUF SRAM Controller operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&self) -> CkgatingR {
        CkgatingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&self) -> SmbR {
        SmbR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&self) -> WrmeR {
        WrmeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM Read Assist Enable"]
    #[inline(always)]
    pub fn raen(&self) -> RaenR {
        RaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - SRAM Read Assist settings"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - SRAM Write Assist Enable"]
    #[inline(always)]
    pub fn waen(&self) -> WaenR {
        WaenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - SRAM Write Assist settings"]
    #[inline(always)]
    pub fn wam(&self) -> WamR {
        WamR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - STBP"]
    #[inline(always)]
    pub fn stbp(&self) -> StbpR {
        StbpR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: PUF_SRAM is valid."]
    #[inline(always)]
    pub fn puf_sram_valid(&mut self) -> PufSramValidW<'_, PufSramSpec> {
        PufSramValidW::new(self, 0)
    }
    #[doc = "Bit 1 - PUF SRAM Controller operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, PufSramSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&mut self) -> CkgatingW<'_, PufSramSpec> {
        CkgatingW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&mut self) -> SmbW<'_, PufSramSpec> {
        SmbW::new(self, 8)
    }
    #[doc = "Bits 10:12 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<'_, PufSramSpec> {
        RmW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&mut self) -> WmW<'_, PufSramSpec> {
        WmW::new(self, 13)
    }
    #[doc = "Bit 16 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&mut self) -> WrmeW<'_, PufSramSpec> {
        WrmeW::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM Read Assist Enable"]
    #[inline(always)]
    pub fn raen(&mut self) -> RaenW<'_, PufSramSpec> {
        RaenW::new(self, 17)
    }
    #[doc = "Bits 18:21 - SRAM Read Assist settings"]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<'_, PufSramSpec> {
        RamW::new(self, 18)
    }
    #[doc = "Bit 22 - SRAM Write Assist Enable"]
    #[inline(always)]
    pub fn waen(&mut self) -> WaenW<'_, PufSramSpec> {
        WaenW::new(self, 22)
    }
    #[doc = "Bits 23:24 - SRAM Write Assist settings"]
    #[inline(always)]
    pub fn wam(&mut self) -> WamW<'_, PufSramSpec> {
        WamW::new(self, 23)
    }
    #[doc = "Bit 25 - STBP"]
    #[inline(always)]
    pub fn stbp(&mut self) -> StbpW<'_, PufSramSpec> {
        StbpW::new(self, 25)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_sram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_sram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PufSramSpec;
impl crate::RegisterSpec for PufSramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`puf_sram::R`](R) reader structure"]
impl crate::Readable for PufSramSpec {}
#[doc = "`write(|w| ..)` method takes [`puf_sram::W`](W) writer structure"]
impl crate::Writable for PufSramSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUF_SRAM to value 0"]
impl crate::Resettable for PufSramSpec {}

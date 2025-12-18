#[doc = "Register `CC_SOCU_PIN` reader"]
pub type R = crate::R<CcSocuPinSpec>;
#[doc = "Register `CC_SOCU_PIN` writer"]
pub type W = crate::W<CcSocuPinSpec>;
#[doc = "Non Secure non-invasive debug enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - Non Secure non-invasive debug enable"]
pub type NidenR = crate::BitReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            false => Niden::Enable,
            true => Niden::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Niden::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Niden::Disable
    }
}
#[doc = "Field `NIDEN` writer - Non Secure non-invasive debug enable"]
pub type NidenW<'a, REG> = crate::BitWriter<'a, REG, Niden>;
impl<'a, REG> NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disable)
    }
}
#[doc = "Non Secure debug enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Non Secure debug enable"]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::Enable,
            true => Dbgen::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbgen::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbgen::Disable
    }
}
#[doc = "Field `DBGEN` writer - Non Secure debug enable"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disable)
    }
}
#[doc = "Secure non-invasive debug enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spniden {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<Spniden> for bool {
    #[inline(always)]
    fn from(variant: Spniden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug enable"]
pub type SpnidenR = crate::BitReader<Spniden>;
impl SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spniden {
        match self.bits {
            false => Spniden::Enable,
            true => Spniden::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spniden::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spniden::Disable
    }
}
#[doc = "Field `SPNIDEN` writer - Secure non-invasive debug enable"]
pub type SpnidenW<'a, REG> = crate::BitWriter<'a, REG, Spniden>;
impl<'a, REG> SpnidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disable)
    }
}
#[doc = "Secure invasive debug enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiden {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<Spiden> for bool {
    #[inline(always)]
    fn from(variant: Spiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug enable"]
pub type SpidenR = crate::BitReader<Spiden>;
impl SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiden {
        match self.bits {
            false => Spiden::Enable,
            true => Spiden::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spiden::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spiden::Disable
    }
}
#[doc = "Field `SPIDEN` writer - Secure invasive debug enable"]
pub type SpidenW<'a, REG> = crate::BitWriter<'a, REG, Spiden>;
impl<'a, REG> SpidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disable)
    }
}
#[doc = "JTAG TAP enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapen {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<Tapen> for bool {
    #[inline(always)]
    fn from(variant: Tapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - JTAG TAP enable"]
pub type TapenR = crate::BitReader<Tapen>;
impl TapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapen {
        match self.bits {
            false => Tapen::Enable,
            true => Tapen::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tapen::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tapen::Disable
    }
}
#[doc = "Field `TAPEN` writer - JTAG TAP enable"]
pub type TapenW<'a, REG> = crate::BitWriter<'a, REG, Tapen>;
impl<'a, REG> TapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Disable)
    }
}
#[doc = "ISP Boot Command enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspCmdEn {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<IspCmdEn> for bool {
    #[inline(always)]
    fn from(variant: IspCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CMD_EN` reader - ISP Boot Command enable"]
pub type IspCmdEnR = crate::BitReader<IspCmdEn>;
impl IspCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspCmdEn {
        match self.bits {
            false => IspCmdEn::Enable,
            true => IspCmdEn::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IspCmdEn::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IspCmdEn::Disable
    }
}
#[doc = "Field `ISP_CMD_EN` writer - ISP Boot Command enable"]
pub type IspCmdEnW<'a, REG> = crate::BitWriter<'a, REG, IspCmdEn>;
impl<'a, REG> IspCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Disable)
    }
}
#[doc = "FA Command enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaMeCmdEn {
    #[doc = "0: Use DAP to enable"]
    Enable = 0,
    #[doc = "1: Fixed state"]
    Disable = 1,
}
impl From<FaMeCmdEn> for bool {
    #[inline(always)]
    fn from(variant: FaMeCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FA_ME_CMD_EN` reader - FA Command enable"]
pub type FaMeCmdEnR = crate::BitReader<FaMeCmdEn>;
impl FaMeCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FaMeCmdEn {
        match self.bits {
            false => FaMeCmdEn::Enable,
            true => FaMeCmdEn::Disable,
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FaMeCmdEn::Enable
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FaMeCmdEn::Disable
    }
}
#[doc = "Field `FA_ME_CMD_EN` writer - FA Command enable"]
pub type FaMeCmdEnW<'a, REG> = crate::BitWriter<'a, REG, FaMeCmdEn>;
impl<'a, REG> FaMeCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FaMeCmdEn::Enable)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FaMeCmdEn::Disable)
    }
}
#[doc = "Field `UUID_CHECK` reader - Enforce UUID match during Debug authentication."]
pub type UuidCheckR = crate::BitReader;
#[doc = "Field `UUID_CHECK` writer - Enforce UUID match during Debug authentication."]
pub type UuidCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type InverseValueR = crate::FieldReader<u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type InverseValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn spniden(&self) -> SpnidenR {
        SpnidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline(always)]
    pub fn spiden(&self) -> SpidenR {
        SpidenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline(always)]
    pub fn tapen(&self) -> TapenR {
        TapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline(always)]
    pub fn isp_cmd_en(&self) -> IspCmdEnR {
        IspCmdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline(always)]
    pub fn fa_me_cmd_en(&self) -> FaMeCmdEnR {
        FaMeCmdEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub fn uuid_check(&self) -> UuidCheckR {
        UuidCheckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> InverseValueR {
        InverseValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC_SOCU_PIN")
            .field("niden", &self.niden())
            .field("dbgen", &self.dbgen())
            .field("spniden", &self.spniden())
            .field("spiden", &self.spiden())
            .field("tapen", &self.tapen())
            .field("isp_cmd_en", &self.isp_cmd_en())
            .field("fa_me_cmd_en", &self.fa_me_cmd_en())
            .field("uuid_check", &self.uuid_check())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn niden(&mut self) -> NidenW<'_, CcSocuPinSpec> {
        NidenW::new(self, 0)
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, CcSocuPinSpec> {
        DbgenW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SpnidenW<'_, CcSocuPinSpec> {
        SpnidenW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SpidenW<'_, CcSocuPinSpec> {
        SpidenW::new(self, 3)
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TapenW<'_, CcSocuPinSpec> {
        TapenW::new(self, 4)
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline(always)]
    pub fn isp_cmd_en(&mut self) -> IspCmdEnW<'_, CcSocuPinSpec> {
        IspCmdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline(always)]
    pub fn fa_me_cmd_en(&mut self) -> FaMeCmdEnW<'_, CcSocuPinSpec> {
        FaMeCmdEnW::new(self, 7)
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub fn uuid_check(&mut self) -> UuidCheckW<'_, CcSocuPinSpec> {
        UuidCheckW::new(self, 15)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> InverseValueW<'_, CcSocuPinSpec> {
        InverseValueW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_socu_pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_socu_pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSocuPinSpec;
impl crate::RegisterSpec for CcSocuPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_socu_pin::R`](R) reader structure"]
impl crate::Readable for CcSocuPinSpec {}
#[doc = "`write(|w| ..)` method takes [`cc_socu_pin::W`](W) writer structure"]
impl crate::Writable for CcSocuPinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC_SOCU_PIN to value 0"]
impl crate::Resettable for CcSocuPinSpec {}

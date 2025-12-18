#[doc = "Register `DCFG_CC_SOCU_DFLT` reader"]
pub type R = crate::R<DcfgCcSocuDfltSpec>;
#[doc = "Register `DCFG_CC_SOCU_DFLT` writer"]
pub type W = crate::W<DcfgCcSocuDfltSpec>;
#[doc = "Non Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - Non Secure non-invasive debug fixed state"]
pub type NidenR = crate::BitReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            false => Niden::Disable,
            true => Niden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Niden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Niden::Enable
    }
}
#[doc = "Field `NIDEN` writer - Non Secure non-invasive debug fixed state"]
pub type NidenW<'a, REG> = crate::BitWriter<'a, REG, Niden>;
impl<'a, REG> NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Enable)
    }
}
#[doc = "Non Secure debug fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Non Secure debug fixed state"]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::Disable,
            true => Dbgen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbgen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbgen::Enable
    }
}
#[doc = "Field `DBGEN` writer - Non Secure debug fixed state"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Enable)
    }
}
#[doc = "Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spniden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Spniden> for bool {
    #[inline(always)]
    fn from(variant: Spniden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug fixed state"]
pub type SpnidenR = crate::BitReader<Spniden>;
impl SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spniden {
        match self.bits {
            false => Spniden::Disable,
            true => Spniden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spniden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spniden::Enable
    }
}
#[doc = "Field `SPNIDEN` writer - Secure non-invasive debug fixed state"]
pub type SpnidenW<'a, REG> = crate::BitWriter<'a, REG, Spniden>;
impl<'a, REG> SpnidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Enable)
    }
}
#[doc = "Secure invasive debug fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Spiden> for bool {
    #[inline(always)]
    fn from(variant: Spiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug fixed state"]
pub type SpidenR = crate::BitReader<Spiden>;
impl SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiden {
        match self.bits {
            false => Spiden::Disable,
            true => Spiden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spiden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spiden::Enable
    }
}
#[doc = "Field `SPIDEN` writer - Secure invasive debug fixed state"]
pub type SpidenW<'a, REG> = crate::BitWriter<'a, REG, Spiden>;
impl<'a, REG> SpidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Enable)
    }
}
#[doc = "JTAG TAP fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Tapen> for bool {
    #[inline(always)]
    fn from(variant: Tapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - JTAG TAP fixed state"]
pub type TapenR = crate::BitReader<Tapen>;
impl TapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapen {
        match self.bits {
            false => Tapen::Disable,
            true => Tapen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tapen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tapen::Enable
    }
}
#[doc = "Field `TAPEN` writer - JTAG TAP fixed state"]
pub type TapenW<'a, REG> = crate::BitWriter<'a, REG, Tapen>;
impl<'a, REG> TapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Enable)
    }
}
#[doc = "ISP Boot Command fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspCmdEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<IspCmdEn> for bool {
    #[inline(always)]
    fn from(variant: IspCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CMD_EN` reader - ISP Boot Command fixed state"]
pub type IspCmdEnR = crate::BitReader<IspCmdEn>;
impl IspCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspCmdEn {
        match self.bits {
            false => IspCmdEn::Disable,
            true => IspCmdEn::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IspCmdEn::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IspCmdEn::Enable
    }
}
#[doc = "Field `ISP_CMD_EN` writer - ISP Boot Command fixed state"]
pub type IspCmdEnW<'a, REG> = crate::BitWriter<'a, REG, IspCmdEn>;
impl<'a, REG> IspCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Enable)
    }
}
#[doc = "FA Command fixed state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaMeCmdEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<FaMeCmdEn> for bool {
    #[inline(always)]
    fn from(variant: FaMeCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FA_ME_CMD_EN` reader - FA Command fixed state"]
pub type FaMeCmdEnR = crate::BitReader<FaMeCmdEn>;
impl FaMeCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FaMeCmdEn {
        match self.bits {
            false => FaMeCmdEn::Disable,
            true => FaMeCmdEn::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FaMeCmdEn::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FaMeCmdEn::Enable
    }
}
#[doc = "Field `FA_ME_CMD_EN` writer - FA Command fixed state"]
pub type FaMeCmdEnW<'a, REG> = crate::BitWriter<'a, REG, FaMeCmdEn>;
impl<'a, REG> FaMeCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FaMeCmdEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FaMeCmdEn::Enable)
    }
}
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type InverseValueR = crate::FieldReader<u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type InverseValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn spniden(&self) -> SpnidenR {
        SpnidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    pub fn spiden(&self) -> SpidenR {
        SpidenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    pub fn tapen(&self) -> TapenR {
        TapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    pub fn isp_cmd_en(&self) -> IspCmdEnR {
        IspCmdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    pub fn fa_me_cmd_en(&self) -> FaMeCmdEnR {
        FaMeCmdEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> InverseValueR {
        InverseValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG_CC_SOCU_DFLT")
            .field("niden", &self.niden())
            .field("dbgen", &self.dbgen())
            .field("spniden", &self.spniden())
            .field("spiden", &self.spiden())
            .field("tapen", &self.tapen())
            .field("isp_cmd_en", &self.isp_cmd_en())
            .field("fa_me_cmd_en", &self.fa_me_cmd_en())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn niden(&mut self) -> NidenW<'_, DcfgCcSocuDfltSpec> {
        NidenW::new(self, 0)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, DcfgCcSocuDfltSpec> {
        DbgenW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SpnidenW<'_, DcfgCcSocuDfltSpec> {
        SpnidenW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SpidenW<'_, DcfgCcSocuDfltSpec> {
        SpidenW::new(self, 3)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TapenW<'_, DcfgCcSocuDfltSpec> {
        TapenW::new(self, 4)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    pub fn isp_cmd_en(&mut self) -> IspCmdEnW<'_, DcfgCcSocuDfltSpec> {
        IspCmdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    pub fn fa_me_cmd_en(&mut self) -> FaMeCmdEnW<'_, DcfgCcSocuDfltSpec> {
        FaMeCmdEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> InverseValueW<'_, DcfgCcSocuDfltSpec> {
        InverseValueW::new(self, 16)
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg_cc_socu_dflt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg_cc_socu_dflt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgCcSocuDfltSpec;
impl crate::RegisterSpec for DcfgCcSocuDfltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg_cc_socu_dflt::R`](R) reader structure"]
impl crate::Readable for DcfgCcSocuDfltSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg_cc_socu_dflt::W`](W) writer structure"]
impl crate::Writable for DcfgCcSocuDfltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG_CC_SOCU_DFLT to value 0"]
impl crate::Resettable for DcfgCcSocuDfltSpec {}

#[doc = "Register `PLL0CTRL` reader"]
pub type R = crate::R<Pll0ctrlSpec>;
#[doc = "Register `PLL0CTRL` writer"]
pub type W = crate::W<Pll0ctrlSpec>;
#[doc = "Field `SELR` reader - Bandwidth select R value."]
pub type SelrR = crate::FieldReader;
#[doc = "Field `SELR` writer - Bandwidth select R value."]
pub type SelrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELI` reader - Bandwidth select I value."]
pub type SeliR = crate::FieldReader;
#[doc = "Field `SELI` writer - Bandwidth select I value."]
pub type SeliW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SELP` reader - Bandwidth select P value."]
pub type SelpR = crate::FieldReader;
#[doc = "Field `SELP` writer - Bandwidth select P value."]
pub type SelpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Bypass PLL input clock is sent directly to the PLL output (default).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypasspll {
    #[doc = "0: use PLL."]
    Used = 0,
    #[doc = "1: Bypass PLL input clock is sent directly to the PLL output."]
    Bypassed = 1,
}
impl From<Bypasspll> for bool {
    #[inline(always)]
    fn from(variant: Bypasspll) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPLL` reader - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub type BypasspllR = crate::BitReader<Bypasspll>;
impl BypasspllR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypasspll {
        match self.bits {
            false => Bypasspll::Used,
            true => Bypasspll::Bypassed,
        }
    }
    #[doc = "use PLL."]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == Bypasspll::Used
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Bypasspll::Bypassed
    }
}
#[doc = "Field `BYPASSPLL` writer - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub type BypasspllW<'a, REG> = crate::BitWriter<'a, REG, Bypasspll>;
impl<'a, REG> BypasspllW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use PLL."]
    #[inline(always)]
    pub fn used(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspll::Used)
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspll::Bypassed)
    }
}
#[doc = "bypass of the divide-by-2 divider in the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypasspostdiv2 {
    #[doc = "0: use the divide-by-2 divider in the post-divider."]
    Used = 0,
    #[doc = "1: bypass of the divide-by-2 divider in the post-divider."]
    Bypassed = 1,
}
impl From<Bypasspostdiv2> for bool {
    #[inline(always)]
    fn from(variant: Bypasspostdiv2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPOSTDIV2` reader - bypass of the divide-by-2 divider in the post-divider."]
pub type Bypasspostdiv2R = crate::BitReader<Bypasspostdiv2>;
impl Bypasspostdiv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypasspostdiv2 {
        match self.bits {
            false => Bypasspostdiv2::Used,
            true => Bypasspostdiv2::Bypassed,
        }
    }
    #[doc = "use the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == Bypasspostdiv2::Used
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Bypasspostdiv2::Bypassed
    }
}
#[doc = "Field `BYPASSPOSTDIV2` writer - bypass of the divide-by-2 divider in the post-divider."]
pub type Bypasspostdiv2W<'a, REG> = crate::BitWriter<'a, REG, Bypasspostdiv2>;
impl<'a, REG> Bypasspostdiv2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspostdiv2::Used)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspostdiv2::Bypassed)
    }
}
#[doc = "Field `LIMUPOFF` reader - limup_off = 1 in spread spectrum and fractional PLL applications."]
pub type LimupoffR = crate::BitReader;
#[doc = "Field `LIMUPOFF` writer - limup_off = 1 in spread spectrum and fractional PLL applications."]
pub type LimupoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control of the bandwidth of the PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwdirect {
    #[doc = "0: the bandwidth is changed synchronously with the feedback-divider."]
    Sync = 0,
    #[doc = "1: modify the bandwidth of the PLL directly."]
    Direct = 1,
}
impl From<Bwdirect> for bool {
    #[inline(always)]
    fn from(variant: Bwdirect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWDIRECT` reader - Control of the bandwidth of the PLL."]
pub type BwdirectR = crate::BitReader<Bwdirect>;
impl BwdirectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwdirect {
        match self.bits {
            false => Bwdirect::Sync,
            true => Bwdirect::Direct,
        }
    }
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Bwdirect::Sync
    }
    #[doc = "modify the bandwidth of the PLL directly."]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Bwdirect::Direct
    }
}
#[doc = "Field `BWDIRECT` writer - Control of the bandwidth of the PLL."]
pub type BwdirectW<'a, REG> = crate::BitWriter<'a, REG, Bwdirect>;
impl<'a, REG> BwdirectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Bwdirect::Sync)
    }
    #[doc = "modify the bandwidth of the PLL directly."]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Bwdirect::Direct)
    }
}
#[doc = "bypass of the pre-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypassprediv {
    #[doc = "0: use the pre-divider."]
    Used = 0,
    #[doc = "1: bypass of the pre-divider."]
    Bypassed = 1,
}
impl From<Bypassprediv> for bool {
    #[inline(always)]
    fn from(variant: Bypassprediv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPREDIV` reader - bypass of the pre-divider."]
pub type BypasspredivR = crate::BitReader<Bypassprediv>;
impl BypasspredivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypassprediv {
        match self.bits {
            false => Bypassprediv::Used,
            true => Bypassprediv::Bypassed,
        }
    }
    #[doc = "use the pre-divider."]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == Bypassprediv::Used
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Bypassprediv::Bypassed
    }
}
#[doc = "Field `BYPASSPREDIV` writer - bypass of the pre-divider."]
pub type BypasspredivW<'a, REG> = crate::BitWriter<'a, REG, Bypassprediv>;
impl<'a, REG> BypasspredivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use the pre-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut crate::W<REG> {
        self.variant(Bypassprediv::Used)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Bypassprediv::Bypassed)
    }
}
#[doc = "bypass of the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypasspostdiv {
    #[doc = "0: use the post-divider."]
    Used = 0,
    #[doc = "1: bypass of the post-divider."]
    Bypassed = 1,
}
impl From<Bypasspostdiv> for bool {
    #[inline(always)]
    fn from(variant: Bypasspostdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPOSTDIV` reader - bypass of the post-divider."]
pub type BypasspostdivR = crate::BitReader<Bypasspostdiv>;
impl BypasspostdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypasspostdiv {
        match self.bits {
            false => Bypasspostdiv::Used,
            true => Bypasspostdiv::Bypassed,
        }
    }
    #[doc = "use the post-divider."]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == Bypasspostdiv::Used
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Bypasspostdiv::Bypassed
    }
}
#[doc = "Field `BYPASSPOSTDIV` writer - bypass of the post-divider."]
pub type BypasspostdivW<'a, REG> = crate::BitWriter<'a, REG, Bypasspostdiv>;
impl<'a, REG> BypasspostdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspostdiv::Used)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Bypasspostdiv::Bypassed)
    }
}
#[doc = "enable the output clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "0: disable the output clock."]
    Disable = 0,
    #[doc = "1: enable the output clock."]
    Enable = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - enable the output clock."]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::Disable,
            true => Clken::Enable,
        }
    }
    #[doc = "disable the output clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clken::Disable
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clken::Enable
    }
}
#[doc = "Field `CLKEN` writer - enable the output clock."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable the output clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Disable)
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Enable)
    }
}
#[doc = "free running mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmen {
    #[doc = "0: free running mode is disable."]
    Disable = 0,
    #[doc = "1: free running mode is enable."]
    Enable = 1,
}
impl From<Frmen> for bool {
    #[inline(always)]
    fn from(variant: Frmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMEN` reader - free running mode."]
pub type FrmenR = crate::BitReader<Frmen>;
impl FrmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmen {
        match self.bits {
            false => Frmen::Disable,
            true => Frmen::Enable,
        }
    }
    #[doc = "free running mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Frmen::Disable
    }
    #[doc = "free running mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Frmen::Enable
    }
}
#[doc = "Field `FRMEN` writer - free running mode."]
pub type FrmenW<'a, REG> = crate::BitWriter<'a, REG, Frmen>;
impl<'a, REG> FrmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "free running mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Frmen::Disable)
    }
    #[doc = "free running mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Frmen::Enable)
    }
}
#[doc = "Field `FRMCLKSTABLE` reader - free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
pub type FrmclkstableR = crate::BitReader;
#[doc = "Field `FRMCLKSTABLE` writer - free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
pub type FrmclkstableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "skew mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skewen {
    #[doc = "0: skew mode is disable."]
    Disable = 0,
    #[doc = "1: skew mode is enable."]
    Enable = 1,
}
impl From<Skewen> for bool {
    #[inline(always)]
    fn from(variant: Skewen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEWEN` reader - skew mode."]
pub type SkewenR = crate::BitReader<Skewen>;
impl SkewenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skewen {
        match self.bits {
            false => Skewen::Disable,
            true => Skewen::Enable,
        }
    }
    #[doc = "skew mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Skewen::Disable
    }
    #[doc = "skew mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Skewen::Enable
    }
}
#[doc = "Field `SKEWEN` writer - skew mode."]
pub type SkewenW<'a, REG> = crate::BitWriter<'a, REG, Skewen>;
impl<'a, REG> SkewenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "skew mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Skewen::Disable)
    }
    #[doc = "skew mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Skewen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SelrR {
        SelrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SeliR {
        SeliR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&self) -> SelpR {
        SelpR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn bypasspll(&self) -> BypasspllR {
        BypasspllR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv2(&self) -> Bypasspostdiv2R {
        Bypasspostdiv2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub fn limupoff(&self) -> LimupoffR {
        LimupoffR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub fn bwdirect(&self) -> BwdirectR {
        BwdirectR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassprediv(&self) -> BypasspredivR {
        BypasspredivR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv(&self) -> BypasspostdivR {
        BypasspostdivR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - free running mode."]
    #[inline(always)]
    pub fn frmen(&self) -> FrmenR {
        FrmenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub fn frmclkstable(&self) -> FrmclkstableR {
        FrmclkstableR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - skew mode."]
    #[inline(always)]
    pub fn skewen(&self) -> SkewenR {
        SkewenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&mut self) -> SelrW<'_, Pll0ctrlSpec> {
        SelrW::new(self, 0)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&mut self) -> SeliW<'_, Pll0ctrlSpec> {
        SeliW::new(self, 4)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&mut self) -> SelpW<'_, Pll0ctrlSpec> {
        SelpW::new(self, 10)
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn bypasspll(&mut self) -> BypasspllW<'_, Pll0ctrlSpec> {
        BypasspllW::new(self, 15)
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv2(&mut self) -> Bypasspostdiv2W<'_, Pll0ctrlSpec> {
        Bypasspostdiv2W::new(self, 16)
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub fn limupoff(&mut self) -> LimupoffW<'_, Pll0ctrlSpec> {
        LimupoffW::new(self, 17)
    }
    #[doc = "Bit 18 - Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub fn bwdirect(&mut self) -> BwdirectW<'_, Pll0ctrlSpec> {
        BwdirectW::new(self, 18)
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassprediv(&mut self) -> BypasspredivW<'_, Pll0ctrlSpec> {
        BypasspredivW::new(self, 19)
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv(&mut self) -> BypasspostdivW<'_, Pll0ctrlSpec> {
        BypasspostdivW::new(self, 20)
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Pll0ctrlSpec> {
        ClkenW::new(self, 21)
    }
    #[doc = "Bit 22 - free running mode."]
    #[inline(always)]
    pub fn frmen(&mut self) -> FrmenW<'_, Pll0ctrlSpec> {
        FrmenW::new(self, 22)
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub fn frmclkstable(&mut self) -> FrmclkstableW<'_, Pll0ctrlSpec> {
        FrmclkstableW::new(self, 23)
    }
    #[doc = "Bit 24 - skew mode."]
    #[inline(always)]
    pub fn skewen(&mut self) -> SkewenW<'_, Pll0ctrlSpec> {
        SkewenW::new(self, 24)
    }
}
#[doc = "PLL0 550m control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0ctrlSpec;
impl crate::RegisterSpec for Pll0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0ctrl::R`](R) reader structure"]
impl crate::Readable for Pll0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0ctrl::W`](W) writer structure"]
impl crate::Writable for Pll0ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0CTRL to value 0"]
impl crate::Resettable for Pll0ctrlSpec {}

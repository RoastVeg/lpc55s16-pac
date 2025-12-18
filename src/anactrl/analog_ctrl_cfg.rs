#[doc = "Register `ANALOG_CTRL_CFG` reader"]
pub type R = crate::R<AnalogCtrlCfgSpec>;
#[doc = "Register `ANALOG_CTRL_CFG` writer"]
pub type W = crate::W<AnalogCtrlCfgSpec>;
#[doc = "FRO192M trimming and 'Enable' source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fro192mTrimSrc {
    #[doc = "0: FRO192M trimming and 'Enable' comes from eFUSE."]
    Efuse = 0,
    #[doc = "1: FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    Fro192mctrl = 1,
}
impl From<Fro192mTrimSrc> for bool {
    #[inline(always)]
    fn from(variant: Fro192mTrimSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO192M_TRIM_SRC` reader - FRO192M trimming and 'Enable' source."]
pub type Fro192mTrimSrcR = crate::BitReader<Fro192mTrimSrc>;
impl Fro192mTrimSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fro192mTrimSrc {
        match self.bits {
            false => Fro192mTrimSrc::Efuse,
            true => Fro192mTrimSrc::Fro192mctrl,
        }
    }
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    #[inline(always)]
    pub fn is_efuse(&self) -> bool {
        *self == Fro192mTrimSrc::Efuse
    }
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    #[inline(always)]
    pub fn is_fro192mctrl(&self) -> bool {
        *self == Fro192mTrimSrc::Fro192mctrl
    }
}
#[doc = "Field `FRO192M_TRIM_SRC` writer - FRO192M trimming and 'Enable' source."]
pub type Fro192mTrimSrcW<'a, REG> = crate::BitWriter<'a, REG, Fro192mTrimSrc>;
impl<'a, REG> Fro192mTrimSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    #[inline(always)]
    pub fn efuse(self) -> &'a mut crate::W<REG> {
        self.variant(Fro192mTrimSrc::Efuse)
    }
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    #[inline(always)]
    pub fn fro192mctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Fro192mTrimSrc::Fro192mctrl)
    }
}
impl R {
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub fn fro192m_trim_src(&self) -> Fro192mTrimSrcR {
        Fro192mTrimSrcR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANALOG_CTRL_CFG")
            .field("fro192m_trim_src", &self.fro192m_trim_src())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub fn fro192m_trim_src(&mut self) -> Fro192mTrimSrcW<'_, AnalogCtrlCfgSpec> {
        Fro192mTrimSrcW::new(self, 0)
    }
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_ctrl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_ctrl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtrlCfgSpec;
impl crate::RegisterSpec for AnalogCtrlCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctrl_cfg::R`](R) reader structure"]
impl crate::Readable for AnalogCtrlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctrl_cfg::W`](W) writer structure"]
impl crate::Writable for AnalogCtrlCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANALOG_CTRL_CFG to value 0"]
impl crate::Resettable for AnalogCtrlCfgSpec {}

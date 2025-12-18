#[doc = "Register `LDOPMU` reader"]
pub type R = crate::R<LdopmuSpec>;
#[doc = "Register `LDOPMU` writer"]
pub type W = crate::W<LdopmuSpec>;
#[doc = "Sets the Always-On domain LDO output level.\n\nValue on reset: 24"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vadj {
    #[doc = "0: 1.22 V."]
    V1p220 = 0,
    #[doc = "1: 0.7 V."]
    V0p700 = 1,
    #[doc = "2: 0.725 V."]
    V0p725 = 2,
    #[doc = "3: 0.75 V."]
    V0p750 = 3,
    #[doc = "4: 0.775 V."]
    V0p775 = 4,
    #[doc = "5: 0.8 V."]
    V0p800 = 5,
    #[doc = "6: 0.825 V."]
    V0p825 = 6,
    #[doc = "7: 0.85 V."]
    V0p850 = 7,
    #[doc = "8: 0.875 V."]
    V0p875 = 8,
    #[doc = "9: 0.9 V."]
    V0p900 = 9,
    #[doc = "10: 0.96 V."]
    V0p960 = 10,
    #[doc = "11: 0.97 V."]
    V0p970 = 11,
    #[doc = "12: 0.98 V."]
    V0p980 = 12,
    #[doc = "13: 0.99 V."]
    V0p990 = 13,
    #[doc = "14: 1 V."]
    V1p000 = 14,
    #[doc = "15: 1.01 V."]
    V1p010 = 15,
    #[doc = "16: 1.02 V."]
    V1p020 = 16,
    #[doc = "17: 1.03 V."]
    V1p030 = 17,
    #[doc = "18: 1.04 V."]
    V1p040 = 18,
    #[doc = "19: 1.05 V."]
    V1p050 = 19,
    #[doc = "20: 1.06 V."]
    V1p060 = 20,
    #[doc = "21: 1.07 V."]
    V1p070 = 21,
    #[doc = "22: 1.08 V."]
    V1p080 = 22,
    #[doc = "23: 1.09 V."]
    V1p090 = 23,
    #[doc = "24: 1.1 V."]
    V1p100 = 24,
    #[doc = "25: 1.11 V."]
    V1p110 = 25,
    #[doc = "26: 1.12 V."]
    V1p120 = 26,
    #[doc = "27: 1.13 V."]
    V1p130 = 27,
    #[doc = "28: 1.14 V."]
    V1p140 = 28,
    #[doc = "29: 1.15 V."]
    V1p150 = 29,
    #[doc = "30: 1.16 V."]
    V1p160 = 30,
    #[doc = "31: 1.22 V."]
    V1p220_1 = 31,
}
impl From<Vadj> for u8 {
    #[inline(always)]
    fn from(variant: Vadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vadj {
    type Ux = u8;
}
impl crate::IsEnum for Vadj {}
#[doc = "Field `VADJ` reader - Sets the Always-On domain LDO output level."]
pub type VadjR = crate::FieldReader<Vadj>;
impl VadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vadj {
        match self.bits {
            0 => Vadj::V1p220,
            1 => Vadj::V0p700,
            2 => Vadj::V0p725,
            3 => Vadj::V0p750,
            4 => Vadj::V0p775,
            5 => Vadj::V0p800,
            6 => Vadj::V0p825,
            7 => Vadj::V0p850,
            8 => Vadj::V0p875,
            9 => Vadj::V0p900,
            10 => Vadj::V0p960,
            11 => Vadj::V0p970,
            12 => Vadj::V0p980,
            13 => Vadj::V0p990,
            14 => Vadj::V1p000,
            15 => Vadj::V1p010,
            16 => Vadj::V1p020,
            17 => Vadj::V1p030,
            18 => Vadj::V1p040,
            19 => Vadj::V1p050,
            20 => Vadj::V1p060,
            21 => Vadj::V1p070,
            22 => Vadj::V1p080,
            23 => Vadj::V1p090,
            24 => Vadj::V1p100,
            25 => Vadj::V1p110,
            26 => Vadj::V1p120,
            27 => Vadj::V1p130,
            28 => Vadj::V1p140,
            29 => Vadj::V1p150,
            30 => Vadj::V1p160,
            31 => Vadj::V1p220_1,
            _ => unreachable!(),
        }
    }
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn is_v_1p220(&self) -> bool {
        *self == Vadj::V1p220
    }
    #[doc = "0.7 V."]
    #[inline(always)]
    pub fn is_v_0p700(&self) -> bool {
        *self == Vadj::V0p700
    }
    #[doc = "0.725 V."]
    #[inline(always)]
    pub fn is_v_0p725(&self) -> bool {
        *self == Vadj::V0p725
    }
    #[doc = "0.75 V."]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == Vadj::V0p750
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == Vadj::V0p775
    }
    #[doc = "0.8 V."]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == Vadj::V0p800
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == Vadj::V0p825
    }
    #[doc = "0.85 V."]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == Vadj::V0p850
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == Vadj::V0p875
    }
    #[doc = "0.9 V."]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == Vadj::V0p900
    }
    #[doc = "0.96 V."]
    #[inline(always)]
    pub fn is_v_0p960(&self) -> bool {
        *self == Vadj::V0p960
    }
    #[doc = "0.97 V."]
    #[inline(always)]
    pub fn is_v_0p970(&self) -> bool {
        *self == Vadj::V0p970
    }
    #[doc = "0.98 V."]
    #[inline(always)]
    pub fn is_v_0p980(&self) -> bool {
        *self == Vadj::V0p980
    }
    #[doc = "0.99 V."]
    #[inline(always)]
    pub fn is_v_0p990(&self) -> bool {
        *self == Vadj::V0p990
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn is_v_1p000(&self) -> bool {
        *self == Vadj::V1p000
    }
    #[doc = "1.01 V."]
    #[inline(always)]
    pub fn is_v_1p010(&self) -> bool {
        *self == Vadj::V1p010
    }
    #[doc = "1.02 V."]
    #[inline(always)]
    pub fn is_v_1p020(&self) -> bool {
        *self == Vadj::V1p020
    }
    #[doc = "1.03 V."]
    #[inline(always)]
    pub fn is_v_1p030(&self) -> bool {
        *self == Vadj::V1p030
    }
    #[doc = "1.04 V."]
    #[inline(always)]
    pub fn is_v_1p040(&self) -> bool {
        *self == Vadj::V1p040
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn is_v_1p050(&self) -> bool {
        *self == Vadj::V1p050
    }
    #[doc = "1.06 V."]
    #[inline(always)]
    pub fn is_v_1p060(&self) -> bool {
        *self == Vadj::V1p060
    }
    #[doc = "1.07 V."]
    #[inline(always)]
    pub fn is_v_1p070(&self) -> bool {
        *self == Vadj::V1p070
    }
    #[doc = "1.08 V."]
    #[inline(always)]
    pub fn is_v_1p080(&self) -> bool {
        *self == Vadj::V1p080
    }
    #[doc = "1.09 V."]
    #[inline(always)]
    pub fn is_v_1p090(&self) -> bool {
        *self == Vadj::V1p090
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn is_v_1p100(&self) -> bool {
        *self == Vadj::V1p100
    }
    #[doc = "1.11 V."]
    #[inline(always)]
    pub fn is_v_1p110(&self) -> bool {
        *self == Vadj::V1p110
    }
    #[doc = "1.12 V."]
    #[inline(always)]
    pub fn is_v_1p120(&self) -> bool {
        *self == Vadj::V1p120
    }
    #[doc = "1.13 V."]
    #[inline(always)]
    pub fn is_v_1p130(&self) -> bool {
        *self == Vadj::V1p130
    }
    #[doc = "1.14 V."]
    #[inline(always)]
    pub fn is_v_1p140(&self) -> bool {
        *self == Vadj::V1p140
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn is_v_1p150(&self) -> bool {
        *self == Vadj::V1p150
    }
    #[doc = "1.16 V."]
    #[inline(always)]
    pub fn is_v_1p160(&self) -> bool {
        *self == Vadj::V1p160
    }
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn is_v_1p220_1(&self) -> bool {
        *self == Vadj::V1p220_1
    }
}
#[doc = "Field `VADJ` writer - Sets the Always-On domain LDO output level."]
pub type VadjW<'a, REG> = crate::FieldWriter<'a, REG, 5, Vadj, crate::Safe>;
impl<'a, REG> VadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn v_1p220(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p220)
    }
    #[doc = "0.7 V."]
    #[inline(always)]
    pub fn v_0p700(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p700)
    }
    #[doc = "0.725 V."]
    #[inline(always)]
    pub fn v_0p725(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p725)
    }
    #[doc = "0.75 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p775)
    }
    #[doc = "0.8 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p825)
    }
    #[doc = "0.85 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p875)
    }
    #[doc = "0.9 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p900)
    }
    #[doc = "0.96 V."]
    #[inline(always)]
    pub fn v_0p960(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p960)
    }
    #[doc = "0.97 V."]
    #[inline(always)]
    pub fn v_0p970(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p970)
    }
    #[doc = "0.98 V."]
    #[inline(always)]
    pub fn v_0p980(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p980)
    }
    #[doc = "0.99 V."]
    #[inline(always)]
    pub fn v_0p990(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V0p990)
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn v_1p000(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p000)
    }
    #[doc = "1.01 V."]
    #[inline(always)]
    pub fn v_1p010(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p010)
    }
    #[doc = "1.02 V."]
    #[inline(always)]
    pub fn v_1p020(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p020)
    }
    #[doc = "1.03 V."]
    #[inline(always)]
    pub fn v_1p030(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p030)
    }
    #[doc = "1.04 V."]
    #[inline(always)]
    pub fn v_1p040(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p040)
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn v_1p050(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p050)
    }
    #[doc = "1.06 V."]
    #[inline(always)]
    pub fn v_1p060(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p060)
    }
    #[doc = "1.07 V."]
    #[inline(always)]
    pub fn v_1p070(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p070)
    }
    #[doc = "1.08 V."]
    #[inline(always)]
    pub fn v_1p080(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p080)
    }
    #[doc = "1.09 V."]
    #[inline(always)]
    pub fn v_1p090(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p090)
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn v_1p100(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p100)
    }
    #[doc = "1.11 V."]
    #[inline(always)]
    pub fn v_1p110(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p110)
    }
    #[doc = "1.12 V."]
    #[inline(always)]
    pub fn v_1p120(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p120)
    }
    #[doc = "1.13 V."]
    #[inline(always)]
    pub fn v_1p130(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p130)
    }
    #[doc = "1.14 V."]
    #[inline(always)]
    pub fn v_1p140(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p140)
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn v_1p150(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p150)
    }
    #[doc = "1.16 V."]
    #[inline(always)]
    pub fn v_1p160(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p160)
    }
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn v_1p220_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vadj::V1p220_1)
    }
}
#[doc = "Field `VADJ_PWD` reader - Sets the Always-On domain LDO output level in all power down modes."]
pub type VadjPwdR = crate::FieldReader;
#[doc = "Field `VADJ_PWD` writer - Sets the Always-On domain LDO output level in all power down modes."]
pub type VadjPwdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VADJ_BOOST` reader - Sets the Always-On domain LDO Boost output level."]
pub type VadjBoostR = crate::FieldReader;
#[doc = "Field `VADJ_BOOST` writer - Sets the Always-On domain LDO Boost output level."]
pub type VadjBoostW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VADJ_BOOST_PWD` reader - Sets the Always-On domain LDO Boost output level in all power down modes."]
pub type VadjBoostPwdR = crate::FieldReader;
#[doc = "Field `VADJ_BOOST_PWD` writer - Sets the Always-On domain LDO Boost output level in all power down modes."]
pub type VadjBoostPwdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Controls LDOMEM bleed current.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bleed {
    #[doc = "0: Bleed current is disable."]
    Disable = 0,
    #[doc = "1: Bleed current is enable."]
    Enable = 1,
}
impl From<Bleed> for bool {
    #[inline(always)]
    fn from(variant: Bleed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLEED` reader - Controls LDOMEM bleed current."]
pub type BleedR = crate::BitReader<Bleed>;
impl BleedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bleed {
        match self.bits {
            false => Bleed::Disable,
            true => Bleed::Enable,
        }
    }
    #[doc = "Bleed current is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bleed::Disable
    }
    #[doc = "Bleed current is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bleed::Enable
    }
}
#[doc = "Field `BLEED` writer - Controls LDOMEM bleed current."]
pub type BleedW<'a, REG> = crate::BitWriter<'a, REG, Bleed>;
impl<'a, REG> BleedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bleed current is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bleed::Disable)
    }
    #[doc = "Bleed current is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bleed::Enable)
    }
}
#[doc = "Control the LDO AO boost mode in ACTIVE mode.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoostEna {
    #[doc = "0: LDO AO Boost Mode is disable."]
    Disable = 0,
    #[doc = "1: LDO AO Boost Mode is enable."]
    Enable = 1,
}
impl From<BoostEna> for bool {
    #[inline(always)]
    fn from(variant: BoostEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOST_ENA` reader - Control the LDO AO boost mode in ACTIVE mode."]
pub type BoostEnaR = crate::BitReader<BoostEna>;
impl BoostEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BoostEna {
        match self.bits {
            false => BoostEna::Disable,
            true => BoostEna::Enable,
        }
    }
    #[doc = "LDO AO Boost Mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BoostEna::Disable
    }
    #[doc = "LDO AO Boost Mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BoostEna::Enable
    }
}
#[doc = "Field `BOOST_ENA` writer - Control the LDO AO boost mode in ACTIVE mode."]
pub type BoostEnaW<'a, REG> = crate::BitWriter<'a, REG, BoostEna>;
impl<'a, REG> BoostEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO AO Boost Mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BoostEna::Disable)
    }
    #[doc = "LDO AO Boost Mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BoostEna::Enable)
    }
}
#[doc = "Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoostEnaPwd {
    #[doc = "0: LDO AO Boost Mode is disable."]
    Disable = 0,
    #[doc = "1: LDO AO Boost Mode is enable."]
    Enable = 1,
}
impl From<BoostEnaPwd> for bool {
    #[inline(always)]
    fn from(variant: BoostEnaPwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOST_ENA_PWD` reader - Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
pub type BoostEnaPwdR = crate::BitReader<BoostEnaPwd>;
impl BoostEnaPwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BoostEnaPwd {
        match self.bits {
            false => BoostEnaPwd::Disable,
            true => BoostEnaPwd::Enable,
        }
    }
    #[doc = "LDO AO Boost Mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BoostEnaPwd::Disable
    }
    #[doc = "LDO AO Boost Mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BoostEnaPwd::Enable
    }
}
#[doc = "Field `BOOST_ENA_PWD` writer - Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
pub type BoostEnaPwdW<'a, REG> = crate::BitWriter<'a, REG, BoostEnaPwd>;
impl<'a, REG> BoostEnaPwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO AO Boost Mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BoostEnaPwd::Disable)
    }
    #[doc = "LDO AO Boost Mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BoostEnaPwd::Enable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub fn vadj(&self) -> VadjR {
        VadjR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Sets the Always-On domain LDO output level in all power down modes."]
    #[inline(always)]
    pub fn vadj_pwd(&self) -> VadjPwdR {
        VadjPwdR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub fn vadj_boost(&self) -> VadjBoostR {
        VadjBoostR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[inline(always)]
    pub fn vadj_boost_pwd(&self) -> VadjBoostPwdR {
        VadjBoostPwdR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - Controls LDOMEM bleed current."]
    #[inline(always)]
    pub fn bleed(&self) -> BleedR {
        BleedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Control the LDO AO boost mode in ACTIVE mode."]
    #[inline(always)]
    pub fn boost_ena(&self) -> BoostEnaR {
        BoostEnaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn boost_ena_pwd(&self) -> BoostEnaPwdR {
        BoostEnaPwdR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOPMU")
            .field("vadj", &self.vadj())
            .field("vadj_pwd", &self.vadj_pwd())
            .field("vadj_boost", &self.vadj_boost())
            .field("vadj_boost_pwd", &self.vadj_boost_pwd())
            .field("bleed", &self.bleed())
            .field("boost_ena", &self.boost_ena())
            .field("boost_ena_pwd", &self.boost_ena_pwd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub fn vadj(&mut self) -> VadjW<'_, LdopmuSpec> {
        VadjW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Sets the Always-On domain LDO output level in all power down modes."]
    #[inline(always)]
    pub fn vadj_pwd(&mut self) -> VadjPwdW<'_, LdopmuSpec> {
        VadjPwdW::new(self, 5)
    }
    #[doc = "Bits 10:14 - Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub fn vadj_boost(&mut self) -> VadjBoostW<'_, LdopmuSpec> {
        VadjBoostW::new(self, 10)
    }
    #[doc = "Bits 15:19 - Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[inline(always)]
    pub fn vadj_boost_pwd(&mut self) -> VadjBoostPwdW<'_, LdopmuSpec> {
        VadjBoostPwdW::new(self, 15)
    }
    #[doc = "Bit 20 - Controls LDOMEM bleed current."]
    #[inline(always)]
    pub fn bleed(&mut self) -> BleedW<'_, LdopmuSpec> {
        BleedW::new(self, 20)
    }
    #[doc = "Bit 24 - Control the LDO AO boost mode in ACTIVE mode."]
    #[inline(always)]
    pub fn boost_ena(&mut self) -> BoostEnaW<'_, LdopmuSpec> {
        BoostEnaW::new(self, 24)
    }
    #[doc = "Bit 25 - Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[inline(always)]
    pub fn boost_ena_pwd(&mut self) -> BoostEnaPwdW<'_, LdopmuSpec> {
        BoostEnaPwdW::new(self, 25)
    }
}
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ldopmu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldopmu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdopmuSpec;
impl crate::RegisterSpec for LdopmuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldopmu::R`](R) reader structure"]
impl crate::Readable for LdopmuSpec {}
#[doc = "`write(|w| ..)` method takes [`ldopmu::W`](W) writer structure"]
impl crate::Writable for LdopmuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDOPMU to value 0x010e_f718"]
impl crate::Resettable for LdopmuSpec {
    const RESET_VALUE: u32 = 0x010e_f718;
}

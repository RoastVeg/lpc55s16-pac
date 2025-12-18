#[doc = "Register `BODVBAT` reader"]
pub type R = crate::R<BodvbatSpec>;
#[doc = "Register `BODVBAT` writer"]
pub type W = crate::W<BodvbatSpec>;
#[doc = "BoD trigger level.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Triglvl {
    #[doc = "0: 1.00 V."]
    V1p00 = 0,
    #[doc = "1: 1.10 V."]
    V1p10 = 1,
    #[doc = "2: 1.20 V."]
    V1p20 = 2,
    #[doc = "3: 1.30 V."]
    V1p30 = 3,
    #[doc = "4: 1.40 V."]
    V1p40 = 4,
    #[doc = "5: 1.50 V."]
    V1p50 = 5,
    #[doc = "6: 1.60 V."]
    V1p60 = 6,
    #[doc = "7: 1.65 V."]
    V1p65 = 7,
    #[doc = "8: 1.70 V."]
    V1p70 = 8,
    #[doc = "9: 1.75 V."]
    V1p75 = 9,
    #[doc = "10: 1.80 V."]
    V1p80 = 10,
    #[doc = "11: 1.90 V."]
    V1p90 = 11,
    #[doc = "12: 2.00 V."]
    V2p00 = 12,
    #[doc = "13: 2.10 V."]
    V2p10 = 13,
    #[doc = "14: 2.20 V."]
    V2p20 = 14,
    #[doc = "15: 2.30 V."]
    V2p30 = 15,
    #[doc = "16: 2.40 V."]
    V2p40 = 16,
    #[doc = "17: 2.50 V."]
    V2p50 = 17,
    #[doc = "18: 2.60 V."]
    V2p60 = 18,
    #[doc = "19: 2.70 V."]
    V2p70 = 19,
    #[doc = "20: 2.806 V."]
    V2p80 = 20,
    #[doc = "21: 2.90 V."]
    V2p90 = 21,
    #[doc = "22: 3.00 V."]
    V3p00 = 22,
    #[doc = "23: 3.10 V."]
    V3p10 = 23,
    #[doc = "24: 3.20 V."]
    V3p20 = 24,
    #[doc = "25: 3.30 V."]
    V3p30_2 = 25,
    #[doc = "26: 3.30 V."]
    V3p30_3 = 26,
    #[doc = "27: 3.30 V."]
    V3p30_4 = 27,
    #[doc = "28: 3.30 V."]
    V3p30_5 = 28,
    #[doc = "29: 3.30 V."]
    V3p30_6 = 29,
    #[doc = "30: 3.30 V."]
    V3p30_7 = 30,
    #[doc = "31: 3.30 V."]
    V3p30_8 = 31,
}
impl From<Triglvl> for u8 {
    #[inline(always)]
    fn from(variant: Triglvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Triglvl {
    type Ux = u8;
}
impl crate::IsEnum for Triglvl {}
#[doc = "Field `TRIGLVL` reader - BoD trigger level."]
pub type TriglvlR = crate::FieldReader<Triglvl>;
impl TriglvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triglvl {
        match self.bits {
            0 => Triglvl::V1p00,
            1 => Triglvl::V1p10,
            2 => Triglvl::V1p20,
            3 => Triglvl::V1p30,
            4 => Triglvl::V1p40,
            5 => Triglvl::V1p50,
            6 => Triglvl::V1p60,
            7 => Triglvl::V1p65,
            8 => Triglvl::V1p70,
            9 => Triglvl::V1p75,
            10 => Triglvl::V1p80,
            11 => Triglvl::V1p90,
            12 => Triglvl::V2p00,
            13 => Triglvl::V2p10,
            14 => Triglvl::V2p20,
            15 => Triglvl::V2p30,
            16 => Triglvl::V2p40,
            17 => Triglvl::V2p50,
            18 => Triglvl::V2p60,
            19 => Triglvl::V2p70,
            20 => Triglvl::V2p80,
            21 => Triglvl::V2p90,
            22 => Triglvl::V3p00,
            23 => Triglvl::V3p10,
            24 => Triglvl::V3p20,
            25 => Triglvl::V3p30_2,
            26 => Triglvl::V3p30_3,
            27 => Triglvl::V3p30_4,
            28 => Triglvl::V3p30_5,
            29 => Triglvl::V3p30_6,
            30 => Triglvl::V3p30_7,
            31 => Triglvl::V3p30_8,
            _ => unreachable!(),
        }
    }
    #[doc = "1.00 V."]
    #[inline(always)]
    pub fn is_v_1p00(&self) -> bool {
        *self == Triglvl::V1p00
    }
    #[doc = "1.10 V."]
    #[inline(always)]
    pub fn is_v_1p10(&self) -> bool {
        *self == Triglvl::V1p10
    }
    #[doc = "1.20 V."]
    #[inline(always)]
    pub fn is_v_1p20(&self) -> bool {
        *self == Triglvl::V1p20
    }
    #[doc = "1.30 V."]
    #[inline(always)]
    pub fn is_v_1p30(&self) -> bool {
        *self == Triglvl::V1p30
    }
    #[doc = "1.40 V."]
    #[inline(always)]
    pub fn is_v_1p40(&self) -> bool {
        *self == Triglvl::V1p40
    }
    #[doc = "1.50 V."]
    #[inline(always)]
    pub fn is_v_1p50(&self) -> bool {
        *self == Triglvl::V1p50
    }
    #[doc = "1.60 V."]
    #[inline(always)]
    pub fn is_v_1p60(&self) -> bool {
        *self == Triglvl::V1p60
    }
    #[doc = "1.65 V."]
    #[inline(always)]
    pub fn is_v_1p65(&self) -> bool {
        *self == Triglvl::V1p65
    }
    #[doc = "1.70 V."]
    #[inline(always)]
    pub fn is_v_1p70(&self) -> bool {
        *self == Triglvl::V1p70
    }
    #[doc = "1.75 V."]
    #[inline(always)]
    pub fn is_v_1p75(&self) -> bool {
        *self == Triglvl::V1p75
    }
    #[doc = "1.80 V."]
    #[inline(always)]
    pub fn is_v_1p80(&self) -> bool {
        *self == Triglvl::V1p80
    }
    #[doc = "1.90 V."]
    #[inline(always)]
    pub fn is_v_1p90(&self) -> bool {
        *self == Triglvl::V1p90
    }
    #[doc = "2.00 V."]
    #[inline(always)]
    pub fn is_v_2p00(&self) -> bool {
        *self == Triglvl::V2p00
    }
    #[doc = "2.10 V."]
    #[inline(always)]
    pub fn is_v_2p10(&self) -> bool {
        *self == Triglvl::V2p10
    }
    #[doc = "2.20 V."]
    #[inline(always)]
    pub fn is_v_2p20(&self) -> bool {
        *self == Triglvl::V2p20
    }
    #[doc = "2.30 V."]
    #[inline(always)]
    pub fn is_v_2p30(&self) -> bool {
        *self == Triglvl::V2p30
    }
    #[doc = "2.40 V."]
    #[inline(always)]
    pub fn is_v_2p40(&self) -> bool {
        *self == Triglvl::V2p40
    }
    #[doc = "2.50 V."]
    #[inline(always)]
    pub fn is_v_2p50(&self) -> bool {
        *self == Triglvl::V2p50
    }
    #[doc = "2.60 V."]
    #[inline(always)]
    pub fn is_v_2p60(&self) -> bool {
        *self == Triglvl::V2p60
    }
    #[doc = "2.70 V."]
    #[inline(always)]
    pub fn is_v_2p70(&self) -> bool {
        *self == Triglvl::V2p70
    }
    #[doc = "2.806 V."]
    #[inline(always)]
    pub fn is_v_2p80(&self) -> bool {
        *self == Triglvl::V2p80
    }
    #[doc = "2.90 V."]
    #[inline(always)]
    pub fn is_v_2p90(&self) -> bool {
        *self == Triglvl::V2p90
    }
    #[doc = "3.00 V."]
    #[inline(always)]
    pub fn is_v_3p00(&self) -> bool {
        *self == Triglvl::V3p00
    }
    #[doc = "3.10 V."]
    #[inline(always)]
    pub fn is_v_3p10(&self) -> bool {
        *self == Triglvl::V3p10
    }
    #[doc = "3.20 V."]
    #[inline(always)]
    pub fn is_v_3p20(&self) -> bool {
        *self == Triglvl::V3p20
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_2(&self) -> bool {
        *self == Triglvl::V3p30_2
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_3(&self) -> bool {
        *self == Triglvl::V3p30_3
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_4(&self) -> bool {
        *self == Triglvl::V3p30_4
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_5(&self) -> bool {
        *self == Triglvl::V3p30_5
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_6(&self) -> bool {
        *self == Triglvl::V3p30_6
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_7(&self) -> bool {
        *self == Triglvl::V3p30_7
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn is_v_3p30_8(&self) -> bool {
        *self == Triglvl::V3p30_8
    }
}
#[doc = "Field `TRIGLVL` writer - BoD trigger level."]
pub type TriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Triglvl, crate::Safe>;
impl<'a, REG> TriglvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.00 V."]
    #[inline(always)]
    pub fn v_1p00(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p00)
    }
    #[doc = "1.10 V."]
    #[inline(always)]
    pub fn v_1p10(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p10)
    }
    #[doc = "1.20 V."]
    #[inline(always)]
    pub fn v_1p20(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p20)
    }
    #[doc = "1.30 V."]
    #[inline(always)]
    pub fn v_1p30(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p30)
    }
    #[doc = "1.40 V."]
    #[inline(always)]
    pub fn v_1p40(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p40)
    }
    #[doc = "1.50 V."]
    #[inline(always)]
    pub fn v_1p50(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p50)
    }
    #[doc = "1.60 V."]
    #[inline(always)]
    pub fn v_1p60(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p60)
    }
    #[doc = "1.65 V."]
    #[inline(always)]
    pub fn v_1p65(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p65)
    }
    #[doc = "1.70 V."]
    #[inline(always)]
    pub fn v_1p70(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p70)
    }
    #[doc = "1.75 V."]
    #[inline(always)]
    pub fn v_1p75(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p75)
    }
    #[doc = "1.80 V."]
    #[inline(always)]
    pub fn v_1p80(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p80)
    }
    #[doc = "1.90 V."]
    #[inline(always)]
    pub fn v_1p90(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V1p90)
    }
    #[doc = "2.00 V."]
    #[inline(always)]
    pub fn v_2p00(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p00)
    }
    #[doc = "2.10 V."]
    #[inline(always)]
    pub fn v_2p10(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p10)
    }
    #[doc = "2.20 V."]
    #[inline(always)]
    pub fn v_2p20(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p20)
    }
    #[doc = "2.30 V."]
    #[inline(always)]
    pub fn v_2p30(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p30)
    }
    #[doc = "2.40 V."]
    #[inline(always)]
    pub fn v_2p40(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p40)
    }
    #[doc = "2.50 V."]
    #[inline(always)]
    pub fn v_2p50(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p50)
    }
    #[doc = "2.60 V."]
    #[inline(always)]
    pub fn v_2p60(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p60)
    }
    #[doc = "2.70 V."]
    #[inline(always)]
    pub fn v_2p70(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p70)
    }
    #[doc = "2.806 V."]
    #[inline(always)]
    pub fn v_2p80(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p80)
    }
    #[doc = "2.90 V."]
    #[inline(always)]
    pub fn v_2p90(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V2p90)
    }
    #[doc = "3.00 V."]
    #[inline(always)]
    pub fn v_3p00(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p00)
    }
    #[doc = "3.10 V."]
    #[inline(always)]
    pub fn v_3p10(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p10)
    }
    #[doc = "3.20 V."]
    #[inline(always)]
    pub fn v_3p20(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p20)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_2(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_2)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_3(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_3)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_4(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_4)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_5(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_5)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_6(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_6)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_7(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_7)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_8(self) -> &'a mut crate::W<REG> {
        self.variant(Triglvl::V3p30_8)
    }
}
#[doc = "BoD Hysteresis control.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyst {
    #[doc = "0: 25 mV."]
    Hyst25mv = 0,
    #[doc = "1: 50 mV."]
    Hyst50mv = 1,
    #[doc = "2: 75 mV."]
    Hyst75mv = 2,
    #[doc = "3: 100 mV."]
    Hyst100mv = 3,
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Hyst {}
#[doc = "Field `HYST` reader - BoD Hysteresis control."]
pub type HystR = crate::FieldReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            0 => Hyst::Hyst25mv,
            1 => Hyst::Hyst50mv,
            2 => Hyst::Hyst75mv,
            3 => Hyst::Hyst100mv,
            _ => unreachable!(),
        }
    }
    #[doc = "25 mV."]
    #[inline(always)]
    pub fn is_hyst_25mv(&self) -> bool {
        *self == Hyst::Hyst25mv
    }
    #[doc = "50 mV."]
    #[inline(always)]
    pub fn is_hyst_50mv(&self) -> bool {
        *self == Hyst::Hyst50mv
    }
    #[doc = "75 mV."]
    #[inline(always)]
    pub fn is_hyst_75mv(&self) -> bool {
        *self == Hyst::Hyst75mv
    }
    #[doc = "100 mV."]
    #[inline(always)]
    pub fn is_hyst_100mv(&self) -> bool {
        *self == Hyst::Hyst100mv
    }
}
#[doc = "Field `HYST` writer - BoD Hysteresis control."]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hyst, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25 mV."]
    #[inline(always)]
    pub fn hyst_25mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst25mv)
    }
    #[doc = "50 mV."]
    #[inline(always)]
    pub fn hyst_50mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst50mv)
    }
    #[doc = "75 mV."]
    #[inline(always)]
    pub fn hyst_75mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst75mv)
    }
    #[doc = "100 mV."]
    #[inline(always)]
    pub fn hyst_100mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst100mv)
    }
}
impl R {
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&self) -> TriglvlR {
        TriglvlR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BODVBAT")
            .field("triglvl", &self.triglvl())
            .field("hyst", &self.hyst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TriglvlW<'_, BodvbatSpec> {
        TriglvlW::new(self, 0)
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, BodvbatSpec> {
        HystW::new(self, 5)
    }
}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bodvbat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodvbat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodvbatSpec;
impl crate::RegisterSpec for BodvbatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodvbat::R`](R) reader structure"]
impl crate::Readable for BodvbatSpec {}
#[doc = "`write(|w| ..)` method takes [`bodvbat::W`](W) writer structure"]
impl crate::Writable for BodvbatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BODVBAT to value 0x47"]
impl crate::Resettable for BodvbatSpec {
    const RESET_VALUE: u32 = 0x47;
}

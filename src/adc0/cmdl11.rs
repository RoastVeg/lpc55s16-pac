#[doc = "Register `CMDL11` reader"]
pub type R = crate::R<Cmdl11Spec>;
#[doc = "Register `CMDL11` writer"]
pub type W = crate::W<Cmdl11Spec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adch {
    #[doc = "0: Select CH0A or CH0B or CH0A/CH0B pair."]
    Adch0 = 0,
    #[doc = "1: Select CH1A or CH1B or CH1A/CH1B pair."]
    Adch1 = 1,
    #[doc = "2: Select CH2A or CH2B or CH2A/CH2B pair."]
    Adch2 = 2,
    #[doc = "3: Select CH3A or CH3B or CH3A/CH3B pair."]
    Adch3 = 3,
    #[doc = "4: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch4 = 4,
    #[doc = "5: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch5 = 5,
    #[doc = "6: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch6 = 6,
    #[doc = "7: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch7 = 7,
    #[doc = "8: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch8 = 8,
    #[doc = "9: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch9 = 9,
    #[doc = "30: Select CH30A or CH30B or CH30A/CH30B pair."]
    Adch30 = 30,
    #[doc = "31: Select CH31A or CH31B or CH31A/CH31B pair."]
    Adch31 = 31,
}
impl From<Adch> for u8 {
    #[inline(always)]
    fn from(variant: Adch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adch {
    type Ux = u8;
}
impl crate::IsEnum for Adch {}
#[doc = "Field `ADCH` reader - Input channel select"]
pub type AdchR = crate::FieldReader<Adch>;
impl AdchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adch> {
        match self.bits {
            0 => Some(Adch::Adch0),
            1 => Some(Adch::Adch1),
            2 => Some(Adch::Adch2),
            3 => Some(Adch::Adch3),
            4 => Some(Adch::Adch4),
            5 => Some(Adch::Adch5),
            6 => Some(Adch::Adch6),
            7 => Some(Adch::Adch7),
            8 => Some(Adch::Adch8),
            9 => Some(Adch::Adch9),
            30 => Some(Adch::Adch30),
            31 => Some(Adch::Adch31),
            _ => None,
        }
    }
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        *self == Adch::Adch0
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        *self == Adch::Adch1
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        *self == Adch::Adch2
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        *self == Adch::Adch3
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        *self == Adch::Adch4
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        *self == Adch::Adch5
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        *self == Adch::Adch6
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        *self == Adch::Adch7
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        *self == Adch::Adch8
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        *self == Adch::Adch9
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn is_adch_30(&self) -> bool {
        *self == Adch::Adch30
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        *self == Adch::Adch31
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub type AdchW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adch>;
impl<'a, REG> AdchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch0)
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch1)
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch2)
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch9)
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn adch_30(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch30)
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch31)
    }
}
#[doc = "Conversion Type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctype {
    #[doc = "0: Single-Ended Mode. Only A side channel is converted."]
    Ctype0 = 0,
    #[doc = "1: Single-Ended Mode. Only B side channel is converted."]
    Ctype1 = 1,
    #[doc = "2: Differential Mode. A-B."]
    Ctype2 = 2,
    #[doc = "3: Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    Ctype3 = 3,
}
impl From<Ctype> for u8 {
    #[inline(always)]
    fn from(variant: Ctype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctype {
    type Ux = u8;
}
impl crate::IsEnum for Ctype {}
#[doc = "Field `CTYPE` reader - Conversion Type"]
pub type CtypeR = crate::FieldReader<Ctype>;
impl CtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctype {
        match self.bits {
            0 => Ctype::Ctype0,
            1 => Ctype::Ctype1,
            2 => Ctype::Ctype2,
            3 => Ctype::Ctype3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline(always)]
    pub fn is_ctype_0(&self) -> bool {
        *self == Ctype::Ctype0
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline(always)]
    pub fn is_ctype_1(&self) -> bool {
        *self == Ctype::Ctype1
    }
    #[doc = "Differential Mode. A-B."]
    #[inline(always)]
    pub fn is_ctype_2(&self) -> bool {
        *self == Ctype::Ctype2
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline(always)]
    pub fn is_ctype_3(&self) -> bool {
        *self == Ctype::Ctype3
    }
}
#[doc = "Field `CTYPE` writer - Conversion Type"]
pub type CtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctype, crate::Safe>;
impl<'a, REG> CtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline(always)]
    pub fn ctype_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Ctype0)
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline(always)]
    pub fn ctype_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Ctype1)
    }
    #[doc = "Differential Mode. A-B."]
    #[inline(always)]
    pub fn ctype_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Ctype2)
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline(always)]
    pub fn ctype_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Ctype3)
    }
}
#[doc = "Select resolution of conversions\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    Mode0 = 0,
    #[doc = "1: High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    Mode1 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Select resolution of conversions"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode0,
            true => Mode::Mode1,
        }
    }
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
    }
}
#[doc = "Field `MODE` writer - Select resolution of conversions"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0)
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> AdchR {
        AdchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CtypeR {
        CtypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL11")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> AdchW<'_, Cmdl11Spec> {
        AdchW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&mut self) -> CtypeW<'_, Cmdl11Spec> {
        CtypeW::new(self, 5)
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cmdl11Spec> {
        ModeW::new(self, 7)
    }
}
#[doc = "ADC Command Low Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdl11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdl11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmdl11Spec;
impl crate::RegisterSpec for Cmdl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdl11::R`](R) reader structure"]
impl crate::Readable for Cmdl11Spec {}
#[doc = "`write(|w| ..)` method takes [`cmdl11::W`](W) writer structure"]
impl crate::Writable for Cmdl11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDL11 to value 0"]
impl crate::Resettable for Cmdl11Spec {}

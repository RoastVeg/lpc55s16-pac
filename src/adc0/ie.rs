#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "FIFO 0 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwmie0 {
    #[doc = "0: FIFO 0 watermark interrupts are not enabled."]
    Fwmie0_0 = 0,
    #[doc = "1: FIFO 0 watermark interrupts are enabled."]
    Fwmie0_1 = 1,
}
impl From<Fwmie0> for bool {
    #[inline(always)]
    fn from(variant: Fwmie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMIE0` reader - FIFO 0 Watermark Interrupt Enable"]
pub type Fwmie0R = crate::BitReader<Fwmie0>;
impl Fwmie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwmie0 {
        match self.bits {
            false => Fwmie0::Fwmie0_0,
            true => Fwmie0::Fwmie0_1,
        }
    }
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn is_fwmie0_0(&self) -> bool {
        *self == Fwmie0::Fwmie0_0
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn is_fwmie0_1(&self) -> bool {
        *self == Fwmie0::Fwmie0_1
    }
}
#[doc = "Field `FWMIE0` writer - FIFO 0 Watermark Interrupt Enable"]
pub type Fwmie0W<'a, REG> = crate::BitWriter<'a, REG, Fwmie0>;
impl<'a, REG> Fwmie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie0::Fwmie0_0)
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie0::Fwmie0_1)
    }
}
#[doc = "Result FIFO 0 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fofie0 {
    #[doc = "0: FIFO 0 overflow interrupts are not enabled."]
    Fofie0_0 = 0,
    #[doc = "1: FIFO 0 overflow interrupts are enabled."]
    Fofie0_1 = 1,
}
impl From<Fofie0> for bool {
    #[inline(always)]
    fn from(variant: Fofie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFIE0` reader - Result FIFO 0 Overflow Interrupt Enable"]
pub type Fofie0R = crate::BitReader<Fofie0>;
impl Fofie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fofie0 {
        match self.bits {
            false => Fofie0::Fofie0_0,
            true => Fofie0::Fofie0_1,
        }
    }
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn is_fofie0_0(&self) -> bool {
        *self == Fofie0::Fofie0_0
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline(always)]
    pub fn is_fofie0_1(&self) -> bool {
        *self == Fofie0::Fofie0_1
    }
}
#[doc = "Field `FOFIE0` writer - Result FIFO 0 Overflow Interrupt Enable"]
pub type Fofie0W<'a, REG> = crate::BitWriter<'a, REG, Fofie0>;
impl<'a, REG> Fofie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn fofie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie0::Fofie0_0)
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline(always)]
    pub fn fofie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie0::Fofie0_1)
    }
}
#[doc = "FIFO1 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwmie1 {
    #[doc = "0: FIFO1 watermark interrupts are not enabled."]
    Fwmie1_0 = 0,
    #[doc = "1: FIFO1 watermark interrupts are enabled."]
    Fwmie1_1 = 1,
}
impl From<Fwmie1> for bool {
    #[inline(always)]
    fn from(variant: Fwmie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMIE1` reader - FIFO1 Watermark Interrupt Enable"]
pub type Fwmie1R = crate::BitReader<Fwmie1>;
impl Fwmie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwmie1 {
        match self.bits {
            false => Fwmie1::Fwmie1_0,
            true => Fwmie1::Fwmie1_1,
        }
    }
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn is_fwmie1_0(&self) -> bool {
        *self == Fwmie1::Fwmie1_0
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn is_fwmie1_1(&self) -> bool {
        *self == Fwmie1::Fwmie1_1
    }
}
#[doc = "Field `FWMIE1` writer - FIFO1 Watermark Interrupt Enable"]
pub type Fwmie1W<'a, REG> = crate::BitWriter<'a, REG, Fwmie1>;
impl<'a, REG> Fwmie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie1::Fwmie1_0)
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie1::Fwmie1_1)
    }
}
#[doc = "Result FIFO1 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fofie1 {
    #[doc = "0: No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    Fofie1_0 = 0,
    #[doc = "1: At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    Fofie1_1 = 1,
}
impl From<Fofie1> for bool {
    #[inline(always)]
    fn from(variant: Fofie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFIE1` reader - Result FIFO1 Overflow Interrupt Enable"]
pub type Fofie1R = crate::BitReader<Fofie1>;
impl Fofie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fofie1 {
        match self.bits {
            false => Fofie1::Fofie1_0,
            true => Fofie1::Fofie1_1,
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fofie1_0(&self) -> bool {
        *self == Fofie1::Fofie1_0
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fofie1_1(&self) -> bool {
        *self == Fofie1::Fofie1_1
    }
}
#[doc = "Field `FOFIE1` writer - Result FIFO1 Overflow Interrupt Enable"]
pub type Fofie1W<'a, REG> = crate::BitWriter<'a, REG, Fofie1>;
impl<'a, REG> Fofie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie1::Fofie1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie1::Fofie1_1)
    }
}
#[doc = "Trigger Exception Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TexcIe {
    #[doc = "0: Trigger exception interrupts are disabled."]
    TexcIe0 = 0,
    #[doc = "1: Trigger exception interrupts are enabled."]
    TexcIe1 = 1,
}
impl From<TexcIe> for bool {
    #[inline(always)]
    fn from(variant: TexcIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXC_IE` reader - Trigger Exception Interrupt Enable"]
pub type TexcIeR = crate::BitReader<TexcIe>;
impl TexcIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TexcIe {
        match self.bits {
            false => TexcIe::TexcIe0,
            true => TexcIe::TexcIe1,
        }
    }
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline(always)]
    pub fn is_texc_ie_0(&self) -> bool {
        *self == TexcIe::TexcIe0
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline(always)]
    pub fn is_texc_ie_1(&self) -> bool {
        *self == TexcIe::TexcIe1
    }
}
#[doc = "Field `TEXC_IE` writer - Trigger Exception Interrupt Enable"]
pub type TexcIeW<'a, REG> = crate::BitWriter<'a, REG, TexcIe>;
impl<'a, REG> TexcIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline(always)]
    pub fn texc_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(TexcIe::TexcIe0)
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline(always)]
    pub fn texc_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(TexcIe::TexcIe1)
    }
}
#[doc = "Trigger Completion Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TcompIe {
    #[doc = "0: Trigger completion interrupts are disabled."]
    TcompIe0 = 0,
    #[doc = "1: Trigger completion interrupts are enabled for trigger source 0 only."]
    TcompIe1 = 1,
    #[doc = "2: Trigger completion interrupts are enabled for trigger source 1 only."]
    TcompIe2 = 2,
    #[doc = "3: Associated trigger completion interrupts are enabled."]
    TcompIe3 = 3,
    #[doc = "4: Associated trigger completion interrupts are enabled."]
    TcompIe4 = 4,
    #[doc = "5: Associated trigger completion interrupts are enabled."]
    TcompIe5 = 5,
    #[doc = "6: Associated trigger completion interrupts are enabled."]
    TcompIe6 = 6,
    #[doc = "7: Associated trigger completion interrupts are enabled."]
    TcompIe7 = 7,
    #[doc = "8: Associated trigger completion interrupts are enabled."]
    TcompIe8 = 8,
    #[doc = "9: Associated trigger completion interrupts are enabled."]
    TcompIe9 = 9,
    #[doc = "65535: Trigger completion interrupts are enabled for every trigger source."]
    TcompIe65535 = 65535,
}
impl From<TcompIe> for u16 {
    #[inline(always)]
    fn from(variant: TcompIe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TcompIe {
    type Ux = u16;
}
impl crate::IsEnum for TcompIe {}
#[doc = "Field `TCOMP_IE` reader - Trigger Completion Interrupt Enable"]
pub type TcompIeR = crate::FieldReader<TcompIe>;
impl TcompIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TcompIe> {
        match self.bits {
            0 => Some(TcompIe::TcompIe0),
            1 => Some(TcompIe::TcompIe1),
            2 => Some(TcompIe::TcompIe2),
            3 => Some(TcompIe::TcompIe3),
            4 => Some(TcompIe::TcompIe4),
            5 => Some(TcompIe::TcompIe5),
            6 => Some(TcompIe::TcompIe6),
            7 => Some(TcompIe::TcompIe7),
            8 => Some(TcompIe::TcompIe8),
            9 => Some(TcompIe::TcompIe9),
            65535 => Some(TcompIe::TcompIe65535),
            _ => None,
        }
    }
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_0(&self) -> bool {
        *self == TcompIe::TcompIe0
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline(always)]
    pub fn is_tcomp_ie_1(&self) -> bool {
        *self == TcompIe::TcompIe1
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline(always)]
    pub fn is_tcomp_ie_2(&self) -> bool {
        *self == TcompIe::TcompIe2
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_3(&self) -> bool {
        *self == TcompIe::TcompIe3
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_4(&self) -> bool {
        *self == TcompIe::TcompIe4
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_5(&self) -> bool {
        *self == TcompIe::TcompIe5
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_6(&self) -> bool {
        *self == TcompIe::TcompIe6
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_7(&self) -> bool {
        *self == TcompIe::TcompIe7
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_8(&self) -> bool {
        *self == TcompIe::TcompIe8
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn is_tcomp_ie_9(&self) -> bool {
        *self == TcompIe::TcompIe9
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline(always)]
    pub fn is_tcomp_ie_65535(&self) -> bool {
        *self == TcompIe::TcompIe65535
    }
}
#[doc = "Field `TCOMP_IE` writer - Trigger Completion Interrupt Enable"]
pub type TcompIeW<'a, REG> = crate::FieldWriter<'a, REG, 16, TcompIe>;
impl<'a, REG> TcompIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe0)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline(always)]
    pub fn tcomp_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe1)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline(always)]
    pub fn tcomp_ie_2(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe2)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_3(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe3)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_4(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe4)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_5(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe5)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_6(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe6)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_7(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe7)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_8(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe8)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_9(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe9)
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline(always)]
    pub fn tcomp_ie_65535(self) -> &'a mut crate::W<REG> {
        self.variant(TcompIe::TcompIe65535)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&self) -> Fwmie0R {
        Fwmie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&self) -> Fofie0R {
        Fofie0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&self) -> Fwmie1R {
        Fwmie1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&self) -> Fofie1R {
        Fofie1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&self) -> TexcIeR {
        TexcIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&self) -> TcompIeR {
        TcompIeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("fwmie0", &self.fwmie0())
            .field("fofie0", &self.fofie0())
            .field("fwmie1", &self.fwmie1())
            .field("fofie1", &self.fofie1())
            .field("texc_ie", &self.texc_ie())
            .field("tcomp_ie", &self.tcomp_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&mut self) -> Fwmie0W<'_, IeSpec> {
        Fwmie0W::new(self, 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&mut self) -> Fofie0W<'_, IeSpec> {
        Fofie0W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&mut self) -> Fwmie1W<'_, IeSpec> {
        Fwmie1W::new(self, 2)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&mut self) -> Fofie1W<'_, IeSpec> {
        Fofie1W::new(self, 3)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&mut self) -> TexcIeW<'_, IeSpec> {
        TexcIeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&mut self) -> TcompIeW<'_, IeSpec> {
        TcompIeW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {}

#[doc = "Register `OUTPUT_MUX[%s]` reader"]
pub type R = crate::R<OutputMuxSpec>;
#[doc = "Register `OUTPUT_MUX[%s]` writer"]
pub type W = crate::W<OutputMuxSpec>;
#[doc = "Selects the source to be connected to PLU Output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outputn {
    #[doc = "0: The PLU output 0."]
    PluOutput0 = 0,
    #[doc = "1: The PLU output 1."]
    PluOutput1 = 1,
    #[doc = "2: The PLU output 2."]
    PluOutput2 = 2,
    #[doc = "3: The PLU output 3."]
    PluOutput3 = 3,
    #[doc = "4: The PLU output 4."]
    PluOutput4 = 4,
    #[doc = "5: The PLU output 5."]
    PluOutput5 = 5,
    #[doc = "6: The PLU output 6."]
    PluOutput6 = 6,
    #[doc = "7: The PLU output 7."]
    PluOutput7 = 7,
    #[doc = "8: The PLU output 8."]
    PluOutput8 = 8,
    #[doc = "9: The PLU output 9."]
    PluOutput9 = 9,
    #[doc = "10: The PLU output 10."]
    PluOutput10 = 10,
    #[doc = "11: The PLU output 11."]
    PluOutput11 = 11,
    #[doc = "12: The PLU output 12."]
    PluOutput12 = 12,
    #[doc = "13: The PLU output 13."]
    PluOutput13 = 13,
    #[doc = "14: The PLU output 14."]
    PluOutput14 = 14,
    #[doc = "15: The PLU output 15."]
    PluOutput15 = 15,
    #[doc = "16: The PLU output 16."]
    PluOutput16 = 16,
    #[doc = "17: The PLU output 17."]
    PluOutput17 = 17,
    #[doc = "18: The PLU output 18."]
    PluOutput18 = 18,
    #[doc = "19: The PLU output 19."]
    PluOutput19 = 19,
    #[doc = "20: The PLU output 20."]
    PluOutput20 = 20,
    #[doc = "21: The PLU output 21."]
    PluOutput21 = 21,
    #[doc = "22: The PLU output 22."]
    PluOutput22 = 22,
    #[doc = "23: The PLU output 23."]
    PluOutput23 = 23,
    #[doc = "24: The PLU output 24."]
    PluOutput24 = 24,
    #[doc = "25: The PLU output 25."]
    PluOutput25 = 25,
    #[doc = "26: state(0)."]
    State0 = 26,
    #[doc = "27: state(1)."]
    State1 = 27,
    #[doc = "28: state(2)."]
    State2 = 28,
    #[doc = "29: state(3)."]
    State3 = 29,
}
impl From<Outputn> for u8 {
    #[inline(always)]
    fn from(variant: Outputn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outputn {
    type Ux = u8;
}
impl crate::IsEnum for Outputn {}
#[doc = "Field `OUTPUTn` reader - Selects the source to be connected to PLU Output 0."]
pub type OutputnR = crate::FieldReader<Outputn>;
impl OutputnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outputn> {
        match self.bits {
            0 => Some(Outputn::PluOutput0),
            1 => Some(Outputn::PluOutput1),
            2 => Some(Outputn::PluOutput2),
            3 => Some(Outputn::PluOutput3),
            4 => Some(Outputn::PluOutput4),
            5 => Some(Outputn::PluOutput5),
            6 => Some(Outputn::PluOutput6),
            7 => Some(Outputn::PluOutput7),
            8 => Some(Outputn::PluOutput8),
            9 => Some(Outputn::PluOutput9),
            10 => Some(Outputn::PluOutput10),
            11 => Some(Outputn::PluOutput11),
            12 => Some(Outputn::PluOutput12),
            13 => Some(Outputn::PluOutput13),
            14 => Some(Outputn::PluOutput14),
            15 => Some(Outputn::PluOutput15),
            16 => Some(Outputn::PluOutput16),
            17 => Some(Outputn::PluOutput17),
            18 => Some(Outputn::PluOutput18),
            19 => Some(Outputn::PluOutput19),
            20 => Some(Outputn::PluOutput20),
            21 => Some(Outputn::PluOutput21),
            22 => Some(Outputn::PluOutput22),
            23 => Some(Outputn::PluOutput23),
            24 => Some(Outputn::PluOutput24),
            25 => Some(Outputn::PluOutput25),
            26 => Some(Outputn::State0),
            27 => Some(Outputn::State1),
            28 => Some(Outputn::State2),
            29 => Some(Outputn::State3),
            _ => None,
        }
    }
    #[doc = "The PLU output 0."]
    #[inline(always)]
    pub fn is_plu_output0(&self) -> bool {
        *self == Outputn::PluOutput0
    }
    #[doc = "The PLU output 1."]
    #[inline(always)]
    pub fn is_plu_output1(&self) -> bool {
        *self == Outputn::PluOutput1
    }
    #[doc = "The PLU output 2."]
    #[inline(always)]
    pub fn is_plu_output2(&self) -> bool {
        *self == Outputn::PluOutput2
    }
    #[doc = "The PLU output 3."]
    #[inline(always)]
    pub fn is_plu_output3(&self) -> bool {
        *self == Outputn::PluOutput3
    }
    #[doc = "The PLU output 4."]
    #[inline(always)]
    pub fn is_plu_output4(&self) -> bool {
        *self == Outputn::PluOutput4
    }
    #[doc = "The PLU output 5."]
    #[inline(always)]
    pub fn is_plu_output5(&self) -> bool {
        *self == Outputn::PluOutput5
    }
    #[doc = "The PLU output 6."]
    #[inline(always)]
    pub fn is_plu_output6(&self) -> bool {
        *self == Outputn::PluOutput6
    }
    #[doc = "The PLU output 7."]
    #[inline(always)]
    pub fn is_plu_output7(&self) -> bool {
        *self == Outputn::PluOutput7
    }
    #[doc = "The PLU output 8."]
    #[inline(always)]
    pub fn is_plu_output8(&self) -> bool {
        *self == Outputn::PluOutput8
    }
    #[doc = "The PLU output 9."]
    #[inline(always)]
    pub fn is_plu_output9(&self) -> bool {
        *self == Outputn::PluOutput9
    }
    #[doc = "The PLU output 10."]
    #[inline(always)]
    pub fn is_plu_output10(&self) -> bool {
        *self == Outputn::PluOutput10
    }
    #[doc = "The PLU output 11."]
    #[inline(always)]
    pub fn is_plu_output11(&self) -> bool {
        *self == Outputn::PluOutput11
    }
    #[doc = "The PLU output 12."]
    #[inline(always)]
    pub fn is_plu_output12(&self) -> bool {
        *self == Outputn::PluOutput12
    }
    #[doc = "The PLU output 13."]
    #[inline(always)]
    pub fn is_plu_output13(&self) -> bool {
        *self == Outputn::PluOutput13
    }
    #[doc = "The PLU output 14."]
    #[inline(always)]
    pub fn is_plu_output14(&self) -> bool {
        *self == Outputn::PluOutput14
    }
    #[doc = "The PLU output 15."]
    #[inline(always)]
    pub fn is_plu_output15(&self) -> bool {
        *self == Outputn::PluOutput15
    }
    #[doc = "The PLU output 16."]
    #[inline(always)]
    pub fn is_plu_output16(&self) -> bool {
        *self == Outputn::PluOutput16
    }
    #[doc = "The PLU output 17."]
    #[inline(always)]
    pub fn is_plu_output17(&self) -> bool {
        *self == Outputn::PluOutput17
    }
    #[doc = "The PLU output 18."]
    #[inline(always)]
    pub fn is_plu_output18(&self) -> bool {
        *self == Outputn::PluOutput18
    }
    #[doc = "The PLU output 19."]
    #[inline(always)]
    pub fn is_plu_output19(&self) -> bool {
        *self == Outputn::PluOutput19
    }
    #[doc = "The PLU output 20."]
    #[inline(always)]
    pub fn is_plu_output20(&self) -> bool {
        *self == Outputn::PluOutput20
    }
    #[doc = "The PLU output 21."]
    #[inline(always)]
    pub fn is_plu_output21(&self) -> bool {
        *self == Outputn::PluOutput21
    }
    #[doc = "The PLU output 22."]
    #[inline(always)]
    pub fn is_plu_output22(&self) -> bool {
        *self == Outputn::PluOutput22
    }
    #[doc = "The PLU output 23."]
    #[inline(always)]
    pub fn is_plu_output23(&self) -> bool {
        *self == Outputn::PluOutput23
    }
    #[doc = "The PLU output 24."]
    #[inline(always)]
    pub fn is_plu_output24(&self) -> bool {
        *self == Outputn::PluOutput24
    }
    #[doc = "The PLU output 25."]
    #[inline(always)]
    pub fn is_plu_output25(&self) -> bool {
        *self == Outputn::PluOutput25
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        *self == Outputn::State0
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        *self == Outputn::State1
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        *self == Outputn::State2
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        *self == Outputn::State3
    }
}
#[doc = "Field `OUTPUTn` writer - Selects the source to be connected to PLU Output 0."]
pub type OutputnW<'a, REG> = crate::FieldWriter<'a, REG, 5, Outputn>;
impl<'a, REG> OutputnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The PLU output 0."]
    #[inline(always)]
    pub fn plu_output0(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput0)
    }
    #[doc = "The PLU output 1."]
    #[inline(always)]
    pub fn plu_output1(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput1)
    }
    #[doc = "The PLU output 2."]
    #[inline(always)]
    pub fn plu_output2(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput2)
    }
    #[doc = "The PLU output 3."]
    #[inline(always)]
    pub fn plu_output3(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput3)
    }
    #[doc = "The PLU output 4."]
    #[inline(always)]
    pub fn plu_output4(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput4)
    }
    #[doc = "The PLU output 5."]
    #[inline(always)]
    pub fn plu_output5(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput5)
    }
    #[doc = "The PLU output 6."]
    #[inline(always)]
    pub fn plu_output6(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput6)
    }
    #[doc = "The PLU output 7."]
    #[inline(always)]
    pub fn plu_output7(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput7)
    }
    #[doc = "The PLU output 8."]
    #[inline(always)]
    pub fn plu_output8(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput8)
    }
    #[doc = "The PLU output 9."]
    #[inline(always)]
    pub fn plu_output9(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput9)
    }
    #[doc = "The PLU output 10."]
    #[inline(always)]
    pub fn plu_output10(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput10)
    }
    #[doc = "The PLU output 11."]
    #[inline(always)]
    pub fn plu_output11(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput11)
    }
    #[doc = "The PLU output 12."]
    #[inline(always)]
    pub fn plu_output12(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput12)
    }
    #[doc = "The PLU output 13."]
    #[inline(always)]
    pub fn plu_output13(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput13)
    }
    #[doc = "The PLU output 14."]
    #[inline(always)]
    pub fn plu_output14(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput14)
    }
    #[doc = "The PLU output 15."]
    #[inline(always)]
    pub fn plu_output15(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput15)
    }
    #[doc = "The PLU output 16."]
    #[inline(always)]
    pub fn plu_output16(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput16)
    }
    #[doc = "The PLU output 17."]
    #[inline(always)]
    pub fn plu_output17(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput17)
    }
    #[doc = "The PLU output 18."]
    #[inline(always)]
    pub fn plu_output18(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput18)
    }
    #[doc = "The PLU output 19."]
    #[inline(always)]
    pub fn plu_output19(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput19)
    }
    #[doc = "The PLU output 20."]
    #[inline(always)]
    pub fn plu_output20(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput20)
    }
    #[doc = "The PLU output 21."]
    #[inline(always)]
    pub fn plu_output21(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput21)
    }
    #[doc = "The PLU output 22."]
    #[inline(always)]
    pub fn plu_output22(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput22)
    }
    #[doc = "The PLU output 23."]
    #[inline(always)]
    pub fn plu_output23(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput23)
    }
    #[doc = "The PLU output 24."]
    #[inline(always)]
    pub fn plu_output24(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput24)
    }
    #[doc = "The PLU output 25."]
    #[inline(always)]
    pub fn plu_output25(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::PluOutput25)
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::State0)
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::State1)
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::State2)
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut crate::W<REG> {
        self.variant(Outputn::State3)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub fn outputn(&self) -> OutputnR {
        OutputnR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub fn outputn(&mut self) -> OutputnW<'_, OutputMuxSpec> {
        OutputnW::new(self, 0)
    }
}
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n\n\nYou can [`read`](crate::Reg::read) this register and get [`output_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputMuxSpec;
impl crate::RegisterSpec for OutputMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`output_mux::R`](R) reader structure"]
impl crate::Readable for OutputMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`output_mux::W`](W) writer structure"]
impl crate::Writable for OutputMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPUT_MUX[%s] to value 0"]
impl crate::Resettable for OutputMuxSpec {}

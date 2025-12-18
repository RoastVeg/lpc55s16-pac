#[doc = "Register `LUT_INP_MUX%s` reader"]
pub type R = crate::R<LutInpMuxSpec>;
#[doc = "Register `LUT_INP_MUX%s` writer"]
pub type W = crate::W<LutInpMuxSpec>;
#[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LutnInpx {
    #[doc = "0: The PLU primary inputs 0."]
    PluInputs0 = 0,
    #[doc = "1: The PLU primary inputs 1."]
    PluInputs1 = 1,
    #[doc = "2: The PLU primary inputs 2."]
    PluInputs2 = 2,
    #[doc = "3: The PLU primary inputs 3."]
    PluInputs3 = 3,
    #[doc = "4: The PLU primary inputs 4."]
    PluInputs4 = 4,
    #[doc = "5: The PLU primary inputs 5."]
    PluInputs5 = 5,
    #[doc = "6: The output of LUT0."]
    LutOutputs0 = 6,
    #[doc = "7: The output of LUT1."]
    LutOutputs1 = 7,
    #[doc = "8: The output of LUT2."]
    LutOutputs2 = 8,
    #[doc = "9: The output of LUT3."]
    LutOutputs3 = 9,
    #[doc = "10: The output of LUT4."]
    LutOutputs4 = 10,
    #[doc = "11: The output of LUT5."]
    LutOutputs5 = 11,
    #[doc = "12: The output of LUT6."]
    LutOutputs6 = 12,
    #[doc = "13: The output of LUT7."]
    LutOutputs7 = 13,
    #[doc = "14: The output of LUT8."]
    LutOutputs8 = 14,
    #[doc = "15: The output of LUT9."]
    LutOutputs9 = 15,
    #[doc = "16: The output of LUT10."]
    LutOutputs10 = 16,
    #[doc = "17: The output of LUT11."]
    LutOutputs11 = 17,
    #[doc = "18: The output of LUT12."]
    LutOutputs12 = 18,
    #[doc = "19: The output of LUT13."]
    LutOutputs13 = 19,
    #[doc = "20: The output of LUT14."]
    LutOutputs14 = 20,
    #[doc = "21: The output of LUT15."]
    LutOutputs15 = 21,
    #[doc = "22: The output of LUT16."]
    LutOutputs16 = 22,
    #[doc = "23: The output of LUT17."]
    LutOutputs17 = 23,
    #[doc = "24: The output of LUT18."]
    LutOutputs18 = 24,
    #[doc = "25: The output of LUT19."]
    LutOutputs19 = 25,
    #[doc = "26: The output of LUT20."]
    LutOutputs20 = 26,
    #[doc = "27: The output of LUT21."]
    LutOutputs21 = 27,
    #[doc = "28: The output of LUT22."]
    LutOutputs22 = 28,
    #[doc = "29: The output of LUT23."]
    LutOutputs23 = 29,
    #[doc = "30: The output of LUT24."]
    LutOutputs24 = 30,
    #[doc = "31: The output of LUT25."]
    LutOutputs25 = 31,
    #[doc = "32: state(0)."]
    State0 = 32,
    #[doc = "33: state(1)."]
    State1 = 33,
    #[doc = "34: state(2)."]
    State2 = 34,
    #[doc = "35: state(3)."]
    State3 = 35,
}
impl From<LutnInpx> for u8 {
    #[inline(always)]
    fn from(variant: LutnInpx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LutnInpx {
    type Ux = u8;
}
impl crate::IsEnum for LutnInpx {}
#[doc = "Field `LUTn_INPx` reader - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
pub type LutnInpxR = crate::FieldReader<LutnInpx>;
impl LutnInpxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LutnInpx> {
        match self.bits {
            0 => Some(LutnInpx::PluInputs0),
            1 => Some(LutnInpx::PluInputs1),
            2 => Some(LutnInpx::PluInputs2),
            3 => Some(LutnInpx::PluInputs3),
            4 => Some(LutnInpx::PluInputs4),
            5 => Some(LutnInpx::PluInputs5),
            6 => Some(LutnInpx::LutOutputs0),
            7 => Some(LutnInpx::LutOutputs1),
            8 => Some(LutnInpx::LutOutputs2),
            9 => Some(LutnInpx::LutOutputs3),
            10 => Some(LutnInpx::LutOutputs4),
            11 => Some(LutnInpx::LutOutputs5),
            12 => Some(LutnInpx::LutOutputs6),
            13 => Some(LutnInpx::LutOutputs7),
            14 => Some(LutnInpx::LutOutputs8),
            15 => Some(LutnInpx::LutOutputs9),
            16 => Some(LutnInpx::LutOutputs10),
            17 => Some(LutnInpx::LutOutputs11),
            18 => Some(LutnInpx::LutOutputs12),
            19 => Some(LutnInpx::LutOutputs13),
            20 => Some(LutnInpx::LutOutputs14),
            21 => Some(LutnInpx::LutOutputs15),
            22 => Some(LutnInpx::LutOutputs16),
            23 => Some(LutnInpx::LutOutputs17),
            24 => Some(LutnInpx::LutOutputs18),
            25 => Some(LutnInpx::LutOutputs19),
            26 => Some(LutnInpx::LutOutputs20),
            27 => Some(LutnInpx::LutOutputs21),
            28 => Some(LutnInpx::LutOutputs22),
            29 => Some(LutnInpx::LutOutputs23),
            30 => Some(LutnInpx::LutOutputs24),
            31 => Some(LutnInpx::LutOutputs25),
            32 => Some(LutnInpx::State0),
            33 => Some(LutnInpx::State1),
            34 => Some(LutnInpx::State2),
            35 => Some(LutnInpx::State3),
            _ => None,
        }
    }
    #[doc = "The PLU primary inputs 0."]
    #[inline(always)]
    pub fn is_plu_inputs0(&self) -> bool {
        *self == LutnInpx::PluInputs0
    }
    #[doc = "The PLU primary inputs 1."]
    #[inline(always)]
    pub fn is_plu_inputs1(&self) -> bool {
        *self == LutnInpx::PluInputs1
    }
    #[doc = "The PLU primary inputs 2."]
    #[inline(always)]
    pub fn is_plu_inputs2(&self) -> bool {
        *self == LutnInpx::PluInputs2
    }
    #[doc = "The PLU primary inputs 3."]
    #[inline(always)]
    pub fn is_plu_inputs3(&self) -> bool {
        *self == LutnInpx::PluInputs3
    }
    #[doc = "The PLU primary inputs 4."]
    #[inline(always)]
    pub fn is_plu_inputs4(&self) -> bool {
        *self == LutnInpx::PluInputs4
    }
    #[doc = "The PLU primary inputs 5."]
    #[inline(always)]
    pub fn is_plu_inputs5(&self) -> bool {
        *self == LutnInpx::PluInputs5
    }
    #[doc = "The output of LUT0."]
    #[inline(always)]
    pub fn is_lut_outputs0(&self) -> bool {
        *self == LutnInpx::LutOutputs0
    }
    #[doc = "The output of LUT1."]
    #[inline(always)]
    pub fn is_lut_outputs1(&self) -> bool {
        *self == LutnInpx::LutOutputs1
    }
    #[doc = "The output of LUT2."]
    #[inline(always)]
    pub fn is_lut_outputs2(&self) -> bool {
        *self == LutnInpx::LutOutputs2
    }
    #[doc = "The output of LUT3."]
    #[inline(always)]
    pub fn is_lut_outputs3(&self) -> bool {
        *self == LutnInpx::LutOutputs3
    }
    #[doc = "The output of LUT4."]
    #[inline(always)]
    pub fn is_lut_outputs4(&self) -> bool {
        *self == LutnInpx::LutOutputs4
    }
    #[doc = "The output of LUT5."]
    #[inline(always)]
    pub fn is_lut_outputs5(&self) -> bool {
        *self == LutnInpx::LutOutputs5
    }
    #[doc = "The output of LUT6."]
    #[inline(always)]
    pub fn is_lut_outputs6(&self) -> bool {
        *self == LutnInpx::LutOutputs6
    }
    #[doc = "The output of LUT7."]
    #[inline(always)]
    pub fn is_lut_outputs7(&self) -> bool {
        *self == LutnInpx::LutOutputs7
    }
    #[doc = "The output of LUT8."]
    #[inline(always)]
    pub fn is_lut_outputs8(&self) -> bool {
        *self == LutnInpx::LutOutputs8
    }
    #[doc = "The output of LUT9."]
    #[inline(always)]
    pub fn is_lut_outputs9(&self) -> bool {
        *self == LutnInpx::LutOutputs9
    }
    #[doc = "The output of LUT10."]
    #[inline(always)]
    pub fn is_lut_outputs10(&self) -> bool {
        *self == LutnInpx::LutOutputs10
    }
    #[doc = "The output of LUT11."]
    #[inline(always)]
    pub fn is_lut_outputs11(&self) -> bool {
        *self == LutnInpx::LutOutputs11
    }
    #[doc = "The output of LUT12."]
    #[inline(always)]
    pub fn is_lut_outputs12(&self) -> bool {
        *self == LutnInpx::LutOutputs12
    }
    #[doc = "The output of LUT13."]
    #[inline(always)]
    pub fn is_lut_outputs13(&self) -> bool {
        *self == LutnInpx::LutOutputs13
    }
    #[doc = "The output of LUT14."]
    #[inline(always)]
    pub fn is_lut_outputs14(&self) -> bool {
        *self == LutnInpx::LutOutputs14
    }
    #[doc = "The output of LUT15."]
    #[inline(always)]
    pub fn is_lut_outputs15(&self) -> bool {
        *self == LutnInpx::LutOutputs15
    }
    #[doc = "The output of LUT16."]
    #[inline(always)]
    pub fn is_lut_outputs16(&self) -> bool {
        *self == LutnInpx::LutOutputs16
    }
    #[doc = "The output of LUT17."]
    #[inline(always)]
    pub fn is_lut_outputs17(&self) -> bool {
        *self == LutnInpx::LutOutputs17
    }
    #[doc = "The output of LUT18."]
    #[inline(always)]
    pub fn is_lut_outputs18(&self) -> bool {
        *self == LutnInpx::LutOutputs18
    }
    #[doc = "The output of LUT19."]
    #[inline(always)]
    pub fn is_lut_outputs19(&self) -> bool {
        *self == LutnInpx::LutOutputs19
    }
    #[doc = "The output of LUT20."]
    #[inline(always)]
    pub fn is_lut_outputs20(&self) -> bool {
        *self == LutnInpx::LutOutputs20
    }
    #[doc = "The output of LUT21."]
    #[inline(always)]
    pub fn is_lut_outputs21(&self) -> bool {
        *self == LutnInpx::LutOutputs21
    }
    #[doc = "The output of LUT22."]
    #[inline(always)]
    pub fn is_lut_outputs22(&self) -> bool {
        *self == LutnInpx::LutOutputs22
    }
    #[doc = "The output of LUT23."]
    #[inline(always)]
    pub fn is_lut_outputs23(&self) -> bool {
        *self == LutnInpx::LutOutputs23
    }
    #[doc = "The output of LUT24."]
    #[inline(always)]
    pub fn is_lut_outputs24(&self) -> bool {
        *self == LutnInpx::LutOutputs24
    }
    #[doc = "The output of LUT25."]
    #[inline(always)]
    pub fn is_lut_outputs25(&self) -> bool {
        *self == LutnInpx::LutOutputs25
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        *self == LutnInpx::State0
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        *self == LutnInpx::State1
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        *self == LutnInpx::State2
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        *self == LutnInpx::State3
    }
}
#[doc = "Field `LUTn_INPx` writer - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
pub type LutnInpxW<'a, REG> = crate::FieldWriter<'a, REG, 6, LutnInpx>;
impl<'a, REG> LutnInpxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The PLU primary inputs 0."]
    #[inline(always)]
    pub fn plu_inputs0(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs0)
    }
    #[doc = "The PLU primary inputs 1."]
    #[inline(always)]
    pub fn plu_inputs1(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs1)
    }
    #[doc = "The PLU primary inputs 2."]
    #[inline(always)]
    pub fn plu_inputs2(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs2)
    }
    #[doc = "The PLU primary inputs 3."]
    #[inline(always)]
    pub fn plu_inputs3(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs3)
    }
    #[doc = "The PLU primary inputs 4."]
    #[inline(always)]
    pub fn plu_inputs4(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs4)
    }
    #[doc = "The PLU primary inputs 5."]
    #[inline(always)]
    pub fn plu_inputs5(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::PluInputs5)
    }
    #[doc = "The output of LUT0."]
    #[inline(always)]
    pub fn lut_outputs0(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs0)
    }
    #[doc = "The output of LUT1."]
    #[inline(always)]
    pub fn lut_outputs1(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs1)
    }
    #[doc = "The output of LUT2."]
    #[inline(always)]
    pub fn lut_outputs2(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs2)
    }
    #[doc = "The output of LUT3."]
    #[inline(always)]
    pub fn lut_outputs3(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs3)
    }
    #[doc = "The output of LUT4."]
    #[inline(always)]
    pub fn lut_outputs4(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs4)
    }
    #[doc = "The output of LUT5."]
    #[inline(always)]
    pub fn lut_outputs5(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs5)
    }
    #[doc = "The output of LUT6."]
    #[inline(always)]
    pub fn lut_outputs6(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs6)
    }
    #[doc = "The output of LUT7."]
    #[inline(always)]
    pub fn lut_outputs7(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs7)
    }
    #[doc = "The output of LUT8."]
    #[inline(always)]
    pub fn lut_outputs8(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs8)
    }
    #[doc = "The output of LUT9."]
    #[inline(always)]
    pub fn lut_outputs9(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs9)
    }
    #[doc = "The output of LUT10."]
    #[inline(always)]
    pub fn lut_outputs10(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs10)
    }
    #[doc = "The output of LUT11."]
    #[inline(always)]
    pub fn lut_outputs11(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs11)
    }
    #[doc = "The output of LUT12."]
    #[inline(always)]
    pub fn lut_outputs12(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs12)
    }
    #[doc = "The output of LUT13."]
    #[inline(always)]
    pub fn lut_outputs13(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs13)
    }
    #[doc = "The output of LUT14."]
    #[inline(always)]
    pub fn lut_outputs14(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs14)
    }
    #[doc = "The output of LUT15."]
    #[inline(always)]
    pub fn lut_outputs15(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs15)
    }
    #[doc = "The output of LUT16."]
    #[inline(always)]
    pub fn lut_outputs16(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs16)
    }
    #[doc = "The output of LUT17."]
    #[inline(always)]
    pub fn lut_outputs17(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs17)
    }
    #[doc = "The output of LUT18."]
    #[inline(always)]
    pub fn lut_outputs18(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs18)
    }
    #[doc = "The output of LUT19."]
    #[inline(always)]
    pub fn lut_outputs19(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs19)
    }
    #[doc = "The output of LUT20."]
    #[inline(always)]
    pub fn lut_outputs20(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs20)
    }
    #[doc = "The output of LUT21."]
    #[inline(always)]
    pub fn lut_outputs21(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs21)
    }
    #[doc = "The output of LUT22."]
    #[inline(always)]
    pub fn lut_outputs22(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs22)
    }
    #[doc = "The output of LUT23."]
    #[inline(always)]
    pub fn lut_outputs23(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs23)
    }
    #[doc = "The output of LUT24."]
    #[inline(always)]
    pub fn lut_outputs24(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs24)
    }
    #[doc = "The output of LUT25."]
    #[inline(always)]
    pub fn lut_outputs25(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::LutOutputs25)
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::State0)
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::State1)
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::State2)
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut crate::W<REG> {
        self.variant(LutnInpx::State3)
    }
}
impl R {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[inline(always)]
    pub fn lutn_inpx(&self) -> LutnInpxR {
        LutnInpxR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[inline(always)]
    pub fn lutn_inpx(&mut self) -> LutnInpxW<'_, LutInpMuxSpec> {
        LutnInpxW::new(self, 0)
    }
}
#[doc = "LUTn input x MUX\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_inp_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_inp_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutInpMuxSpec;
impl crate::RegisterSpec for LutInpMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_inp_mux::R`](R) reader structure"]
impl crate::Readable for LutInpMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_inp_mux::W`](W) writer structure"]
impl crate::Writable for LutInpMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_INP_MUX%s to value 0"]
impl crate::Resettable for LutInpMuxSpec {}

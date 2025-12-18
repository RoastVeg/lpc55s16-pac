#[doc = "Register `TIMER0CAPTSEL[%s]` reader"]
pub type R = crate::R<Timer0captselSpec>;
#[doc = "Register `TIMER0CAPTSEL[%s]` writer"]
pub type W = crate::W<Timer0captselSpec>;
#[doc = "Input number to TIMER0 capture inputs 0 to 4\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captsel {
    #[doc = "0: CT_INP0 function selected from IOCON register"]
    Val0 = 0,
    #[doc = "1: CT_INP1 function selected from IOCON register"]
    Val1 = 1,
    #[doc = "2: CT_INP2 function selected from IOCON register"]
    Val2 = 2,
    #[doc = "3: CT_INP3 function selected from IOCON register"]
    Val3 = 3,
    #[doc = "4: CT_INP4 function selected from IOCON register"]
    Val4 = 4,
    #[doc = "5: CT_INP5 function selected from IOCON register"]
    Val5 = 5,
    #[doc = "6: CT_INP6 function selected from IOCON register"]
    Val6 = 6,
    #[doc = "7: CT_INP7 function selected from IOCON register"]
    Val7 = 7,
    #[doc = "8: CT_INP8 function selected from IOCON register"]
    Val8 = 8,
    #[doc = "9: CT_INP9 function selected from IOCON register"]
    Val9 = 9,
    #[doc = "10: CT_INP10 function selected from IOCON register"]
    Val10 = 10,
    #[doc = "11: CT_INP11 function selected from IOCON register"]
    Val11 = 11,
    #[doc = "12: CT_INP12 function selected from IOCON register"]
    Val12 = 12,
    #[doc = "13: CT_INP13 function selected from IOCON register"]
    Val13 = 13,
    #[doc = "14: CT_INP14 function selected from IOCON register"]
    Val14 = 14,
    #[doc = "15: CT_INP15 function selected from IOCON register"]
    Val15 = 15,
    #[doc = "16: CT_INP16 function selected from IOCON register"]
    Val16 = 16,
    #[doc = "17: None"]
    Val17 = 17,
    #[doc = "18: None"]
    Val18 = 18,
    #[doc = "19: None"]
    Val19 = 19,
    #[doc = "20: USB0_FRAME_TOGGLE"]
    Val20 = 20,
    #[doc = "21: USB1_FRAME_TOGGLE"]
    Val21 = 21,
    #[doc = "22: COMP_OUTPUT output from analog comparator"]
    Val22 = 22,
    #[doc = "23: I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    Val23 = 23,
    #[doc = "24: I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    Val24 = 24,
    #[doc = "25: None"]
    Val25_25 = 25,
    #[doc = "26: None"]
    Val25_26 = 26,
    #[doc = "27: None"]
    Val25_27 = 27,
    #[doc = "28: None"]
    Val25_28 = 28,
    #[doc = "29: None"]
    Val25_29 = 29,
    #[doc = "30: None"]
    Val25_30 = 30,
    #[doc = "31: None"]
    Val25_31 = 31,
}
impl From<Captsel> for u8 {
    #[inline(always)]
    fn from(variant: Captsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captsel {
    type Ux = u8;
}
impl crate::IsEnum for Captsel {}
#[doc = "Field `CAPTSEL` reader - Input number to TIMER0 capture inputs 0 to 4"]
pub type CaptselR = crate::FieldReader<Captsel>;
impl CaptselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captsel {
        match self.bits {
            0 => Captsel::Val0,
            1 => Captsel::Val1,
            2 => Captsel::Val2,
            3 => Captsel::Val3,
            4 => Captsel::Val4,
            5 => Captsel::Val5,
            6 => Captsel::Val6,
            7 => Captsel::Val7,
            8 => Captsel::Val8,
            9 => Captsel::Val9,
            10 => Captsel::Val10,
            11 => Captsel::Val11,
            12 => Captsel::Val12,
            13 => Captsel::Val13,
            14 => Captsel::Val14,
            15 => Captsel::Val15,
            16 => Captsel::Val16,
            17 => Captsel::Val17,
            18 => Captsel::Val18,
            19 => Captsel::Val19,
            20 => Captsel::Val20,
            21 => Captsel::Val21,
            22 => Captsel::Val22,
            23 => Captsel::Val23,
            24 => Captsel::Val24,
            25 => Captsel::Val25_25,
            26 => Captsel::Val25_26,
            27 => Captsel::Val25_27,
            28 => Captsel::Val25_28,
            29 => Captsel::Val25_29,
            30 => Captsel::Val25_30,
            31 => Captsel::Val25_31,
            _ => unreachable!(),
        }
    }
    #[doc = "CT_INP0 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Captsel::Val0
    }
    #[doc = "CT_INP1 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Captsel::Val1
    }
    #[doc = "CT_INP2 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Captsel::Val2
    }
    #[doc = "CT_INP3 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Captsel::Val3
    }
    #[doc = "CT_INP4 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Captsel::Val4
    }
    #[doc = "CT_INP5 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == Captsel::Val5
    }
    #[doc = "CT_INP6 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == Captsel::Val6
    }
    #[doc = "CT_INP7 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == Captsel::Val7
    }
    #[doc = "CT_INP8 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Captsel::Val8
    }
    #[doc = "CT_INP9 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == Captsel::Val9
    }
    #[doc = "CT_INP10 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == Captsel::Val10
    }
    #[doc = "CT_INP11 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == Captsel::Val11
    }
    #[doc = "CT_INP12 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == Captsel::Val12
    }
    #[doc = "CT_INP13 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == Captsel::Val13
    }
    #[doc = "CT_INP14 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == Captsel::Val14
    }
    #[doc = "CT_INP15 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == Captsel::Val15
    }
    #[doc = "CT_INP16 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == Captsel::Val16
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == Captsel::Val17
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == Captsel::Val18
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == Captsel::Val19
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == Captsel::Val20
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == Captsel::Val21
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == Captsel::Val22
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        *self == Captsel::Val23
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val24(&self) -> bool {
        *self == Captsel::Val24
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_25(&self) -> bool {
        *self == Captsel::Val25_25
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_26(&self) -> bool {
        *self == Captsel::Val25_26
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_27(&self) -> bool {
        *self == Captsel::Val25_27
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_28(&self) -> bool {
        *self == Captsel::Val25_28
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_29(&self) -> bool {
        *self == Captsel::Val25_29
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_30(&self) -> bool {
        *self == Captsel::Val25_30
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val25_31(&self) -> bool {
        *self == Captsel::Val25_31
    }
}
#[doc = "Field `CAPTSEL` writer - Input number to TIMER0 capture inputs 0 to 4"]
pub type CaptselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Captsel, crate::Safe>;
impl<'a, REG> CaptselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CT_INP0 function selected from IOCON register"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val0)
    }
    #[doc = "CT_INP1 function selected from IOCON register"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val1)
    }
    #[doc = "CT_INP2 function selected from IOCON register"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val2)
    }
    #[doc = "CT_INP3 function selected from IOCON register"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val3)
    }
    #[doc = "CT_INP4 function selected from IOCON register"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val4)
    }
    #[doc = "CT_INP5 function selected from IOCON register"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val5)
    }
    #[doc = "CT_INP6 function selected from IOCON register"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val6)
    }
    #[doc = "CT_INP7 function selected from IOCON register"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val7)
    }
    #[doc = "CT_INP8 function selected from IOCON register"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val8)
    }
    #[doc = "CT_INP9 function selected from IOCON register"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val9)
    }
    #[doc = "CT_INP10 function selected from IOCON register"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val10)
    }
    #[doc = "CT_INP11 function selected from IOCON register"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val11)
    }
    #[doc = "CT_INP12 function selected from IOCON register"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val12)
    }
    #[doc = "CT_INP13 function selected from IOCON register"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val13)
    }
    #[doc = "CT_INP14 function selected from IOCON register"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val14)
    }
    #[doc = "CT_INP15 function selected from IOCON register"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val15)
    }
    #[doc = "CT_INP16 function selected from IOCON register"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val16)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val17)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val18)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val19)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val20)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val21)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val22)
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val23)
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val24(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val24)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_25(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_25)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_26(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_26)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_27(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_27)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_28(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_28)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_29(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_29)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_30(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_30)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25_31(self) -> &'a mut crate::W<REG> {
        self.variant(Captsel::Val25_31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER0 capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CaptselR {
        CaptselR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0CAPTSEL")
            .field("captsel", &self.captsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER0 capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&mut self) -> CaptselW<'_, Timer0captselSpec> {
        CaptselW::new(self, 0)
    }
}
#[doc = "Capture select registers for TIMER0 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0captsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0captsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0captselSpec;
impl crate::RegisterSpec for Timer0captselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0captsel::R`](R) reader structure"]
impl crate::Readable for Timer0captselSpec {}
#[doc = "`write(|w| ..)` method takes [`timer0captsel::W`](W) writer structure"]
impl crate::Writable for Timer0captselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER0CAPTSEL[%s] to value 0x1f"]
impl crate::Resettable for Timer0captselSpec {
    const RESET_VALUE: u32 = 0x1f;
}

#[doc = "Register `SCT0_INMUX[%s]` reader"]
pub type R = crate::R<Sct0InmuxSpec>;
#[doc = "Register `SCT0_INMUX[%s]` writer"]
pub type W = crate::W<Sct0InmuxSpec>;
#[doc = "Input number to SCT0 inputs 0 to 6..\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InpN {
    #[doc = "0: SCT_GPI0 function selected from IOCON register"]
    Val0 = 0,
    #[doc = "1: SCT_GPI1 function selected from IOCON register"]
    Val1 = 1,
    #[doc = "2: SCT_GPI2 function selected from IOCON register"]
    Val2 = 2,
    #[doc = "3: SCT_GPI3 function selected from IOCON register"]
    Val3 = 3,
    #[doc = "4: SCT_GPI4 function selected from IOCON register"]
    Val4 = 4,
    #[doc = "5: SCT_GPI5 function selected from IOCON register"]
    Val5 = 5,
    #[doc = "6: SCT_GPI6 function selected from IOCON register"]
    Val6 = 6,
    #[doc = "7: SCT_GPI7 function selected from IOCON register"]
    Val7 = 7,
    #[doc = "8: T0_OUT0 ctimer 0 match\\[0\\] output"]
    Val8 = 8,
    #[doc = "9: T1_OUT0 ctimer 1 match\\[0\\] output"]
    Val9 = 9,
    #[doc = "10: T2_OUT0 ctimer 2 match\\[0\\] output"]
    Val10 = 10,
    #[doc = "11: T3_OUT0 ctimer 3 match\\[0\\] output"]
    Val11 = 11,
    #[doc = "12: T4_OUT0 ctimer 4 match\\[0\\] output"]
    Val12 = 12,
    #[doc = "13: ADC_IRQ interrupt request from ADC"]
    Val13 = 13,
    #[doc = "14: GPIOINT_BMATCH"]
    Val14 = 14,
    #[doc = "15: USB0_FRAME_TOGGLE"]
    Val15 = 15,
    #[doc = "16: USB1_FRAME_TOGGLE"]
    Val16 = 16,
    #[doc = "17: COMP_OUTPUT output from analog comparator"]
    Val17 = 17,
    #[doc = "18: I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    Val18 = 18,
    #[doc = "19: I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    Val19 = 19,
    #[doc = "20: I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    Val20 = 20,
    #[doc = "21: I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    Val21 = 21,
    #[doc = "22: ARM_TXEV interrupt event from cpu0 or cpu1"]
    Val22 = 22,
    #[doc = "23: DEBUG_HALTED from cpu0 or cpu1"]
    Val23 = 23,
    #[doc = "24: None"]
    Val24_24 = 24,
    #[doc = "25: None"]
    Val24_25 = 25,
    #[doc = "26: None"]
    Val24_26 = 26,
    #[doc = "27: None"]
    Val24_27 = 27,
    #[doc = "28: None"]
    Val24_28 = 28,
    #[doc = "29: None"]
    Val24_29 = 29,
    #[doc = "30: None"]
    Val24_30 = 30,
    #[doc = "31: None"]
    Val24_31 = 31,
}
impl From<InpN> for u8 {
    #[inline(always)]
    fn from(variant: InpN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InpN {
    type Ux = u8;
}
impl crate::IsEnum for InpN {}
#[doc = "Field `INP_N` reader - Input number to SCT0 inputs 0 to 6.."]
pub type InpNR = crate::FieldReader<InpN>;
impl InpNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InpN {
        match self.bits {
            0 => InpN::Val0,
            1 => InpN::Val1,
            2 => InpN::Val2,
            3 => InpN::Val3,
            4 => InpN::Val4,
            5 => InpN::Val5,
            6 => InpN::Val6,
            7 => InpN::Val7,
            8 => InpN::Val8,
            9 => InpN::Val9,
            10 => InpN::Val10,
            11 => InpN::Val11,
            12 => InpN::Val12,
            13 => InpN::Val13,
            14 => InpN::Val14,
            15 => InpN::Val15,
            16 => InpN::Val16,
            17 => InpN::Val17,
            18 => InpN::Val18,
            19 => InpN::Val19,
            20 => InpN::Val20,
            21 => InpN::Val21,
            22 => InpN::Val22,
            23 => InpN::Val23,
            24 => InpN::Val24_24,
            25 => InpN::Val24_25,
            26 => InpN::Val24_26,
            27 => InpN::Val24_27,
            28 => InpN::Val24_28,
            29 => InpN::Val24_29,
            30 => InpN::Val24_30,
            31 => InpN::Val24_31,
            _ => unreachable!(),
        }
    }
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == InpN::Val0
    }
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == InpN::Val1
    }
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == InpN::Val2
    }
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == InpN::Val3
    }
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == InpN::Val4
    }
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == InpN::Val5
    }
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == InpN::Val6
    }
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == InpN::Val7
    }
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == InpN::Val8
    }
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == InpN::Val9
    }
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == InpN::Val10
    }
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == InpN::Val11
    }
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == InpN::Val12
    }
    #[doc = "ADC_IRQ interrupt request from ADC"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == InpN::Val13
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == InpN::Val14
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == InpN::Val15
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == InpN::Val16
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == InpN::Val17
    }
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == InpN::Val18
    }
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == InpN::Val19
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == InpN::Val20
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == InpN::Val21
    }
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == InpN::Val22
    }
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        *self == InpN::Val23
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_24(&self) -> bool {
        *self == InpN::Val24_24
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_25(&self) -> bool {
        *self == InpN::Val24_25
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_26(&self) -> bool {
        *self == InpN::Val24_26
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_27(&self) -> bool {
        *self == InpN::Val24_27
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_28(&self) -> bool {
        *self == InpN::Val24_28
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_29(&self) -> bool {
        *self == InpN::Val24_29
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_30(&self) -> bool {
        *self == InpN::Val24_30
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val24_31(&self) -> bool {
        *self == InpN::Val24_31
    }
}
#[doc = "Field `INP_N` writer - Input number to SCT0 inputs 0 to 6.."]
pub type InpNW<'a, REG> = crate::FieldWriter<'a, REG, 5, InpN, crate::Safe>;
impl<'a, REG> InpNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val0)
    }
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val1)
    }
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val2)
    }
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val3)
    }
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val4)
    }
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val5)
    }
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val6)
    }
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val7)
    }
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val8)
    }
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val9)
    }
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val10)
    }
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val11)
    }
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val12)
    }
    #[doc = "ADC_IRQ interrupt request from ADC"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val13)
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val14)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val15)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val16)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val17)
    }
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val18)
    }
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val19)
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val20)
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val21)
    }
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val22)
    }
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val23)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_24(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_24)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_25(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_25)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_26(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_26)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_27(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_27)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_28(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_28)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_29(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_29)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_30(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_30)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val24_31(self) -> &'a mut crate::W<REG> {
        self.variant(InpN::Val24_31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&self) -> InpNR {
        InpNR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCT0_INMUX")
            .field("inp_n", &self.inp_n())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&mut self) -> InpNW<'_, Sct0InmuxSpec> {
        InpNW::new(self, 0)
    }
}
#[doc = "Input mux register for SCT0 input\n\nYou can [`read`](crate::Reg::read) this register and get [`sct0_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sct0_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sct0InmuxSpec;
impl crate::RegisterSpec for Sct0InmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sct0_inmux::R`](R) reader structure"]
impl crate::Readable for Sct0InmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`sct0_inmux::W`](W) writer structure"]
impl crate::Writable for Sct0InmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCT0_INMUX[%s] to value 0x1f"]
impl crate::Resettable for Sct0InmuxSpec {
    const RESET_VALUE: u32 = 0x1f;
}

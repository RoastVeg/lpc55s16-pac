#[doc = "Register `DMA0_ITRIG_INMUX[%s]` reader"]
pub type R = crate::R<Dma0ItrigInmuxSpec>;
#[doc = "Register `DMA0_ITRIG_INMUX[%s]` writer"]
pub type W = crate::W<Dma0ItrigInmuxSpec>;
#[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 22).\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inp {
    #[doc = "0: Pin interrupt 0"]
    Val0 = 0,
    #[doc = "1: Pin interrupt 1"]
    Val1 = 1,
    #[doc = "2: Pin interrupt 2"]
    Val2 = 2,
    #[doc = "3: Pin interrupt 3"]
    Val3 = 3,
    #[doc = "4: Timer CTIMER0 Match 0"]
    Val4 = 4,
    #[doc = "5: Timer CTIMER0 Match 1"]
    Val5 = 5,
    #[doc = "6: Timer CTIMER1 Match 0"]
    Val6 = 6,
    #[doc = "7: Timer CTIMER1 Match 1"]
    Val7 = 7,
    #[doc = "8: Timer CTIMER2 Match 0"]
    Val8 = 8,
    #[doc = "9: Timer CTIMER2 Match 1"]
    Val9 = 9,
    #[doc = "10: Timer CTIMER3 Match 0"]
    Val10 = 10,
    #[doc = "11: Timer CTIMER3 Match 1"]
    Val11 = 11,
    #[doc = "12: Timer CTIMER4 Match 0"]
    Val12 = 12,
    #[doc = "13: Timer CTIMER4 Match 1"]
    Val13 = 13,
    #[doc = "14: COMP_OUTPUT"]
    Val14 = 14,
    #[doc = "15: DMA0 output trigger mux 0"]
    Val15 = 15,
    #[doc = "16: DMA0 output trigger mux 1"]
    Val16 = 16,
    #[doc = "17: DMA0 output trigger mux 1"]
    Val17 = 17,
    #[doc = "18: DMA0 output trigger mux 3"]
    Val18 = 18,
    #[doc = "19: SCT0 DMA request 0"]
    Val19 = 19,
    #[doc = "20: SCT0 DMA request 1"]
    Val20 = 20,
    #[doc = "21: HASH DMA RX trigger"]
    Val21 = 21,
    #[doc = "22: None"]
    Val22_22 = 22,
    #[doc = "23: None"]
    Val22_23 = 23,
    #[doc = "24: None"]
    Val22_24 = 24,
    #[doc = "25: None"]
    Val22_25 = 25,
    #[doc = "26: None"]
    Val22_26 = 26,
    #[doc = "27: None"]
    Val22_27 = 27,
    #[doc = "28: None"]
    Val22_28 = 28,
    #[doc = "29: None"]
    Val22_29 = 29,
    #[doc = "30: None"]
    Val22_30 = 30,
    #[doc = "31: None"]
    Val22_31 = 31,
}
impl From<Inp> for u8 {
    #[inline(always)]
    fn from(variant: Inp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inp {
    type Ux = u8;
}
impl crate::IsEnum for Inp {}
#[doc = "Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpR = crate::FieldReader<Inp>;
impl InpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inp {
        match self.bits {
            0 => Inp::Val0,
            1 => Inp::Val1,
            2 => Inp::Val2,
            3 => Inp::Val3,
            4 => Inp::Val4,
            5 => Inp::Val5,
            6 => Inp::Val6,
            7 => Inp::Val7,
            8 => Inp::Val8,
            9 => Inp::Val9,
            10 => Inp::Val10,
            11 => Inp::Val11,
            12 => Inp::Val12,
            13 => Inp::Val13,
            14 => Inp::Val14,
            15 => Inp::Val15,
            16 => Inp::Val16,
            17 => Inp::Val17,
            18 => Inp::Val18,
            19 => Inp::Val19,
            20 => Inp::Val20,
            21 => Inp::Val21,
            22 => Inp::Val22_22,
            23 => Inp::Val22_23,
            24 => Inp::Val22_24,
            25 => Inp::Val22_25,
            26 => Inp::Val22_26,
            27 => Inp::Val22_27,
            28 => Inp::Val22_28,
            29 => Inp::Val22_29,
            30 => Inp::Val22_30,
            31 => Inp::Val22_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin interrupt 0"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Inp::Val0
    }
    #[doc = "Pin interrupt 1"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Inp::Val1
    }
    #[doc = "Pin interrupt 2"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Inp::Val2
    }
    #[doc = "Pin interrupt 3"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Inp::Val3
    }
    #[doc = "Timer CTIMER0 Match 0"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Inp::Val4
    }
    #[doc = "Timer CTIMER0 Match 1"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == Inp::Val5
    }
    #[doc = "Timer CTIMER1 Match 0"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == Inp::Val6
    }
    #[doc = "Timer CTIMER1 Match 1"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == Inp::Val7
    }
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Inp::Val8
    }
    #[doc = "Timer CTIMER2 Match 1"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == Inp::Val9
    }
    #[doc = "Timer CTIMER3 Match 0"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == Inp::Val10
    }
    #[doc = "Timer CTIMER3 Match 1"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == Inp::Val11
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == Inp::Val12
    }
    #[doc = "Timer CTIMER4 Match 1"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == Inp::Val13
    }
    #[doc = "COMP_OUTPUT"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == Inp::Val14
    }
    #[doc = "DMA0 output trigger mux 0"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == Inp::Val15
    }
    #[doc = "DMA0 output trigger mux 1"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == Inp::Val16
    }
    #[doc = "DMA0 output trigger mux 1"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == Inp::Val17
    }
    #[doc = "DMA0 output trigger mux 3"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == Inp::Val18
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == Inp::Val19
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == Inp::Val20
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == Inp::Val21
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_22(&self) -> bool {
        *self == Inp::Val22_22
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_23(&self) -> bool {
        *self == Inp::Val22_23
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_24(&self) -> bool {
        *self == Inp::Val22_24
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_25(&self) -> bool {
        *self == Inp::Val22_25
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_26(&self) -> bool {
        *self == Inp::Val22_26
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_27(&self) -> bool {
        *self == Inp::Val22_27
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_28(&self) -> bool {
        *self == Inp::Val22_28
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_29(&self) -> bool {
        *self == Inp::Val22_29
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_30(&self) -> bool {
        *self == Inp::Val22_30
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val22_31(&self) -> bool {
        *self == Inp::Val22_31
    }
}
#[doc = "Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 5, Inp, crate::Safe>;
impl<'a, REG> InpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin interrupt 0"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val0)
    }
    #[doc = "Pin interrupt 1"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val1)
    }
    #[doc = "Pin interrupt 2"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val2)
    }
    #[doc = "Pin interrupt 3"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val3)
    }
    #[doc = "Timer CTIMER0 Match 0"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val4)
    }
    #[doc = "Timer CTIMER0 Match 1"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val5)
    }
    #[doc = "Timer CTIMER1 Match 0"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val6)
    }
    #[doc = "Timer CTIMER1 Match 1"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val7)
    }
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val8)
    }
    #[doc = "Timer CTIMER2 Match 1"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val9)
    }
    #[doc = "Timer CTIMER3 Match 0"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val10)
    }
    #[doc = "Timer CTIMER3 Match 1"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val11)
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val12)
    }
    #[doc = "Timer CTIMER4 Match 1"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val13)
    }
    #[doc = "COMP_OUTPUT"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val14)
    }
    #[doc = "DMA0 output trigger mux 0"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val15)
    }
    #[doc = "DMA0 output trigger mux 1"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val16)
    }
    #[doc = "DMA0 output trigger mux 1"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val17)
    }
    #[doc = "DMA0 output trigger mux 3"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val18)
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val19)
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val20)
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val21)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_22(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_22)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_23(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_23)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_24(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_24)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_25(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_25)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_26(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_26)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_27(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_27)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_28(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_28)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_29(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_29)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_30(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_30)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val22_31(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22_31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_INMUX")
            .field("inp", &self.inp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Dma0ItrigInmuxSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "Trigger select register for DMA0 channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_itrig_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ItrigInmuxSpec;
impl crate::RegisterSpec for Dma0ItrigInmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma0_itrig_inmux::R`](R) reader structure"]
impl crate::Readable for Dma0ItrigInmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0_itrig_inmux::W`](W) writer structure"]
impl crate::Writable for Dma0ItrigInmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0_ITRIG_INMUX[%s] to value 0x1f"]
impl crate::Resettable for Dma0ItrigInmuxSpec {
    const RESET_VALUE: u32 = 0x1f;
}

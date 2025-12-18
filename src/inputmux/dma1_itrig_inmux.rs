#[doc = "Register `DMA1_ITRIG_INMUX[%s]` reader"]
pub type R = crate::R<Dma1ItrigInmuxSpec>;
#[doc = "Register `DMA1_ITRIG_INMUX[%s]` writer"]
pub type W = crate::W<Dma1ItrigInmuxSpec>;
#[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9).\n\nValue on reset: 15"]
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
    #[doc = "6: Timer CTIMER2 Match 0"]
    Val6 = 6,
    #[doc = "7: Timer CTIMER4 Match 0"]
    Val7 = 7,
    #[doc = "8: DMA1 output trigger mux 0"]
    Val8 = 8,
    #[doc = "9: DMA1 output trigger mux 1"]
    Val9 = 9,
    #[doc = "10: DMA1 output trigger mux 2"]
    Val10 = 10,
    #[doc = "11: DMA1 output trigger mux 3"]
    Val11 = 11,
    #[doc = "12: SCT0 DMA request 0"]
    Val12 = 12,
    #[doc = "13: SCT0 DMA request 1"]
    Val13 = 13,
    #[doc = "14: HASH DMA RX trigger"]
    Val14 = 14,
    #[doc = "15: None"]
    Val15 = 15,
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
#[doc = "Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
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
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == Inp::Val6
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == Inp::Val7
    }
    #[doc = "DMA1 output trigger mux 0"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Inp::Val8
    }
    #[doc = "DMA1 output trigger mux 1"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == Inp::Val9
    }
    #[doc = "DMA1 output trigger mux 2"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == Inp::Val10
    }
    #[doc = "DMA1 output trigger mux 3"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == Inp::Val11
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == Inp::Val12
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == Inp::Val13
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == Inp::Val14
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == Inp::Val15
    }
}
#[doc = "Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Inp, crate::Safe>;
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
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val6)
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val7)
    }
    #[doc = "DMA1 output trigger mux 0"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val8)
    }
    #[doc = "DMA1 output trigger mux 1"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val9)
    }
    #[doc = "DMA1 output trigger mux 2"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val10)
    }
    #[doc = "DMA1 output trigger mux 3"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val11)
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val12)
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val13)
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val14)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val15)
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_INMUX")
            .field("inp", &self.inp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Dma1ItrigInmuxSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "Trigger select register for DMA1 channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_itrig_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_itrig_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ItrigInmuxSpec;
impl crate::RegisterSpec for Dma1ItrigInmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1_itrig_inmux::R`](R) reader structure"]
impl crate::Readable for Dma1ItrigInmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1_itrig_inmux::W`](W) writer structure"]
impl crate::Writable for Dma1ItrigInmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_ITRIG_INMUX[%s] to value 0x0f"]
impl crate::Resettable for Dma1ItrigInmuxSpec {
    const RESET_VALUE: u32 = 0x0f;
}

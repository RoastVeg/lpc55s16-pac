#[doc = "Register `RX_CLR` reader"]
pub type R = crate::R<RxClrSpec>;
#[doc = "Register `RX_CLR` writer"]
pub type W = crate::W<RxClrSpec>;
#[doc = "The ENVADJ field adjusts the trip point for the envelope detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Envadj {
    #[doc = "0: Trip-Level Voltage is 0.1000 V"]
    Value0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.1125 V"]
    Value1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.1250 V"]
    Value2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.0875 V"]
    Value3 = 3,
}
impl From<Envadj> for u8 {
    #[inline(always)]
    fn from(variant: Envadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Envadj {
    type Ux = u8;
}
impl crate::IsEnum for Envadj {}
#[doc = "Field `ENVADJ` reader - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type EnvadjR = crate::FieldReader<Envadj>;
impl EnvadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Envadj> {
        match self.bits {
            0 => Some(Envadj::Value0),
            1 => Some(Envadj::Value1),
            2 => Some(Envadj::Value2),
            3 => Some(Envadj::Value3),
            _ => None,
        }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Envadj::Value0
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Envadj::Value1
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Envadj::Value2
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Envadj::Value3
    }
}
#[doc = "Field `ENVADJ` writer - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type EnvadjW<'a, REG> = crate::FieldWriter<'a, REG, 3, Envadj>;
impl<'a, REG> EnvadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Value0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Value1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Value2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Value3)
    }
}
#[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Disconadj {
    #[doc = "0: Trip-Level Voltage is 0.56875 V"]
    Value0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.55000 V"]
    Value1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.58125 V"]
    Value2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.60000 V"]
    Value3 = 3,
}
impl From<Disconadj> for u8 {
    #[inline(always)]
    fn from(variant: Disconadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Disconadj {
    type Ux = u8;
}
impl crate::IsEnum for Disconadj {}
#[doc = "Field `DISCONADJ` reader - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub type DisconadjR = crate::FieldReader<Disconadj>;
impl DisconadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Disconadj> {
        match self.bits {
            0 => Some(Disconadj::Value0),
            1 => Some(Disconadj::Value1),
            2 => Some(Disconadj::Value2),
            3 => Some(Disconadj::Value3),
            _ => None,
        }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Disconadj::Value0
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Disconadj::Value1
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Disconadj::Value2
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Disconadj::Value3
    }
}
#[doc = "Field `DISCONADJ` writer - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub type DisconadjW<'a, REG> = crate::FieldWriter<'a, REG, 3, Disconadj>;
impl<'a, REG> DisconadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Value0)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Value1)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Value2)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Value3)
    }
}
#[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdbypass {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    Value1 = 1,
}
impl From<Rxdbypass> for bool {
    #[inline(always)]
    fn from(variant: Rxdbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDBYPASS` reader - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub type RxdbypassR = crate::BitReader<Rxdbypass>;
impl RxdbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdbypass {
        match self.bits {
            false => Rxdbypass::Value0,
            true => Rxdbypass::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Rxdbypass::Value0
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxdbypass::Value1
    }
}
#[doc = "Field `RXDBYPASS` writer - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub type RxdbypassW<'a, REG> = crate::BitWriter<'a, REG, Rxdbypass>;
impl<'a, REG> RxdbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdbypass::Value0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdbypass::Value1)
    }
}
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> EnvadjR {
        EnvadjR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&self) -> DisconadjR {
        DisconadjR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RxdbypassR {
        RxdbypassR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> EnvadjW<'_, RxClrSpec> {
        EnvadjW::new(self, 0)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DisconadjW<'_, RxClrSpec> {
        DisconadjW::new(self, 4)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RxdbypassW<'_, RxClrSpec> {
        RxdbypassW::new(self, 22)
    }
}
#[doc = "USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxClrSpec;
impl crate::RegisterSpec for RxClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_clr::R`](R) reader structure"]
impl crate::Readable for RxClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_clr::W`](W) writer structure"]
impl crate::Writable for RxClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CLR to value 0"]
impl crate::Resettable for RxClrSpec {}

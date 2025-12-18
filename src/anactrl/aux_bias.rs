#[doc = "Register `AUX_BIAS` reader"]
pub type R = crate::R<AuxBiasSpec>;
#[doc = "Register `AUX_BIAS` writer"]
pub type W = crate::W<AuxBiasSpec>;
#[doc = "Control output of 1V reference voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vref1venable {
    #[doc = "0: Output of 1V reference voltage buffer is bypassed."]
    Disable = 0,
    #[doc = "1: Output of 1V reference voltage is enabled."]
    Enable = 1,
}
impl From<Vref1venable> for bool {
    #[inline(always)]
    fn from(variant: Vref1venable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREF1VENABLE` reader - Control output of 1V reference voltage."]
pub type Vref1venableR = crate::BitReader<Vref1venable>;
impl Vref1venableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vref1venable {
        match self.bits {
            false => Vref1venable::Disable,
            true => Vref1venable::Enable,
        }
    }
    #[doc = "Output of 1V reference voltage buffer is bypassed."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vref1venable::Disable
    }
    #[doc = "Output of 1V reference voltage is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vref1venable::Enable
    }
}
#[doc = "Field `VREF1VENABLE` writer - Control output of 1V reference voltage."]
pub type Vref1venableW<'a, REG> = crate::BitWriter<'a, REG, Vref1venable>;
impl<'a, REG> Vref1venableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output of 1V reference voltage buffer is bypassed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vref1venable::Disable)
    }
    #[doc = "Output of 1V reference voltage is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vref1venable::Enable)
    }
}
#[doc = "Field `ITRIM` reader - current trimming control word."]
pub type ItrimR = crate::FieldReader;
#[doc = "Field `ITRIM` writer - current trimming control word."]
pub type ItrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PTATITRIM` reader - current trimming control word for ptat current."]
pub type PtatitrimR = crate::FieldReader;
#[doc = "Field `PTATITRIM` writer - current trimming control word for ptat current."]
pub type PtatitrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREF1VTRIM` reader - voltage trimming control word."]
pub type Vref1vtrimR = crate::FieldReader;
#[doc = "Field `VREF1VTRIM` writer - voltage trimming control word."]
pub type Vref1vtrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREF1VCURVETRIM` reader - Control bit to configure trimming state of mirror."]
pub type Vref1vcurvetrimR = crate::FieldReader;
#[doc = "Field `VREF1VCURVETRIM` writer - Control bit to configure trimming state of mirror."]
pub type Vref1vcurvetrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITRIMCTRL0` reader - Control bit to configure trimming state of mirror."]
pub type Itrimctrl0R = crate::BitReader;
#[doc = "Field `ITRIMCTRL0` writer - Control bit to configure trimming state of mirror."]
pub type Itrimctrl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITRIMCTRL1` reader - Control bit to configure trimming state of mirror."]
pub type Itrimctrl1R = crate::BitReader;
#[doc = "Field `ITRIMCTRL1` writer - Control bit to configure trimming state of mirror."]
pub type Itrimctrl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Control output of 1V reference voltage."]
    #[inline(always)]
    pub fn vref1venable(&self) -> Vref1venableR {
        Vref1venableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - current trimming control word."]
    #[inline(always)]
    pub fn itrim(&self) -> ItrimR {
        ItrimR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - current trimming control word for ptat current."]
    #[inline(always)]
    pub fn ptatitrim(&self) -> PtatitrimR {
        PtatitrimR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - voltage trimming control word."]
    #[inline(always)]
    pub fn vref1vtrim(&self) -> Vref1vtrimR {
        Vref1vtrimR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn vref1vcurvetrim(&self) -> Vref1vcurvetrimR {
        Vref1vcurvetrimR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl0(&self) -> Itrimctrl0R {
        Itrimctrl0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl1(&self) -> Itrimctrl1R {
        Itrimctrl1R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Control output of 1V reference voltage."]
    #[inline(always)]
    pub fn vref1venable(&mut self) -> Vref1venableW<'_, AuxBiasSpec> {
        Vref1venableW::new(self, 1)
    }
    #[doc = "Bits 2:6 - current trimming control word."]
    #[inline(always)]
    pub fn itrim(&mut self) -> ItrimW<'_, AuxBiasSpec> {
        ItrimW::new(self, 2)
    }
    #[doc = "Bits 7:11 - current trimming control word for ptat current."]
    #[inline(always)]
    pub fn ptatitrim(&mut self) -> PtatitrimW<'_, AuxBiasSpec> {
        PtatitrimW::new(self, 7)
    }
    #[doc = "Bits 12:16 - voltage trimming control word."]
    #[inline(always)]
    pub fn vref1vtrim(&mut self) -> Vref1vtrimW<'_, AuxBiasSpec> {
        Vref1vtrimW::new(self, 12)
    }
    #[doc = "Bits 17:19 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn vref1vcurvetrim(&mut self) -> Vref1vcurvetrimW<'_, AuxBiasSpec> {
        Vref1vcurvetrimW::new(self, 17)
    }
    #[doc = "Bit 20 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl0(&mut self) -> Itrimctrl0W<'_, AuxBiasSpec> {
        Itrimctrl0W::new(self, 20)
    }
    #[doc = "Bit 21 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl1(&mut self) -> Itrimctrl1W<'_, AuxBiasSpec> {
        Itrimctrl1W::new(self, 21)
    }
}
#[doc = "AUX_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxBiasSpec;
impl crate::RegisterSpec for AuxBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_bias::R`](R) reader structure"]
impl crate::Readable for AuxBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`aux_bias::W`](W) writer structure"]
impl crate::Writable for AuxBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUX_BIAS to value 0x0007_03a0"]
impl crate::Resettable for AuxBiasSpec {
    const RESET_VALUE: u32 = 0x0007_03a0;
}

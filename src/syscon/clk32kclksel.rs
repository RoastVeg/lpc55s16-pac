#[doc = "Register `CLK32KCLKSEL` reader"]
pub type R = crate::R<Clk32kclkselSpec>;
#[doc = "Register `CLK32KCLKSEL` writer"]
pub type W = crate::W<Clk32kclkselSpec>;
#[doc = "clock low speed source select for HS USB.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: Oscillator 32 kHz clock."]
    Enum0x0 = 0,
    #[doc = "1: FRO1MHz_divided clock."]
    Enum0x1 = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - clock low speed source select for HS USB."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Enum0x0,
            true => Sel::Enum0x1,
        }
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == Sel::Enum0x0
    }
    #[doc = "FRO1MHz_divided clock."]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == Sel::Enum0x1
    }
}
#[doc = "Field `SEL` writer - clock low speed source select for HS USB."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x0)
    }
    #[doc = "FRO1MHz_divided clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Enum0x1)
    }
}
impl R {
    #[doc = "Bit 3 - clock low speed source select for HS USB."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32KCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - clock low speed source select for HS USB."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, Clk32kclkselSpec> {
        SelW::new(self, 3)
    }
}
#[doc = "clock low speed source select for HS USB.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk32kclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk32kclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clk32kclkselSpec;
impl crate::RegisterSpec for Clk32kclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk32kclksel::R`](R) reader structure"]
impl crate::Readable for Clk32kclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clk32kclksel::W`](W) writer structure"]
impl crate::Writable for Clk32kclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK32KCLKSEL to value 0"]
impl crate::Resettable for Clk32kclkselSpec {}

#[doc = "Register `FLEXFRG3CTRL` reader"]
pub type R = crate::R<FlexfrgctrlFlexfrg3ctrlSpec>;
#[doc = "Register `FLEXFRG3CTRL` writer"]
pub type W = crate::W<FlexfrgctrlFlexfrg3ctrlSpec>;
#[doc = "Field `DIV` reader - Denominator of the fractional rate divider."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Denominator of the fractional rate divider."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULT` reader - Numerator of the fractional rate divider."]
pub type MultR = crate::FieldReader;
#[doc = "Field `MULT` writer - Numerator of the fractional rate divider."]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGCTRL_FLEXFRG3CTRL")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, FlexfrgctrlFlexfrg3ctrlSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&mut self) -> MultW<'_, FlexfrgctrlFlexfrg3ctrlSpec> {
        MultW::new(self, 8)
    }
}
#[doc = "Fractional rate divider for flexcomm 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg3ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg3ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexfrgctrlFlexfrg3ctrlSpec;
impl crate::RegisterSpec for FlexfrgctrlFlexfrg3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrgctrl_flexfrg3ctrl::R`](R) reader structure"]
impl crate::Readable for FlexfrgctrlFlexfrg3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flexfrgctrl_flexfrg3ctrl::W`](W) writer structure"]
impl crate::Writable for FlexfrgctrlFlexfrg3ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEXFRG3CTRL to value 0xff"]
impl crate::Resettable for FlexfrgctrlFlexfrg3ctrlSpec {
    const RESET_VALUE: u32 = 0xff;
}

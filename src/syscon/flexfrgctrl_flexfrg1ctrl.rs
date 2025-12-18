#[doc = "Register `FLEXFRG1CTRL` reader"]
pub type R = crate::R<FlexfrgctrlFlexfrg1ctrlSpec>;
#[doc = "Register `FLEXFRG1CTRL` writer"]
pub type W = crate::W<FlexfrgctrlFlexfrg1ctrlSpec>;
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
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, FlexfrgctrlFlexfrg1ctrlSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&mut self) -> MultW<'_, FlexfrgctrlFlexfrg1ctrlSpec> {
        MultW::new(self, 8)
    }
}
#[doc = "Fractional rate divider for flexcomm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg1ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg1ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexfrgctrlFlexfrg1ctrlSpec;
impl crate::RegisterSpec for FlexfrgctrlFlexfrg1ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrgctrl_flexfrg1ctrl::R`](R) reader structure"]
impl crate::Readable for FlexfrgctrlFlexfrg1ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flexfrgctrl_flexfrg1ctrl::W`](W) writer structure"]
impl crate::Writable for FlexfrgctrlFlexfrg1ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEXFRG1CTRL to value 0xff"]
impl crate::Resettable for FlexfrgctrlFlexfrg1ctrlSpec {
    const RESET_VALUE: u32 = 0xff;
}

#[doc = "Register `FLEXFRG7CTRL` reader"]
pub type R = crate::R<FlexfrgctrlFlexfrg7ctrlSpec>;
#[doc = "Register `FLEXFRG7CTRL` writer"]
pub type W = crate::W<FlexfrgctrlFlexfrg7ctrlSpec>;
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
    pub fn div(&mut self) -> DivW<'_, FlexfrgctrlFlexfrg7ctrlSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&mut self) -> MultW<'_, FlexfrgctrlFlexfrg7ctrlSpec> {
        MultW::new(self, 8)
    }
}
#[doc = "Fractional rate divider for flexcomm 7\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg7ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg7ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexfrgctrlFlexfrg7ctrlSpec;
impl crate::RegisterSpec for FlexfrgctrlFlexfrg7ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrgctrl_flexfrg7ctrl::R`](R) reader structure"]
impl crate::Readable for FlexfrgctrlFlexfrg7ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flexfrgctrl_flexfrg7ctrl::W`](W) writer structure"]
impl crate::Writable for FlexfrgctrlFlexfrg7ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEXFRG7CTRL to value 0xff"]
impl crate::Resettable for FlexfrgctrlFlexfrg7ctrlSpec {
    const RESET_VALUE: u32 = 0xff;
}

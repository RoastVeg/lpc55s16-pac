#[doc = "Register `IENF` reader"]
pub type R = crate::R<IenfSpec>;
#[doc = "Register `IENF` writer"]
pub type W = crate::W<IenfSpec>;
#[doc = "Field `ENAF` reader - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub type EnafR = crate::FieldReader;
#[doc = "Field `ENAF` writer - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub type EnafW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&self) -> EnafR {
        EnafR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&mut self) -> EnafW<'_, IenfSpec> {
        EnafW::new(self, 0)
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenfSpec;
impl crate::RegisterSpec for IenfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ienf::R`](R) reader structure"]
impl crate::Readable for IenfSpec {}
#[doc = "`write(|w| ..)` method takes [`ienf::W`](W) writer structure"]
impl crate::Writable for IenfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IENF to value 0"]
impl crate::Resettable for IenfSpec {}

#[doc = "Register `PINTSECSEL[%s]` reader"]
pub type R = crate::R<PintsecselSpec>;
#[doc = "Register `PINTSECSEL[%s]` writer"]
pub type W = crate::W<PintsecselSpec>;
#[doc = "Field `INTPIN` reader - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub type IntpinR = crate::FieldReader;
#[doc = "Field `INTPIN` writer - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub type IntpinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&self) -> IntpinR {
        IntpinR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&mut self) -> IntpinW<'_, PintsecselSpec> {
        IntpinW::new(self, 0)
    }
}
#[doc = "Pin interrupt secure select register\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsecsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintsecsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintsecselSpec;
impl crate::RegisterSpec for PintsecselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pintsecsel::R`](R) reader structure"]
impl crate::Readable for PintsecselSpec {}
#[doc = "`write(|w| ..)` method takes [`pintsecsel::W`](W) writer structure"]
impl crate::Writable for PintsecselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINTSECSEL[%s] to value 0x3f"]
impl crate::Resettable for PintsecselSpec {
    const RESET_VALUE: u32 = 0x3f;
}

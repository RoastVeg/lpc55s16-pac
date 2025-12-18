#[doc = "Register `PINTSEL[%s]` reader"]
pub type R = crate::R<PintselSpec>;
#[doc = "Register `PINTSEL[%s]` writer"]
pub type W = crate::W<PintselSpec>;
#[doc = "Field `INTPIN` reader - Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
pub type IntpinR = crate::FieldReader;
#[doc = "Field `INTPIN` writer - Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
pub type IntpinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub fn intpin(&self) -> IntpinR {
        IntpinR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINTSEL")
            .field("intpin", &self.intpin())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub fn intpin(&mut self) -> IntpinW<'_, PintselSpec> {
        IntpinW::new(self, 0)
    }
}
#[doc = "Pin interrupt select register\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintselSpec;
impl crate::RegisterSpec for PintselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pintsel::R`](R) reader structure"]
impl crate::Readable for PintselSpec {}
#[doc = "`write(|w| ..)` method takes [`pintsel::W`](W) writer structure"]
impl crate::Writable for PintselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINTSEL[%s] to value 0x7f"]
impl crate::Resettable for PintselSpec {
    const RESET_VALUE: u32 = 0x7f;
}

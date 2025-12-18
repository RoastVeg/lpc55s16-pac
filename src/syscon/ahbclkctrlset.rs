#[doc = "Register `AHBCLKCTRLSET[%s]` reader"]
pub type R = crate::R<AhbclkctrlsetSpec>;
#[doc = "Register `AHBCLKCTRLSET[%s]` writer"]
pub type W = crate::W<AhbclkctrlsetSpec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLSET")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, AhbclkctrlsetSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrlset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrlset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbclkctrlsetSpec;
impl crate::RegisterSpec for AhbclkctrlsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbclkctrlset::R`](R) reader structure"]
impl crate::Readable for AhbclkctrlsetSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbclkctrlset::W`](W) writer structure"]
impl crate::Writable for AhbclkctrlsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCLKCTRLSET[%s] to value 0"]
impl crate::Resettable for AhbclkctrlsetSpec {}

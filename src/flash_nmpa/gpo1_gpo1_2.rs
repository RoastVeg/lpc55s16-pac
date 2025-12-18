#[doc = "Register `GPO1_2` reader"]
pub type R = crate::R<Gpo1Gpo1_2Spec>;
#[doc = "Register `GPO1_2` writer"]
pub type W = crate::W<Gpo1Gpo1_2Spec>;
#[doc = "Field `HVST` reader - High Voltage Stress: 0=not done; 1=done."]
pub type HvstR = crate::BitReader;
#[doc = "Field `HVST` writer - High Voltage Stress: 0=not done; 1=done."]
pub type HvstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub fn hvst(&self) -> HvstR {
        HvstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub fn hvst(&mut self) -> HvstW<'_, Gpo1Gpo1_2Spec> {
        HvstW::new(self, 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo1Gpo1_2Spec> {
        FieldW::new(self, 1)
    }
}
#[doc = "GPO1 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo1Gpo1_2Spec;
impl crate::RegisterSpec for Gpo1Gpo1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo1_gpo1_2::R`](R) reader structure"]
impl crate::Readable for Gpo1Gpo1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo1_gpo1_2::W`](W) writer structure"]
impl crate::Writable for Gpo1Gpo1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO1_2 to value 0"]
impl crate::Resettable for Gpo1Gpo1_2Spec {}

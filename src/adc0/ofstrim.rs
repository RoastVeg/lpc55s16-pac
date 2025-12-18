#[doc = "Register `OFSTRIM` reader"]
pub type R = crate::R<OfstrimSpec>;
#[doc = "Register `OFSTRIM` writer"]
pub type W = crate::W<OfstrimSpec>;
#[doc = "Field `OFSTRIM_A` reader - Trim for offset"]
pub type OfstrimAR = crate::FieldReader;
#[doc = "Field `OFSTRIM_A` writer - Trim for offset"]
pub type OfstrimAW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFSTRIM_B` reader - Trim for offset"]
pub type OfstrimBR = crate::FieldReader;
#[doc = "Field `OFSTRIM_B` writer - Trim for offset"]
pub type OfstrimBW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&self) -> OfstrimAR {
        OfstrimAR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&self) -> OfstrimBR {
        OfstrimBR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFSTRIM")
            .field("ofstrim_a", &self.ofstrim_a())
            .field("ofstrim_b", &self.ofstrim_b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&mut self) -> OfstrimAW<'_, OfstrimSpec> {
        OfstrimAW::new(self, 0)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&mut self) -> OfstrimBW<'_, OfstrimSpec> {
        OfstrimBW::new(self, 16)
    }
}
#[doc = "ADC Offset Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofstrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofstrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OfstrimSpec;
impl crate::RegisterSpec for OfstrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofstrim::R`](R) reader structure"]
impl crate::Readable for OfstrimSpec {}
#[doc = "`write(|w| ..)` method takes [`ofstrim::W`](W) writer structure"]
impl crate::Writable for OfstrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFSTRIM to value 0"]
impl crate::Resettable for OfstrimSpec {}

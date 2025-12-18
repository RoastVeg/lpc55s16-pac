#[doc = "Register `BOD` reader"]
pub type R = crate::R<BodSpec>;
#[doc = "Register `BOD` writer"]
pub type W = crate::W<BodSpec>;
#[doc = "Field `BOD_VBAT_TRIM_VALID` reader - no description available"]
pub type BodVbatTrimValidR = crate::BitReader;
#[doc = "Field `BOD_VBAT_TRIM_VALID` writer - no description available"]
pub type BodVbatTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_VBAT_TRIGLVL` reader - no description available"]
pub type BodVbatTriglvlR = crate::FieldReader;
#[doc = "Field `BOD_VBAT_TRIGLVL` writer - no description available"]
pub type BodVbatTriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BOD_VBAT_HYST` reader - no description available"]
pub type BodVbatHystR = crate::FieldReader;
#[doc = "Field `BOD_VBAT_HYST` writer - no description available"]
pub type BodVbatHystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOD_CORE_TRIM_VALID` reader - no description available"]
pub type BodCoreTrimValidR = crate::BitReader;
#[doc = "Field `BOD_CORE_TRIM_VALID` writer - no description available"]
pub type BodCoreTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_CORE_TRIGLVL` reader - no description available"]
pub type BodCoreTriglvlR = crate::FieldReader;
#[doc = "Field `BOD_CORE_TRIGLVL` writer - no description available"]
pub type BodCoreTriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BOD_CORE_HYST` reader - no description available"]
pub type BodCoreHystR = crate::FieldReader;
#[doc = "Field `BOD_CORE_HYST` writer - no description available"]
pub type BodCoreHystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_trim_valid(&self) -> BodVbatTrimValidR {
        BodVbatTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_triglvl(&self) -> BodVbatTriglvlR {
        BodVbatTriglvlR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_hyst(&self) -> BodVbatHystR {
        BodVbatHystR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn bod_core_trim_valid(&self) -> BodCoreTrimValidR {
        BodCoreTrimValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - no description available"]
    #[inline(always)]
    pub fn bod_core_triglvl(&self) -> BodCoreTriglvlR {
        BodCoreTriglvlR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 21:22 - no description available"]
    #[inline(always)]
    pub fn bod_core_hyst(&self) -> BodCoreHystR {
        BodCoreHystR::new(((self.bits >> 21) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD")
            .field("bod_vbat_trim_valid", &self.bod_vbat_trim_valid())
            .field("bod_vbat_triglvl", &self.bod_vbat_triglvl())
            .field("bod_vbat_hyst", &self.bod_vbat_hyst())
            .field("bod_core_trim_valid", &self.bod_core_trim_valid())
            .field("bod_core_triglvl", &self.bod_core_triglvl())
            .field("bod_core_hyst", &self.bod_core_hyst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_trim_valid(&mut self) -> BodVbatTrimValidW<'_, BodSpec> {
        BodVbatTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_triglvl(&mut self) -> BodVbatTriglvlW<'_, BodSpec> {
        BodVbatTriglvlW::new(self, 1)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_hyst(&mut self) -> BodVbatHystW<'_, BodSpec> {
        BodVbatHystW::new(self, 6)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn bod_core_trim_valid(&mut self) -> BodCoreTrimValidW<'_, BodSpec> {
        BodCoreTrimValidW::new(self, 16)
    }
    #[doc = "Bits 17:19 - no description available"]
    #[inline(always)]
    pub fn bod_core_triglvl(&mut self) -> BodCoreTriglvlW<'_, BodSpec> {
        BodCoreTriglvlW::new(self, 17)
    }
    #[doc = "Bits 21:22 - no description available"]
    #[inline(always)]
    pub fn bod_core_hyst(&mut self) -> BodCoreHystW<'_, BodSpec> {
        BodCoreHystW::new(self, 21)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`bod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodSpec;
impl crate::RegisterSpec for BodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod::R`](R) reader structure"]
impl crate::Readable for BodSpec {}
#[doc = "`write(|w| ..)` method takes [`bod::W`](W) writer structure"]
impl crate::Writable for BodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOD to value 0"]
impl crate::Resettable for BodSpec {}

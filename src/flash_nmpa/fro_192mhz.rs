#[doc = "Register `FRO_192MHZ` reader"]
pub type R = crate::R<Fro192mhzSpec>;
#[doc = "Register `FRO_192MHZ` writer"]
pub type W = crate::W<Fro192mhzSpec>;
#[doc = "Field `FRO192M_TRIM_VALID` reader - no description available"]
pub type Fro192mTrimValidR = crate::BitReader;
#[doc = "Field `FRO192M_TRIM_VALID` writer - no description available"]
pub type Fro192mTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRO192M_BIASTRIM` reader - FRO192M_BIASTRIM\\[5:0\\]."]
pub type Fro192mBiastrimR = crate::FieldReader;
#[doc = "Field `FRO192M_BIASTRIM` writer - FRO192M_BIASTRIM\\[5:0\\]."]
pub type Fro192mBiastrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRO192M_TEMPTRIM` reader - FRO192M_TEMPTRIM\\[6:0\\]."]
pub type Fro192mTemptrimR = crate::FieldReader;
#[doc = "Field `FRO192M_TEMPTRIM` writer - FRO192M_TEMPTRIM\\[6:0\\]."]
pub type Fro192mTemptrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FRO192M_DACTRIM` reader - FRO192M_DACTRIM\\[7:0\\]."]
pub type Fro192mDactrimR = crate::FieldReader;
#[doc = "Field `FRO192M_DACTRIM` writer - FRO192M_DACTRIM\\[7:0\\]."]
pub type Fro192mDactrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro192m_trim_valid(&self) -> Fro192mTrimValidR {
        Fro192mTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub fn fro192m_biastrim(&self) -> Fro192mBiastrimR {
        Fro192mBiastrimR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub fn fro192m_temptrim(&self) -> Fro192mTemptrimR {
        Fro192mTemptrimR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 17:24 - FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub fn fro192m_dactrim(&self) -> Fro192mDactrimR {
        Fro192mDactrimR::new(((self.bits >> 17) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro192m_trim_valid(&mut self) -> Fro192mTrimValidW<'_, Fro192mhzSpec> {
        Fro192mTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:6 - FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub fn fro192m_biastrim(&mut self) -> Fro192mBiastrimW<'_, Fro192mhzSpec> {
        Fro192mBiastrimW::new(self, 1)
    }
    #[doc = "Bits 8:14 - FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub fn fro192m_temptrim(&mut self) -> Fro192mTemptrimW<'_, Fro192mhzSpec> {
        Fro192mTemptrimW::new(self, 8)
    }
    #[doc = "Bits 17:24 - FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub fn fro192m_dactrim(&mut self) -> Fro192mDactrimW<'_, Fro192mhzSpec> {
        Fro192mDactrimW::new(self, 17)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`fro_192mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro_192mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fro192mhzSpec;
impl crate::RegisterSpec for Fro192mhzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro_192mhz::R`](R) reader structure"]
impl crate::Readable for Fro192mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`fro_192mhz::W`](W) writer structure"]
impl crate::Writable for Fro192mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRO_192MHZ to value 0"]
impl crate::Resettable for Fro192mhzSpec {}

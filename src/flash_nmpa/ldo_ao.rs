#[doc = "Register `LDO_AO` reader"]
pub type R = crate::R<LdoAoSpec>;
#[doc = "Register `LDO_AO` writer"]
pub type W = crate::W<LdoAoSpec>;
#[doc = "Field `ACTIVE_TRIM_VALID` reader - no description available"]
pub type ActiveTrimValidR = crate::BitReader;
#[doc = "Field `ACTIVE_TRIM_VALID` writer - no description available"]
pub type ActiveTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_TRIM` reader - no description available"]
pub type ActiveTrimR = crate::FieldReader;
#[doc = "Field `ACTIVE_TRIM` writer - no description available"]
pub type ActiveTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DSLP_TRIM_VALID` reader - no description available"]
pub type DslpTrimValidR = crate::BitReader;
#[doc = "Field `DSLP_TRIM_VALID` writer - no description available"]
pub type DslpTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSLP_TRIM` reader - no description available"]
pub type DslpTrimR = crate::FieldReader;
#[doc = "Field `DSLP_TRIM` writer - no description available"]
pub type DslpTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PDWN_TRIM_VALID` reader - no description available"]
pub type PdwnTrimValidR = crate::BitReader;
#[doc = "Field `PDWN_TRIM_VALID` writer - no description available"]
pub type PdwnTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN_TRIM` reader - no description available"]
pub type PdwnTrimR = crate::FieldReader;
#[doc = "Field `PDWN_TRIM` writer - no description available"]
pub type PdwnTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPDW_TRIM_VALID` reader - no description available"]
pub type DpdwTrimValidR = crate::BitReader;
#[doc = "Field `DPDW_TRIM_VALID` writer - no description available"]
pub type DpdwTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDW_TRIM` reader - no description available"]
pub type DpdwTrimR = crate::FieldReader;
#[doc = "Field `DPDW_TRIM` writer - no description available"]
pub type DpdwTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn active_trim_valid(&self) -> ActiveTrimValidR {
        ActiveTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn active_trim(&self) -> ActiveTrimR {
        ActiveTrimR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn dslp_trim_valid(&self) -> DslpTrimValidR {
        DslpTrimValidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - no description available"]
    #[inline(always)]
    pub fn dslp_trim(&self) -> DslpTrimR {
        DslpTrimR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim_valid(&self) -> PdwnTrimValidR {
        PdwnTrimValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim(&self) -> PdwnTrimR {
        PdwnTrimR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim_valid(&self) -> DpdwTrimValidR {
        DpdwTrimValidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim(&self) -> DpdwTrimR {
        DpdwTrimR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_AO")
            .field("active_trim_valid", &self.active_trim_valid())
            .field("active_trim", &self.active_trim())
            .field("dslp_trim_valid", &self.dslp_trim_valid())
            .field("dslp_trim", &self.dslp_trim())
            .field("pdwn_trim_valid", &self.pdwn_trim_valid())
            .field("pdwn_trim", &self.pdwn_trim())
            .field("dpdw_trim_valid", &self.dpdw_trim_valid())
            .field("dpdw_trim", &self.dpdw_trim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn active_trim_valid(&mut self) -> ActiveTrimValidW<'_, LdoAoSpec> {
        ActiveTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn active_trim(&mut self) -> ActiveTrimW<'_, LdoAoSpec> {
        ActiveTrimW::new(self, 1)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn dslp_trim_valid(&mut self) -> DslpTrimValidW<'_, LdoAoSpec> {
        DslpTrimValidW::new(self, 8)
    }
    #[doc = "Bits 9:13 - no description available"]
    #[inline(always)]
    pub fn dslp_trim(&mut self) -> DslpTrimW<'_, LdoAoSpec> {
        DslpTrimW::new(self, 9)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim_valid(&mut self) -> PdwnTrimValidW<'_, LdoAoSpec> {
        PdwnTrimValidW::new(self, 16)
    }
    #[doc = "Bits 17:21 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim(&mut self) -> PdwnTrimW<'_, LdoAoSpec> {
        PdwnTrimW::new(self, 17)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim_valid(&mut self) -> DpdwTrimValidW<'_, LdoAoSpec> {
        DpdwTrimValidW::new(self, 24)
    }
    #[doc = "Bits 25:29 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim(&mut self) -> DpdwTrimW<'_, LdoAoSpec> {
        DpdwTrimW::new(self, 25)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo_ao::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_ao::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoAoSpec;
impl crate::RegisterSpec for LdoAoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo_ao::R`](R) reader structure"]
impl crate::Readable for LdoAoSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo_ao::W`](W) writer structure"]
impl crate::Writable for LdoAoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO_AO to value 0"]
impl crate::Resettable for LdoAoSpec {}

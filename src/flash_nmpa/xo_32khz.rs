#[doc = "Register `XO_32KHZ` reader"]
pub type R = crate::R<Xo32khzSpec>;
#[doc = "Register `XO_32KHZ` writer"]
pub type W = crate::W<Xo32khzSpec>;
#[doc = "Field `XO32K_XIN_TRIM_VALID` reader - no description available"]
pub type Xo32kXinTrimValidR = crate::BitReader;
#[doc = "Field `XO32K_XIN_TRIM_VALID` writer - no description available"]
pub type Xo32kXinTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO32K_XIN_CAPCAL_6PF` reader - no description available"]
pub type Xo32kXinCapcal6pfR = crate::FieldReader;
#[doc = "Field `XO32K_XIN_CAPCAL_6PF` writer - no description available"]
pub type Xo32kXinCapcal6pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32K_XIN_CAPCAL_8PF` reader - no description available"]
pub type Xo32kXinCapcal8pfR = crate::FieldReader;
#[doc = "Field `XO32K_XIN_CAPCAL_8PF` writer - no description available"]
pub type Xo32kXinCapcal8pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32K_XOUT_TRIM_VALID` reader - no description available"]
pub type Xo32kXoutTrimValidR = crate::BitReader;
#[doc = "Field `XO32K_XOUT_TRIM_VALID` writer - no description available"]
pub type Xo32kXoutTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO32K_XOUT_CAPCAL_6PF` reader - no description available"]
pub type Xo32kXoutCapcal6pfR = crate::FieldReader;
#[doc = "Field `XO32K_XOUT_CAPCAL_6PF` writer - no description available"]
pub type Xo32kXoutCapcal6pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32K_XOUT_CAPCAL_8PF` reader - no description available"]
pub type Xo32kXoutCapcal8pfR = crate::FieldReader;
#[doc = "Field `XO32K_XOUT_CAPCAL_8PF` writer - no description available"]
pub type Xo32kXoutCapcal8pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_trim_valid(&self) -> Xo32kXinTrimValidR {
        Xo32kXinTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_6pf(&self) -> Xo32kXinCapcal6pfR {
        Xo32kXinCapcal6pfR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_8pf(&self) -> Xo32kXinCapcal8pfR {
        Xo32kXinCapcal8pfR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_trim_valid(&self) -> Xo32kXoutTrimValidR {
        Xo32kXoutTrimValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_6pf(&self) -> Xo32kXoutCapcal6pfR {
        Xo32kXoutCapcal6pfR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_8pf(&self) -> Xo32kXoutCapcal8pfR {
        Xo32kXoutCapcal8pfR::new(((self.bits >> 23) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_trim_valid(&mut self) -> Xo32kXinTrimValidW<'_, Xo32khzSpec> {
        Xo32kXinTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_6pf(&mut self) -> Xo32kXinCapcal6pfW<'_, Xo32khzSpec> {
        Xo32kXinCapcal6pfW::new(self, 1)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_8pf(&mut self) -> Xo32kXinCapcal8pfW<'_, Xo32khzSpec> {
        Xo32kXinCapcal8pfW::new(self, 8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_trim_valid(&mut self) -> Xo32kXoutTrimValidW<'_, Xo32khzSpec> {
        Xo32kXoutTrimValidW::new(self, 15)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_6pf(&mut self) -> Xo32kXoutCapcal6pfW<'_, Xo32khzSpec> {
        Xo32kXoutCapcal6pfW::new(self, 16)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_8pf(&mut self) -> Xo32kXoutCapcal8pfW<'_, Xo32khzSpec> {
        Xo32kXoutCapcal8pfW::new(self, 23)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`xo_32khz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo_32khz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xo32khzSpec;
impl crate::RegisterSpec for Xo32khzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xo_32khz::R`](R) reader structure"]
impl crate::Readable for Xo32khzSpec {}
#[doc = "`write(|w| ..)` method takes [`xo_32khz::W`](W) writer structure"]
impl crate::Writable for Xo32khzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XO_32KHZ to value 0"]
impl crate::Resettable for Xo32khzSpec {}

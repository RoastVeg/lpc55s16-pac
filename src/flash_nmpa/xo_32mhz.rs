#[doc = "Register `XO_32MHZ` reader"]
pub type R = crate::R<Xo32mhzSpec>;
#[doc = "Register `XO_32MHZ` writer"]
pub type W = crate::W<Xo32mhzSpec>;
#[doc = "Field `XO32M_XIN_TRIM_VALID` reader - no description available"]
pub type Xo32mXinTrimValidR = crate::BitReader;
#[doc = "Field `XO32M_XIN_TRIM_VALID` writer - no description available"]
pub type Xo32mXinTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO32M_XIN_CAPCAL_6PF` reader - no description available"]
pub type Xo32mXinCapcal6pfR = crate::FieldReader;
#[doc = "Field `XO32M_XIN_CAPCAL_6PF` writer - no description available"]
pub type Xo32mXinCapcal6pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32M_XIN_CAPCAL_8PF` reader - no description available"]
pub type Xo32mXinCapcal8pfR = crate::FieldReader;
#[doc = "Field `XO32M_XIN_CAPCAL_8PF` writer - no description available"]
pub type Xo32mXinCapcal8pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32M_XOUT_TRIM_VALID` reader - no description available"]
pub type Xo32mXoutTrimValidR = crate::BitReader;
#[doc = "Field `XO32M_XOUT_TRIM_VALID` writer - no description available"]
pub type Xo32mXoutTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO32M_XOUT_CAPCAL_6PF` reader - no description available"]
pub type Xo32mXoutCapcal6pfR = crate::FieldReader;
#[doc = "Field `XO32M_XOUT_CAPCAL_6PF` writer - no description available"]
pub type Xo32mXoutCapcal6pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32M_XOUT_CAPCAL_8PF` reader - no description available"]
pub type Xo32mXoutCapcal8pfR = crate::FieldReader;
#[doc = "Field `XO32M_XOUT_CAPCAL_8PF` writer - no description available"]
pub type Xo32mXoutCapcal8pfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XO32M_XO_SLAVE_STATUS` reader - no description available"]
pub type Xo32mXoSlaveStatusR = crate::BitReader;
#[doc = "Field `XO32M_XO_SLAVE_STATUS` writer - no description available"]
pub type Xo32mXoSlaveStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XO32M_XO_AC_BUF_STATUS` reader - no description available"]
pub type Xo32mXoAcBufStatusR = crate::BitReader;
#[doc = "Field `XO32M_XO_AC_BUF_STATUS` writer - no description available"]
pub type Xo32mXoAcBufStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_trim_valid(&self) -> Xo32mXinTrimValidR {
        Xo32mXinTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_capcal_6pf(&self) -> Xo32mXinCapcal6pfR {
        Xo32mXinCapcal6pfR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_capcal_8pf(&self) -> Xo32mXinCapcal8pfR {
        Xo32mXinCapcal8pfR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_trim_valid(&self) -> Xo32mXoutTrimValidR {
        Xo32mXoutTrimValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_capcal_6pf(&self) -> Xo32mXoutCapcal6pfR {
        Xo32mXoutCapcal6pfR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_capcal_8pf(&self) -> Xo32mXoutCapcal8pfR {
        Xo32mXoutCapcal8pfR::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn xo32m_xo_slave_status(&self) -> Xo32mXoSlaveStatusR {
        Xo32mXoSlaveStatusR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn xo32m_xo_ac_buf_status(&self) -> Xo32mXoAcBufStatusR {
        Xo32mXoAcBufStatusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_trim_valid(&mut self) -> Xo32mXinTrimValidW<'_, Xo32mhzSpec> {
        Xo32mXinTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_capcal_6pf(&mut self) -> Xo32mXinCapcal6pfW<'_, Xo32mhzSpec> {
        Xo32mXinCapcal6pfW::new(self, 1)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32m_xin_capcal_8pf(&mut self) -> Xo32mXinCapcal8pfW<'_, Xo32mhzSpec> {
        Xo32mXinCapcal8pfW::new(self, 8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_trim_valid(&mut self) -> Xo32mXoutTrimValidW<'_, Xo32mhzSpec> {
        Xo32mXoutTrimValidW::new(self, 15)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_capcal_6pf(&mut self) -> Xo32mXoutCapcal6pfW<'_, Xo32mhzSpec> {
        Xo32mXoutCapcal6pfW::new(self, 16)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32m_xout_capcal_8pf(&mut self) -> Xo32mXoutCapcal8pfW<'_, Xo32mhzSpec> {
        Xo32mXoutCapcal8pfW::new(self, 23)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn xo32m_xo_slave_status(&mut self) -> Xo32mXoSlaveStatusW<'_, Xo32mhzSpec> {
        Xo32mXoSlaveStatusW::new(self, 30)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn xo32m_xo_ac_buf_status(&mut self) -> Xo32mXoAcBufStatusW<'_, Xo32mhzSpec> {
        Xo32mXoAcBufStatusW::new(self, 31)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`xo_32mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo_32mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xo32mhzSpec;
impl crate::RegisterSpec for Xo32mhzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xo_32mhz::R`](R) reader structure"]
impl crate::Readable for Xo32mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`xo_32mhz::W`](W) writer structure"]
impl crate::Writable for Xo32mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XO_32MHZ to value 0"]
impl crate::Resettable for Xo32mhzSpec {}

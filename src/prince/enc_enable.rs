#[doc = "Register `ENC_ENABLE` reader"]
pub type R = crate::R<EncEnableSpec>;
#[doc = "Register `ENC_ENABLE` writer"]
pub type W = crate::W<EncEnableSpec>;
#[doc = "Enables PRINCE encryption for flash programming.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Encryption of writes to the flash controller DATAW* registers is disabled."]
    Disabled = 0,
    #[doc = "1: Encryption of writes to the flash controller DATAW* registers is enabled. Reading of PRINCE-encrypted flash regions is disabled."]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enables PRINCE encryption for flash programming."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Encryption of writes to the flash controller DATAW* registers is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Encryption of writes to the flash controller DATAW* registers is enabled. Reading of PRINCE-encrypted flash regions is disabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - Enables PRINCE encryption for flash programming."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encryption of writes to the flash controller DATAW* registers is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Encryption of writes to the flash controller DATAW* registers is enabled. Reading of PRINCE-encrypted flash regions is disabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables PRINCE encryption for flash programming."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENC_ENABLE")
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enables PRINCE encryption for flash programming."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, EncEnableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Encryption Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enc_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EncEnableSpec;
impl crate::RegisterSpec for EncEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enc_enable::R`](R) reader structure"]
impl crate::Readable for EncEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enc_enable::W`](W) writer structure"]
impl crate::Writable for EncEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENC_ENABLE to value 0"]
impl crate::Resettable for EncEnableSpec {}

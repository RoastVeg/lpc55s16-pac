#[doc = "Register `SLVADR3` reader"]
pub type R = crate::R<Slvadr3Spec>;
#[doc = "Register `SLVADR3` writer"]
pub type W = crate::W<Slvadr3Spec>;
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadisable {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    Enabled = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    Disabled = 1,
}
impl From<Sadisable> for bool {
    #[inline(always)]
    fn from(variant: Sadisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SADISABLE` reader - Slave Address n Disable."]
pub type SadisableR = crate::BitReader<Sadisable>;
impl SadisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sadisable {
        match self.bits {
            false => Sadisable::Enabled,
            true => Sadisable::Disabled,
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sadisable::Enabled
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sadisable::Disabled
    }
}
#[doc = "Field `SADISABLE` writer - Slave Address n Disable."]
pub type SadisableW<'a, REG> = crate::BitWriter<'a, REG, Sadisable>;
impl<'a, REG> SadisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Enabled)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Disabled)
    }
}
#[doc = "Field `SLVADR` reader - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrR = crate::FieldReader;
#[doc = "Field `SLVADR` writer - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SadisableR {
        SadisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SlvadrR {
        SlvadrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&mut self) -> SadisableW<'_, Slvadr3Spec> {
        SadisableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&mut self) -> SlvadrW<'_, Slvadr3Spec> {
        SlvadrW::new(self, 1)
    }
}
#[doc = "Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slvadr3Spec;
impl crate::RegisterSpec for Slvadr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvadr3::R`](R) reader structure"]
impl crate::Readable for Slvadr3Spec {}
#[doc = "`write(|w| ..)` method takes [`slvadr3::W`](W) writer structure"]
impl crate::Writable for Slvadr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLVADR3 to value 0x01"]
impl crate::Resettable for Slvadr3Spec {
    const RESET_VALUE: u32 = 0x01;
}

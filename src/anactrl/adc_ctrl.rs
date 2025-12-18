#[doc = "Register `ADC_CTRL` reader"]
pub type R = crate::R<AdcCtrlSpec>;
#[doc = "Register `ADC_CTRL` writer"]
pub type W = crate::W<AdcCtrlSpec>;
#[doc = "Switch On/Off VBAT divider branch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatdivenable {
    #[doc = "0: VBAT divider branch is disabled."]
    Disable = 0,
    #[doc = "1: VBAT divider branch is enabled."]
    Enable = 1,
}
impl From<Vbatdivenable> for bool {
    #[inline(always)]
    fn from(variant: Vbatdivenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATDIVENABLE` reader - Switch On/Off VBAT divider branch."]
pub type VbatdivenableR = crate::BitReader<Vbatdivenable>;
impl VbatdivenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatdivenable {
        match self.bits {
            false => Vbatdivenable::Disable,
            true => Vbatdivenable::Enable,
        }
    }
    #[doc = "VBAT divider branch is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vbatdivenable::Disable
    }
    #[doc = "VBAT divider branch is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vbatdivenable::Enable
    }
}
#[doc = "Field `VBATDIVENABLE` writer - Switch On/Off VBAT divider branch."]
pub type VbatdivenableW<'a, REG> = crate::BitWriter<'a, REG, Vbatdivenable>;
impl<'a, REG> VbatdivenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT divider branch is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatdivenable::Disable)
    }
    #[doc = "VBAT divider branch is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatdivenable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub fn vbatdivenable(&self) -> VbatdivenableR {
        VbatdivenableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub fn vbatdivenable(&mut self) -> VbatdivenableW<'_, AdcCtrlSpec> {
        VbatdivenableW::new(self, 0)
    }
}
#[doc = "General Purpose ADC VBAT Divider branch control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCtrlSpec;
impl crate::RegisterSpec for AdcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ctrl::R`](R) reader structure"]
impl crate::Readable for AdcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_ctrl::W`](W) writer structure"]
impl crate::Writable for AdcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_CTRL to value 0"]
impl crate::Resettable for AdcCtrlSpec {}

#[doc = "Register `GPIOPSYNC` reader"]
pub type R = crate::R<GpiopsyncSpec>;
#[doc = "Register `GPIOPSYNC` writer"]
pub type W = crate::W<GpiopsyncSpec>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psync {
    #[doc = "0: use the first stage of synchonization inside GPIO_INT module."]
    Used = 0,
    #[doc = "1: bypass of the first stage of synchonization inside GPIO_INT module."]
    Bypass = 1,
}
impl From<Psync> for bool {
    #[inline(always)]
    fn from(variant: Psync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSYNC` reader - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
pub type PsyncR = crate::BitReader<Psync>;
impl PsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psync {
        match self.bits {
            false => Psync::Used,
            true => Psync::Bypass,
        }
    }
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == Psync::Used
    }
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Psync::Bypass
    }
}
#[doc = "Field `PSYNC` writer - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
pub type PsyncW<'a, REG> = crate::BitWriter<'a, REG, Psync>;
impl<'a, REG> PsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn used(self) -> &'a mut crate::W<REG> {
        self.variant(Psync::Used)
    }
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Psync::Bypass)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn psync(&self) -> PsyncR {
        PsyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn psync(&mut self) -> PsyncW<'_, GpiopsyncSpec> {
        PsyncW::new(self, 0)
    }
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiopsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiopsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiopsyncSpec;
impl crate::RegisterSpec for GpiopsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiopsync::R`](R) reader structure"]
impl crate::Readable for GpiopsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiopsync::W`](W) writer structure"]
impl crate::Writable for GpiopsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOPSYNC to value 0"]
impl crate::Resettable for GpiopsyncSpec {}

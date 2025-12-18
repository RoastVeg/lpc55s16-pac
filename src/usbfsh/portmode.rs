#[doc = "Register `PORTMODE` reader"]
pub type R = crate::R<PortmodeSpec>;
#[doc = "Register `PORTMODE` writer"]
pub type W = crate::W<PortmodeSpec>;
#[doc = "Field `ID` reader - Port ID pin value."]
pub type IdR = crate::BitReader;
#[doc = "Field `ID` writer - Port ID pin value."]
pub type IdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_EN` reader - Port ID pin pull-up enable."]
pub type IdEnR = crate::BitReader;
#[doc = "Field `ID_EN` writer - Port ID pin pull-up enable."]
pub type IdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_ENABLE` reader - 1: device 0: host."]
pub type DevEnableR = crate::BitReader;
#[doc = "Field `DEV_ENABLE` writer - 1: device 0: host."]
pub type DevEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&self) -> IdEnR {
        IdEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DevEnableR {
        DevEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMODE")
            .field("id", &self.id())
            .field("id_en", &self.id_en())
            .field("dev_enable", &self.dev_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, PortmodeSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&mut self) -> IdEnW<'_, PortmodeSpec> {
        IdEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DevEnableW<'_, PortmodeSpec> {
        DevEnableW::new(self, 16)
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nYou can [`read`](crate::Reg::read) this register and get [`portmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortmodeSpec;
impl crate::RegisterSpec for PortmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portmode::R`](R) reader structure"]
impl crate::Readable for PortmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`portmode::W`](W) writer structure"]
impl crate::Writable for PortmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTMODE to value 0"]
impl crate::Resettable for PortmodeSpec {}

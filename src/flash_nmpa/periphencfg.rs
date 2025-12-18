#[doc = "Register `PERIPHENCFG` reader"]
pub type R = crate::R<PeriphencfgSpec>;
#[doc = "Register `PERIPHENCFG` writer"]
pub type W = crate::W<PeriphencfgSpec>;
#[doc = "Field `PERIPHERAL_CONFIGURATION` reader - no description available"]
pub type PeripheralConfigurationR = crate::FieldReader<u16>;
#[doc = "Field `PERIPHERAL_CONFIGURATION` writer - no description available"]
pub type PeripheralConfigurationW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CPU1_ENABLE` reader - no description available"]
pub type Cpu1EnableR = crate::BitReader;
#[doc = "Field `CPU1_ENABLE` writer - no description available"]
pub type Cpu1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn peripheral_configuration(&self) -> PeripheralConfigurationR {
        PeripheralConfigurationR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn cpu1_enable(&self) -> Cpu1EnableR {
        Cpu1EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPHENCFG")
            .field("peripheral_configuration", &self.peripheral_configuration())
            .field("cpu1_enable", &self.cpu1_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn peripheral_configuration(&mut self) -> PeripheralConfigurationW<'_, PeriphencfgSpec> {
        PeripheralConfigurationW::new(self, 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn cpu1_enable(&mut self) -> Cpu1EnableW<'_, PeriphencfgSpec> {
        Cpu1EnableW::new(self, 31)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`periphencfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periphencfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphencfgSpec;
impl crate::RegisterSpec for PeriphencfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphencfg::R`](R) reader structure"]
impl crate::Readable for PeriphencfgSpec {}
#[doc = "`write(|w| ..)` method takes [`periphencfg::W`](W) writer structure"]
impl crate::Writable for PeriphencfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPHENCFG to value 0"]
impl crate::Resettable for PeriphencfgSpec {}

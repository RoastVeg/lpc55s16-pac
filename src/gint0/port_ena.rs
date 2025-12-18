#[doc = "Register `PORT_ENA[%s]` reader"]
pub type R = crate::R<PortEnaSpec>;
#[doc = "Register `PORT_ENA[%s]` writer"]
pub type W = crate::W<PortEnaSpec>;
#[doc = "Field `ENA` reader - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
pub type EnaR = crate::FieldReader<u32>;
#[doc = "Field `ENA` writer - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
pub type EnaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena(&mut self) -> EnaW<'_, PortEnaSpec> {
        EnaW::new(self, 0)
    }
}
#[doc = "GPIO grouped interrupt port 0 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortEnaSpec;
impl crate::RegisterSpec for PortEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_ena::R`](R) reader structure"]
impl crate::Readable for PortEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`port_ena::W`](W) writer structure"]
impl crate::Writable for PortEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORT_ENA[%s] to value 0"]
impl crate::Resettable for PortEnaSpec {}

#[doc = "Register `PORT_POL[%s]` reader"]
pub type R = crate::R<PortPolSpec>;
#[doc = "Register `PORT_POL[%s]` writer"]
pub type W = crate::W<PortPolSpec>;
#[doc = "Field `POL` reader - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type PolR = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type PolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, PortPolSpec> {
        PolW::new(self, 0)
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortPolSpec;
impl crate::RegisterSpec for PortPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_pol::R`](R) reader structure"]
impl crate::Readable for PortPolSpec {}
#[doc = "`write(|w| ..)` method takes [`port_pol::W`](W) writer structure"]
impl crate::Writable for PortPolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORT_POL[%s] to value 0xffff_ffff"]
impl crate::Resettable for PortPolSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

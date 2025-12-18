#[doc = "Register `INSTRUCTION_TIMER` reader"]
pub type R = crate::R<InstructionTimerSpec>;
#[doc = "Register `INSTRUCTION_TIMER` writer"]
pub type W = crate::W<InstructionTimerSpec>;
#[doc = "Field `INSTIM` reader - INSTRUCTION TIMER 32-bit value"]
pub type InstimR = crate::FieldReader<u32>;
#[doc = "Field `INSTIM` writer - INSTRUCTION TIMER 32-bit value"]
pub type InstimW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - INSTRUCTION TIMER 32-bit value"]
    #[inline(always)]
    pub fn instim(&self) -> InstimR {
        InstimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INSTRUCTION TIMER 32-bit value"]
    #[inline(always)]
    pub fn instim(&mut self) -> InstimW<'_, InstructionTimerSpec> {
        InstimW::new(self, 0)
    }
}
#[doc = "The INSTRUCTION TIMER itself\n\nYou can [`read`](crate::Reg::read) this register and get [`instruction_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instruction_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstructionTimerSpec;
impl crate::RegisterSpec for InstructionTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instruction_timer::R`](R) reader structure"]
impl crate::Readable for InstructionTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`instruction_timer::W`](W) writer structure"]
impl crate::Writable for InstructionTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSTRUCTION_TIMER to value 0xffff_ffff"]
impl crate::Resettable for InstructionTimerSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

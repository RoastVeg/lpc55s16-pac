#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Register `STATE` writer"]
pub type W = crate::W<StateSpec>;
#[doc = "Field `STATE_L` reader - State variable."]
pub type StateLR = crate::FieldReader;
#[doc = "Field `STATE_L` writer - State variable."]
pub type StateLW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `STATE_H` reader - State variable."]
pub type StateHR = crate::FieldReader;
#[doc = "Field `STATE_H` writer - State variable."]
pub type StateHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&self) -> StateLR {
        StateLR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&self) -> StateHR {
        StateHR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&mut self) -> StateLW<'_, StateSpec> {
        StateLW::new(self, 0)
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&mut self) -> StateHW<'_, StateSpec> {
        StateHW::new(self, 16)
    }
}
#[doc = "SCT state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`write(|w| ..)` method takes [`state::W`](W) writer structure"]
impl crate::Writable for StateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}

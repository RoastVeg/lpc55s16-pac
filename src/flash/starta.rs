#[doc = "Register `STARTA` reader"]
pub type R = crate::R<StartaSpec>;
#[doc = "Register `STARTA` writer"]
pub type W = crate::W<StartaSpec>;
#[doc = "Field `STARTA` reader - Address / Start address for commands that take an address (range) as a parameter."]
pub type StartaR = crate::FieldReader<u32>;
#[doc = "Field `STARTA` writer - Address / Start address for commands that take an address (range) as a parameter."]
pub type StartaW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&self) -> StartaR {
        StartaR::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTA")
            .field("starta", &self.starta())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&mut self) -> StartaW<'_, StartaSpec> {
        StartaW::new(self, 0)
    }
}
#[doc = "start (or only) address for next flash command\n\nYou can [`read`](crate::Reg::read) this register and get [`starta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartaSpec;
impl crate::RegisterSpec for StartaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`starta::R`](R) reader structure"]
impl crate::Readable for StartaSpec {}
#[doc = "`write(|w| ..)` method takes [`starta::W`](W) writer structure"]
impl crate::Writable for StartaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STARTA to value 0"]
impl crate::Resettable for StartaSpec {}

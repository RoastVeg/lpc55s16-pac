#[doc = "Register `STOPA` reader"]
pub type R = crate::R<StopaSpec>;
#[doc = "Register `STOPA` writer"]
pub type W = crate::W<StopaSpec>;
#[doc = "Field `STOPA` reader - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
pub type StopaR = crate::FieldReader<u32>;
#[doc = "Field `STOPA` writer - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
pub type StopaW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub fn stopa(&self) -> StopaR {
        StopaR::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOPA")
            .field("stopa", &self.stopa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Stop address for commands that take an address range as a parameter (the word specified by STOPA is included in the address range)."]
    #[inline(always)]
    pub fn stopa(&mut self) -> StopaW<'_, StopaSpec> {
        StopaW::new(self, 0)
    }
}
#[doc = "end address for next flash command, if command operates on address ranges\n\nYou can [`read`](crate::Reg::read) this register and get [`stopa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopaSpec;
impl crate::RegisterSpec for StopaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stopa::R`](R) reader structure"]
impl crate::Readable for StopaSpec {}
#[doc = "`write(|w| ..)` method takes [`stopa::W`](W) writer structure"]
impl crate::Writable for StopaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STOPA to value 0"]
impl crate::Resettable for StopaSpec {}

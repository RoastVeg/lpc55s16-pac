#[doc = "Register `ETSCV` reader"]
pub type R = crate::R<EtscvSpec>;
#[doc = "Register `ETSCV` writer"]
pub type W = crate::W<EtscvSpec>;
#[doc = "Field `ETSC` reader - External timestamp counter."]
pub type EtscR = crate::FieldReader<u16>;
#[doc = "Field `ETSC` writer - External timestamp counter."]
pub type EtscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&self) -> EtscR {
        EtscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&mut self) -> EtscW<'_, EtscvSpec> {
        EtscW::new(self, 0)
    }
}
#[doc = "External Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`etscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtscvSpec;
impl crate::RegisterSpec for EtscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etscv::R`](R) reader structure"]
impl crate::Readable for EtscvSpec {}
#[doc = "`write(|w| ..)` method takes [`etscv::W`](W) writer structure"]
impl crate::Writable for EtscvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETSCV to value 0"]
impl crate::Resettable for EtscvSpec {}

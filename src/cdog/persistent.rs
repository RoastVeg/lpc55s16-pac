#[doc = "Register `PERSISTENT` reader"]
pub type R = crate::R<PersistentSpec>;
#[doc = "Register `PERSISTENT` writer"]
pub type W = crate::W<PersistentSpec>;
#[doc = "Field `PERSIS` reader - 32 regs free for user SW to enjoy"]
pub type PersisR = crate::FieldReader<u32>;
#[doc = "Field `PERSIS` writer - 32 regs free for user SW to enjoy"]
pub type PersisW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 regs free for user SW to enjoy"]
    #[inline(always)]
    pub fn persis(&self) -> PersisR {
        PersisR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERSISTENT")
            .field("persis", &self.persis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 regs free for user SW to enjoy"]
    #[inline(always)]
    pub fn persis(&mut self) -> PersisW<'_, PersistentSpec> {
        PersisW::new(self, 0)
    }
}
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage\n\nYou can [`read`](crate::Reg::read) this register and get [`persistent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`persistent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PersistentSpec;
impl crate::RegisterSpec for PersistentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`persistent::R`](R) reader structure"]
impl crate::Readable for PersistentSpec {}
#[doc = "`write(|w| ..)` method takes [`persistent::W`](W) writer structure"]
impl crate::Writable for PersistentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERSISTENT to value 0"]
impl crate::Resettable for PersistentSpec {}

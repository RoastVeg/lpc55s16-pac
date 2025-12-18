#[doc = "Register `DATAW[%s]` reader"]
pub type R = crate::R<DatawSpec>;
#[doc = "Register `DATAW[%s]` writer"]
pub type W = crate::W<DatawSpec>;
#[doc = "Field `DATAW` reader - no description available"]
pub type DatawR = crate::FieldReader<u32>;
#[doc = "Field `DATAW` writer - no description available"]
pub type DatawW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dataw(&self) -> DatawR {
        DatawR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAW")
            .field("dataw", &self.dataw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dataw(&mut self) -> DatawW<'_, DatawSpec> {
        DatawW::new(self, 0)
    }
}
#[doc = "data register, word 0-7; Memory data, or command parameter, or command result.\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatawSpec;
impl crate::RegisterSpec for DatawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataw::R`](R) reader structure"]
impl crate::Readable for DatawSpec {}
#[doc = "`write(|w| ..)` method takes [`dataw::W`](W) writer structure"]
impl crate::Writable for DatawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAW[%s] to value 0"]
impl crate::Resettable for DatawSpec {}

#[doc = "Register `HEADER` reader"]
pub type R = crate::R<HeaderSpec>;
#[doc = "Register `HEADER` writer"]
pub type W = crate::W<HeaderSpec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, HeaderSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "Valid Key Sore Header : 0x95959595\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeaderSpec;
impl crate::RegisterSpec for HeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`header::R`](R) reader structure"]
impl crate::Readable for HeaderSpec {}
#[doc = "`write(|w| ..)` method takes [`header::W`](W) writer structure"]
impl crate::Writable for HeaderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HEADER to value 0"]
impl crate::Resettable for HeaderSpec {}

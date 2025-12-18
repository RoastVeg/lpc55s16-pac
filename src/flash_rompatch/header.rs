#[doc = "Register `HEADER` reader"]
pub type R = crate::R<HeaderSpec>;
#[doc = "Register `HEADER` writer"]
pub type W = crate::W<HeaderSpec>;
#[doc = "Field `ENTRIES` reader - ."]
pub type EntriesR = crate::FieldReader;
#[doc = "Field `ENTRIES` writer - ."]
pub type EntriesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SUB_TYPE` reader - ."]
pub type SubTypeR = crate::FieldReader;
#[doc = "Field `SUB_TYPE` writer - ."]
pub type SubTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TYPE` reader - ."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - ."]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDENTIFIER` reader - ."]
pub type IdentifierR = crate::FieldReader;
#[doc = "Field `IDENTIFIER` writer - ."]
pub type IdentifierW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ."]
    #[inline(always)]
    pub fn entries(&self) -> EntriesR {
        EntriesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ."]
    #[inline(always)]
    pub fn sub_type(&self) -> SubTypeR {
        SubTypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ."]
    #[inline(always)]
    pub fn identifier(&self) -> IdentifierR {
        IdentifierR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEADER")
            .field("entries", &self.entries())
            .field("sub_type", &self.sub_type())
            .field("type_", &self.type_())
            .field("identifier", &self.identifier())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ."]
    #[inline(always)]
    pub fn entries(&mut self) -> EntriesW<'_, HeaderSpec> {
        EntriesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ."]
    #[inline(always)]
    pub fn sub_type(&mut self) -> SubTypeW<'_, HeaderSpec> {
        SubTypeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, HeaderSpec> {
        TypeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - ."]
    #[inline(always)]
    pub fn identifier(&mut self) -> IdentifierW<'_, HeaderSpec> {
        IdentifierW::new(self, 24)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

#[doc = "Register `REQUEST` reader"]
pub type R = crate::R<RequestSpec>;
#[doc = "Register `REQUEST` writer"]
pub type W = crate::W<RequestSpec>;
#[doc = "Field `REQ` reader - Request Value"]
pub type ReqR = crate::FieldReader<u32>;
#[doc = "Field `REQ` writer - Request Value"]
pub type ReqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Request Value"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQUEST").field("req", &self.req()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Request Value"]
    #[inline(always)]
    pub fn req(&mut self) -> ReqW<'_, RequestSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "CRC seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`request::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`request::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RequestSpec;
impl crate::RegisterSpec for RequestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`request::R`](R) reader structure"]
impl crate::Readable for RequestSpec {}
#[doc = "`write(|w| ..)` method takes [`request::W`](W) writer structure"]
impl crate::Writable for RequestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REQUEST to value 0xffff"]
impl crate::Resettable for RequestSpec {
    const RESET_VALUE: u32 = 0xffff;
}

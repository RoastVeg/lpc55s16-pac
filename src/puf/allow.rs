#[doc = "Register `ALLOW` reader"]
pub type R = crate::R<AllowSpec>;
#[doc = "Register `ALLOW` writer"]
pub type W = crate::W<AllowSpec>;
#[doc = "Field `ALLOWENROLL` reader - Enroll operation is allowed"]
pub type AllowenrollR = crate::BitReader;
#[doc = "Field `ALLOWSTART` reader - Start operation is allowed"]
pub type AllowstartR = crate::BitReader;
#[doc = "Field `ALLOWSETKEY` reader - Set Key operations are allowed"]
pub type AllowsetkeyR = crate::BitReader;
#[doc = "Field `ALLOWGETKEY` reader - Get Key operation is allowed"]
pub type AllowgetkeyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enroll operation is allowed"]
    #[inline(always)]
    pub fn allowenroll(&self) -> AllowenrollR {
        AllowenrollR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start operation is allowed"]
    #[inline(always)]
    pub fn allowstart(&self) -> AllowstartR {
        AllowstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set Key operations are allowed"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> AllowsetkeyR {
        AllowsetkeyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Get Key operation is allowed"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> AllowgetkeyR {
        AllowgetkeyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALLOW")
            .field("allowenroll", &self.allowenroll())
            .field("allowstart", &self.allowstart())
            .field("allowsetkey", &self.allowsetkey())
            .field("allowgetkey", &self.allowgetkey())
            .finish()
    }
}
impl W {}
#[doc = "PUF Allow register\n\nYou can [`read`](crate::Reg::read) this register and get [`allow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`allow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AllowSpec;
impl crate::RegisterSpec for AllowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`allow::R`](R) reader structure"]
impl crate::Readable for AllowSpec {}
#[doc = "`write(|w| ..)` method takes [`allow::W`](W) writer structure"]
impl crate::Writable for AllowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALLOW to value 0"]
impl crate::Resettable for AllowSpec {}

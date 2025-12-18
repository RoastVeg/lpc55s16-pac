#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `zeroize` reader - Begin Zeroize operation for PUF and go to Error state"]
pub type ZeroizeR = crate::BitReader;
#[doc = "Field `zeroize` writer - Begin Zeroize operation for PUF and go to Error state"]
pub type ZeroizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enroll` reader - Begin Enroll operation"]
pub type EnrollR = crate::BitReader;
#[doc = "Field `enroll` writer - Begin Enroll operation"]
pub type EnrollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start` reader - Begin Start operation"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Begin Start operation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERATEKEY` reader - Begin Set Intrinsic Key operation"]
pub type GeneratekeyR = crate::BitReader;
#[doc = "Field `GENERATEKEY` writer - Begin Set Intrinsic Key operation"]
pub type GeneratekeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETKEY` reader - Begin Set User Key operation"]
pub type SetkeyR = crate::BitReader;
#[doc = "Field `SETKEY` writer - Begin Set User Key operation"]
pub type SetkeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GETKEY` reader - Begin Get Key operation"]
pub type GetkeyR = crate::BitReader;
#[doc = "Field `GETKEY` writer - Begin Get Key operation"]
pub type GetkeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&self) -> ZeroizeR {
        ZeroizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&self) -> EnrollR {
        EnrollR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&self) -> GeneratekeyR {
        GeneratekeyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&self) -> SetkeyR {
        SetkeyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&self) -> GetkeyR {
        GetkeyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("zeroize", &self.zeroize())
            .field("enroll", &self.enroll())
            .field("start", &self.start())
            .field("generatekey", &self.generatekey())
            .field("setkey", &self.setkey())
            .field("getkey", &self.getkey())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&mut self) -> ZeroizeW<'_, CtrlSpec> {
        ZeroizeW::new(self, 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&mut self) -> EnrollW<'_, CtrlSpec> {
        EnrollW::new(self, 1)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtrlSpec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&mut self) -> GeneratekeyW<'_, CtrlSpec> {
        GeneratekeyW::new(self, 3)
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&mut self) -> SetkeyW<'_, CtrlSpec> {
        SetkeyW::new(self, 4)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&mut self) -> GetkeyW<'_, CtrlSpec> {
        GetkeyW::new(self, 6)
    }
}
#[doc = "PUF Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}

#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `BLOCKENROLL_SETKEY` reader - Block enroll operation. Write 1 to set, cleared on reset."]
pub type BlockenrollSetkeyR = crate::BitReader;
#[doc = "Field `BLOCKENROLL_SETKEY` writer - Block enroll operation. Write 1 to set, cleared on reset."]
pub type BlockenrollSetkeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKKEYOUTPUT` reader - Block set key operation. Write 1 to set, cleared on reset."]
pub type BlockkeyoutputR = crate::BitReader;
#[doc = "Field `BLOCKKEYOUTPUT` writer - Block set key operation. Write 1 to set, cleared on reset."]
pub type BlockkeyoutputW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BlockenrollSetkeyR {
        BlockenrollSetkeyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BlockkeyoutputR {
        BlockkeyoutputR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("blockenroll_setkey", &self.blockenroll_setkey())
            .field("blockkeyoutput", &self.blockkeyoutput())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&mut self) -> BlockenrollSetkeyW<'_, CfgSpec> {
        BlockenrollSetkeyW::new(self, 0)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&mut self) -> BlockkeyoutputW<'_, CfgSpec> {
        BlockkeyoutputW::new(self, 1)
    }
}
#[doc = "PUF config register for block bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}

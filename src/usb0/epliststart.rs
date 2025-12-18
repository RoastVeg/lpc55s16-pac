#[doc = "Register `EPLISTSTART` reader"]
pub type R = crate::R<EpliststartSpec>;
#[doc = "Register `EPLISTSTART` writer"]
pub type W = crate::W<EpliststartSpec>;
#[doc = "Field `EP_LIST` reader - Start address of the USB EP Command/Status List."]
pub type EpListR = crate::FieldReader<u32>;
#[doc = "Field `EP_LIST` writer - Start address of the USB EP Command/Status List."]
pub type EpListW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&self) -> EpListR {
        EpListR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&mut self) -> EpListW<'_, EpliststartSpec> {
        EpListW::new(self, 8)
    }
}
#[doc = "USB EP Command/Status List start address\n\nYou can [`read`](crate::Reg::read) this register and get [`epliststart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epliststart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpliststartSpec;
impl crate::RegisterSpec for EpliststartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epliststart::R`](R) reader structure"]
impl crate::Readable for EpliststartSpec {}
#[doc = "`write(|w| ..)` method takes [`epliststart::W`](W) writer structure"]
impl crate::Writable for EpliststartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPLISTSTART to value 0"]
impl crate::Resettable for EpliststartSpec {}

#[doc = "Register `HCPERIODICSTART` reader"]
pub type R = crate::R<HcperiodicstartSpec>;
#[doc = "Register `HCPERIODICSTART` writer"]
pub type W = crate::W<HcperiodicstartSpec>;
#[doc = "Field `PS` reader - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
pub type PsR = crate::FieldReader<u16>;
#[doc = "Field `PS` writer - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, HcperiodicstartSpec> {
        PsW::new(self, 0)
    }
}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodicstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcperiodicstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcperiodicstartSpec;
impl crate::RegisterSpec for HcperiodicstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcperiodicstart::R`](R) reader structure"]
impl crate::Readable for HcperiodicstartSpec {}
#[doc = "`write(|w| ..)` method takes [`hcperiodicstart::W`](W) writer structure"]
impl crate::Writable for HcperiodicstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPERIODICSTART to value 0"]
impl crate::Resettable for HcperiodicstartSpec {}

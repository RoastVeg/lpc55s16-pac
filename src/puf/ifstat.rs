#[doc = "Register `IFSTAT` reader"]
pub type R = crate::R<IfstatSpec>;
#[doc = "Register `IFSTAT` writer"]
pub type W = crate::W<IfstatSpec>;
#[doc = "Field `ERROR` reader - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IfstatSpec> {
        ErrorW::new(self, 0)
    }
}
#[doc = "PUF Interface Status and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfstatSpec;
impl crate::RegisterSpec for IfstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifstat::R`](R) reader structure"]
impl crate::Readable for IfstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ifstat::W`](W) writer structure"]
impl crate::Writable for IfstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFSTAT to value 0"]
impl crate::Resettable for IfstatSpec {}

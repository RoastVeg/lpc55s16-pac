#[doc = "Register `W0_31` reader"]
pub type R = crate::R<W0_31Spec>;
#[doc = "Register `W0_31` writer"]
pub type W = crate::W<W0_31Spec>;
#[doc = "Field `PWORD` reader - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PwordR = crate::FieldReader<u32>;
#[doc = "Field `PWORD` writer - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PwordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&self) -> PwordR {
        PwordR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0_31")
            .field("pword", &self.pword())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&mut self) -> PwordW<'_, W0_31Spec> {
        PwordW::new(self, 0)
    }
}
#[doc = "Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W0_31Spec;
impl crate::RegisterSpec for W0_31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w0_31::R`](R) reader structure"]
impl crate::Readable for W0_31Spec {}
#[doc = "`write(|w| ..)` method takes [`w0_31::W`](W) writer structure"]
impl crate::Writable for W0_31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W0_31 to value 0"]
impl crate::Resettable for W0_31Spec {}

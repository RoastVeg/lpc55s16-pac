#[doc = "Register `DIS_ROM_HIDING` reader"]
pub type R = crate::R<DisRomHidingSpec>;
#[doc = "Register `DIS_ROM_HIDING` writer"]
pub type W = crate::W<DisRomHidingSpec>;
#[doc = "Field `DIS_ROM_HIDING` reader - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
pub type DisRomHidingR = crate::FieldReader<u32>;
#[doc = "Field `DIS_ROM_HIDING` writer - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
pub type DisRomHidingW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub fn dis_rom_hiding(&self) -> DisRomHidingR {
        DisRomHidingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub fn dis_rom_hiding(&mut self) -> DisRomHidingW<'_, DisRomHidingSpec> {
        DisRomHidingW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dis_rom_hiding::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis_rom_hiding::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisRomHidingSpec;
impl crate::RegisterSpec for DisRomHidingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dis_rom_hiding::R`](R) reader structure"]
impl crate::Readable for DisRomHidingSpec {}
#[doc = "`write(|w| ..)` method takes [`dis_rom_hiding::W`](W) writer structure"]
impl crate::Writable for DisRomHidingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIS_ROM_HIDING to value 0"]
impl crate::Resettable for DisRomHidingSpec {}

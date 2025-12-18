#[doc = "Register `MRBA` reader"]
pub type R = crate::R<MrbaSpec>;
#[doc = "Register `MRBA` writer"]
pub type W = crate::W<MrbaSpec>;
#[doc = "Field `BA` reader - Base address for the message RAM in the chip memory map."]
pub type BaR = crate::FieldReader<u16>;
#[doc = "Field `BA` writer - Base address for the message RAM in the chip memory map."]
pub type BaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub fn ba(&self) -> BaR {
        BaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub fn ba(&mut self) -> BaW<'_, MrbaSpec> {
        BaW::new(self, 16)
    }
}
#[doc = "CAN Message RAM Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mrba::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrba::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrbaSpec;
impl crate::RegisterSpec for MrbaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrba::R`](R) reader structure"]
impl crate::Readable for MrbaSpec {}
#[doc = "`write(|w| ..)` method takes [`mrba::W`](W) writer structure"]
impl crate::Writable for MrbaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRBA to value 0"]
impl crate::Resettable for MrbaSpec {}

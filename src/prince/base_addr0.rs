#[doc = "Register `BASE_ADDR0` reader"]
pub type R = crate::R<BaseAddr0Spec>;
#[doc = "Register `BASE_ADDR0` writer"]
pub type W = crate::W<BaseAddr0Spec>;
#[doc = "Field `ADDR_FIXED` reader - Fixed portion of the base address of region 0."]
pub type AddrFixedR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_PRG` reader - Programmable portion of the base address of region 0."]
pub type AddrPrgR = crate::FieldReader;
#[doc = "Field `ADDR_PRG` writer - Programmable portion of the base address of region 0."]
pub type AddrPrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_fixed(&self) -> AddrFixedR {
        AddrFixedR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&self) -> AddrPrgR {
        AddrPrgR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASE_ADDR0")
            .field("addr_fixed", &self.addr_fixed())
            .field("addr_prg", &self.addr_prg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&mut self) -> AddrPrgW<'_, BaseAddr0Spec> {
        AddrPrgW::new(self, 18)
    }
}
#[doc = "Base Address for region 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`base_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddr0Spec;
impl crate::RegisterSpec for BaseAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_addr0::R`](R) reader structure"]
impl crate::Readable for BaseAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`base_addr0::W`](W) writer structure"]
impl crate::Writable for BaseAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE_ADDR0 to value 0"]
impl crate::Resettable for BaseAddr0Spec {}

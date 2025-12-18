#[doc = "Register `BASE_ADDR1` reader"]
pub type R = crate::R<BaseAddr1Spec>;
#[doc = "Register `BASE_ADDR1` writer"]
pub type W = crate::W<BaseAddr1Spec>;
#[doc = "Field `ADDR_FIXED` reader - Fixed portion of the base address of region 1."]
pub type AddrFixedR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_PRG` reader - Programmable portion of the base address of region 1."]
pub type AddrPrgR = crate::FieldReader;
#[doc = "Field `ADDR_PRG` writer - Programmable portion of the base address of region 1."]
pub type AddrPrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr_fixed(&self) -> AddrFixedR {
        AddrFixedR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr_prg(&self) -> AddrPrgR {
        AddrPrgR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASE_ADDR1")
            .field("addr_fixed", &self.addr_fixed())
            .field("addr_prg", &self.addr_prg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr_prg(&mut self) -> AddrPrgW<'_, BaseAddr1Spec> {
        AddrPrgW::new(self, 18)
    }
}
#[doc = "Base Address for region 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`base_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddr1Spec;
impl crate::RegisterSpec for BaseAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_addr1::R`](R) reader structure"]
impl crate::Readable for BaseAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`base_addr1::W`](W) writer structure"]
impl crate::Writable for BaseAddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE_ADDR1 to value 0x0004_0000"]
impl crate::Resettable for BaseAddr1Spec {
    const RESET_VALUE: u32 = 0x0004_0000;
}

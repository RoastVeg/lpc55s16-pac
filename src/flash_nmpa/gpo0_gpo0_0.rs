#[doc = "Register `GPO0_0` reader"]
pub type R = crate::R<Gpo0Gpo0_0Spec>;
#[doc = "Register `GPO0_0` writer"]
pub type W = crate::W<Gpo0Gpo0_0Spec>;
#[doc = "Field `FRO_TRIM_VALID` reader - no description available"]
pub type FroTrimValidR = crate::BitReader;
#[doc = "Field `FRO_TRIM_VALID` writer - no description available"]
pub type FroTrimValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRO32K_NTAT` reader - no description available"]
pub type Fro32kNtatR = crate::FieldReader;
#[doc = "Field `FRO32K_NTAT` writer - no description available"]
pub type Fro32kNtatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FRO32K_PTAT` reader - no description available"]
pub type Fro32kPtatR = crate::FieldReader;
#[doc = "Field `FRO32K_PTAT` writer - no description available"]
pub type Fro32kPtatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FRO32K_CAPCAL` reader - no description available"]
pub type Fro32kCapcalR = crate::FieldReader<u16>;
#[doc = "Field `FRO32K_CAPCAL` writer - no description available"]
pub type Fro32kCapcalW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u16>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro_trim_valid(&self) -> FroTrimValidR {
        FroTrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - no description available"]
    #[inline(always)]
    pub fn fro32k_ntat(&self) -> Fro32kNtatR {
        Fro32kNtatR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    pub fn fro32k_ptat(&self) -> Fro32kPtatR {
        Fro32kPtatR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:15 - no description available"]
    #[inline(always)]
    pub fn fro32k_capcal(&self) -> Fro32kCapcalR {
        Fro32kCapcalR::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_GPO0_0")
            .field("fro_trim_valid", &self.fro_trim_valid())
            .field("fro32k_ntat", &self.fro32k_ntat())
            .field("fro32k_ptat", &self.fro32k_ptat())
            .field("fro32k_capcal", &self.fro32k_capcal())
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro_trim_valid(&mut self) -> FroTrimValidW<'_, Gpo0Gpo0_0Spec> {
        FroTrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:3 - no description available"]
    #[inline(always)]
    pub fn fro32k_ntat(&mut self) -> Fro32kNtatW<'_, Gpo0Gpo0_0Spec> {
        Fro32kNtatW::new(self, 1)
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    pub fn fro32k_ptat(&mut self) -> Fro32kPtatW<'_, Gpo0Gpo0_0Spec> {
        Fro32kPtatW::new(self, 4)
    }
    #[doc = "Bits 7:15 - no description available"]
    #[inline(always)]
    pub fn fro32k_capcal(&mut self) -> Fro32kCapcalW<'_, Gpo0Gpo0_0Spec> {
        Fro32kCapcalW::new(self, 7)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo0Gpo0_0Spec> {
        FieldW::new(self, 16)
    }
}
#[doc = "GPO0 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo0Gpo0_0Spec;
impl crate::RegisterSpec for Gpo0Gpo0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo0_gpo0_0::R`](R) reader structure"]
impl crate::Readable for Gpo0Gpo0_0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo0_gpo0_0::W`](W) writer structure"]
impl crate::Writable for Gpo0Gpo0_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO0_0 to value 0"]
impl crate::Resettable for Gpo0Gpo0_0Spec {}

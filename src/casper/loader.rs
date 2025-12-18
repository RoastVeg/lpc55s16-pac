#[doc = "Register `LOADER` reader"]
pub type R = crate::R<LoaderSpec>;
#[doc = "Register `LOADER` writer"]
pub type W = crate::W<LoaderSpec>;
#[doc = "Field `COUNT` reader - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlbpair {
    #[doc = "0: Bank-pair 0 (1st)"]
    Pair0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    Pair1 = 1,
}
impl From<Ctrlbpair> for bool {
    #[inline(always)]
    fn from(variant: Ctrlbpair) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLBPAIR` reader - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub type CtrlbpairR = crate::BitReader<Ctrlbpair>;
impl CtrlbpairR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlbpair {
        match self.bits {
            false => Ctrlbpair::Pair0,
            true => Ctrlbpair::Pair1,
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == Ctrlbpair::Pair0
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == Ctrlbpair::Pair1
    }
}
#[doc = "Field `CTRLBPAIR` writer - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub type CtrlbpairW<'a, REG> = crate::BitWriter<'a, REG, Ctrlbpair>;
impl<'a, REG> CtrlbpairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlbpair::Pair0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlbpair::Pair1)
    }
}
#[doc = "Field `CTRLOFF` reader - DWord Offset of CTRL pair to load next."]
pub type CtrloffR = crate::FieldReader<u16>;
#[doc = "Field `CTRLOFF` writer - DWord Offset of CTRL pair to load next."]
pub type CtrloffW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&self) -> CtrlbpairR {
        CtrlbpairR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&self) -> CtrloffR {
        CtrloffR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, LoaderSpec> {
        CountW::new(self, 0)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&mut self) -> CtrlbpairW<'_, LoaderSpec> {
        CtrlbpairW::new(self, 16)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&mut self) -> CtrloffW<'_, LoaderSpec> {
        CtrloffW::new(self, 18)
    }
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`loader::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loader::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoaderSpec;
impl crate::RegisterSpec for LoaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loader::R`](R) reader structure"]
impl crate::Readable for LoaderSpec {}
#[doc = "`write(|w| ..)` method takes [`loader::W`](W) writer structure"]
impl crate::Writable for LoaderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOADER to value 0"]
impl crate::Resettable for LoaderSpec {}

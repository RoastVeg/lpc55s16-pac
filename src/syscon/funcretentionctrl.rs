#[doc = "Register `FUNCRETENTIONCTRL` reader"]
pub type R = crate::R<FuncretentionctrlSpec>;
#[doc = "Register `FUNCRETENTIONCTRL` writer"]
pub type W = crate::W<FuncretentionctrlSpec>;
#[doc = "functional retention in power down only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Funcretena {
    #[doc = "0: disable functional retention."]
    Disable = 0,
    #[doc = "1: enable functional retention."]
    Enable = 1,
}
impl From<Funcretena> for bool {
    #[inline(always)]
    fn from(variant: Funcretena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUNCRETENA` reader - functional retention in power down only."]
pub type FuncretenaR = crate::BitReader<Funcretena>;
impl FuncretenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Funcretena {
        match self.bits {
            false => Funcretena::Disable,
            true => Funcretena::Enable,
        }
    }
    #[doc = "disable functional retention."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Funcretena::Disable
    }
    #[doc = "enable functional retention."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Funcretena::Enable
    }
}
#[doc = "Field `FUNCRETENA` writer - functional retention in power down only."]
pub type FuncretenaW<'a, REG> = crate::BitWriter<'a, REG, Funcretena>;
impl<'a, REG> FuncretenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable functional retention."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Funcretena::Disable)
    }
    #[doc = "enable functional retention."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Funcretena::Enable)
    }
}
#[doc = "Field `RET_START` reader - Start address divided by 4 inside SRAMX bank."]
pub type RetStartR = crate::FieldReader<u16>;
#[doc = "Field `RET_START` writer - Start address divided by 4 inside SRAMX bank."]
pub type RetStartW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `RET_LENTH` reader - lenth of Scan chains to save."]
pub type RetLenthR = crate::FieldReader<u16>;
#[doc = "Field `RET_LENTH` writer - lenth of Scan chains to save."]
pub type RetLenthW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - functional retention in power down only."]
    #[inline(always)]
    pub fn funcretena(&self) -> FuncretenaR {
        FuncretenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:13 - Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub fn ret_start(&self) -> RetStartR {
        RetStartR::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bits 14:23 - lenth of Scan chains to save."]
    #[inline(always)]
    pub fn ret_lenth(&self) -> RetLenthR {
        RetLenthR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - functional retention in power down only."]
    #[inline(always)]
    pub fn funcretena(&mut self) -> FuncretenaW<'_, FuncretentionctrlSpec> {
        FuncretenaW::new(self, 0)
    }
    #[doc = "Bits 1:13 - Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub fn ret_start(&mut self) -> RetStartW<'_, FuncretentionctrlSpec> {
        RetStartW::new(self, 1)
    }
    #[doc = "Bits 14:23 - lenth of Scan chains to save."]
    #[inline(always)]
    pub fn ret_lenth(&mut self) -> RetLenthW<'_, FuncretentionctrlSpec> {
        RetLenthW::new(self, 14)
    }
}
#[doc = "Functional retention control register\n\nYou can [`read`](crate::Reg::read) this register and get [`funcretentionctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`funcretentionctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncretentionctrlSpec;
impl crate::RegisterSpec for FuncretentionctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`funcretentionctrl::R`](R) reader structure"]
impl crate::Readable for FuncretentionctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`funcretentionctrl::W`](W) writer structure"]
impl crate::Writable for FuncretentionctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNCRETENTIONCTRL to value 0x0050_c000"]
impl crate::Resettable for FuncretentionctrlSpec {
    const RESET_VALUE: u32 = 0x0050_c000;
}

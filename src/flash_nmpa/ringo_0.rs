#[doc = "Register `RINGO_0` reader"]
pub type R = crate::R<Ringo0Spec>;
#[doc = "Register `RINGO_0` writer"]
pub type W = crate::W<Ringo0Spec>;
#[doc = "Field `RINGO_0_CTRL_VALID` reader - 1: RINGO_0_CTRL is valid."]
pub type Ringo0CtrlValidR = crate::BitReader;
#[doc = "Field `RINGO_0_CTRL_VALID` writer - 1: RINGO_0_CTRL is valid."]
pub type Ringo0CtrlValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINGO_0_CTRL` reader - To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
pub type Ringo0CtrlR = crate::FieldReader<u32>;
#[doc = "Field `RINGO_0_CTRL` writer - To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
pub type Ringo0CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 1: RINGO_0_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_0_ctrl_valid(&self) -> Ringo0CtrlValidR {
        Ringo0CtrlValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_0_ctrl(&self) -> Ringo0CtrlR {
        Ringo0CtrlR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 1: RINGO_0_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_0_ctrl_valid(&mut self) -> Ringo0CtrlValidW<'_, Ringo0Spec> {
        Ringo0CtrlValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_0_ctrl(&mut self) -> Ringo0CtrlW<'_, Ringo0Spec> {
        Ringo0CtrlW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringo0Spec;
impl crate::RegisterSpec for Ringo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringo_0::R`](R) reader structure"]
impl crate::Readable for Ringo0Spec {}
#[doc = "`write(|w| ..)` method takes [`ringo_0::W`](W) writer structure"]
impl crate::Writable for Ringo0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RINGO_0 to value 0"]
impl crate::Resettable for Ringo0Spec {}

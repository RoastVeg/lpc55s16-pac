#[doc = "Register `RINGO_1` reader"]
pub type R = crate::R<Ringo1Spec>;
#[doc = "Register `RINGO_1` writer"]
pub type W = crate::W<Ringo1Spec>;
#[doc = "Field `RINGO_1_CTRL_VALID` reader - 1: RINGO_1_CTRL is valid."]
pub type Ringo1CtrlValidR = crate::BitReader;
#[doc = "Field `RINGO_1_CTRL_VALID` writer - 1: RINGO_1_CTRL is valid."]
pub type Ringo1CtrlValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINGO_1_CTRL` reader - To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
pub type Ringo1CtrlR = crate::FieldReader<u32>;
#[doc = "Field `RINGO_1_CTRL` writer - To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
pub type Ringo1CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 1: RINGO_1_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_1_ctrl_valid(&self) -> Ringo1CtrlValidR {
        Ringo1CtrlValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_1_ctrl(&self) -> Ringo1CtrlR {
        Ringo1CtrlR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO_1")
            .field("ringo_1_ctrl_valid", &self.ringo_1_ctrl_valid())
            .field("ringo_1_ctrl", &self.ringo_1_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: RINGO_1_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_1_ctrl_valid(&mut self) -> Ringo1CtrlValidW<'_, Ringo1Spec> {
        Ringo1CtrlValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_1_ctrl(&mut self) -> Ringo1CtrlW<'_, Ringo1Spec> {
        Ringo1CtrlW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringo1Spec;
impl crate::RegisterSpec for Ringo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringo_1::R`](R) reader structure"]
impl crate::Readable for Ringo1Spec {}
#[doc = "`write(|w| ..)` method takes [`ringo_1::W`](W) writer structure"]
impl crate::Writable for Ringo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RINGO_1 to value 0"]
impl crate::Resettable for Ringo1Spec {}

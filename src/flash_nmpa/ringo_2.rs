#[doc = "Register `RINGO_2` reader"]
pub type R = crate::R<Ringo2Spec>;
#[doc = "Register `RINGO_2` writer"]
pub type W = crate::W<Ringo2Spec>;
#[doc = "Field `RINGO_2_CTRL_VALID` reader - 1: RINGO_2_CTRL is valid."]
pub type Ringo2CtrlValidR = crate::BitReader;
#[doc = "Field `RINGO_2_CTRL_VALID` writer - 1: RINGO_2_CTRL is valid."]
pub type Ringo2CtrlValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINGO_2_CTRL` reader - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
pub type Ringo2CtrlR = crate::FieldReader<u32>;
#[doc = "Field `RINGO_2_CTRL` writer - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
pub type Ringo2CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_2_ctrl_valid(&self) -> Ringo2CtrlValidR {
        Ringo2CtrlValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_2_ctrl(&self) -> Ringo2CtrlR {
        Ringo2CtrlR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO_2")
            .field("ringo_2_ctrl_valid", &self.ringo_2_ctrl_valid())
            .field("ringo_2_ctrl", &self.ringo_2_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_2_ctrl_valid(&mut self) -> Ringo2CtrlValidW<'_, Ringo2Spec> {
        Ringo2CtrlValidW::new(self, 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_2_ctrl(&mut self) -> Ringo2CtrlW<'_, Ringo2Spec> {
        Ringo2CtrlW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringo2Spec;
impl crate::RegisterSpec for Ringo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringo_2::R`](R) reader structure"]
impl crate::Readable for Ringo2Spec {}
#[doc = "`write(|w| ..)` method takes [`ringo_2::W`](W) writer structure"]
impl crate::Writable for Ringo2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RINGO_2 to value 0"]
impl crate::Resettable for Ringo2Spec {}

#[doc = "Register `INTSETSTAT` reader"]
pub type R = crate::R<IntsetstatSpec>;
#[doc = "Register `INTSETSTAT` writer"]
pub type W = crate::W<IntsetstatSpec>;
#[doc = "Field `EP_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type EpSetIntR = crate::FieldReader<u16>;
#[doc = "Field `EP_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type EpSetIntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FRAME_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type FrameSetIntR = crate::BitReader;
#[doc = "Field `FRAME_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type FrameSetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type DevSetIntR = crate::BitReader;
#[doc = "Field `DEV_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
pub type DevSetIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EpSetIntR {
        EpSetIntR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FrameSetIntR {
        FrameSetIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DevSetIntR {
        DevSetIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&mut self) -> EpSetIntW<'_, IntsetstatSpec> {
        EpSetIntW::new(self, 0)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&mut self) -> FrameSetIntW<'_, IntsetstatSpec> {
        FrameSetIntW::new(self, 30)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&mut self) -> DevSetIntW<'_, IntsetstatSpec> {
        DevSetIntW::new(self, 31)
    }
}
#[doc = "USB set interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsetstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsetstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetstatSpec;
impl crate::RegisterSpec for IntsetstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsetstat::R`](R) reader structure"]
impl crate::Readable for IntsetstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intsetstat::W`](W) writer structure"]
impl crate::Writable for IntsetstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSETSTAT to value 0"]
impl crate::Resettable for IntsetstatSpec {}

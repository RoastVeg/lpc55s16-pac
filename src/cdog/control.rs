#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `LOCK_CTRL` reader - Lock control field"]
pub type LockCtrlR = crate::FieldReader;
#[doc = "Field `LOCK_CTRL` writer - Lock control field"]
pub type LockCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMEOUT_CTRL` reader - TIMEOUT control"]
pub type TimeoutCtrlR = crate::FieldReader;
#[doc = "Field `TIMEOUT_CTRL` writer - TIMEOUT control"]
pub type TimeoutCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MISCOMPARE_CTRL` reader - MISCOMPARE control field"]
pub type MiscompareCtrlR = crate::FieldReader;
#[doc = "Field `MISCOMPARE_CTRL` writer - MISCOMPARE control field"]
pub type MiscompareCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEQUENCE_CTRL` reader - SEQUENCE control field"]
pub type SequenceCtrlR = crate::FieldReader;
#[doc = "Field `SEQUENCE_CTRL` writer - SEQUENCE control field"]
pub type SequenceCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONTROL_CTRL` reader - CONTROL control field"]
pub type ControlCtrlR = crate::FieldReader;
#[doc = "Field `CONTROL_CTRL` writer - CONTROL control field"]
pub type ControlCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STATE_CTRL` reader - STATE control field"]
pub type StateCtrlR = crate::FieldReader;
#[doc = "Field `STATE_CTRL` writer - STATE control field"]
pub type StateCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDRESS_CTRL` reader - ADDRESS control field"]
pub type AddressCtrlR = crate::FieldReader;
#[doc = "Field `ADDRESS_CTRL` writer - ADDRESS control field"]
pub type AddressCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRQ_PAUSE` reader - IRQ pause control field"]
pub type IrqPauseR = crate::FieldReader;
#[doc = "Field `IRQ_PAUSE` writer - IRQ pause control field"]
pub type IrqPauseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEBUG_HALT_CTRL` reader - DEBUG_HALT control field"]
pub type DebugHaltCtrlR = crate::FieldReader;
#[doc = "Field `DEBUG_HALT_CTRL` writer - DEBUG_HALT control field"]
pub type DebugHaltCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Lock control field"]
    #[inline(always)]
    pub fn lock_ctrl(&self) -> LockCtrlR {
        LockCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - TIMEOUT control"]
    #[inline(always)]
    pub fn timeout_ctrl(&self) -> TimeoutCtrlR {
        TimeoutCtrlR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - MISCOMPARE control field"]
    #[inline(always)]
    pub fn miscompare_ctrl(&self) -> MiscompareCtrlR {
        MiscompareCtrlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SEQUENCE control field"]
    #[inline(always)]
    pub fn sequence_ctrl(&self) -> SequenceCtrlR {
        SequenceCtrlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - CONTROL control field"]
    #[inline(always)]
    pub fn control_ctrl(&self) -> ControlCtrlR {
        ControlCtrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - STATE control field"]
    #[inline(always)]
    pub fn state_ctrl(&self) -> StateCtrlR {
        StateCtrlR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - ADDRESS control field"]
    #[inline(always)]
    pub fn address_ctrl(&self) -> AddressCtrlR {
        AddressCtrlR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 28:29 - IRQ pause control field"]
    #[inline(always)]
    pub fn irq_pause(&self) -> IrqPauseR {
        IrqPauseR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - DEBUG_HALT control field"]
    #[inline(always)]
    pub fn debug_halt_ctrl(&self) -> DebugHaltCtrlR {
        DebugHaltCtrlR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("lock_ctrl", &self.lock_ctrl())
            .field("timeout_ctrl", &self.timeout_ctrl())
            .field("miscompare_ctrl", &self.miscompare_ctrl())
            .field("sequence_ctrl", &self.sequence_ctrl())
            .field("control_ctrl", &self.control_ctrl())
            .field("state_ctrl", &self.state_ctrl())
            .field("address_ctrl", &self.address_ctrl())
            .field("irq_pause", &self.irq_pause())
            .field("debug_halt_ctrl", &self.debug_halt_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Lock control field"]
    #[inline(always)]
    pub fn lock_ctrl(&mut self) -> LockCtrlW<'_, ControlSpec> {
        LockCtrlW::new(self, 0)
    }
    #[doc = "Bits 2:4 - TIMEOUT control"]
    #[inline(always)]
    pub fn timeout_ctrl(&mut self) -> TimeoutCtrlW<'_, ControlSpec> {
        TimeoutCtrlW::new(self, 2)
    }
    #[doc = "Bits 5:7 - MISCOMPARE control field"]
    #[inline(always)]
    pub fn miscompare_ctrl(&mut self) -> MiscompareCtrlW<'_, ControlSpec> {
        MiscompareCtrlW::new(self, 5)
    }
    #[doc = "Bits 8:10 - SEQUENCE control field"]
    #[inline(always)]
    pub fn sequence_ctrl(&mut self) -> SequenceCtrlW<'_, ControlSpec> {
        SequenceCtrlW::new(self, 8)
    }
    #[doc = "Bits 11:13 - CONTROL control field"]
    #[inline(always)]
    pub fn control_ctrl(&mut self) -> ControlCtrlW<'_, ControlSpec> {
        ControlCtrlW::new(self, 11)
    }
    #[doc = "Bits 14:16 - STATE control field"]
    #[inline(always)]
    pub fn state_ctrl(&mut self) -> StateCtrlW<'_, ControlSpec> {
        StateCtrlW::new(self, 14)
    }
    #[doc = "Bits 17:19 - ADDRESS control field"]
    #[inline(always)]
    pub fn address_ctrl(&mut self) -> AddressCtrlW<'_, ControlSpec> {
        AddressCtrlW::new(self, 17)
    }
    #[doc = "Bits 28:29 - IRQ pause control field"]
    #[inline(always)]
    pub fn irq_pause(&mut self) -> IrqPauseW<'_, ControlSpec> {
        IrqPauseW::new(self, 28)
    }
    #[doc = "Bits 30:31 - DEBUG_HALT control field"]
    #[inline(always)]
    pub fn debug_halt_ctrl(&mut self) -> DebugHaltCtrlW<'_, ControlSpec> {
        DebugHaltCtrlW::new(self, 30)
    }
}
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0x5009_2492"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x5009_2492;
}

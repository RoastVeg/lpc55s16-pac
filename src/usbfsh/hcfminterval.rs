#[doc = "Register `HCFMINTERVAL` reader"]
pub type R = crate::R<HcfmintervalSpec>;
#[doc = "Register `HCFMINTERVAL` writer"]
pub type W = crate::W<HcfmintervalSpec>;
#[doc = "Field `FI` reader - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub type FiR = crate::FieldReader<u16>;
#[doc = "Field `FI` writer - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub type FiW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `FSMPS` reader - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub type FsmpsR = crate::FieldReader<u16>;
#[doc = "Field `FSMPS` writer - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub type FsmpsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FIT` reader - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FitR = crate::BitReader;
#[doc = "Field `FIT` writer - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&self) -> FiR {
        FiR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&self) -> FsmpsR {
        FsmpsR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&self) -> FitR {
        FitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&mut self) -> FiW<'_, HcfmintervalSpec> {
        FiW::new(self, 0)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&mut self) -> FsmpsW<'_, HcfmintervalSpec> {
        FsmpsW::new(self, 16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&mut self) -> FitW<'_, HcfmintervalSpec> {
        FitW::new(self, 31)
    }
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfminterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfminterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfmintervalSpec;
impl crate::RegisterSpec for HcfmintervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfminterval::R`](R) reader structure"]
impl crate::Readable for HcfmintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfminterval::W`](W) writer structure"]
impl crate::Writable for HcfmintervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFMINTERVAL to value 0x2edf"]
impl crate::Resettable for HcfmintervalSpec {
    const RESET_VALUE: u32 = 0x2edf;
}

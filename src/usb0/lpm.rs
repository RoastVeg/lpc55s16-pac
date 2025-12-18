#[doc = "Register `LPM` reader"]
pub type R = crate::R<LpmSpec>;
#[doc = "Register `LPM` writer"]
pub type W = crate::W<LpmSpec>;
#[doc = "Field `HIRD_HW` reader - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
pub type HirdHwR = crate::FieldReader;
#[doc = "Field `HIRD_SW` reader - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
pub type HirdSwR = crate::FieldReader;
#[doc = "Field `HIRD_SW` writer - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
pub type HirdSwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATA_PENDING` reader - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
pub type DataPendingR = crate::BitReader;
#[doc = "Field `DATA_PENDING` writer - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
pub type DataPendingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW. This is the HIRD value from the last received LPM token"]
    #[inline(always)]
    pub fn hird_hw(&self) -> HirdHwR {
        HirdHwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HirdSwR {
        HirdSwR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    pub fn data_pending(&self) -> DataPendingR {
        DataPendingR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW. This is the time duration required by the USB device system to come out of LPM initiated suspend after receiving the host initiated LPM resume."]
    #[inline(always)]
    pub fn hird_sw(&mut self) -> HirdSwW<'_, LpmSpec> {
        HirdSwW::new(self, 4)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives. If LPM supported bit is set to one and this bit is zero, HW will return an ACK handshake on every LPM token it receives. If SW has still data pending and LPM is supported, it must set this bit to 1."]
    #[inline(always)]
    pub fn data_pending(&mut self) -> DataPendingW<'_, LpmSpec> {
        DataPendingW::new(self, 8)
    }
}
#[doc = "USB Link Power Management register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmSpec;
impl crate::RegisterSpec for LpmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm::R`](R) reader structure"]
impl crate::Readable for LpmSpec {}
#[doc = "`write(|w| ..)` method takes [`lpm::W`](W) writer structure"]
impl crate::Writable for LpmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPM to value 0"]
impl crate::Resettable for LpmSpec {}

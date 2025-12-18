#[doc = "Register `HCCOMMANDSTATUS` reader"]
pub type R = crate::R<HccommandstatusSpec>;
#[doc = "Register `HCCOMMANDSTATUS` writer"]
pub type W = crate::W<HccommandstatusSpec>;
#[doc = "Field `HCR` reader - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub type HcrR = crate::BitReader;
#[doc = "Field `HCR` writer - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub type HcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLF` reader - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub type ClfR = crate::BitReader;
#[doc = "Field `CLF` writer - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub type ClfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLF` reader - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub type BlfR = crate::BitReader;
#[doc = "Field `BLF` writer - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub type BlfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR` reader - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub type OcrR = crate::BitReader;
#[doc = "Field `OCR` writer - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub type OcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC` reader - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub type SocR = crate::FieldReader;
#[doc = "Field `SOC` writer - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub type SocW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&self) -> HcrR {
        HcrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&self) -> ClfR {
        ClfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&self) -> BlfR {
        BlfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&self) -> OcrR {
        OcrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&self) -> SocR {
        SocR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&mut self) -> HcrW<'_, HccommandstatusSpec> {
        HcrW::new(self, 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&mut self) -> ClfW<'_, HccommandstatusSpec> {
        ClfW::new(self, 1)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&mut self) -> BlfW<'_, HccommandstatusSpec> {
        BlfW::new(self, 2)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&mut self) -> OcrW<'_, HccommandstatusSpec> {
        OcrW::new(self, 3)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&mut self) -> SocW<'_, HccommandstatusSpec> {
        SocW::new(self, 6)
    }
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)\n\nYou can [`read`](crate::Reg::read) this register and get [`hccommandstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccommandstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccommandstatusSpec;
impl crate::RegisterSpec for HccommandstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccommandstatus::R`](R) reader structure"]
impl crate::Readable for HccommandstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hccommandstatus::W`](W) writer structure"]
impl crate::Writable for HccommandstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCCOMMANDSTATUS to value 0"]
impl crate::Resettable for HccommandstatusSpec {}

#[doc = "Register `PVT_MONITOR_0_ARRAY2` reader"]
pub type R = crate::R<PvtMonitor0PvtMonitor0Array2Spec>;
#[doc = "Register `PVT_MONITOR_0_ARRAY2` writer"]
pub type W = crate::W<PvtMonitor0PvtMonitor0Array2Spec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2")
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, PvtMonitor0PvtMonitor0Array2Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_array2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_array2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvtMonitor0PvtMonitor0Array2Spec;
impl crate::RegisterSpec for PvtMonitor0PvtMonitor0Array2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_monitor_0_pvt_monitor_0_array2::R`](R) reader structure"]
impl crate::Readable for PvtMonitor0PvtMonitor0Array2Spec {}
#[doc = "`write(|w| ..)` method takes [`pvt_monitor_0_pvt_monitor_0_array2::W`](W) writer structure"]
impl crate::Writable for PvtMonitor0PvtMonitor0Array2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT_MONITOR_0_ARRAY2 to value 0"]
impl crate::Resettable for PvtMonitor0PvtMonitor0Array2Spec {}

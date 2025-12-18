#[doc = "Register `VENDOR_USAGE` reader"]
pub type R = crate::R<VendorUsageSpec>;
#[doc = "Register `VENDOR_USAGE` writer"]
pub type W = crate::W<VendorUsageSpec>;
#[doc = "Field `VENDOR_USAGE` reader - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub type VendorUsageR = crate::FieldReader<u16>;
#[doc = "Field `VENDOR_USAGE` writer - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub type VendorUsageW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&self) -> VendorUsageR {
        VendorUsageR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&mut self) -> VendorUsageW<'_, VendorUsageSpec> {
        VendorUsageW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`vendor_usage::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendor_usage::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendorUsageSpec;
impl crate::RegisterSpec for VendorUsageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendor_usage::R`](R) reader structure"]
impl crate::Readable for VendorUsageSpec {}
#[doc = "`write(|w| ..)` method takes [`vendor_usage::W`](W) writer structure"]
impl crate::Writable for VendorUsageSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VENDOR_USAGE to value 0"]
impl crate::Resettable for VendorUsageSpec {}

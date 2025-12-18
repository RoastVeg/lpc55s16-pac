#[doc = "Register `VENDOR_USAGE` reader"]
pub type R = crate::R<VendorUsageSpec>;
#[doc = "Register `VENDOR_USAGE` writer"]
pub type W = crate::W<VendorUsageSpec>;
#[doc = "Field `DBG_VENDOR_USAGE` reader - DBG_VENDOR_USAGE."]
pub type DbgVendorUsageR = crate::FieldReader<u16>;
#[doc = "Field `DBG_VENDOR_USAGE` writer - DBG_VENDOR_USAGE."]
pub type DbgVendorUsageW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type InverseValueR = crate::FieldReader<u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type InverseValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&self) -> DbgVendorUsageR {
        DbgVendorUsageR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> InverseValueR {
        InverseValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENDOR_USAGE")
            .field("dbg_vendor_usage", &self.dbg_vendor_usage())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&mut self) -> DbgVendorUsageW<'_, VendorUsageSpec> {
        DbgVendorUsageW::new(self, 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> InverseValueW<'_, VendorUsageSpec> {
        InverseValueW::new(self, 16)
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

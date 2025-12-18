#[doc = "Register `DATABUFSTART` reader"]
pub type R = crate::R<DatabufstartSpec>;
#[doc = "Register `DATABUFSTART` writer"]
pub type W = crate::W<DatabufstartSpec>;
#[doc = "Field `DA_BUF` reader - Start address of the buffer pointer page where all endpoint data buffers are located."]
pub type DaBufR = crate::FieldReader<u16>;
#[doc = "Field `DA_BUF` writer - Start address of the buffer pointer page where all endpoint data buffers are located."]
pub type DaBufW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31 - Start address of the buffer pointer page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&self) -> DaBufR {
        DaBufR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFSTART")
            .field("da_buf", &self.da_buf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:31 - Start address of the buffer pointer page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&mut self) -> DaBufW<'_, DatabufstartSpec> {
        DaBufW::new(self, 22)
    }
}
#[doc = "USB Data buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`databufstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databufstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufstartSpec;
impl crate::RegisterSpec for DatabufstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databufstart::R`](R) reader structure"]
impl crate::Readable for DatabufstartSpec {}
#[doc = "`write(|w| ..)` method takes [`databufstart::W`](W) writer structure"]
impl crate::Writable for DatabufstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUFSTART to value 0"]
impl crate::Resettable for DatabufstartSpec {}

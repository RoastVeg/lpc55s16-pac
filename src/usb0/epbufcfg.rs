#[doc = "Register `EPBUFCFG` reader"]
pub type R = crate::R<EpbufcfgSpec>;
#[doc = "Register `EPBUFCFG` writer"]
pub type W = crate::W<EpbufcfgSpec>;
#[doc = "Field `BUF_SB` reader - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
pub type BufSbR = crate::FieldReader;
#[doc = "Field `BUF_SB` writer - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
pub type BufSbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&self) -> BufSbR {
        BufSbR::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPBUFCFG")
            .field("buf_sb", &self.buf_sb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&mut self) -> BufSbW<'_, EpbufcfgSpec> {
        BufSbW::new(self, 2)
    }
}
#[doc = "USB Endpoint Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`epbufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epbufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpbufcfgSpec;
impl crate::RegisterSpec for EpbufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epbufcfg::R`](R) reader structure"]
impl crate::Readable for EpbufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`epbufcfg::W`](W) writer structure"]
impl crate::Writable for EpbufcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPBUFCFG to value 0"]
impl crate::Resettable for EpbufcfgSpec {}

#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `LBCK` reader - Loop back mode."]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop back mode."]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Control of transmit pin."]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - Control of transmit pin."]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - Monitors the actual value of the CAN_RXD."]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Monitors the actual value of the CAN_RXD."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Loop back mode."]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of transmit pin."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Loop back mode."]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<'_, TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of transmit pin."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, TestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - Monitors the actual value of the CAN_RXD."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, TestSpec> {
        RxW::new(self, 7)
    }
}
#[doc = "Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}

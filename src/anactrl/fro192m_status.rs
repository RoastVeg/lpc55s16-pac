#[doc = "Register `FRO192M_STATUS` reader"]
pub type R = crate::R<Fro192mStatusSpec>;
#[doc = "Register `FRO192M_STATUS` writer"]
pub type W = crate::W<Fro192mStatusSpec>;
#[doc = "Output clock valid signal. Indicates that CCO clock has settled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkValid {
    #[doc = "0: No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    Noclkout = 0,
    #[doc = "1: Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    Clkout = 1,
}
impl From<ClkValid> for bool {
    #[inline(always)]
    fn from(variant: ClkValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_VALID` reader - Output clock valid signal. Indicates that CCO clock has settled."]
pub type ClkValidR = crate::BitReader<ClkValid>;
impl ClkValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkValid {
        match self.bits {
            false => ClkValid::Noclkout,
            true => ClkValid::Clkout,
        }
    }
    #[doc = "No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    #[inline(always)]
    pub fn is_noclkout(&self) -> bool {
        *self == ClkValid::Noclkout
    }
    #[doc = "Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == ClkValid::Clkout
    }
}
#[doc = "Field `ATB_VCTRL` reader - CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
pub type AtbVctrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output clock valid signal. Indicates that CCO clock has settled."]
    #[inline(always)]
    pub fn clk_valid(&self) -> ClkValidR {
        ClkValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[inline(always)]
    pub fn atb_vctrl(&self) -> AtbVctrlR {
        AtbVctrlR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "192MHz Free Running OScillator (FRO) Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fro192m_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro192m_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fro192mStatusSpec;
impl crate::RegisterSpec for Fro192mStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro192m_status::R`](R) reader structure"]
impl crate::Readable for Fro192mStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fro192m_status::W`](W) writer structure"]
impl crate::Writable for Fro192mStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRO192M_STATUS to value 0x03"]
impl crate::Resettable for Fro192mStatusSpec {
    const RESET_VALUE: u32 = 0x03;
}

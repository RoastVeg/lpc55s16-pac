#[doc = "Register `LDO_XO32M` reader"]
pub type R = crate::R<LdoXo32mSpec>;
#[doc = "Register `LDO_XO32M` writer"]
pub type W = crate::W<LdoXo32mSpec>;
#[doc = "Activate LDO bypass.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Disable bypass mode (for normal operations)."]
    Disable = 0,
    #[doc = "1: Activate LDO bypass."]
    Enable = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Activate LDO bypass."]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Disable,
            true => Bypass::Enable,
        }
    }
    #[doc = "Disable bypass mode (for normal operations)."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bypass::Disable
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bypass::Enable
    }
}
#[doc = "Field `BYPASS` writer - Activate LDO bypass."]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable bypass mode (for normal operations)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Disable)
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Enable)
    }
}
#[doc = ".\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highz {
    #[doc = "0: Output in High normal state."]
    Normalmpedance = 0,
    #[doc = "1: Output in High Impedance state."]
    Highimpedance = 1,
}
impl From<Highz> for bool {
    #[inline(always)]
    fn from(variant: Highz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHZ` reader - ."]
pub type HighzR = crate::BitReader<Highz>;
impl HighzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highz {
        match self.bits {
            false => Highz::Normalmpedance,
            true => Highz::Highimpedance,
        }
    }
    #[doc = "Output in High normal state."]
    #[inline(always)]
    pub fn is_normalmpedance(&self) -> bool {
        *self == Highz::Normalmpedance
    }
    #[doc = "Output in High Impedance state."]
    #[inline(always)]
    pub fn is_highimpedance(&self) -> bool {
        *self == Highz::Highimpedance
    }
}
#[doc = "Field `HIGHZ` writer - ."]
pub type HighzW<'a, REG> = crate::BitWriter<'a, REG, Highz>;
impl<'a, REG> HighzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output in High normal state."]
    #[inline(always)]
    pub fn normalmpedance(self) -> &'a mut crate::W<REG> {
        self.variant(Highz::Normalmpedance)
    }
    #[doc = "Output in High Impedance state."]
    #[inline(always)]
    pub fn highimpedance(self) -> &'a mut crate::W<REG> {
        self.variant(Highz::Highimpedance)
    }
}
#[doc = "Sets the LDO output level.\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vout {
    #[doc = "0: 0.750 V."]
    V0p750 = 0,
    #[doc = "1: 0.775 V."]
    V0p775 = 1,
    #[doc = "2: 0.800 V."]
    V0p800 = 2,
    #[doc = "3: 0.825 V."]
    V0p825 = 3,
    #[doc = "4: 0.850 V."]
    V0p850 = 4,
    #[doc = "5: 0.875 V."]
    V0p875 = 5,
    #[doc = "6: 0.900 V."]
    V0p900 = 6,
    #[doc = "7: 0.925 V."]
    V0p925 = 7,
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(variant: Vout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vout {
    type Ux = u8;
}
impl crate::IsEnum for Vout {}
#[doc = "Field `VOUT` reader - Sets the LDO output level."]
pub type VoutR = crate::FieldReader<Vout>;
impl VoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vout {
        match self.bits {
            0 => Vout::V0p750,
            1 => Vout::V0p775,
            2 => Vout::V0p800,
            3 => Vout::V0p825,
            4 => Vout::V0p850,
            5 => Vout::V0p875,
            6 => Vout::V0p900,
            7 => Vout::V0p925,
            _ => unreachable!(),
        }
    }
    #[doc = "0.750 V."]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == Vout::V0p750
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == Vout::V0p775
    }
    #[doc = "0.800 V."]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == Vout::V0p800
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == Vout::V0p825
    }
    #[doc = "0.850 V."]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == Vout::V0p850
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == Vout::V0p875
    }
    #[doc = "0.900 V."]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == Vout::V0p900
    }
    #[doc = "0.925 V."]
    #[inline(always)]
    pub fn is_v_0p925(&self) -> bool {
        *self == Vout::V0p925
    }
}
#[doc = "Field `VOUT` writer - Sets the LDO output level."]
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vout, crate::Safe>;
impl<'a, REG> VoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.750 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p775)
    }
    #[doc = "0.800 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p825)
    }
    #[doc = "0.850 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p875)
    }
    #[doc = "0.900 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p900)
    }
    #[doc = "0.925 V."]
    #[inline(always)]
    pub fn v_0p925(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::V0p925)
    }
}
#[doc = "Field `IBIAS` reader - Adjust the biasing current."]
pub type IbiasR = crate::FieldReader;
#[doc = "Field `IBIAS` writer - Adjust the biasing current."]
pub type IbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STABMODE` reader - Stability configuration."]
pub type StabmodeR = crate::FieldReader;
#[doc = "Field `STABMODE` writer - Stability configuration."]
pub type StabmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&self) -> HighzR {
        HighzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&self) -> IbiasR {
        IbiasR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&self) -> StabmodeR {
        StabmodeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_XO32M")
            .field("bypass", &self.bypass())
            .field("highz", &self.highz())
            .field("vout", &self.vout())
            .field("ibias", &self.ibias())
            .field("stabmode", &self.stabmode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, LdoXo32mSpec> {
        BypassW::new(self, 1)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&mut self) -> HighzW<'_, LdoXo32mSpec> {
        HighzW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<'_, LdoXo32mSpec> {
        VoutW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&mut self) -> IbiasW<'_, LdoXo32mSpec> {
        IbiasW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&mut self) -> StabmodeW<'_, LdoXo32mSpec> {
        StabmodeW::new(self, 8)
    }
}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo_xo32m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_xo32m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoXo32mSpec;
impl crate::RegisterSpec for LdoXo32mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo_xo32m::R`](R) reader structure"]
impl crate::Readable for LdoXo32mSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo_xo32m::W`](W) writer structure"]
impl crate::Writable for LdoXo32mSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO_XO32M to value 0x03a0"]
impl crate::Resettable for LdoXo32mSpec {
    const RESET_VALUE: u32 = 0x03a0;
}

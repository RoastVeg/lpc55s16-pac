#[doc = "Register `USB1NEEDCLKCTRL` reader"]
pub type R = crate::R<Usb1needclkctrlSpec>;
#[doc = "Register `USB1NEEDCLKCTRL` writer"]
pub type W = crate::W<Usb1needclkctrlSpec>;
#[doc = "USB1-HS Device need_clock signal control:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApHsDevNeedclk {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HwCtrl = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    Forced = 1,
}
impl From<ApHsDevNeedclk> for bool {
    #[inline(always)]
    fn from(variant: ApHsDevNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` reader - USB1-HS Device need_clock signal control:"]
pub type ApHsDevNeedclkR = crate::BitReader<ApHsDevNeedclk>;
impl ApHsDevNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApHsDevNeedclk {
        match self.bits {
            false => ApHsDevNeedclk::HwCtrl,
            true => ApHsDevNeedclk::Forced,
        }
    }
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == ApHsDevNeedclk::HwCtrl
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == ApHsDevNeedclk::Forced
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` writer - USB1-HS Device need_clock signal control:"]
pub type ApHsDevNeedclkW<'a, REG> = crate::BitWriter<'a, REG, ApHsDevNeedclk>;
impl<'a, REG> ApHsDevNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApHsDevNeedclk::HwCtrl)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(ApHsDevNeedclk::Forced)
    }
}
#[doc = "USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolHsDevNeedclk {
    #[doc = "0: Falling edge of DEV_NEEDCLK triggers wake-up."]
    Falling = 0,
    #[doc = "1: Rising edge of DEV_NEEDCLK triggers wake-up."]
    Rising = 1,
}
impl From<PolHsDevNeedclk> for bool {
    #[inline(always)]
    fn from(variant: PolHsDevNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` reader - USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub type PolHsDevNeedclkR = crate::BitReader<PolHsDevNeedclk>;
impl PolHsDevNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolHsDevNeedclk {
        match self.bits {
            false => PolHsDevNeedclk::Falling,
            true => PolHsDevNeedclk::Rising,
        }
    }
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PolHsDevNeedclk::Falling
    }
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PolHsDevNeedclk::Rising
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` writer - USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub type PolHsDevNeedclkW<'a, REG> = crate::BitWriter<'a, REG, PolHsDevNeedclk>;
impl<'a, REG> PolHsDevNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PolHsDevNeedclk::Falling)
    }
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PolHsDevNeedclk::Rising)
    }
}
#[doc = "USB1-HS Host need clock signal control:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApHsHostNeedclk {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HwCtrl = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    Forced = 1,
}
impl From<ApHsHostNeedclk> for bool {
    #[inline(always)]
    fn from(variant: ApHsHostNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` reader - USB1-HS Host need clock signal control:"]
pub type ApHsHostNeedclkR = crate::BitReader<ApHsHostNeedclk>;
impl ApHsHostNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApHsHostNeedclk {
        match self.bits {
            false => ApHsHostNeedclk::HwCtrl,
            true => ApHsHostNeedclk::Forced,
        }
    }
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == ApHsHostNeedclk::HwCtrl
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == ApHsHostNeedclk::Forced
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` writer - USB1-HS Host need clock signal control:"]
pub type ApHsHostNeedclkW<'a, REG> = crate::BitWriter<'a, REG, ApHsHostNeedclk>;
impl<'a, REG> ApHsHostNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApHsHostNeedclk::HwCtrl)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(ApHsHostNeedclk::Forced)
    }
}
#[doc = "USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolHsHostNeedclk {
    #[doc = "0: Falling edge of HOST_NEEDCLK triggers wake-up."]
    Falling = 0,
    #[doc = "1: Rising edge of HOST_NEEDCLK triggers wake-up."]
    Rising = 1,
}
impl From<PolHsHostNeedclk> for bool {
    #[inline(always)]
    fn from(variant: PolHsHostNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` reader - USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub type PolHsHostNeedclkR = crate::BitReader<PolHsHostNeedclk>;
impl PolHsHostNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolHsHostNeedclk {
        match self.bits {
            false => PolHsHostNeedclk::Falling,
            true => PolHsHostNeedclk::Rising,
        }
    }
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PolHsHostNeedclk::Falling
    }
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PolHsHostNeedclk::Rising
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` writer - USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub type PolHsHostNeedclkW<'a, REG> = crate::BitWriter<'a, REG, PolHsHostNeedclk>;
impl<'a, REG> PolHsHostNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PolHsHostNeedclk::Falling)
    }
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PolHsHostNeedclk::Rising)
    }
}
#[doc = "Software override of device controller PHY wake up logic.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsDevWakeupN {
    #[doc = "0: Forces USB1_PHY to wake-up."]
    ForceWup = 0,
    #[doc = "1: Normal USB1_PHY behavior."]
    NormalWup = 1,
}
impl From<HsDevWakeupN> for bool {
    #[inline(always)]
    fn from(variant: HsDevWakeupN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` reader - Software override of device controller PHY wake up logic."]
pub type HsDevWakeupNR = crate::BitReader<HsDevWakeupN>;
impl HsDevWakeupNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsDevWakeupN {
        match self.bits {
            false => HsDevWakeupN::ForceWup,
            true => HsDevWakeupN::NormalWup,
        }
    }
    #[doc = "Forces USB1_PHY to wake-up."]
    #[inline(always)]
    pub fn is_force_wup(&self) -> bool {
        *self == HsDevWakeupN::ForceWup
    }
    #[doc = "Normal USB1_PHY behavior."]
    #[inline(always)]
    pub fn is_normal_wup(&self) -> bool {
        *self == HsDevWakeupN::NormalWup
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` writer - Software override of device controller PHY wake up logic."]
pub type HsDevWakeupNW<'a, REG> = crate::BitWriter<'a, REG, HsDevWakeupN>;
impl<'a, REG> HsDevWakeupNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Forces USB1_PHY to wake-up."]
    #[inline(always)]
    pub fn force_wup(self) -> &'a mut crate::W<REG> {
        self.variant(HsDevWakeupN::ForceWup)
    }
    #[doc = "Normal USB1_PHY behavior."]
    #[inline(always)]
    pub fn normal_wup(self) -> &'a mut crate::W<REG> {
        self.variant(HsDevWakeupN::NormalWup)
    }
}
impl R {
    #[doc = "Bit 0 - USB1-HS Device need_clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_dev_needclk(&self) -> ApHsDevNeedclkR {
        ApHsDevNeedclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub fn pol_hs_dev_needclk(&self) -> PolHsDevNeedclkR {
        PolHsDevNeedclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB1-HS Host need clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_host_needclk(&self) -> ApHsHostNeedclkR {
        ApHsHostNeedclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub fn pol_hs_host_needclk(&self) -> PolHsHostNeedclkR {
        PolHsHostNeedclkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HsDevWakeupNR {
        HsDevWakeupNR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1NEEDCLKCTRL")
            .field("ap_hs_dev_needclk", &self.ap_hs_dev_needclk())
            .field("pol_hs_dev_needclk", &self.pol_hs_dev_needclk())
            .field("ap_hs_host_needclk", &self.ap_hs_host_needclk())
            .field("pol_hs_host_needclk", &self.pol_hs_host_needclk())
            .field("hs_dev_wakeup_n", &self.hs_dev_wakeup_n())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB1-HS Device need_clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_dev_needclk(&mut self) -> ApHsDevNeedclkW<'_, Usb1needclkctrlSpec> {
        ApHsDevNeedclkW::new(self, 0)
    }
    #[doc = "Bit 1 - USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub fn pol_hs_dev_needclk(&mut self) -> PolHsDevNeedclkW<'_, Usb1needclkctrlSpec> {
        PolHsDevNeedclkW::new(self, 1)
    }
    #[doc = "Bit 2 - USB1-HS Host need clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_host_needclk(&mut self) -> ApHsHostNeedclkW<'_, Usb1needclkctrlSpec> {
        ApHsHostNeedclkW::new(self, 2)
    }
    #[doc = "Bit 3 - USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub fn pol_hs_host_needclk(&mut self) -> PolHsHostNeedclkW<'_, Usb1needclkctrlSpec> {
        PolHsHostNeedclkW::new(self, 3)
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&mut self) -> HsDevWakeupNW<'_, Usb1needclkctrlSpec> {
        HsDevWakeupNW::new(self, 4)
    }
}
#[doc = "USB1-HS need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1needclkctrlSpec;
impl crate::RegisterSpec for Usb1needclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1needclkctrl::R`](R) reader structure"]
impl crate::Readable for Usb1needclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1needclkctrl::W`](W) writer structure"]
impl crate::Writable for Usb1needclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB1NEEDCLKCTRL to value 0x10"]
impl crate::Resettable for Usb1needclkctrlSpec {
    const RESET_VALUE: u32 = 0x10;
}

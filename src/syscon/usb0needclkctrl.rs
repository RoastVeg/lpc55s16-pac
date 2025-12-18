#[doc = "Register `USB0NEEDCLKCTRL` reader"]
pub type R = crate::R<Usb0needclkctrlSpec>;
#[doc = "Register `USB0NEEDCLKCTRL` writer"]
pub type W = crate::W<Usb0needclkctrlSpec>;
#[doc = "USB0-FS Device USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApFsDevNeedclk {
    #[doc = "0: Under hardware control."]
    HwCtrl = 0,
    #[doc = "1: Forced high."]
    Forced = 1,
}
impl From<ApFsDevNeedclk> for bool {
    #[inline(always)]
    fn from(variant: ApFsDevNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` reader - USB0-FS Device USB0_NEEDCLK signal control:."]
pub type ApFsDevNeedclkR = crate::BitReader<ApFsDevNeedclk>;
impl ApFsDevNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApFsDevNeedclk {
        match self.bits {
            false => ApFsDevNeedclk::HwCtrl,
            true => ApFsDevNeedclk::Forced,
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == ApFsDevNeedclk::HwCtrl
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == ApFsDevNeedclk::Forced
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` writer - USB0-FS Device USB0_NEEDCLK signal control:."]
pub type ApFsDevNeedclkW<'a, REG> = crate::BitWriter<'a, REG, ApFsDevNeedclk>;
impl<'a, REG> ApFsDevNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApFsDevNeedclk::HwCtrl)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(ApFsDevNeedclk::Forced)
    }
}
#[doc = "USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolFsDevNeedclk {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    Falling = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    Rising = 1,
}
impl From<PolFsDevNeedclk> for bool {
    #[inline(always)]
    fn from(variant: PolFsDevNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` reader - USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
pub type PolFsDevNeedclkR = crate::BitReader<PolFsDevNeedclk>;
impl PolFsDevNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolFsDevNeedclk {
        match self.bits {
            false => PolFsDevNeedclk::Falling,
            true => PolFsDevNeedclk::Rising,
        }
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PolFsDevNeedclk::Falling
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PolFsDevNeedclk::Rising
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` writer - USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
pub type PolFsDevNeedclkW<'a, REG> = crate::BitWriter<'a, REG, PolFsDevNeedclk>;
impl<'a, REG> PolFsDevNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PolFsDevNeedclk::Falling)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PolFsDevNeedclk::Rising)
    }
}
#[doc = "USB0-FS Host USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApFsHostNeedclk {
    #[doc = "0: Under hardware control."]
    HwCtrl = 0,
    #[doc = "1: Forced high."]
    Forced = 1,
}
impl From<ApFsHostNeedclk> for bool {
    #[inline(always)]
    fn from(variant: ApFsHostNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` reader - USB0-FS Host USB0_NEEDCLK signal control:."]
pub type ApFsHostNeedclkR = crate::BitReader<ApFsHostNeedclk>;
impl ApFsHostNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApFsHostNeedclk {
        match self.bits {
            false => ApFsHostNeedclk::HwCtrl,
            true => ApFsHostNeedclk::Forced,
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == ApFsHostNeedclk::HwCtrl
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == ApFsHostNeedclk::Forced
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` writer - USB0-FS Host USB0_NEEDCLK signal control:."]
pub type ApFsHostNeedclkW<'a, REG> = crate::BitWriter<'a, REG, ApFsHostNeedclk>;
impl<'a, REG> ApFsHostNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApFsHostNeedclk::HwCtrl)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(ApFsHostNeedclk::Forced)
    }
}
#[doc = "USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolFsHostNeedclk {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    Falling = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    Rising = 1,
}
impl From<PolFsHostNeedclk> for bool {
    #[inline(always)]
    fn from(variant: PolFsHostNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` reader - USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
pub type PolFsHostNeedclkR = crate::BitReader<PolFsHostNeedclk>;
impl PolFsHostNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolFsHostNeedclk {
        match self.bits {
            false => PolFsHostNeedclk::Falling,
            true => PolFsHostNeedclk::Rising,
        }
    }
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PolFsHostNeedclk::Falling
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PolFsHostNeedclk::Rising
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` writer - USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
pub type PolFsHostNeedclkW<'a, REG> = crate::BitWriter<'a, REG, PolFsHostNeedclk>;
impl<'a, REG> PolFsHostNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PolFsHostNeedclk::Falling)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PolFsHostNeedclk::Rising)
    }
}
impl R {
    #[doc = "Bit 0 - USB0-FS Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_needclk(&self) -> ApFsDevNeedclkR {
        ApFsDevNeedclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_needclk(&self) -> PolFsDevNeedclkR {
        PolFsDevNeedclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB0-FS Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_needclk(&self) -> ApFsHostNeedclkR {
        ApFsHostNeedclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_needclk(&self) -> PolFsHostNeedclkR {
        PolFsHostNeedclkR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0NEEDCLKCTRL")
            .field("ap_fs_dev_needclk", &self.ap_fs_dev_needclk())
            .field("pol_fs_dev_needclk", &self.pol_fs_dev_needclk())
            .field("ap_fs_host_needclk", &self.ap_fs_host_needclk())
            .field("pol_fs_host_needclk", &self.pol_fs_host_needclk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB0-FS Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_needclk(&mut self) -> ApFsDevNeedclkW<'_, Usb0needclkctrlSpec> {
        ApFsDevNeedclkW::new(self, 0)
    }
    #[doc = "Bit 1 - USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_needclk(&mut self) -> PolFsDevNeedclkW<'_, Usb0needclkctrlSpec> {
        PolFsDevNeedclkW::new(self, 1)
    }
    #[doc = "Bit 2 - USB0-FS Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_needclk(&mut self) -> ApFsHostNeedclkW<'_, Usb0needclkctrlSpec> {
        ApFsHostNeedclkW::new(self, 2)
    }
    #[doc = "Bit 3 - USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_needclk(&mut self) -> PolFsHostNeedclkW<'_, Usb0needclkctrlSpec> {
        PolFsHostNeedclkW::new(self, 3)
    }
}
#[doc = "USB0-FS need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0needclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0needclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb0needclkctrlSpec;
impl crate::RegisterSpec for Usb0needclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb0needclkctrl::R`](R) reader structure"]
impl crate::Readable for Usb0needclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb0needclkctrl::W`](W) writer structure"]
impl crate::Writable for Usb0needclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB0NEEDCLKCTRL to value 0"]
impl crate::Resettable for Usb0needclkctrlSpec {}

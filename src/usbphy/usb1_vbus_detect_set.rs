#[doc = "Register `USB1_VBUS_DETECT_SET` reader"]
pub type R = crate::R<Usb1VbusDetectSetSpec>;
#[doc = "Register `USB1_VBUS_DETECT_SET` writer"]
pub type W = crate::W<Usb1VbusDetectSetSpec>;
#[doc = "Sets the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusvalidThresh {
    #[doc = "0: 4.0V"]
    Value0 = 0,
    #[doc = "1: 4.1V"]
    Value1 = 1,
    #[doc = "2: 4.2V"]
    Value2 = 2,
    #[doc = "3: 4.3V"]
    Value3 = 3,
    #[doc = "4: 4.4V(Default)"]
    Value4 = 4,
    #[doc = "5: 4.5V"]
    Value5 = 5,
    #[doc = "6: 4.6V"]
    Value6 = 6,
    #[doc = "7: 4.7V"]
    Value7 = 7,
}
impl From<VbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(variant: VbusvalidThresh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusvalidThresh {
    type Ux = u8;
}
impl crate::IsEnum for VbusvalidThresh {}
#[doc = "Field `VBUSVALID_THRESH` reader - Sets the threshold for the VBUSVALID comparator"]
pub type VbusvalidThreshR = crate::FieldReader<VbusvalidThresh>;
impl VbusvalidThreshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidThresh {
        match self.bits {
            0 => VbusvalidThresh::Value0,
            1 => VbusvalidThresh::Value1,
            2 => VbusvalidThresh::Value2,
            3 => VbusvalidThresh::Value3,
            4 => VbusvalidThresh::Value4,
            5 => VbusvalidThresh::Value5,
            6 => VbusvalidThresh::Value6,
            7 => VbusvalidThresh::Value7,
            _ => unreachable!(),
        }
    }
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == VbusvalidThresh::Value0
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbusvalidThresh::Value1
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VbusvalidThresh::Value2
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VbusvalidThresh::Value3
    }
    #[doc = "4.4V(Default)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VbusvalidThresh::Value4
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == VbusvalidThresh::Value5
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == VbusvalidThresh::Value6
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == VbusvalidThresh::Value7
    }
}
#[doc = "Field `VBUSVALID_THRESH` writer - Sets the threshold for the VBUSVALID comparator"]
pub type VbusvalidThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3, VbusvalidThresh, crate::Safe>;
impl<'a, REG> VbusvalidThreshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value0)
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value1)
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value2)
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value3)
    }
    #[doc = "4.4V(Default)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value4)
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value5)
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value6)
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::Value7)
    }
}
#[doc = "VBUS detect signal override enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusOverrideEn {
    #[doc = "0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    Value0 = 0,
    #[doc = "1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    Value1 = 1,
}
impl From<VbusOverrideEn> for bool {
    #[inline(always)]
    fn from(variant: VbusOverrideEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` reader - VBUS detect signal override enable"]
pub type VbusOverrideEnR = crate::BitReader<VbusOverrideEn>;
impl VbusOverrideEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusOverrideEn {
        match self.bits {
            false => VbusOverrideEn::Value0,
            true => VbusOverrideEn::Value1,
        }
    }
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == VbusOverrideEn::Value0
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbusOverrideEn::Value1
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` writer - VBUS detect signal override enable"]
pub type VbusOverrideEnW<'a, REG> = crate::BitWriter<'a, REG, VbusOverrideEn>;
impl<'a, REG> VbusOverrideEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOverrideEn::Value0)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOverrideEn::Value1)
    }
}
#[doc = "Field `SESSEND_OVERRIDE` reader - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type SessendOverrideR = crate::BitReader;
#[doc = "Field `SESSEND_OVERRIDE` writer - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type SessendOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALID_OVERRIDE` reader - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type BvalidOverrideR = crate::BitReader;
#[doc = "Field `BVALID_OVERRIDE` writer - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type BvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALID_OVERRIDE` reader - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type AvalidOverrideR = crate::BitReader;
#[doc = "Field `AVALID_OVERRIDE` writer - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
pub type AvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSVALID_OVERRIDE` reader - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
pub type VbusvalidOverrideR = crate::BitReader;
#[doc = "Field `VBUSVALID_OVERRIDE` writer - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
pub type VbusvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusvalidSel {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    Value0 = 0,
    #[doc = "1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    Value1 = 1,
}
impl From<VbusvalidSel> for bool {
    #[inline(always)]
    fn from(variant: VbusvalidSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusvalidSelR = crate::BitReader<VbusvalidSel>;
impl VbusvalidSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidSel {
        match self.bits {
            false => VbusvalidSel::Value0,
            true => VbusvalidSel::Value1,
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == VbusvalidSel::Value0
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbusvalidSel::Value1
    }
}
#[doc = "Field `VBUSVALID_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusvalidSelW<'a, REG> = crate::BitWriter<'a, REG, VbusvalidSel>;
impl<'a, REG> VbusvalidSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidSel::Value0)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidSel::Value1)
    }
}
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusSourceSel {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    Value0 = 0,
    #[doc = "1: Use the Session Valid comparator results for signal reported to the USB controller"]
    Value1 = 1,
    #[doc = "2: Use the Session Valid comparator results for signal reported to the USB controller"]
    Value2 = 2,
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(variant: VbusSourceSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusSourceSel {
    type Ux = u8;
}
impl crate::IsEnum for VbusSourceSel {}
#[doc = "Field `VBUS_SOURCE_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusSourceSelR = crate::FieldReader<VbusSourceSel>;
impl VbusSourceSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VbusSourceSel> {
        match self.bits {
            0 => Some(VbusSourceSel::Value0),
            1 => Some(VbusSourceSel::Value1),
            2 => Some(VbusSourceSel::Value2),
            _ => None,
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == VbusSourceSel::Value0
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbusSourceSel::Value1
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VbusSourceSel::Value2
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusSourceSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, VbusSourceSel>;
impl<'a, REG> VbusSourceSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::Value0)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::Value1)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::Value2)
    }
}
#[doc = "Field `ID_OVERRIDE_EN` reader - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
pub type IdOverrideEnR = crate::BitReader;
#[doc = "Field `ID_OVERRIDE_EN` writer - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
pub type IdOverrideEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_OVERRIDE` reader - ID override value."]
pub type IdOverrideR = crate::BitReader;
#[doc = "Field `ID_OVERRIDE` writer - ID override value."]
pub type IdOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable ID override using the pinmuxed value:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtIdOverrideEn {
    #[doc = "0: Select the Muxed value chosen using ID_OVERRIDE_EN."]
    Value0 = 0,
    #[doc = "1: Select the external ID value."]
    Value1 = 1,
}
impl From<ExtIdOverrideEn> for bool {
    #[inline(always)]
    fn from(variant: ExtIdOverrideEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_ID_OVERRIDE_EN` reader - Enable ID override using the pinmuxed value:"]
pub type ExtIdOverrideEnR = crate::BitReader<ExtIdOverrideEn>;
impl ExtIdOverrideEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExtIdOverrideEn {
        match self.bits {
            false => ExtIdOverrideEn::Value0,
            true => ExtIdOverrideEn::Value1,
        }
    }
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ExtIdOverrideEn::Value0
    }
    #[doc = "Select the external ID value."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ExtIdOverrideEn::Value1
    }
}
#[doc = "Field `EXT_ID_OVERRIDE_EN` writer - Enable ID override using the pinmuxed value:"]
pub type ExtIdOverrideEnW<'a, REG> = crate::BitWriter<'a, REG, ExtIdOverrideEn>;
impl<'a, REG> ExtIdOverrideEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(ExtIdOverrideEn::Value0)
    }
    #[doc = "Select the external ID value."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ExtIdOverrideEn::Value1)
    }
}
#[doc = "Enable VBUS override using the pinmuxed value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtVbusOverrideEn {
    #[doc = "0: Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    Value0 = 0,
    #[doc = "1: Select the external VBUS VALID value."]
    Value1 = 1,
}
impl From<ExtVbusOverrideEn> for bool {
    #[inline(always)]
    fn from(variant: ExtVbusOverrideEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_VBUS_OVERRIDE_EN` reader - Enable VBUS override using the pinmuxed value."]
pub type ExtVbusOverrideEnR = crate::BitReader<ExtVbusOverrideEn>;
impl ExtVbusOverrideEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExtVbusOverrideEn {
        match self.bits {
            false => ExtVbusOverrideEn::Value0,
            true => ExtVbusOverrideEn::Value1,
        }
    }
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ExtVbusOverrideEn::Value0
    }
    #[doc = "Select the external VBUS VALID value."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ExtVbusOverrideEn::Value1
    }
}
#[doc = "Field `EXT_VBUS_OVERRIDE_EN` writer - Enable VBUS override using the pinmuxed value."]
pub type ExtVbusOverrideEnW<'a, REG> = crate::BitWriter<'a, REG, ExtVbusOverrideEn>;
impl<'a, REG> ExtVbusOverrideEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(ExtVbusOverrideEn::Value0)
    }
    #[doc = "Select the external VBUS VALID value."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ExtVbusOverrideEn::Value1)
    }
}
#[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusvalidToSessvalid {
    #[doc = "0: Use the VBUS_VALID comparator for VBUS_VALID results"]
    Value0 = 0,
    #[doc = "1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    Value1 = 1,
}
impl From<VbusvalidToSessvalid> for bool {
    #[inline(always)]
    fn from(variant: VbusvalidToSessvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` reader - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
pub type VbusvalidToSessvalidR = crate::BitReader<VbusvalidToSessvalid>;
impl VbusvalidToSessvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidToSessvalid {
        match self.bits {
            false => VbusvalidToSessvalid::Value0,
            true => VbusvalidToSessvalid::Value1,
        }
    }
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == VbusvalidToSessvalid::Value0
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbusvalidToSessvalid::Value1
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` writer - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
pub type VbusvalidToSessvalidW<'a, REG> = crate::BitWriter<'a, REG, VbusvalidToSessvalid>;
impl<'a, REG> VbusvalidToSessvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidToSessvalid::Value0)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidToSessvalid::Value1)
    }
}
#[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrupCmps {
    #[doc = "0: Powers down the VBUS_VALID comparator"]
    Value0 = 0,
    #[doc = "7: Enables the VBUS_VALID comparator (default)"]
    Value1 = 7,
}
impl From<PwrupCmps> for u8 {
    #[inline(always)]
    fn from(variant: PwrupCmps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrupCmps {
    type Ux = u8;
}
impl crate::IsEnum for PwrupCmps {}
#[doc = "Field `PWRUP_CMPS` reader - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
pub type PwrupCmpsR = crate::FieldReader<PwrupCmps>;
impl PwrupCmpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrupCmps> {
        match self.bits {
            0 => Some(PwrupCmps::Value0),
            7 => Some(PwrupCmps::Value1),
            _ => None,
        }
    }
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PwrupCmps::Value0
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PwrupCmps::Value1
    }
}
#[doc = "Field `PWRUP_CMPS` writer - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
pub type PwrupCmpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, PwrupCmps>;
impl<'a, REG> PwrupCmpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrupCmps::Value0)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrupCmps::Value1)
    }
}
#[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DischargeVbus {
    #[doc = "0: VBUS discharge resistor is disabled (Default)"]
    Value0 = 0,
    #[doc = "1: VBUS discharge resistor is enabled"]
    Value1 = 1,
}
impl From<DischargeVbus> for bool {
    #[inline(always)]
    fn from(variant: DischargeVbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCHARGE_VBUS` reader - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub type DischargeVbusR = crate::BitReader<DischargeVbus>;
impl DischargeVbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DischargeVbus {
        match self.bits {
            false => DischargeVbus::Value0,
            true => DischargeVbus::Value1,
        }
    }
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DischargeVbus::Value0
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DischargeVbus::Value1
    }
}
#[doc = "Field `DISCHARGE_VBUS` writer - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub type DischargeVbusW<'a, REG> = crate::BitWriter<'a, REG, DischargeVbus>;
impl<'a, REG> DischargeVbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(DischargeVbus::Value0)
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DischargeVbus::Value1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VbusvalidThreshR {
        VbusvalidThreshR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&self) -> VbusOverrideEnR {
        VbusOverrideEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&self) -> SessendOverrideR {
        SessendOverrideR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&self) -> BvalidOverrideR {
        BvalidOverrideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&self) -> AvalidOverrideR {
        AvalidOverrideR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&self) -> VbusvalidOverrideR {
        VbusvalidOverrideR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&self) -> VbusvalidSelR {
        VbusvalidSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&self) -> VbusSourceSelR {
        VbusSourceSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub fn id_override_en(&self) -> IdOverrideEnR {
        IdOverrideEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ID override value."]
    #[inline(always)]
    pub fn id_override(&self) -> IdOverrideR {
        IdOverrideR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub fn ext_id_override_en(&self) -> ExtIdOverrideEnR {
        ExtIdOverrideEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub fn ext_vbus_override_en(&self) -> ExtVbusOverrideEnR {
        ExtVbusOverrideEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&self) -> VbusvalidToSessvalidR {
        VbusvalidToSessvalidR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&self) -> PwrupCmpsR {
        PwrupCmpsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DischargeVbusR {
        DischargeVbusR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_SET")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&mut self) -> VbusvalidThreshW<'_, Usb1VbusDetectSetSpec> {
        VbusvalidThreshW::new(self, 0)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&mut self) -> VbusOverrideEnW<'_, Usb1VbusDetectSetSpec> {
        VbusOverrideEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&mut self) -> SessendOverrideW<'_, Usb1VbusDetectSetSpec> {
        SessendOverrideW::new(self, 4)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&mut self) -> BvalidOverrideW<'_, Usb1VbusDetectSetSpec> {
        BvalidOverrideW::new(self, 5)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&mut self) -> AvalidOverrideW<'_, Usb1VbusDetectSetSpec> {
        AvalidOverrideW::new(self, 6)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&mut self) -> VbusvalidOverrideW<'_, Usb1VbusDetectSetSpec> {
        VbusvalidOverrideW::new(self, 7)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&mut self) -> VbusvalidSelW<'_, Usb1VbusDetectSetSpec> {
        VbusvalidSelW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&mut self) -> VbusSourceSelW<'_, Usb1VbusDetectSetSpec> {
        VbusSourceSelW::new(self, 9)
    }
    #[doc = "Bit 11 - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub fn id_override_en(&mut self) -> IdOverrideEnW<'_, Usb1VbusDetectSetSpec> {
        IdOverrideEnW::new(self, 11)
    }
    #[doc = "Bit 12 - ID override value."]
    #[inline(always)]
    pub fn id_override(&mut self) -> IdOverrideW<'_, Usb1VbusDetectSetSpec> {
        IdOverrideW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub fn ext_id_override_en(&mut self) -> ExtIdOverrideEnW<'_, Usb1VbusDetectSetSpec> {
        ExtIdOverrideEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub fn ext_vbus_override_en(&mut self) -> ExtVbusOverrideEnW<'_, Usb1VbusDetectSetSpec> {
        ExtVbusOverrideEnW::new(self, 14)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&mut self) -> VbusvalidToSessvalidW<'_, Usb1VbusDetectSetSpec> {
        VbusvalidToSessvalidW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&mut self) -> PwrupCmpsW<'_, Usb1VbusDetectSetSpec> {
        PwrupCmpsW::new(self, 20)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&mut self) -> DischargeVbusW<'_, Usb1VbusDetectSetSpec> {
        DischargeVbusW::new(self, 26)
    }
}
#[doc = "USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1VbusDetectSetSpec;
impl crate::RegisterSpec for Usb1VbusDetectSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_vbus_detect_set::R`](R) reader structure"]
impl crate::Readable for Usb1VbusDetectSetSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1_vbus_detect_set::W`](W) writer structure"]
impl crate::Writable for Usb1VbusDetectSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT_SET to value 0x0070_0004"]
impl crate::Resettable for Usb1VbusDetectSetSpec {
    const RESET_VALUE: u32 = 0x0070_0004;
}

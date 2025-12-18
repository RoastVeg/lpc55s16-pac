#[doc = "Register `AUTOCLKGATEOVERRIDE` reader"]
pub type R = crate::R<AutoclkgateoverrideSpec>;
#[doc = "Register `AUTOCLKGATEOVERRIDE` writer"]
pub type W = crate::W<AutoclkgateoverrideSpec>;
#[doc = "Control automatic clock gating of ROM controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rom {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Rom> for bool {
    #[inline(always)]
    fn from(variant: Rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Control automatic clock gating of ROM controller."]
pub type RomR = crate::BitReader<Rom>;
impl RomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rom {
        match self.bits {
            false => Rom::Disable,
            true => Rom::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rom::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rom::Enable
    }
}
#[doc = "Field `ROM` writer - Control automatic clock gating of ROM controller."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG, Rom>;
impl<'a, REG> RomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::Enable)
    }
}
#[doc = "Control automatic clock gating of RAMX controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamxCtrl {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<RamxCtrl> for bool {
    #[inline(always)]
    fn from(variant: RamxCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMX_CTRL` reader - Control automatic clock gating of RAMX controller."]
pub type RamxCtrlR = crate::BitReader<RamxCtrl>;
impl RamxCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamxCtrl {
        match self.bits {
            false => RamxCtrl::Disable,
            true => RamxCtrl::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RamxCtrl::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RamxCtrl::Enable
    }
}
#[doc = "Field `RAMX_CTRL` writer - Control automatic clock gating of RAMX controller."]
pub type RamxCtrlW<'a, REG> = crate::BitWriter<'a, REG, RamxCtrl>;
impl<'a, REG> RamxCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RamxCtrl::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RamxCtrl::Enable)
    }
}
#[doc = "Control automatic clock gating of RAM0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram0Ctrl {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Ram0Ctrl> for bool {
    #[inline(always)]
    fn from(variant: Ram0Ctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0_CTRL` reader - Control automatic clock gating of RAM0 controller."]
pub type Ram0CtrlR = crate::BitReader<Ram0Ctrl>;
impl Ram0CtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram0Ctrl {
        match self.bits {
            false => Ram0Ctrl::Disable,
            true => Ram0Ctrl::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ram0Ctrl::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ram0Ctrl::Enable
    }
}
#[doc = "Field `RAM0_CTRL` writer - Control automatic clock gating of RAM0 controller."]
pub type Ram0CtrlW<'a, REG> = crate::BitWriter<'a, REG, Ram0Ctrl>;
impl<'a, REG> Ram0CtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Ctrl::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0Ctrl::Enable)
    }
}
#[doc = "Control automatic clock gating of RAM1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram1Ctrl {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Ram1Ctrl> for bool {
    #[inline(always)]
    fn from(variant: Ram1Ctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM1_CTRL` reader - Control automatic clock gating of RAM1 controller."]
pub type Ram1CtrlR = crate::BitReader<Ram1Ctrl>;
impl Ram1CtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram1Ctrl {
        match self.bits {
            false => Ram1Ctrl::Disable,
            true => Ram1Ctrl::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ram1Ctrl::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ram1Ctrl::Enable
    }
}
#[doc = "Field `RAM1_CTRL` writer - Control automatic clock gating of RAM1 controller."]
pub type Ram1CtrlW<'a, REG> = crate::BitWriter<'a, REG, Ram1Ctrl>;
impl<'a, REG> Ram1CtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Ctrl::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram1Ctrl::Enable)
    }
}
#[doc = "Control automatic clock gating of RAM2 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram2Ctrl {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Ram2Ctrl> for bool {
    #[inline(always)]
    fn from(variant: Ram2Ctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM2_CTRL` reader - Control automatic clock gating of RAM2 controller."]
pub type Ram2CtrlR = crate::BitReader<Ram2Ctrl>;
impl Ram2CtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram2Ctrl {
        match self.bits {
            false => Ram2Ctrl::Disable,
            true => Ram2Ctrl::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ram2Ctrl::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ram2Ctrl::Enable
    }
}
#[doc = "Field `RAM2_CTRL` writer - Control automatic clock gating of RAM2 controller."]
pub type Ram2CtrlW<'a, REG> = crate::BitWriter<'a, REG, Ram2Ctrl>;
impl<'a, REG> Ram2CtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Ctrl::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ram2Ctrl::Enable)
    }
}
#[doc = "Control automatic clock gating of synchronous bridge controller 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync0Apb {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Sync0Apb> for bool {
    #[inline(always)]
    fn from(variant: Sync0Apb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC0_APB` reader - Control automatic clock gating of synchronous bridge controller 0."]
pub type Sync0ApbR = crate::BitReader<Sync0Apb>;
impl Sync0ApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync0Apb {
        match self.bits {
            false => Sync0Apb::Disable,
            true => Sync0Apb::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sync0Apb::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sync0Apb::Enable
    }
}
#[doc = "Field `SYNC0_APB` writer - Control automatic clock gating of synchronous bridge controller 0."]
pub type Sync0ApbW<'a, REG> = crate::BitWriter<'a, REG, Sync0Apb>;
impl<'a, REG> Sync0ApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0Apb::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0Apb::Enable)
    }
}
#[doc = "Control automatic clock gating of synchronous bridge controller 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync1Apb {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Sync1Apb> for bool {
    #[inline(always)]
    fn from(variant: Sync1Apb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC1_APB` reader - Control automatic clock gating of synchronous bridge controller 1."]
pub type Sync1ApbR = crate::BitReader<Sync1Apb>;
impl Sync1ApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync1Apb {
        match self.bits {
            false => Sync1Apb::Disable,
            true => Sync1Apb::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sync1Apb::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sync1Apb::Enable
    }
}
#[doc = "Field `SYNC1_APB` writer - Control automatic clock gating of synchronous bridge controller 1."]
pub type Sync1ApbW<'a, REG> = crate::BitWriter<'a, REG, Sync1Apb>;
impl<'a, REG> Sync1ApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1Apb::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1Apb::Enable)
    }
}
#[doc = "Control automatic clock gating of CRCGEN controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcgen {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Crcgen> for bool {
    #[inline(always)]
    fn from(variant: Crcgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCGEN` reader - Control automatic clock gating of CRCGEN controller."]
pub type CrcgenR = crate::BitReader<Crcgen>;
impl CrcgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcgen {
        match self.bits {
            false => Crcgen::Disable,
            true => Crcgen::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crcgen::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Crcgen::Enable
    }
}
#[doc = "Field `CRCGEN` writer - Control automatic clock gating of CRCGEN controller."]
pub type CrcgenW<'a, REG> = crate::BitWriter<'a, REG, Crcgen>;
impl<'a, REG> CrcgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcgen::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Crcgen::Enable)
    }
}
#[doc = "Control automatic clock gating of DMA0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdma0 {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Sdma0> for bool {
    #[inline(always)]
    fn from(variant: Sdma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0` reader - Control automatic clock gating of DMA0 controller."]
pub type Sdma0R = crate::BitReader<Sdma0>;
impl Sdma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma0 {
        match self.bits {
            false => Sdma0::Disable,
            true => Sdma0::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sdma0::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sdma0::Enable
    }
}
#[doc = "Field `SDMA0` writer - Control automatic clock gating of DMA0 controller."]
pub type Sdma0W<'a, REG> = crate::BitWriter<'a, REG, Sdma0>;
impl<'a, REG> Sdma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::Enable)
    }
}
#[doc = "Control automatic clock gating of DMA1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdma1 {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Sdma1> for bool {
    #[inline(always)]
    fn from(variant: Sdma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA1` reader - Control automatic clock gating of DMA1 controller."]
pub type Sdma1R = crate::BitReader<Sdma1>;
impl Sdma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma1 {
        match self.bits {
            false => Sdma1::Disable,
            true => Sdma1::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sdma1::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sdma1::Enable
    }
}
#[doc = "Field `SDMA1` writer - Control automatic clock gating of DMA1 controller."]
pub type Sdma1W<'a, REG> = crate::BitWriter<'a, REG, Sdma1>;
impl<'a, REG> Sdma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::Enable)
    }
}
#[doc = "Control automatic clock gating of USB controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0 {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Usb0> for bool {
    #[inline(always)]
    fn from(variant: Usb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0` reader - Control automatic clock gating of USB controller."]
pub type Usb0R = crate::BitReader<Usb0>;
impl Usb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0 {
        match self.bits {
            false => Usb0::Disable,
            true => Usb0::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usb0::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usb0::Enable
    }
}
#[doc = "Field `USB0` writer - Control automatic clock gating of USB controller."]
pub type Usb0W<'a, REG> = crate::BitWriter<'a, REG, Usb0>;
impl<'a, REG> Usb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0::Enable)
    }
}
#[doc = "Control automatic clock gating of synchronous system controller registers bank.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscon {
    #[doc = "0: Automatic clock gating is not overridden."]
    Disable = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    Enable = 1,
}
impl From<Syscon> for bool {
    #[inline(always)]
    fn from(variant: Syscon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCON` reader - Control automatic clock gating of synchronous system controller registers bank."]
pub type SysconR = crate::BitReader<Syscon>;
impl SysconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscon {
        match self.bits {
            false => Syscon::Disable,
            true => Syscon::Enable,
        }
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Syscon::Disable
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Syscon::Enable
    }
}
#[doc = "Field `SYSCON` writer - Control automatic clock gating of synchronous system controller registers bank."]
pub type SysconW<'a, REG> = crate::BitWriter<'a, REG, Syscon>;
impl<'a, REG> SysconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syscon::Disable)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syscon::Enable)
    }
}
#[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Enableupdate {
    #[doc = "0: Bit Fields 0 - 15 of this register are not updated"]
    Disable = 0,
    #[doc = "49374: Bit Fields 0 - 15 of this register are updated"]
    Enable = 49374,
}
impl From<Enableupdate> for u16 {
    #[inline(always)]
    fn from(variant: Enableupdate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enableupdate {
    type Ux = u16;
}
impl crate::IsEnum for Enableupdate {}
#[doc = "Field `ENABLEUPDATE` writer - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
pub type EnableupdateW<'a, REG> = crate::FieldWriter<'a, REG, 16, Enableupdate>;
impl<'a, REG> EnableupdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Bit Fields 0 - 15 of this register are not updated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableupdate::Disable)
    }
    #[doc = "Bit Fields 0 - 15 of this register are updated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableupdate::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub fn ramx_ctrl(&self) -> RamxCtrlR {
        RamxCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub fn ram0_ctrl(&self) -> Ram0CtrlR {
        Ram0CtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub fn ram1_ctrl(&self) -> Ram1CtrlR {
        Ram1CtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub fn ram2_ctrl(&self) -> Ram2CtrlR {
        Ram2CtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub fn sync0_apb(&self) -> Sync0ApbR {
        Sync0ApbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub fn sync1_apb(&self) -> Sync1ApbR {
        Sync1ApbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub fn crcgen(&self) -> CrcgenR {
        CrcgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub fn sdma0(&self) -> Sdma0R {
        Sdma0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub fn sdma1(&self) -> Sdma1R {
        Sdma1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub fn usb0(&self) -> Usb0R {
        Usb0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub fn syscon(&self) -> SysconR {
        SysconR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub fn rom(&mut self) -> RomW<'_, AutoclkgateoverrideSpec> {
        RomW::new(self, 0)
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub fn ramx_ctrl(&mut self) -> RamxCtrlW<'_, AutoclkgateoverrideSpec> {
        RamxCtrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub fn ram0_ctrl(&mut self) -> Ram0CtrlW<'_, AutoclkgateoverrideSpec> {
        Ram0CtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub fn ram1_ctrl(&mut self) -> Ram1CtrlW<'_, AutoclkgateoverrideSpec> {
        Ram1CtrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub fn ram2_ctrl(&mut self) -> Ram2CtrlW<'_, AutoclkgateoverrideSpec> {
        Ram2CtrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub fn sync0_apb(&mut self) -> Sync0ApbW<'_, AutoclkgateoverrideSpec> {
        Sync0ApbW::new(self, 7)
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub fn sync1_apb(&mut self) -> Sync1ApbW<'_, AutoclkgateoverrideSpec> {
        Sync1ApbW::new(self, 8)
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub fn crcgen(&mut self) -> CrcgenW<'_, AutoclkgateoverrideSpec> {
        CrcgenW::new(self, 11)
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub fn sdma0(&mut self) -> Sdma0W<'_, AutoclkgateoverrideSpec> {
        Sdma0W::new(self, 12)
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub fn sdma1(&mut self) -> Sdma1W<'_, AutoclkgateoverrideSpec> {
        Sdma1W::new(self, 13)
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub fn usb0(&mut self) -> Usb0W<'_, AutoclkgateoverrideSpec> {
        Usb0W::new(self, 14)
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub fn syscon(&mut self) -> SysconW<'_, AutoclkgateoverrideSpec> {
        SysconW::new(self, 15)
    }
    #[doc = "Bits 16:31 - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline(always)]
    pub fn enableupdate(&mut self) -> EnableupdateW<'_, AutoclkgateoverrideSpec> {
        EnableupdateW::new(self, 16)
    }
}
#[doc = "Control automatic clock gating\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoclkgateoverrideSpec;
impl crate::RegisterSpec for AutoclkgateoverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autoclkgateoverride::R`](R) reader structure"]
impl crate::Readable for AutoclkgateoverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`autoclkgateoverride::W`](W) writer structure"]
impl crate::Writable for AutoclkgateoverrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE to value 0xffff"]
impl crate::Resettable for AutoclkgateoverrideSpec {
    const RESET_VALUE: u32 = 0xffff;
}

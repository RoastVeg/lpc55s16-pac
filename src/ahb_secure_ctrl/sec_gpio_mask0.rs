#[doc = "Register `SEC_GPIO_MASK0` reader"]
pub type R = crate::R<SecGpioMask0Spec>;
#[doc = "Register `SEC_GPIO_MASK0` writer"]
pub type W = crate::W<SecGpioMask0Spec>;
#[doc = "Secure mask for pin P0_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin0SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin0SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin0SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN0_SEC_MASK` reader - Secure mask for pin P0_0"]
pub type Pio0Pin0SecMaskR = crate::BitReader<Pio0Pin0SecMask>;
impl Pio0Pin0SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin0SecMask {
        match self.bits {
            false => Pio0Pin0SecMask::Blocked,
            true => Pio0Pin0SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin0SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin0SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN0_SEC_MASK` writer - Secure mask for pin P0_0"]
pub type Pio0Pin0SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin0SecMask>;
impl<'a, REG> Pio0Pin0SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin0SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin0SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin1SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin1SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin1SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN1_SEC_MASK` reader - Secure mask for pin P0_1"]
pub type Pio0Pin1SecMaskR = crate::BitReader<Pio0Pin1SecMask>;
impl Pio0Pin1SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin1SecMask {
        match self.bits {
            false => Pio0Pin1SecMask::Blocked,
            true => Pio0Pin1SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin1SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin1SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN1_SEC_MASK` writer - Secure mask for pin P0_1"]
pub type Pio0Pin1SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin1SecMask>;
impl<'a, REG> Pio0Pin1SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin1SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin1SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin2SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin2SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin2SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN2_SEC_MASK` reader - Secure mask for pin P0_2"]
pub type Pio0Pin2SecMaskR = crate::BitReader<Pio0Pin2SecMask>;
impl Pio0Pin2SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin2SecMask {
        match self.bits {
            false => Pio0Pin2SecMask::Blocked,
            true => Pio0Pin2SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin2SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin2SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN2_SEC_MASK` writer - Secure mask for pin P0_2"]
pub type Pio0Pin2SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin2SecMask>;
impl<'a, REG> Pio0Pin2SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin2SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin2SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin3SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin3SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin3SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN3_SEC_MASK` reader - Secure mask for pin P0_3"]
pub type Pio0Pin3SecMaskR = crate::BitReader<Pio0Pin3SecMask>;
impl Pio0Pin3SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin3SecMask {
        match self.bits {
            false => Pio0Pin3SecMask::Blocked,
            true => Pio0Pin3SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin3SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin3SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN3_SEC_MASK` writer - Secure mask for pin P0_3"]
pub type Pio0Pin3SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin3SecMask>;
impl<'a, REG> Pio0Pin3SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin3SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin3SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin4SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin4SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin4SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN4_SEC_MASK` reader - Secure mask for pin P0_4"]
pub type Pio0Pin4SecMaskR = crate::BitReader<Pio0Pin4SecMask>;
impl Pio0Pin4SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin4SecMask {
        match self.bits {
            false => Pio0Pin4SecMask::Blocked,
            true => Pio0Pin4SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin4SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin4SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN4_SEC_MASK` writer - Secure mask for pin P0_4"]
pub type Pio0Pin4SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin4SecMask>;
impl<'a, REG> Pio0Pin4SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin4SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin4SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin5SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin5SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin5SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN5_SEC_MASK` reader - Secure mask for pin P0_5"]
pub type Pio0Pin5SecMaskR = crate::BitReader<Pio0Pin5SecMask>;
impl Pio0Pin5SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin5SecMask {
        match self.bits {
            false => Pio0Pin5SecMask::Blocked,
            true => Pio0Pin5SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin5SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin5SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN5_SEC_MASK` writer - Secure mask for pin P0_5"]
pub type Pio0Pin5SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin5SecMask>;
impl<'a, REG> Pio0Pin5SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin5SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin5SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin6SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin6SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin6SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN6_SEC_MASK` reader - Secure mask for pin P0_6"]
pub type Pio0Pin6SecMaskR = crate::BitReader<Pio0Pin6SecMask>;
impl Pio0Pin6SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin6SecMask {
        match self.bits {
            false => Pio0Pin6SecMask::Blocked,
            true => Pio0Pin6SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin6SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin6SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN6_SEC_MASK` writer - Secure mask for pin P0_6"]
pub type Pio0Pin6SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin6SecMask>;
impl<'a, REG> Pio0Pin6SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin6SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin6SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin7SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin7SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin7SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN7_SEC_MASK` reader - Secure mask for pin P0_7"]
pub type Pio0Pin7SecMaskR = crate::BitReader<Pio0Pin7SecMask>;
impl Pio0Pin7SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin7SecMask {
        match self.bits {
            false => Pio0Pin7SecMask::Blocked,
            true => Pio0Pin7SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin7SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin7SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN7_SEC_MASK` writer - Secure mask for pin P0_7"]
pub type Pio0Pin7SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin7SecMask>;
impl<'a, REG> Pio0Pin7SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin7SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin7SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin8SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin8SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin8SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN8_SEC_MASK` reader - Secure mask for pin P0_8"]
pub type Pio0Pin8SecMaskR = crate::BitReader<Pio0Pin8SecMask>;
impl Pio0Pin8SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin8SecMask {
        match self.bits {
            false => Pio0Pin8SecMask::Blocked,
            true => Pio0Pin8SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin8SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin8SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN8_SEC_MASK` writer - Secure mask for pin P0_8"]
pub type Pio0Pin8SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin8SecMask>;
impl<'a, REG> Pio0Pin8SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin8SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin8SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin9SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin9SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin9SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN9_SEC_MASK` reader - Secure mask for pin P0_9"]
pub type Pio0Pin9SecMaskR = crate::BitReader<Pio0Pin9SecMask>;
impl Pio0Pin9SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin9SecMask {
        match self.bits {
            false => Pio0Pin9SecMask::Blocked,
            true => Pio0Pin9SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin9SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin9SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN9_SEC_MASK` writer - Secure mask for pin P0_9"]
pub type Pio0Pin9SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin9SecMask>;
impl<'a, REG> Pio0Pin9SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin9SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin9SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin10SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin10SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin10SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN10_SEC_MASK` reader - Secure mask for pin P0_10"]
pub type Pio0Pin10SecMaskR = crate::BitReader<Pio0Pin10SecMask>;
impl Pio0Pin10SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin10SecMask {
        match self.bits {
            false => Pio0Pin10SecMask::Blocked,
            true => Pio0Pin10SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin10SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin10SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN10_SEC_MASK` writer - Secure mask for pin P0_10"]
pub type Pio0Pin10SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin10SecMask>;
impl<'a, REG> Pio0Pin10SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin10SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin10SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin11SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin11SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin11SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN11_SEC_MASK` reader - Secure mask for pin P0_11"]
pub type Pio0Pin11SecMaskR = crate::BitReader<Pio0Pin11SecMask>;
impl Pio0Pin11SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin11SecMask {
        match self.bits {
            false => Pio0Pin11SecMask::Blocked,
            true => Pio0Pin11SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin11SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin11SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN11_SEC_MASK` writer - Secure mask for pin P0_11"]
pub type Pio0Pin11SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin11SecMask>;
impl<'a, REG> Pio0Pin11SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin11SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin11SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin12SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin12SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin12SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN12_SEC_MASK` reader - Secure mask for pin P0_12"]
pub type Pio0Pin12SecMaskR = crate::BitReader<Pio0Pin12SecMask>;
impl Pio0Pin12SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin12SecMask {
        match self.bits {
            false => Pio0Pin12SecMask::Blocked,
            true => Pio0Pin12SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin12SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin12SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN12_SEC_MASK` writer - Secure mask for pin P0_12"]
pub type Pio0Pin12SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin12SecMask>;
impl<'a, REG> Pio0Pin12SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin12SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin12SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin13SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin13SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin13SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN13_SEC_MASK` reader - Secure mask for pin P0_13"]
pub type Pio0Pin13SecMaskR = crate::BitReader<Pio0Pin13SecMask>;
impl Pio0Pin13SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin13SecMask {
        match self.bits {
            false => Pio0Pin13SecMask::Blocked,
            true => Pio0Pin13SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin13SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin13SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN13_SEC_MASK` writer - Secure mask for pin P0_13"]
pub type Pio0Pin13SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin13SecMask>;
impl<'a, REG> Pio0Pin13SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin13SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin13SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin14SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin14SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin14SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN14_SEC_MASK` reader - Secure mask for pin P0_14"]
pub type Pio0Pin14SecMaskR = crate::BitReader<Pio0Pin14SecMask>;
impl Pio0Pin14SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin14SecMask {
        match self.bits {
            false => Pio0Pin14SecMask::Blocked,
            true => Pio0Pin14SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin14SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin14SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN14_SEC_MASK` writer - Secure mask for pin P0_14"]
pub type Pio0Pin14SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin14SecMask>;
impl<'a, REG> Pio0Pin14SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin14SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin14SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin15SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin15SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin15SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN15_SEC_MASK` reader - Secure mask for pin P0_15"]
pub type Pio0Pin15SecMaskR = crate::BitReader<Pio0Pin15SecMask>;
impl Pio0Pin15SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin15SecMask {
        match self.bits {
            false => Pio0Pin15SecMask::Blocked,
            true => Pio0Pin15SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin15SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin15SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN15_SEC_MASK` writer - Secure mask for pin P0_15"]
pub type Pio0Pin15SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin15SecMask>;
impl<'a, REG> Pio0Pin15SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin15SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin15SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin16SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin16SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin16SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN16_SEC_MASK` reader - Secure mask for pin P0_16"]
pub type Pio0Pin16SecMaskR = crate::BitReader<Pio0Pin16SecMask>;
impl Pio0Pin16SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin16SecMask {
        match self.bits {
            false => Pio0Pin16SecMask::Blocked,
            true => Pio0Pin16SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin16SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin16SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN16_SEC_MASK` writer - Secure mask for pin P0_16"]
pub type Pio0Pin16SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin16SecMask>;
impl<'a, REG> Pio0Pin16SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin16SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin16SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin17SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin17SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin17SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN17_SEC_MASK` reader - Secure mask for pin P0_17"]
pub type Pio0Pin17SecMaskR = crate::BitReader<Pio0Pin17SecMask>;
impl Pio0Pin17SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin17SecMask {
        match self.bits {
            false => Pio0Pin17SecMask::Blocked,
            true => Pio0Pin17SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin17SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin17SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN17_SEC_MASK` writer - Secure mask for pin P0_17"]
pub type Pio0Pin17SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin17SecMask>;
impl<'a, REG> Pio0Pin17SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin17SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin17SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin18SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin18SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin18SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN18_SEC_MASK` reader - Secure mask for pin P0_18"]
pub type Pio0Pin18SecMaskR = crate::BitReader<Pio0Pin18SecMask>;
impl Pio0Pin18SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin18SecMask {
        match self.bits {
            false => Pio0Pin18SecMask::Blocked,
            true => Pio0Pin18SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin18SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin18SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN18_SEC_MASK` writer - Secure mask for pin P0_18"]
pub type Pio0Pin18SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin18SecMask>;
impl<'a, REG> Pio0Pin18SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin18SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin18SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin19SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin19SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin19SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN19_SEC_MASK` reader - Secure mask for pin P0_19"]
pub type Pio0Pin19SecMaskR = crate::BitReader<Pio0Pin19SecMask>;
impl Pio0Pin19SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin19SecMask {
        match self.bits {
            false => Pio0Pin19SecMask::Blocked,
            true => Pio0Pin19SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin19SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin19SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN19_SEC_MASK` writer - Secure mask for pin P0_19"]
pub type Pio0Pin19SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin19SecMask>;
impl<'a, REG> Pio0Pin19SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin19SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin19SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin20SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin20SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin20SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN20_SEC_MASK` reader - Secure mask for pin P0_20"]
pub type Pio0Pin20SecMaskR = crate::BitReader<Pio0Pin20SecMask>;
impl Pio0Pin20SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin20SecMask {
        match self.bits {
            false => Pio0Pin20SecMask::Blocked,
            true => Pio0Pin20SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin20SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin20SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN20_SEC_MASK` writer - Secure mask for pin P0_20"]
pub type Pio0Pin20SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin20SecMask>;
impl<'a, REG> Pio0Pin20SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin20SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin20SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin21SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin21SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin21SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN21_SEC_MASK` reader - Secure mask for pin P0_21"]
pub type Pio0Pin21SecMaskR = crate::BitReader<Pio0Pin21SecMask>;
impl Pio0Pin21SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin21SecMask {
        match self.bits {
            false => Pio0Pin21SecMask::Blocked,
            true => Pio0Pin21SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin21SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin21SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN21_SEC_MASK` writer - Secure mask for pin P0_21"]
pub type Pio0Pin21SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin21SecMask>;
impl<'a, REG> Pio0Pin21SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin21SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin21SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin22SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin22SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin22SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN22_SEC_MASK` reader - Secure mask for pin P0_22"]
pub type Pio0Pin22SecMaskR = crate::BitReader<Pio0Pin22SecMask>;
impl Pio0Pin22SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin22SecMask {
        match self.bits {
            false => Pio0Pin22SecMask::Blocked,
            true => Pio0Pin22SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin22SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin22SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN22_SEC_MASK` writer - Secure mask for pin P0_22"]
pub type Pio0Pin22SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin22SecMask>;
impl<'a, REG> Pio0Pin22SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin22SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin22SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin23SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin23SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin23SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN23_SEC_MASK` reader - Secure mask for pin P0_23"]
pub type Pio0Pin23SecMaskR = crate::BitReader<Pio0Pin23SecMask>;
impl Pio0Pin23SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin23SecMask {
        match self.bits {
            false => Pio0Pin23SecMask::Blocked,
            true => Pio0Pin23SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin23SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin23SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN23_SEC_MASK` writer - Secure mask for pin P0_23"]
pub type Pio0Pin23SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin23SecMask>;
impl<'a, REG> Pio0Pin23SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin23SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin23SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin24SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin24SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin24SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN24_SEC_MASK` reader - Secure mask for pin P0_24"]
pub type Pio0Pin24SecMaskR = crate::BitReader<Pio0Pin24SecMask>;
impl Pio0Pin24SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin24SecMask {
        match self.bits {
            false => Pio0Pin24SecMask::Blocked,
            true => Pio0Pin24SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin24SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin24SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN24_SEC_MASK` writer - Secure mask for pin P0_24"]
pub type Pio0Pin24SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin24SecMask>;
impl<'a, REG> Pio0Pin24SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin24SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin24SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin25SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin25SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin25SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN25_SEC_MASK` reader - Secure mask for pin P0_25"]
pub type Pio0Pin25SecMaskR = crate::BitReader<Pio0Pin25SecMask>;
impl Pio0Pin25SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin25SecMask {
        match self.bits {
            false => Pio0Pin25SecMask::Blocked,
            true => Pio0Pin25SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin25SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin25SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN25_SEC_MASK` writer - Secure mask for pin P0_25"]
pub type Pio0Pin25SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin25SecMask>;
impl<'a, REG> Pio0Pin25SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin25SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin25SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin26SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin26SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin26SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN26_SEC_MASK` reader - Secure mask for pin P0_26"]
pub type Pio0Pin26SecMaskR = crate::BitReader<Pio0Pin26SecMask>;
impl Pio0Pin26SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin26SecMask {
        match self.bits {
            false => Pio0Pin26SecMask::Blocked,
            true => Pio0Pin26SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin26SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin26SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN26_SEC_MASK` writer - Secure mask for pin P0_26"]
pub type Pio0Pin26SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin26SecMask>;
impl<'a, REG> Pio0Pin26SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin26SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin26SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin27SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin27SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin27SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN27_SEC_MASK` reader - Secure mask for pin P0_27"]
pub type Pio0Pin27SecMaskR = crate::BitReader<Pio0Pin27SecMask>;
impl Pio0Pin27SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin27SecMask {
        match self.bits {
            false => Pio0Pin27SecMask::Blocked,
            true => Pio0Pin27SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin27SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin27SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN27_SEC_MASK` writer - Secure mask for pin P0_27"]
pub type Pio0Pin27SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin27SecMask>;
impl<'a, REG> Pio0Pin27SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin27SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin27SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin28SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin28SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin28SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN28_SEC_MASK` reader - Secure mask for pin P0_28"]
pub type Pio0Pin28SecMaskR = crate::BitReader<Pio0Pin28SecMask>;
impl Pio0Pin28SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin28SecMask {
        match self.bits {
            false => Pio0Pin28SecMask::Blocked,
            true => Pio0Pin28SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin28SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin28SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN28_SEC_MASK` writer - Secure mask for pin P0_28"]
pub type Pio0Pin28SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin28SecMask>;
impl<'a, REG> Pio0Pin28SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin28SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin28SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin29SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin29SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin29SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN29_SEC_MASK` reader - Secure mask for pin P0_29"]
pub type Pio0Pin29SecMaskR = crate::BitReader<Pio0Pin29SecMask>;
impl Pio0Pin29SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin29SecMask {
        match self.bits {
            false => Pio0Pin29SecMask::Blocked,
            true => Pio0Pin29SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin29SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin29SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN29_SEC_MASK` writer - Secure mask for pin P0_29"]
pub type Pio0Pin29SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin29SecMask>;
impl<'a, REG> Pio0Pin29SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin29SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin29SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin30SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin30SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin30SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN30_SEC_MASK` reader - Secure mask for pin P0_30"]
pub type Pio0Pin30SecMaskR = crate::BitReader<Pio0Pin30SecMask>;
impl Pio0Pin30SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin30SecMask {
        match self.bits {
            false => Pio0Pin30SecMask::Blocked,
            true => Pio0Pin30SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin30SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin30SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN30_SEC_MASK` writer - Secure mask for pin P0_30"]
pub type Pio0Pin30SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin30SecMask>;
impl<'a, REG> Pio0Pin30SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin30SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin30SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P0_31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio0Pin31SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio0Pin31SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio0Pin31SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN31_SEC_MASK` reader - Secure mask for pin P0_31"]
pub type Pio0Pin31SecMaskR = crate::BitReader<Pio0Pin31SecMask>;
impl Pio0Pin31SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio0Pin31SecMask {
        match self.bits {
            false => Pio0Pin31SecMask::Blocked,
            true => Pio0Pin31SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio0Pin31SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio0Pin31SecMask::Readable
    }
}
#[doc = "Field `PIO0_PIN31_SEC_MASK` writer - Secure mask for pin P0_31"]
pub type Pio0Pin31SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio0Pin31SecMask>;
impl<'a, REG> Pio0Pin31SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin31SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio0Pin31SecMask::Readable)
    }
}
impl R {
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline(always)]
    pub fn pio0_pin0_sec_mask(&self) -> Pio0Pin0SecMaskR {
        Pio0Pin0SecMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline(always)]
    pub fn pio0_pin1_sec_mask(&self) -> Pio0Pin1SecMaskR {
        Pio0Pin1SecMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline(always)]
    pub fn pio0_pin2_sec_mask(&self) -> Pio0Pin2SecMaskR {
        Pio0Pin2SecMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline(always)]
    pub fn pio0_pin3_sec_mask(&self) -> Pio0Pin3SecMaskR {
        Pio0Pin3SecMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline(always)]
    pub fn pio0_pin4_sec_mask(&self) -> Pio0Pin4SecMaskR {
        Pio0Pin4SecMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline(always)]
    pub fn pio0_pin5_sec_mask(&self) -> Pio0Pin5SecMaskR {
        Pio0Pin5SecMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline(always)]
    pub fn pio0_pin6_sec_mask(&self) -> Pio0Pin6SecMaskR {
        Pio0Pin6SecMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline(always)]
    pub fn pio0_pin7_sec_mask(&self) -> Pio0Pin7SecMaskR {
        Pio0Pin7SecMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline(always)]
    pub fn pio0_pin8_sec_mask(&self) -> Pio0Pin8SecMaskR {
        Pio0Pin8SecMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline(always)]
    pub fn pio0_pin9_sec_mask(&self) -> Pio0Pin9SecMaskR {
        Pio0Pin9SecMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline(always)]
    pub fn pio0_pin10_sec_mask(&self) -> Pio0Pin10SecMaskR {
        Pio0Pin10SecMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline(always)]
    pub fn pio0_pin11_sec_mask(&self) -> Pio0Pin11SecMaskR {
        Pio0Pin11SecMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline(always)]
    pub fn pio0_pin12_sec_mask(&self) -> Pio0Pin12SecMaskR {
        Pio0Pin12SecMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline(always)]
    pub fn pio0_pin13_sec_mask(&self) -> Pio0Pin13SecMaskR {
        Pio0Pin13SecMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline(always)]
    pub fn pio0_pin14_sec_mask(&self) -> Pio0Pin14SecMaskR {
        Pio0Pin14SecMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline(always)]
    pub fn pio0_pin15_sec_mask(&self) -> Pio0Pin15SecMaskR {
        Pio0Pin15SecMaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline(always)]
    pub fn pio0_pin16_sec_mask(&self) -> Pio0Pin16SecMaskR {
        Pio0Pin16SecMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline(always)]
    pub fn pio0_pin17_sec_mask(&self) -> Pio0Pin17SecMaskR {
        Pio0Pin17SecMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline(always)]
    pub fn pio0_pin18_sec_mask(&self) -> Pio0Pin18SecMaskR {
        Pio0Pin18SecMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline(always)]
    pub fn pio0_pin19_sec_mask(&self) -> Pio0Pin19SecMaskR {
        Pio0Pin19SecMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline(always)]
    pub fn pio0_pin20_sec_mask(&self) -> Pio0Pin20SecMaskR {
        Pio0Pin20SecMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline(always)]
    pub fn pio0_pin21_sec_mask(&self) -> Pio0Pin21SecMaskR {
        Pio0Pin21SecMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline(always)]
    pub fn pio0_pin22_sec_mask(&self) -> Pio0Pin22SecMaskR {
        Pio0Pin22SecMaskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline(always)]
    pub fn pio0_pin23_sec_mask(&self) -> Pio0Pin23SecMaskR {
        Pio0Pin23SecMaskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline(always)]
    pub fn pio0_pin24_sec_mask(&self) -> Pio0Pin24SecMaskR {
        Pio0Pin24SecMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline(always)]
    pub fn pio0_pin25_sec_mask(&self) -> Pio0Pin25SecMaskR {
        Pio0Pin25SecMaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline(always)]
    pub fn pio0_pin26_sec_mask(&self) -> Pio0Pin26SecMaskR {
        Pio0Pin26SecMaskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline(always)]
    pub fn pio0_pin27_sec_mask(&self) -> Pio0Pin27SecMaskR {
        Pio0Pin27SecMaskR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline(always)]
    pub fn pio0_pin28_sec_mask(&self) -> Pio0Pin28SecMaskR {
        Pio0Pin28SecMaskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline(always)]
    pub fn pio0_pin29_sec_mask(&self) -> Pio0Pin29SecMaskR {
        Pio0Pin29SecMaskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline(always)]
    pub fn pio0_pin30_sec_mask(&self) -> Pio0Pin30SecMaskR {
        Pio0Pin30SecMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline(always)]
    pub fn pio0_pin31_sec_mask(&self) -> Pio0Pin31SecMaskR {
        Pio0Pin31SecMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline(always)]
    pub fn pio0_pin0_sec_mask(&mut self) -> Pio0Pin0SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin0SecMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline(always)]
    pub fn pio0_pin1_sec_mask(&mut self) -> Pio0Pin1SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin1SecMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline(always)]
    pub fn pio0_pin2_sec_mask(&mut self) -> Pio0Pin2SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin2SecMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline(always)]
    pub fn pio0_pin3_sec_mask(&mut self) -> Pio0Pin3SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin3SecMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline(always)]
    pub fn pio0_pin4_sec_mask(&mut self) -> Pio0Pin4SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin4SecMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline(always)]
    pub fn pio0_pin5_sec_mask(&mut self) -> Pio0Pin5SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin5SecMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline(always)]
    pub fn pio0_pin6_sec_mask(&mut self) -> Pio0Pin6SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin6SecMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline(always)]
    pub fn pio0_pin7_sec_mask(&mut self) -> Pio0Pin7SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin7SecMaskW::new(self, 7)
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline(always)]
    pub fn pio0_pin8_sec_mask(&mut self) -> Pio0Pin8SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin8SecMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline(always)]
    pub fn pio0_pin9_sec_mask(&mut self) -> Pio0Pin9SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin9SecMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline(always)]
    pub fn pio0_pin10_sec_mask(&mut self) -> Pio0Pin10SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin10SecMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline(always)]
    pub fn pio0_pin11_sec_mask(&mut self) -> Pio0Pin11SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin11SecMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline(always)]
    pub fn pio0_pin12_sec_mask(&mut self) -> Pio0Pin12SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin12SecMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline(always)]
    pub fn pio0_pin13_sec_mask(&mut self) -> Pio0Pin13SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin13SecMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline(always)]
    pub fn pio0_pin14_sec_mask(&mut self) -> Pio0Pin14SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin14SecMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline(always)]
    pub fn pio0_pin15_sec_mask(&mut self) -> Pio0Pin15SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin15SecMaskW::new(self, 15)
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline(always)]
    pub fn pio0_pin16_sec_mask(&mut self) -> Pio0Pin16SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin16SecMaskW::new(self, 16)
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline(always)]
    pub fn pio0_pin17_sec_mask(&mut self) -> Pio0Pin17SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin17SecMaskW::new(self, 17)
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline(always)]
    pub fn pio0_pin18_sec_mask(&mut self) -> Pio0Pin18SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin18SecMaskW::new(self, 18)
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline(always)]
    pub fn pio0_pin19_sec_mask(&mut self) -> Pio0Pin19SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin19SecMaskW::new(self, 19)
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline(always)]
    pub fn pio0_pin20_sec_mask(&mut self) -> Pio0Pin20SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin20SecMaskW::new(self, 20)
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline(always)]
    pub fn pio0_pin21_sec_mask(&mut self) -> Pio0Pin21SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin21SecMaskW::new(self, 21)
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline(always)]
    pub fn pio0_pin22_sec_mask(&mut self) -> Pio0Pin22SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin22SecMaskW::new(self, 22)
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline(always)]
    pub fn pio0_pin23_sec_mask(&mut self) -> Pio0Pin23SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin23SecMaskW::new(self, 23)
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline(always)]
    pub fn pio0_pin24_sec_mask(&mut self) -> Pio0Pin24SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin24SecMaskW::new(self, 24)
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline(always)]
    pub fn pio0_pin25_sec_mask(&mut self) -> Pio0Pin25SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin25SecMaskW::new(self, 25)
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline(always)]
    pub fn pio0_pin26_sec_mask(&mut self) -> Pio0Pin26SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin26SecMaskW::new(self, 26)
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline(always)]
    pub fn pio0_pin27_sec_mask(&mut self) -> Pio0Pin27SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin27SecMaskW::new(self, 27)
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline(always)]
    pub fn pio0_pin28_sec_mask(&mut self) -> Pio0Pin28SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin28SecMaskW::new(self, 28)
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline(always)]
    pub fn pio0_pin29_sec_mask(&mut self) -> Pio0Pin29SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin29SecMaskW::new(self, 29)
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline(always)]
    pub fn pio0_pin30_sec_mask(&mut self) -> Pio0Pin30SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin30SecMaskW::new(self, 30)
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline(always)]
    pub fn pio0_pin31_sec_mask(&mut self) -> Pio0Pin31SecMaskW<'_, SecGpioMask0Spec> {
        Pio0Pin31SecMaskW::new(self, 31)
    }
}
#[doc = "Secure GPIO mask for port 0 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecGpioMask0Spec;
impl crate::RegisterSpec for SecGpioMask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_gpio_mask0::R`](R) reader structure"]
impl crate::Readable for SecGpioMask0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_gpio_mask0::W`](W) writer structure"]
impl crate::Writable for SecGpioMask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_GPIO_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for SecGpioMask0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

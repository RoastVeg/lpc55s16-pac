#[doc = "Register `SEC_GPIO_MASK1` reader"]
pub type R = crate::R<SecGpioMask1Spec>;
#[doc = "Register `SEC_GPIO_MASK1` writer"]
pub type W = crate::W<SecGpioMask1Spec>;
#[doc = "Secure mask for pin P1_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin0SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin0SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin0SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN0_SEC_MASK` reader - Secure mask for pin P1_0"]
pub type Pio1Pin0SecMaskR = crate::BitReader<Pio1Pin0SecMask>;
impl Pio1Pin0SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin0SecMask {
        match self.bits {
            false => Pio1Pin0SecMask::Blocked,
            true => Pio1Pin0SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin0SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin0SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN0_SEC_MASK` writer - Secure mask for pin P1_0"]
pub type Pio1Pin0SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin0SecMask>;
impl<'a, REG> Pio1Pin0SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin0SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin0SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin1SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin1SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin1SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN1_SEC_MASK` reader - Secure mask for pin P1_1"]
pub type Pio1Pin1SecMaskR = crate::BitReader<Pio1Pin1SecMask>;
impl Pio1Pin1SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin1SecMask {
        match self.bits {
            false => Pio1Pin1SecMask::Blocked,
            true => Pio1Pin1SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin1SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin1SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN1_SEC_MASK` writer - Secure mask for pin P1_1"]
pub type Pio1Pin1SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin1SecMask>;
impl<'a, REG> Pio1Pin1SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin1SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin1SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin2SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin2SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin2SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN2_SEC_MASK` reader - Secure mask for pin P1_2"]
pub type Pio1Pin2SecMaskR = crate::BitReader<Pio1Pin2SecMask>;
impl Pio1Pin2SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin2SecMask {
        match self.bits {
            false => Pio1Pin2SecMask::Blocked,
            true => Pio1Pin2SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin2SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin2SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN2_SEC_MASK` writer - Secure mask for pin P1_2"]
pub type Pio1Pin2SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin2SecMask>;
impl<'a, REG> Pio1Pin2SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin2SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin2SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin3SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin3SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin3SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN3_SEC_MASK` reader - Secure mask for pin P1_3"]
pub type Pio1Pin3SecMaskR = crate::BitReader<Pio1Pin3SecMask>;
impl Pio1Pin3SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin3SecMask {
        match self.bits {
            false => Pio1Pin3SecMask::Blocked,
            true => Pio1Pin3SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin3SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin3SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN3_SEC_MASK` writer - Secure mask for pin P1_3"]
pub type Pio1Pin3SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin3SecMask>;
impl<'a, REG> Pio1Pin3SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin3SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin3SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin4SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin4SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin4SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN4_SEC_MASK` reader - Secure mask for pin P1_4"]
pub type Pio1Pin4SecMaskR = crate::BitReader<Pio1Pin4SecMask>;
impl Pio1Pin4SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin4SecMask {
        match self.bits {
            false => Pio1Pin4SecMask::Blocked,
            true => Pio1Pin4SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin4SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin4SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN4_SEC_MASK` writer - Secure mask for pin P1_4"]
pub type Pio1Pin4SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin4SecMask>;
impl<'a, REG> Pio1Pin4SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin4SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin4SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin5SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin5SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin5SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN5_SEC_MASK` reader - Secure mask for pin P1_5"]
pub type Pio1Pin5SecMaskR = crate::BitReader<Pio1Pin5SecMask>;
impl Pio1Pin5SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin5SecMask {
        match self.bits {
            false => Pio1Pin5SecMask::Blocked,
            true => Pio1Pin5SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin5SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin5SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN5_SEC_MASK` writer - Secure mask for pin P1_5"]
pub type Pio1Pin5SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin5SecMask>;
impl<'a, REG> Pio1Pin5SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin5SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin5SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin6SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin6SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin6SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN6_SEC_MASK` reader - Secure mask for pin P1_6"]
pub type Pio1Pin6SecMaskR = crate::BitReader<Pio1Pin6SecMask>;
impl Pio1Pin6SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin6SecMask {
        match self.bits {
            false => Pio1Pin6SecMask::Blocked,
            true => Pio1Pin6SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin6SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin6SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN6_SEC_MASK` writer - Secure mask for pin P1_6"]
pub type Pio1Pin6SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin6SecMask>;
impl<'a, REG> Pio1Pin6SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin6SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin6SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin7SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin7SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin7SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN7_SEC_MASK` reader - Secure mask for pin P1_7"]
pub type Pio1Pin7SecMaskR = crate::BitReader<Pio1Pin7SecMask>;
impl Pio1Pin7SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin7SecMask {
        match self.bits {
            false => Pio1Pin7SecMask::Blocked,
            true => Pio1Pin7SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin7SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin7SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN7_SEC_MASK` writer - Secure mask for pin P1_7"]
pub type Pio1Pin7SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin7SecMask>;
impl<'a, REG> Pio1Pin7SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin7SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin7SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin8SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin8SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin8SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN8_SEC_MASK` reader - Secure mask for pin P1_8"]
pub type Pio1Pin8SecMaskR = crate::BitReader<Pio1Pin8SecMask>;
impl Pio1Pin8SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin8SecMask {
        match self.bits {
            false => Pio1Pin8SecMask::Blocked,
            true => Pio1Pin8SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin8SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin8SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN8_SEC_MASK` writer - Secure mask for pin P1_8"]
pub type Pio1Pin8SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin8SecMask>;
impl<'a, REG> Pio1Pin8SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin8SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin8SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin9SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin9SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin9SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN9_SEC_MASK` reader - Secure mask for pin P1_9"]
pub type Pio1Pin9SecMaskR = crate::BitReader<Pio1Pin9SecMask>;
impl Pio1Pin9SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin9SecMask {
        match self.bits {
            false => Pio1Pin9SecMask::Blocked,
            true => Pio1Pin9SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin9SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin9SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN9_SEC_MASK` writer - Secure mask for pin P1_9"]
pub type Pio1Pin9SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin9SecMask>;
impl<'a, REG> Pio1Pin9SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin9SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin9SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin10SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin10SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin10SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN10_SEC_MASK` reader - Secure mask for pin P1_10"]
pub type Pio1Pin10SecMaskR = crate::BitReader<Pio1Pin10SecMask>;
impl Pio1Pin10SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin10SecMask {
        match self.bits {
            false => Pio1Pin10SecMask::Blocked,
            true => Pio1Pin10SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin10SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin10SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN10_SEC_MASK` writer - Secure mask for pin P1_10"]
pub type Pio1Pin10SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin10SecMask>;
impl<'a, REG> Pio1Pin10SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin10SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin10SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin11SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin11SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin11SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN11_SEC_MASK` reader - Secure mask for pin P1_11"]
pub type Pio1Pin11SecMaskR = crate::BitReader<Pio1Pin11SecMask>;
impl Pio1Pin11SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin11SecMask {
        match self.bits {
            false => Pio1Pin11SecMask::Blocked,
            true => Pio1Pin11SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin11SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin11SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN11_SEC_MASK` writer - Secure mask for pin P1_11"]
pub type Pio1Pin11SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin11SecMask>;
impl<'a, REG> Pio1Pin11SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin11SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin11SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin12SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin12SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin12SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN12_SEC_MASK` reader - Secure mask for pin P1_12"]
pub type Pio1Pin12SecMaskR = crate::BitReader<Pio1Pin12SecMask>;
impl Pio1Pin12SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin12SecMask {
        match self.bits {
            false => Pio1Pin12SecMask::Blocked,
            true => Pio1Pin12SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin12SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin12SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN12_SEC_MASK` writer - Secure mask for pin P1_12"]
pub type Pio1Pin12SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin12SecMask>;
impl<'a, REG> Pio1Pin12SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin12SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin12SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin13SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin13SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin13SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN13_SEC_MASK` reader - Secure mask for pin P1_13"]
pub type Pio1Pin13SecMaskR = crate::BitReader<Pio1Pin13SecMask>;
impl Pio1Pin13SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin13SecMask {
        match self.bits {
            false => Pio1Pin13SecMask::Blocked,
            true => Pio1Pin13SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin13SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin13SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN13_SEC_MASK` writer - Secure mask for pin P1_13"]
pub type Pio1Pin13SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin13SecMask>;
impl<'a, REG> Pio1Pin13SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin13SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin13SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin14SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin14SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin14SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN14_SEC_MASK` reader - Secure mask for pin P1_14"]
pub type Pio1Pin14SecMaskR = crate::BitReader<Pio1Pin14SecMask>;
impl Pio1Pin14SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin14SecMask {
        match self.bits {
            false => Pio1Pin14SecMask::Blocked,
            true => Pio1Pin14SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin14SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin14SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN14_SEC_MASK` writer - Secure mask for pin P1_14"]
pub type Pio1Pin14SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin14SecMask>;
impl<'a, REG> Pio1Pin14SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin14SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin14SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin15SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin15SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin15SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN15_SEC_MASK` reader - Secure mask for pin P1_15"]
pub type Pio1Pin15SecMaskR = crate::BitReader<Pio1Pin15SecMask>;
impl Pio1Pin15SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin15SecMask {
        match self.bits {
            false => Pio1Pin15SecMask::Blocked,
            true => Pio1Pin15SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin15SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin15SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN15_SEC_MASK` writer - Secure mask for pin P1_15"]
pub type Pio1Pin15SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin15SecMask>;
impl<'a, REG> Pio1Pin15SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin15SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin15SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin16SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin16SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin16SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN16_SEC_MASK` reader - Secure mask for pin P1_16"]
pub type Pio1Pin16SecMaskR = crate::BitReader<Pio1Pin16SecMask>;
impl Pio1Pin16SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin16SecMask {
        match self.bits {
            false => Pio1Pin16SecMask::Blocked,
            true => Pio1Pin16SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin16SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin16SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN16_SEC_MASK` writer - Secure mask for pin P1_16"]
pub type Pio1Pin16SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin16SecMask>;
impl<'a, REG> Pio1Pin16SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin16SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin16SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin17SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin17SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin17SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN17_SEC_MASK` reader - Secure mask for pin P1_17"]
pub type Pio1Pin17SecMaskR = crate::BitReader<Pio1Pin17SecMask>;
impl Pio1Pin17SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin17SecMask {
        match self.bits {
            false => Pio1Pin17SecMask::Blocked,
            true => Pio1Pin17SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin17SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin17SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN17_SEC_MASK` writer - Secure mask for pin P1_17"]
pub type Pio1Pin17SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin17SecMask>;
impl<'a, REG> Pio1Pin17SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin17SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin17SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin18SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin18SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin18SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN18_SEC_MASK` reader - Secure mask for pin P1_18"]
pub type Pio1Pin18SecMaskR = crate::BitReader<Pio1Pin18SecMask>;
impl Pio1Pin18SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin18SecMask {
        match self.bits {
            false => Pio1Pin18SecMask::Blocked,
            true => Pio1Pin18SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin18SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin18SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN18_SEC_MASK` writer - Secure mask for pin P1_18"]
pub type Pio1Pin18SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin18SecMask>;
impl<'a, REG> Pio1Pin18SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin18SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin18SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin19SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin19SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin19SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN19_SEC_MASK` reader - Secure mask for pin P1_19"]
pub type Pio1Pin19SecMaskR = crate::BitReader<Pio1Pin19SecMask>;
impl Pio1Pin19SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin19SecMask {
        match self.bits {
            false => Pio1Pin19SecMask::Blocked,
            true => Pio1Pin19SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin19SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin19SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN19_SEC_MASK` writer - Secure mask for pin P1_19"]
pub type Pio1Pin19SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin19SecMask>;
impl<'a, REG> Pio1Pin19SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin19SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin19SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin20SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin20SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin20SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN20_SEC_MASK` reader - Secure mask for pin P1_20"]
pub type Pio1Pin20SecMaskR = crate::BitReader<Pio1Pin20SecMask>;
impl Pio1Pin20SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin20SecMask {
        match self.bits {
            false => Pio1Pin20SecMask::Blocked,
            true => Pio1Pin20SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin20SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin20SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN20_SEC_MASK` writer - Secure mask for pin P1_20"]
pub type Pio1Pin20SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin20SecMask>;
impl<'a, REG> Pio1Pin20SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin20SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin20SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin21SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin21SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin21SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN21_SEC_MASK` reader - Secure mask for pin P1_21"]
pub type Pio1Pin21SecMaskR = crate::BitReader<Pio1Pin21SecMask>;
impl Pio1Pin21SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin21SecMask {
        match self.bits {
            false => Pio1Pin21SecMask::Blocked,
            true => Pio1Pin21SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin21SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin21SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN21_SEC_MASK` writer - Secure mask for pin P1_21"]
pub type Pio1Pin21SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin21SecMask>;
impl<'a, REG> Pio1Pin21SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin21SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin21SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin22SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin22SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin22SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN22_SEC_MASK` reader - Secure mask for pin P1_22"]
pub type Pio1Pin22SecMaskR = crate::BitReader<Pio1Pin22SecMask>;
impl Pio1Pin22SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin22SecMask {
        match self.bits {
            false => Pio1Pin22SecMask::Blocked,
            true => Pio1Pin22SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin22SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin22SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN22_SEC_MASK` writer - Secure mask for pin P1_22"]
pub type Pio1Pin22SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin22SecMask>;
impl<'a, REG> Pio1Pin22SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin22SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin22SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin23SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin23SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin23SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN23_SEC_MASK` reader - Secure mask for pin P1_23"]
pub type Pio1Pin23SecMaskR = crate::BitReader<Pio1Pin23SecMask>;
impl Pio1Pin23SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin23SecMask {
        match self.bits {
            false => Pio1Pin23SecMask::Blocked,
            true => Pio1Pin23SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin23SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin23SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN23_SEC_MASK` writer - Secure mask for pin P1_23"]
pub type Pio1Pin23SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin23SecMask>;
impl<'a, REG> Pio1Pin23SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin23SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin23SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin24SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin24SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin24SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN24_SEC_MASK` reader - Secure mask for pin P1_24"]
pub type Pio1Pin24SecMaskR = crate::BitReader<Pio1Pin24SecMask>;
impl Pio1Pin24SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin24SecMask {
        match self.bits {
            false => Pio1Pin24SecMask::Blocked,
            true => Pio1Pin24SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin24SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin24SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN24_SEC_MASK` writer - Secure mask for pin P1_24"]
pub type Pio1Pin24SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin24SecMask>;
impl<'a, REG> Pio1Pin24SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin24SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin24SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin25SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin25SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin25SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN25_SEC_MASK` reader - Secure mask for pin P1_25"]
pub type Pio1Pin25SecMaskR = crate::BitReader<Pio1Pin25SecMask>;
impl Pio1Pin25SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin25SecMask {
        match self.bits {
            false => Pio1Pin25SecMask::Blocked,
            true => Pio1Pin25SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin25SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin25SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN25_SEC_MASK` writer - Secure mask for pin P1_25"]
pub type Pio1Pin25SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin25SecMask>;
impl<'a, REG> Pio1Pin25SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin25SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin25SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin26SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin26SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin26SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN26_SEC_MASK` reader - Secure mask for pin P1_26"]
pub type Pio1Pin26SecMaskR = crate::BitReader<Pio1Pin26SecMask>;
impl Pio1Pin26SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin26SecMask {
        match self.bits {
            false => Pio1Pin26SecMask::Blocked,
            true => Pio1Pin26SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin26SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin26SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN26_SEC_MASK` writer - Secure mask for pin P1_26"]
pub type Pio1Pin26SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin26SecMask>;
impl<'a, REG> Pio1Pin26SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin26SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin26SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin27SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin27SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin27SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN27_SEC_MASK` reader - Secure mask for pin P1_27"]
pub type Pio1Pin27SecMaskR = crate::BitReader<Pio1Pin27SecMask>;
impl Pio1Pin27SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin27SecMask {
        match self.bits {
            false => Pio1Pin27SecMask::Blocked,
            true => Pio1Pin27SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin27SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin27SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN27_SEC_MASK` writer - Secure mask for pin P1_27"]
pub type Pio1Pin27SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin27SecMask>;
impl<'a, REG> Pio1Pin27SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin27SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin27SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin28SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin28SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin28SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN28_SEC_MASK` reader - Secure mask for pin P1_28"]
pub type Pio1Pin28SecMaskR = crate::BitReader<Pio1Pin28SecMask>;
impl Pio1Pin28SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin28SecMask {
        match self.bits {
            false => Pio1Pin28SecMask::Blocked,
            true => Pio1Pin28SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin28SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin28SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN28_SEC_MASK` writer - Secure mask for pin P1_28"]
pub type Pio1Pin28SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin28SecMask>;
impl<'a, REG> Pio1Pin28SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin28SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin28SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin29SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin29SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin29SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN29_SEC_MASK` reader - Secure mask for pin P1_29"]
pub type Pio1Pin29SecMaskR = crate::BitReader<Pio1Pin29SecMask>;
impl Pio1Pin29SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin29SecMask {
        match self.bits {
            false => Pio1Pin29SecMask::Blocked,
            true => Pio1Pin29SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin29SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin29SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN29_SEC_MASK` writer - Secure mask for pin P1_29"]
pub type Pio1Pin29SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin29SecMask>;
impl<'a, REG> Pio1Pin29SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin29SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin29SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin30SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin30SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin30SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN30_SEC_MASK` reader - Secure mask for pin P1_30"]
pub type Pio1Pin30SecMaskR = crate::BitReader<Pio1Pin30SecMask>;
impl Pio1Pin30SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin30SecMask {
        match self.bits {
            false => Pio1Pin30SecMask::Blocked,
            true => Pio1Pin30SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin30SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin30SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN30_SEC_MASK` writer - Secure mask for pin P1_30"]
pub type Pio1Pin30SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin30SecMask>;
impl<'a, REG> Pio1Pin30SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin30SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin30SecMask::Readable)
    }
}
#[doc = "Secure mask for pin P1_31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pio1Pin31SecMask {
    #[doc = "0: Pin state is blocked to non-secure world."]
    Blocked = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    Readable = 1,
}
impl From<Pio1Pin31SecMask> for bool {
    #[inline(always)]
    fn from(variant: Pio1Pin31SecMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1_PIN31_SEC_MASK` reader - Secure mask for pin P1_31"]
pub type Pio1Pin31SecMaskR = crate::BitReader<Pio1Pin31SecMask>;
impl Pio1Pin31SecMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pio1Pin31SecMask {
        match self.bits {
            false => Pio1Pin31SecMask::Blocked,
            true => Pio1Pin31SecMask::Readable,
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Pio1Pin31SecMask::Blocked
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == Pio1Pin31SecMask::Readable
    }
}
#[doc = "Field `PIO1_PIN31_SEC_MASK` writer - Secure mask for pin P1_31"]
pub type Pio1Pin31SecMaskW<'a, REG> = crate::BitWriter<'a, REG, Pio1Pin31SecMask>;
impl<'a, REG> Pio1Pin31SecMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin31SecMask::Blocked)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut crate::W<REG> {
        self.variant(Pio1Pin31SecMask::Readable)
    }
}
impl R {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&self) -> Pio1Pin0SecMaskR {
        Pio1Pin0SecMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&self) -> Pio1Pin1SecMaskR {
        Pio1Pin1SecMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&self) -> Pio1Pin2SecMaskR {
        Pio1Pin2SecMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&self) -> Pio1Pin3SecMaskR {
        Pio1Pin3SecMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&self) -> Pio1Pin4SecMaskR {
        Pio1Pin4SecMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&self) -> Pio1Pin5SecMaskR {
        Pio1Pin5SecMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&self) -> Pio1Pin6SecMaskR {
        Pio1Pin6SecMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&self) -> Pio1Pin7SecMaskR {
        Pio1Pin7SecMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&self) -> Pio1Pin8SecMaskR {
        Pio1Pin8SecMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&self) -> Pio1Pin9SecMaskR {
        Pio1Pin9SecMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&self) -> Pio1Pin10SecMaskR {
        Pio1Pin10SecMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&self) -> Pio1Pin11SecMaskR {
        Pio1Pin11SecMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&self) -> Pio1Pin12SecMaskR {
        Pio1Pin12SecMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&self) -> Pio1Pin13SecMaskR {
        Pio1Pin13SecMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&self) -> Pio1Pin14SecMaskR {
        Pio1Pin14SecMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&self) -> Pio1Pin15SecMaskR {
        Pio1Pin15SecMaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&self) -> Pio1Pin16SecMaskR {
        Pio1Pin16SecMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&self) -> Pio1Pin17SecMaskR {
        Pio1Pin17SecMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&self) -> Pio1Pin18SecMaskR {
        Pio1Pin18SecMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&self) -> Pio1Pin19SecMaskR {
        Pio1Pin19SecMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&self) -> Pio1Pin20SecMaskR {
        Pio1Pin20SecMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&self) -> Pio1Pin21SecMaskR {
        Pio1Pin21SecMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&self) -> Pio1Pin22SecMaskR {
        Pio1Pin22SecMaskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&self) -> Pio1Pin23SecMaskR {
        Pio1Pin23SecMaskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&self) -> Pio1Pin24SecMaskR {
        Pio1Pin24SecMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&self) -> Pio1Pin25SecMaskR {
        Pio1Pin25SecMaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&self) -> Pio1Pin26SecMaskR {
        Pio1Pin26SecMaskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&self) -> Pio1Pin27SecMaskR {
        Pio1Pin27SecMaskR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&self) -> Pio1Pin28SecMaskR {
        Pio1Pin28SecMaskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&self) -> Pio1Pin29SecMaskR {
        Pio1Pin29SecMaskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&self) -> Pio1Pin30SecMaskR {
        Pio1Pin30SecMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&self) -> Pio1Pin31SecMaskR {
        Pio1Pin31SecMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&mut self) -> Pio1Pin0SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin0SecMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&mut self) -> Pio1Pin1SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin1SecMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&mut self) -> Pio1Pin2SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin2SecMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&mut self) -> Pio1Pin3SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin3SecMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&mut self) -> Pio1Pin4SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin4SecMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&mut self) -> Pio1Pin5SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin5SecMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&mut self) -> Pio1Pin6SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin6SecMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&mut self) -> Pio1Pin7SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin7SecMaskW::new(self, 7)
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&mut self) -> Pio1Pin8SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin8SecMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&mut self) -> Pio1Pin9SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin9SecMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&mut self) -> Pio1Pin10SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin10SecMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&mut self) -> Pio1Pin11SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin11SecMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&mut self) -> Pio1Pin12SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin12SecMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&mut self) -> Pio1Pin13SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin13SecMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&mut self) -> Pio1Pin14SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin14SecMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&mut self) -> Pio1Pin15SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin15SecMaskW::new(self, 15)
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&mut self) -> Pio1Pin16SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin16SecMaskW::new(self, 16)
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&mut self) -> Pio1Pin17SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin17SecMaskW::new(self, 17)
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&mut self) -> Pio1Pin18SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin18SecMaskW::new(self, 18)
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&mut self) -> Pio1Pin19SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin19SecMaskW::new(self, 19)
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&mut self) -> Pio1Pin20SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin20SecMaskW::new(self, 20)
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&mut self) -> Pio1Pin21SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin21SecMaskW::new(self, 21)
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&mut self) -> Pio1Pin22SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin22SecMaskW::new(self, 22)
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&mut self) -> Pio1Pin23SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin23SecMaskW::new(self, 23)
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&mut self) -> Pio1Pin24SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin24SecMaskW::new(self, 24)
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&mut self) -> Pio1Pin25SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin25SecMaskW::new(self, 25)
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&mut self) -> Pio1Pin26SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin26SecMaskW::new(self, 26)
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&mut self) -> Pio1Pin27SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin27SecMaskW::new(self, 27)
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&mut self) -> Pio1Pin28SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin28SecMaskW::new(self, 28)
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&mut self) -> Pio1Pin29SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin29SecMaskW::new(self, 29)
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&mut self) -> Pio1Pin30SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin30SecMaskW::new(self, 30)
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&mut self) -> Pio1Pin31SecMaskW<'_, SecGpioMask1Spec> {
        Pio1Pin31SecMaskW::new(self, 31)
    }
}
#[doc = "Secure GPIO mask for port 1 pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_gpio_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_gpio_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecGpioMask1Spec;
impl crate::RegisterSpec for SecGpioMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_gpio_mask1::R`](R) reader structure"]
impl crate::Readable for SecGpioMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_gpio_mask1::W`](W) writer structure"]
impl crate::Writable for SecGpioMask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_GPIO_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for SecGpioMask1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

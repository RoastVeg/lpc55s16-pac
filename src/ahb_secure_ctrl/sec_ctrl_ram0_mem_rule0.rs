#[doc = "Register `SEC_CTRL_RAM0_MEM_RULE0` reader"]
pub type R = crate::R<SecCtrlRam0MemRule0Spec>;
#[doc = "Register `SEC_CTRL_RAM0_MEM_RULE0` writer"]
pub type W = crate::W<SecCtrlRam0MemRule0Spec>;
#[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule0 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule0> for u8 {
    #[inline(always)]
    fn from(variant: Rule0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule0 {
    type Ux = u8;
}
impl crate::IsEnum for Rule0 {}
#[doc = "Field `RULE0` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0R = crate::FieldReader<Rule0>;
impl Rule0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule0 {
        match self.bits {
            0 => Rule0::EnumNsNp,
            1 => Rule0::EnumNsP,
            2 => Rule0::EnumSNp,
            3 => Rule0::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule0::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule0::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule0::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule0::EnumSP
    }
}
#[doc = "Field `RULE0` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule0, crate::Safe>;
impl<'a, REG> Rule0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::EnumSP)
    }
}
#[doc = "secure control rule1. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule1 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule1> for u8 {
    #[inline(always)]
    fn from(variant: Rule1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule1 {
    type Ux = u8;
}
impl crate::IsEnum for Rule1 {}
#[doc = "Field `RULE1` reader - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type Rule1R = crate::FieldReader<Rule1>;
impl Rule1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule1 {
        match self.bits {
            0 => Rule1::EnumNsNp,
            1 => Rule1::EnumNsP,
            2 => Rule1::EnumSNp,
            3 => Rule1::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule1::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule1::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule1::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule1::EnumSP
    }
}
#[doc = "Field `RULE1` writer - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type Rule1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule1, crate::Safe>;
impl<'a, REG> Rule1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::EnumSP)
    }
}
#[doc = "secure control rule2. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule2 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule2> for u8 {
    #[inline(always)]
    fn from(variant: Rule2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule2 {
    type Ux = u8;
}
impl crate::IsEnum for Rule2 {}
#[doc = "Field `RULE2` reader - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub type Rule2R = crate::FieldReader<Rule2>;
impl Rule2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule2 {
        match self.bits {
            0 => Rule2::EnumNsNp,
            1 => Rule2::EnumNsP,
            2 => Rule2::EnumSNp,
            3 => Rule2::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule2::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule2::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule2::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule2::EnumSP
    }
}
#[doc = "Field `RULE2` writer - secure control rule2. it can be set when check_reg's write_lock is '0'"]
pub type Rule2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule2, crate::Safe>;
impl<'a, REG> Rule2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::EnumSP)
    }
}
#[doc = "secure control rule3. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule3 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule3> for u8 {
    #[inline(always)]
    fn from(variant: Rule3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule3 {
    type Ux = u8;
}
impl crate::IsEnum for Rule3 {}
#[doc = "Field `RULE3` reader - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub type Rule3R = crate::FieldReader<Rule3>;
impl Rule3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule3 {
        match self.bits {
            0 => Rule3::EnumNsNp,
            1 => Rule3::EnumNsP,
            2 => Rule3::EnumSNp,
            3 => Rule3::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule3::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule3::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule3::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule3::EnumSP
    }
}
#[doc = "Field `RULE3` writer - secure control rule3. it can be set when check_reg's write_lock is '0'"]
pub type Rule3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule3, crate::Safe>;
impl<'a, REG> Rule3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::EnumSP)
    }
}
#[doc = "secure control rule4. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule4 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule4> for u8 {
    #[inline(always)]
    fn from(variant: Rule4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule4 {
    type Ux = u8;
}
impl crate::IsEnum for Rule4 {}
#[doc = "Field `RULE4` reader - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type Rule4R = crate::FieldReader<Rule4>;
impl Rule4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule4 {
        match self.bits {
            0 => Rule4::EnumNsNp,
            1 => Rule4::EnumNsP,
            2 => Rule4::EnumSNp,
            3 => Rule4::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule4::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule4::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule4::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule4::EnumSP
    }
}
#[doc = "Field `RULE4` writer - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type Rule4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule4, crate::Safe>;
impl<'a, REG> Rule4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::EnumSP)
    }
}
#[doc = "secure control rule5. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule5 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule5> for u8 {
    #[inline(always)]
    fn from(variant: Rule5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule5 {
    type Ux = u8;
}
impl crate::IsEnum for Rule5 {}
#[doc = "Field `RULE5` reader - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type Rule5R = crate::FieldReader<Rule5>;
impl Rule5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule5 {
        match self.bits {
            0 => Rule5::EnumNsNp,
            1 => Rule5::EnumNsP,
            2 => Rule5::EnumSNp,
            3 => Rule5::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule5::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule5::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule5::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule5::EnumSP
    }
}
#[doc = "Field `RULE5` writer - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type Rule5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule5, crate::Safe>;
impl<'a, REG> Rule5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::EnumSP)
    }
}
#[doc = "secure control rule6. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule6 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule6> for u8 {
    #[inline(always)]
    fn from(variant: Rule6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule6 {
    type Ux = u8;
}
impl crate::IsEnum for Rule6 {}
#[doc = "Field `RULE6` reader - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub type Rule6R = crate::FieldReader<Rule6>;
impl Rule6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule6 {
        match self.bits {
            0 => Rule6::EnumNsNp,
            1 => Rule6::EnumNsP,
            2 => Rule6::EnumSNp,
            3 => Rule6::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule6::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule6::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule6::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule6::EnumSP
    }
}
#[doc = "Field `RULE6` writer - secure control rule6. it can be set when check_reg's write_lock is '0'"]
pub type Rule6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule6, crate::Safe>;
impl<'a, REG> Rule6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::EnumSP)
    }
}
#[doc = "secure control rule7. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule7 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Rule7> for u8 {
    #[inline(always)]
    fn from(variant: Rule7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule7 {
    type Ux = u8;
}
impl crate::IsEnum for Rule7 {}
#[doc = "Field `RULE7` reader - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub type Rule7R = crate::FieldReader<Rule7>;
impl Rule7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule7 {
        match self.bits {
            0 => Rule7::EnumNsNp,
            1 => Rule7::EnumNsP,
            2 => Rule7::EnumSNp,
            3 => Rule7::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Rule7::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Rule7::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Rule7::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Rule7::EnumSP
    }
}
#[doc = "Field `RULE7` writer - secure control rule7. it can be set when check_reg's write_lock is '0'"]
pub type Rule7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule7, crate::Safe>;
impl<'a, REG> Rule7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&self) -> Rule0R {
        Rule0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&self) -> Rule1R {
        Rule1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&self) -> Rule2R {
        Rule2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&self) -> Rule3R {
        Rule3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&self) -> Rule4R {
        Rule4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&self) -> Rule5R {
        Rule5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule6(&self) -> Rule6R {
        Rule6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule7(&self) -> Rule7R {
        Rule7R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL_RAM0_MEM_RULE0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&mut self) -> Rule0W<'_, SecCtrlRam0MemRule0Spec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&mut self) -> Rule1W<'_, SecCtrlRam0MemRule0Spec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&mut self) -> Rule2W<'_, SecCtrlRam0MemRule0Spec> {
        Rule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&mut self) -> Rule3W<'_, SecCtrlRam0MemRule0Spec> {
        Rule3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&mut self) -> Rule4W<'_, SecCtrlRam0MemRule0Spec> {
        Rule4W::new(self, 16)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&mut self) -> Rule5W<'_, SecCtrlRam0MemRule0Spec> {
        Rule5W::new(self, 20)
    }
    #[doc = "Bits 24:25 - secure control rule6. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule6(&mut self) -> Rule6W<'_, SecCtrlRam0MemRule0Spec> {
        Rule6W::new(self, 24)
    }
    #[doc = "Bits 28:29 - secure control rule7. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule7(&mut self) -> Rule7W<'_, SecCtrlRam0MemRule0Spec> {
        Rule7W::new(self, 28)
    }
}
#[doc = "Security access rules for RAM0 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram0_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram0_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam0MemRule0Spec;
impl crate::RegisterSpec for SecCtrlRam0MemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram0_mem_rule0::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam0MemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram0_mem_rule0::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam0MemRule0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM0_MEM_RULE0 to value 0"]
impl crate::Resettable for SecCtrlRam0MemRule0Spec {}

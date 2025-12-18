#[doc = "Register `SEC_CTRL_RAMX_MEM_RULE0` reader"]
pub type R = crate::R<SecCtrlRamxMemRule0Spec>;
#[doc = "Register `SEC_CTRL_RAMX_MEM_RULE0` writer"]
pub type W = crate::W<SecCtrlRamxMemRule0Spec>;
#[doc = "secure control rule0. it can be set when check_reg's write_lock is '0'\n\nValue on reset: 0"]
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
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&mut self) -> Rule0W<'_, SecCtrlRamxMemRule0Spec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&mut self) -> Rule1W<'_, SecCtrlRamxMemRule0Spec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - secure control rule2. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&mut self) -> Rule2W<'_, SecCtrlRamxMemRule0Spec> {
        Rule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - secure control rule3. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&mut self) -> Rule3W<'_, SecCtrlRamxMemRule0Spec> {
        Rule3W::new(self, 12)
    }
}
#[doc = "Security access rules for RAMX slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ramx_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ramx_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRamxMemRule0Spec;
impl crate::RegisterSpec for SecCtrlRamxMemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ramx_mem_rule0::R`](R) reader structure"]
impl crate::Readable for SecCtrlRamxMemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ramx_mem_rule0::W`](W) writer structure"]
impl crate::Writable for SecCtrlRamxMemRule0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_CTRL_RAMX_MEM_RULE0 to value 0"]
impl crate::Resettable for SecCtrlRamxMemRule0Spec {}

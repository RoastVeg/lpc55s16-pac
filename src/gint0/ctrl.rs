#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: No request. No interrupt request is pending."]
    NoRequest = 0,
    #[doc = "1: Request active. Interrupt request is active."]
    RequestActive = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
pub type IntR = crate::BitReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int {
        match self.bits {
            false => Int::NoRequest,
            true => Int::RequestActive,
        }
    }
    #[doc = "No request. No interrupt request is pending."]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == Int::NoRequest
    }
    #[doc = "Request active. Interrupt request is active."]
    #[inline(always)]
    pub fn is_request_active(&self) -> bool {
        *self == Int::RequestActive
    }
}
#[doc = "Field `INT` writer - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG, Int>;
impl<'a, REG> IntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request. No interrupt request is pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut crate::W<REG> {
        self.variant(Int::NoRequest)
    }
    #[doc = "Request active. Interrupt request is active."]
    #[inline(always)]
    pub fn request_active(self) -> &'a mut crate::W<REG> {
        self.variant(Int::RequestActive)
    }
}
#[doc = "Combine enabled inputs for group interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comb {
    #[doc = "0: Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    Or = 0,
    #[doc = "1: And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    And = 1,
}
impl From<Comb> for bool {
    #[inline(always)]
    fn from(variant: Comb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMB` reader - Combine enabled inputs for group interrupt"]
pub type CombR = crate::BitReader<Comb>;
impl CombR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comb {
        match self.bits {
            false => Comb::Or,
            true => Comb::And,
        }
    }
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == Comb::Or
    }
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == Comb::And
    }
}
#[doc = "Field `COMB` writer - Combine enabled inputs for group interrupt"]
pub type CombW<'a, REG> = crate::BitWriter<'a, REG, Comb>;
impl<'a, REG> CombW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    #[inline(always)]
    pub fn or(self) -> &'a mut crate::W<REG> {
        self.variant(Comb::Or)
    }
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    #[inline(always)]
    pub fn and(self) -> &'a mut crate::W<REG> {
        self.variant(Comb::And)
    }
}
#[doc = "Group interrupt trigger\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Edge-triggered."]
    EdgeTriggered = 0,
    #[doc = "1: Level-triggered."]
    LevelTriggered = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - Group interrupt trigger"]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            false => Trig::EdgeTriggered,
            true => Trig::LevelTriggered,
        }
    }
    #[doc = "Edge-triggered."]
    #[inline(always)]
    pub fn is_edge_triggered(&self) -> bool {
        *self == Trig::EdgeTriggered
    }
    #[doc = "Level-triggered."]
    #[inline(always)]
    pub fn is_level_triggered(&self) -> bool {
        *self == Trig::LevelTriggered
    }
}
#[doc = "Field `TRIG` writer - Group interrupt trigger"]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge-triggered."]
    #[inline(always)]
    pub fn edge_triggered(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::EdgeTriggered)
    }
    #[doc = "Level-triggered."]
    #[inline(always)]
    pub fn level_triggered(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::LevelTriggered)
    }
}
impl R {
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&self) -> CombR {
        CombR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("int", &self.int())
            .field("comb", &self.comb())
            .field("trig", &self.trig())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, CtrlSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub fn comb(&mut self) -> CombW<'_, CtrlSpec> {
        CombW::new(self, 1)
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, CtrlSpec> {
        TrigW::new(self, 2)
    }
}
#[doc = "GPIO grouped interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}

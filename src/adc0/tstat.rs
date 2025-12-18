#[doc = "Register `TSTAT` reader"]
pub type R = crate::R<TstatSpec>;
#[doc = "Register `TSTAT` writer"]
pub type W = crate::W<TstatSpec>;
#[doc = "Trigger Exception Number\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TexcNum {
    #[doc = "0: No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    TexcNum0 = 0,
    #[doc = "1: Trigger 0 has been interrupted by a high priority exception."]
    TexcNum1 = 1,
    #[doc = "2: Trigger 1 has been interrupted by a high priority exception."]
    TexcNum2 = 2,
    #[doc = "3: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum3 = 3,
    #[doc = "4: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum4 = 4,
    #[doc = "5: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum5 = 5,
    #[doc = "6: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum6 = 6,
    #[doc = "7: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum7 = 7,
    #[doc = "8: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum8 = 8,
    #[doc = "9: Associated trigger sequence has interrupted by a high priority exception."]
    TexcNum9 = 9,
    #[doc = "65535: Every trigger sequence has been interrupted by a high priority exception."]
    TexcNum65535 = 65535,
}
impl From<TexcNum> for u16 {
    #[inline(always)]
    fn from(variant: TexcNum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TexcNum {
    type Ux = u16;
}
impl crate::IsEnum for TexcNum {}
#[doc = "Field `TEXC_NUM` reader - Trigger Exception Number"]
pub type TexcNumR = crate::FieldReader<TexcNum>;
impl TexcNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TexcNum> {
        match self.bits {
            0 => Some(TexcNum::TexcNum0),
            1 => Some(TexcNum::TexcNum1),
            2 => Some(TexcNum::TexcNum2),
            3 => Some(TexcNum::TexcNum3),
            4 => Some(TexcNum::TexcNum4),
            5 => Some(TexcNum::TexcNum5),
            6 => Some(TexcNum::TexcNum6),
            7 => Some(TexcNum::TexcNum7),
            8 => Some(TexcNum::TexcNum8),
            9 => Some(TexcNum::TexcNum9),
            65535 => Some(TexcNum::TexcNum65535),
            _ => None,
        }
    }
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    #[inline(always)]
    pub fn is_texc_num_0(&self) -> bool {
        *self == TexcNum::TexcNum0
    }
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_1(&self) -> bool {
        *self == TexcNum::TexcNum1
    }
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_2(&self) -> bool {
        *self == TexcNum::TexcNum2
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_3(&self) -> bool {
        *self == TexcNum::TexcNum3
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_4(&self) -> bool {
        *self == TexcNum::TexcNum4
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_5(&self) -> bool {
        *self == TexcNum::TexcNum5
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_6(&self) -> bool {
        *self == TexcNum::TexcNum6
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_7(&self) -> bool {
        *self == TexcNum::TexcNum7
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_8(&self) -> bool {
        *self == TexcNum::TexcNum8
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_9(&self) -> bool {
        *self == TexcNum::TexcNum9
    }
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn is_texc_num_65535(&self) -> bool {
        *self == TexcNum::TexcNum65535
    }
}
#[doc = "Field `TEXC_NUM` writer - Trigger Exception Number"]
pub type TexcNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, TexcNum>;
impl<'a, REG> TexcNumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    #[inline(always)]
    pub fn texc_num_0(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum0)
    }
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_1(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum1)
    }
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_2(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum2)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_3(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum3)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_4(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum4)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_5(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum5)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_6(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum6)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_7(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum7)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_8(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum8)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_9(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum9)
    }
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_65535(self) -> &'a mut crate::W<REG> {
        self.variant(TexcNum::TexcNum65535)
    }
}
#[doc = "Trigger Completion Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TcompFlag {
    #[doc = "0: No triggers have been completed. Trigger completion interrupts are disabled."]
    TcompFlag0 = 0,
    #[doc = "1: Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    TcompFlag1 = 1,
    #[doc = "2: Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    TcompFlag2 = 2,
    #[doc = "3: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag3 = 3,
    #[doc = "4: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag4 = 4,
    #[doc = "5: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag5 = 5,
    #[doc = "6: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag6 = 6,
    #[doc = "7: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag7 = 7,
    #[doc = "8: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag8 = 8,
    #[doc = "9: Associated trigger sequence has completed and has enabled completion interrupts."]
    TcompFlag9 = 9,
    #[doc = "65535: Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    TcompFlag65535 = 65535,
}
impl From<TcompFlag> for u16 {
    #[inline(always)]
    fn from(variant: TcompFlag) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TcompFlag {
    type Ux = u16;
}
impl crate::IsEnum for TcompFlag {}
#[doc = "Field `TCOMP_FLAG` reader - Trigger Completion Flag"]
pub type TcompFlagR = crate::FieldReader<TcompFlag>;
impl TcompFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TcompFlag> {
        match self.bits {
            0 => Some(TcompFlag::TcompFlag0),
            1 => Some(TcompFlag::TcompFlag1),
            2 => Some(TcompFlag::TcompFlag2),
            3 => Some(TcompFlag::TcompFlag3),
            4 => Some(TcompFlag::TcompFlag4),
            5 => Some(TcompFlag::TcompFlag5),
            6 => Some(TcompFlag::TcompFlag6),
            7 => Some(TcompFlag::TcompFlag7),
            8 => Some(TcompFlag::TcompFlag8),
            9 => Some(TcompFlag::TcompFlag9),
            65535 => Some(TcompFlag::TcompFlag65535),
            _ => None,
        }
    }
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn is_tcomp_flag_0(&self) -> bool {
        *self == TcompFlag::TcompFlag0
    }
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_1(&self) -> bool {
        *self == TcompFlag::TcompFlag1
    }
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_2(&self) -> bool {
        *self == TcompFlag::TcompFlag2
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_3(&self) -> bool {
        *self == TcompFlag::TcompFlag3
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_4(&self) -> bool {
        *self == TcompFlag::TcompFlag4
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_5(&self) -> bool {
        *self == TcompFlag::TcompFlag5
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_6(&self) -> bool {
        *self == TcompFlag::TcompFlag6
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_7(&self) -> bool {
        *self == TcompFlag::TcompFlag7
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_8(&self) -> bool {
        *self == TcompFlag::TcompFlag8
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_9(&self) -> bool {
        *self == TcompFlag::TcompFlag9
    }
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    #[inline(always)]
    pub fn is_tcomp_flag_65535(&self) -> bool {
        *self == TcompFlag::TcompFlag65535
    }
}
#[doc = "Field `TCOMP_FLAG` writer - Trigger Completion Flag"]
pub type TcompFlagW<'a, REG> = crate::FieldWriter<'a, REG, 16, TcompFlag>;
impl<'a, REG> TcompFlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_flag_0(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag0)
    }
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_1(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag1)
    }
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_2(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag2)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_3(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag3)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_4(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag4)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_5(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag5)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_6(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag6)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_7(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag7)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_8(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag8)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_9(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag9)
    }
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_65535(self) -> &'a mut crate::W<REG> {
        self.variant(TcompFlag::TcompFlag65535)
    }
}
impl R {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    pub fn texc_num(&self) -> TexcNumR {
        TexcNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    pub fn tcomp_flag(&self) -> TcompFlagR {
        TcompFlagR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTAT")
            .field("texc_num", &self.texc_num())
            .field("tcomp_flag", &self.tcomp_flag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    pub fn texc_num(&mut self) -> TexcNumW<'_, TstatSpec> {
        TexcNumW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    pub fn tcomp_flag(&mut self) -> TcompFlagW<'_, TstatSpec> {
        TcompFlagW::new(self, 16)
    }
}
#[doc = "Trigger Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstatSpec;
impl crate::RegisterSpec for TstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstat::R`](R) reader structure"]
impl crate::Readable for TstatSpec {}
#[doc = "`write(|w| ..)` method takes [`tstat::W`](W) writer structure"]
impl crate::Writable for TstatSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets TSTAT to value 0"]
impl crate::Resettable for TstatSpec {}

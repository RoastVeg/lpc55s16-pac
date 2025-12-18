#[doc = "Register `SHAREDCTRLSET%s` reader"]
pub type R = crate::R<SharedctrlsetSpec>;
#[doc = "Register `SHAREDCTRLSET%s` writer"]
pub type W = crate::W<SharedctrlsetSpec>;
#[doc = "Selects the source for SCK of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sharedscksel {
    #[doc = "0: SCK for this shared signal set comes from Flexcomm 0."]
    Flexcomm0 = 0,
    #[doc = "1: SCK for this shared signal set comes from Flexcomm 1."]
    Flexcomm1 = 1,
    #[doc = "2: SCK for this shared signal set comes from Flexcomm 2."]
    Flexcomm2 = 2,
    #[doc = "3: SCK for this shared signal set comes from Flexcomm 3."]
    Flexcomm3 = 3,
    #[doc = "4: SCK for this shared signal set comes from Flexcomm 4."]
    Flexcomm4 = 4,
    #[doc = "5: SCK for this shared signal set comes from Flexcomm 5."]
    Flexcomm5 = 5,
    #[doc = "6: SCK for this shared signal set comes from Flexcomm 6."]
    Flexcomm6 = 6,
    #[doc = "7: SCK for this shared signal set comes from Flexcomm 7."]
    Flexcomm7 = 7,
}
impl From<Sharedscksel> for u8 {
    #[inline(always)]
    fn from(variant: Sharedscksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sharedscksel {
    type Ux = u8;
}
impl crate::IsEnum for Sharedscksel {}
#[doc = "Field `SHAREDSCKSEL` reader - Selects the source for SCK of this shared signal set."]
pub type SharedsckselR = crate::FieldReader<Sharedscksel>;
impl SharedsckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sharedscksel {
        match self.bits {
            0 => Sharedscksel::Flexcomm0,
            1 => Sharedscksel::Flexcomm1,
            2 => Sharedscksel::Flexcomm2,
            3 => Sharedscksel::Flexcomm3,
            4 => Sharedscksel::Flexcomm4,
            5 => Sharedscksel::Flexcomm5,
            6 => Sharedscksel::Flexcomm6,
            7 => Sharedscksel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Sharedscksel::Flexcomm0
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Sharedscksel::Flexcomm1
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Sharedscksel::Flexcomm2
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Sharedscksel::Flexcomm3
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Sharedscksel::Flexcomm4
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Sharedscksel::Flexcomm5
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Sharedscksel::Flexcomm6
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Sharedscksel::Flexcomm7
    }
}
#[doc = "Field `SHAREDSCKSEL` writer - Selects the source for SCK of this shared signal set."]
pub type SharedsckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sharedscksel, crate::Safe>;
impl<'a, REG> SharedsckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm0)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm1)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm2)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm3)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm4)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm5)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm6)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedscksel::Flexcomm7)
    }
}
#[doc = "Selects the source for WS of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sharedwssel {
    #[doc = "0: WS for this shared signal set comes from Flexcomm 0."]
    Flexcomm0 = 0,
    #[doc = "1: WS for this shared signal set comes from Flexcomm 1."]
    Flexcomm1 = 1,
    #[doc = "2: WS for this shared signal set comes from Flexcomm 2."]
    Flexcomm2 = 2,
    #[doc = "3: WS for this shared signal set comes from Flexcomm 3."]
    Flexcomm3 = 3,
    #[doc = "4: WS for this shared signal set comes from Flexcomm 4."]
    Flexcomm4 = 4,
    #[doc = "5: WS for this shared signal set comes from Flexcomm 5."]
    Flexcomm5 = 5,
    #[doc = "6: WS for this shared signal set comes from Flexcomm 6."]
    Flexcomm6 = 6,
    #[doc = "7: WS for this shared signal set comes from Flexcomm 7."]
    Flexcomm7 = 7,
}
impl From<Sharedwssel> for u8 {
    #[inline(always)]
    fn from(variant: Sharedwssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sharedwssel {
    type Ux = u8;
}
impl crate::IsEnum for Sharedwssel {}
#[doc = "Field `SHAREDWSSEL` reader - Selects the source for WS of this shared signal set."]
pub type SharedwsselR = crate::FieldReader<Sharedwssel>;
impl SharedwsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sharedwssel {
        match self.bits {
            0 => Sharedwssel::Flexcomm0,
            1 => Sharedwssel::Flexcomm1,
            2 => Sharedwssel::Flexcomm2,
            3 => Sharedwssel::Flexcomm3,
            4 => Sharedwssel::Flexcomm4,
            5 => Sharedwssel::Flexcomm5,
            6 => Sharedwssel::Flexcomm6,
            7 => Sharedwssel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Sharedwssel::Flexcomm0
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Sharedwssel::Flexcomm1
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Sharedwssel::Flexcomm2
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Sharedwssel::Flexcomm3
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Sharedwssel::Flexcomm4
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Sharedwssel::Flexcomm5
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Sharedwssel::Flexcomm6
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Sharedwssel::Flexcomm7
    }
}
#[doc = "Field `SHAREDWSSEL` writer - Selects the source for WS of this shared signal set."]
pub type SharedwsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sharedwssel, crate::Safe>;
impl<'a, REG> SharedwsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm0)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm1)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm2)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm3)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm4)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm5)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm6)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Sharedwssel::Flexcomm7)
    }
}
#[doc = "Selects the source for DATA input for this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shareddatasel {
    #[doc = "0: DATA input for this shared signal set comes from Flexcomm 0."]
    Flexcomm0 = 0,
    #[doc = "1: DATA input for this shared signal set comes from Flexcomm 1."]
    Flexcomm1 = 1,
    #[doc = "2: DATA input for this shared signal set comes from Flexcomm 2."]
    Flexcomm2 = 2,
    #[doc = "3: DATA input for this shared signal set comes from Flexcomm 3."]
    Flexcomm3 = 3,
    #[doc = "4: DATA input for this shared signal set comes from Flexcomm 4."]
    Flexcomm4 = 4,
    #[doc = "5: DATA input for this shared signal set comes from Flexcomm 5."]
    Flexcomm5 = 5,
    #[doc = "6: DATA input for this shared signal set comes from Flexcomm 6."]
    Flexcomm6 = 6,
    #[doc = "7: DATA input for this shared signal set comes from Flexcomm 7."]
    Flexcomm7 = 7,
}
impl From<Shareddatasel> for u8 {
    #[inline(always)]
    fn from(variant: Shareddatasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shareddatasel {
    type Ux = u8;
}
impl crate::IsEnum for Shareddatasel {}
#[doc = "Field `SHAREDDATASEL` reader - Selects the source for DATA input for this shared signal set."]
pub type ShareddataselR = crate::FieldReader<Shareddatasel>;
impl ShareddataselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shareddatasel {
        match self.bits {
            0 => Shareddatasel::Flexcomm0,
            1 => Shareddatasel::Flexcomm1,
            2 => Shareddatasel::Flexcomm2,
            3 => Shareddatasel::Flexcomm3,
            4 => Shareddatasel::Flexcomm4,
            5 => Shareddatasel::Flexcomm5,
            6 => Shareddatasel::Flexcomm6,
            7 => Shareddatasel::Flexcomm7,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == Shareddatasel::Flexcomm0
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == Shareddatasel::Flexcomm1
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == Shareddatasel::Flexcomm2
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == Shareddatasel::Flexcomm3
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == Shareddatasel::Flexcomm4
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == Shareddatasel::Flexcomm5
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == Shareddatasel::Flexcomm6
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == Shareddatasel::Flexcomm7
    }
}
#[doc = "Field `SHAREDDATASEL` writer - Selects the source for DATA input for this shared signal set."]
pub type ShareddataselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Shareddatasel, crate::Safe>;
impl<'a, REG> ShareddataselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm0)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm1)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm2)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm3)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm4)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm5)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm6)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(Shareddatasel::Flexcomm7)
    }
}
#[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0dataouten {
    #[doc = "0: Data output from FC0 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC0 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc0dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc0dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0DATAOUTEN` reader - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc0dataoutenR = crate::BitReader<Fc0dataouten>;
impl Fc0dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc0dataouten {
        match self.bits {
            false => Fc0dataouten::Input,
            true => Fc0dataouten::Output,
        }
    }
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc0dataouten::Input
    }
    #[doc = "Data output from FC0 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc0dataouten::Output
    }
}
#[doc = "Field `FC0DATAOUTEN` writer - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc0dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc0dataouten>;
impl<'a, REG> Fc0dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0dataouten::Input)
    }
    #[doc = "Data output from FC0 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0dataouten::Output)
    }
}
#[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1dataouten {
    #[doc = "0: Data output from FC1 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC1 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc1dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc1dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1DATAOUTEN` reader - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc1dataoutenR = crate::BitReader<Fc1dataouten>;
impl Fc1dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc1dataouten {
        match self.bits {
            false => Fc1dataouten::Input,
            true => Fc1dataouten::Output,
        }
    }
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc1dataouten::Input
    }
    #[doc = "Data output from FC1 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc1dataouten::Output
    }
}
#[doc = "Field `FC1DATAOUTEN` writer - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc1dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc1dataouten>;
impl<'a, REG> Fc1dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1dataouten::Input)
    }
    #[doc = "Data output from FC1 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1dataouten::Output)
    }
}
#[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2dataouten {
    #[doc = "0: Data output from FC2 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC2 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc2dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc2dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2DATAOUTEN` reader - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc2dataoutenR = crate::BitReader<Fc2dataouten>;
impl Fc2dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc2dataouten {
        match self.bits {
            false => Fc2dataouten::Input,
            true => Fc2dataouten::Output,
        }
    }
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc2dataouten::Input
    }
    #[doc = "Data output from FC2 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc2dataouten::Output
    }
}
#[doc = "Field `FC2DATAOUTEN` writer - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc2dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc2dataouten>;
impl<'a, REG> Fc2dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2dataouten::Input)
    }
    #[doc = "Data output from FC2 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2dataouten::Output)
    }
}
#[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4dataouten {
    #[doc = "0: Data output from FC4 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC4 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc4dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc4dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4DATAOUTEN` reader - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc4dataoutenR = crate::BitReader<Fc4dataouten>;
impl Fc4dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc4dataouten {
        match self.bits {
            false => Fc4dataouten::Input,
            true => Fc4dataouten::Output,
        }
    }
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc4dataouten::Input
    }
    #[doc = "Data output from FC4 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc4dataouten::Output
    }
}
#[doc = "Field `FC4DATAOUTEN` writer - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc4dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc4dataouten>;
impl<'a, REG> Fc4dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4dataouten::Input)
    }
    #[doc = "Data output from FC4 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4dataouten::Output)
    }
}
#[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5dataouten {
    #[doc = "0: Data output from FC5 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC5 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc5dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc5dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5DATAOUTEN` reader - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc5dataoutenR = crate::BitReader<Fc5dataouten>;
impl Fc5dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc5dataouten {
        match self.bits {
            false => Fc5dataouten::Input,
            true => Fc5dataouten::Output,
        }
    }
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc5dataouten::Input
    }
    #[doc = "Data output from FC5 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc5dataouten::Output
    }
}
#[doc = "Field `FC5DATAOUTEN` writer - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc5dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc5dataouten>;
impl<'a, REG> Fc5dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5dataouten::Input)
    }
    #[doc = "Data output from FC5 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5dataouten::Output)
    }
}
#[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6dataouten {
    #[doc = "0: Data output from FC6 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC6 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc6dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc6dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6DATAOUTEN` reader - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc6dataoutenR = crate::BitReader<Fc6dataouten>;
impl Fc6dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc6dataouten {
        match self.bits {
            false => Fc6dataouten::Input,
            true => Fc6dataouten::Output,
        }
    }
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc6dataouten::Input
    }
    #[doc = "Data output from FC6 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc6dataouten::Output
    }
}
#[doc = "Field `FC6DATAOUTEN` writer - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc6dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc6dataouten>;
impl<'a, REG> Fc6dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6dataouten::Input)
    }
    #[doc = "Data output from FC6 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6dataouten::Output)
    }
}
#[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7dataouten {
    #[doc = "0: Data output from FC7 does not contribute to this shared set."]
    Input = 0,
    #[doc = "1: Data output from FC7 does contribute to this shared set."]
    Output = 1,
}
impl From<Fc7dataouten> for bool {
    #[inline(always)]
    fn from(variant: Fc7dataouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7DATAOUTEN` reader - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc7dataoutenR = crate::BitReader<Fc7dataouten>;
impl Fc7dataoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc7dataouten {
        match self.bits {
            false => Fc7dataouten::Input,
            true => Fc7dataouten::Output,
        }
    }
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fc7dataouten::Input
    }
    #[doc = "Data output from FC7 does contribute to this shared set."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fc7dataouten::Output
    }
}
#[doc = "Field `FC7DATAOUTEN` writer - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
pub type Fc7dataoutenW<'a, REG> = crate::BitWriter<'a, REG, Fc7dataouten>;
impl<'a, REG> Fc7dataoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7dataouten::Input)
    }
    #[doc = "Data output from FC7 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7dataouten::Output)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub fn sharedscksel(&self) -> SharedsckselR {
        SharedsckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub fn sharedwssel(&self) -> SharedwsselR {
        SharedwsselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub fn shareddatasel(&self) -> ShareddataselR {
        ShareddataselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc0dataouten(&self) -> Fc0dataoutenR {
        Fc0dataoutenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc1dataouten(&self) -> Fc1dataoutenR {
        Fc1dataoutenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc2dataouten(&self) -> Fc2dataoutenR {
        Fc2dataoutenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc4dataouten(&self) -> Fc4dataoutenR {
        Fc4dataoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc5dataouten(&self) -> Fc5dataoutenR {
        Fc5dataoutenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc6dataouten(&self) -> Fc6dataoutenR {
        Fc6dataoutenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc7dataouten(&self) -> Fc7dataoutenR {
        Fc7dataoutenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub fn sharedscksel(&mut self) -> SharedsckselW<'_, SharedctrlsetSpec> {
        SharedsckselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub fn sharedwssel(&mut self) -> SharedwsselW<'_, SharedctrlsetSpec> {
        SharedwsselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub fn shareddatasel(&mut self) -> ShareddataselW<'_, SharedctrlsetSpec> {
        ShareddataselW::new(self, 8)
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc0dataouten(&mut self) -> Fc0dataoutenW<'_, SharedctrlsetSpec> {
        Fc0dataoutenW::new(self, 16)
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc1dataouten(&mut self) -> Fc1dataoutenW<'_, SharedctrlsetSpec> {
        Fc1dataoutenW::new(self, 17)
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc2dataouten(&mut self) -> Fc2dataoutenW<'_, SharedctrlsetSpec> {
        Fc2dataoutenW::new(self, 18)
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc4dataouten(&mut self) -> Fc4dataoutenW<'_, SharedctrlsetSpec> {
        Fc4dataoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc5dataouten(&mut self) -> Fc5dataoutenW<'_, SharedctrlsetSpec> {
        Fc5dataoutenW::new(self, 21)
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc6dataouten(&mut self) -> Fc6dataoutenW<'_, SharedctrlsetSpec> {
        Fc6dataoutenW::new(self, 22)
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc7dataouten(&mut self) -> Fc7dataoutenW<'_, SharedctrlsetSpec> {
        Fc7dataoutenW::new(self, 23)
    }
}
#[doc = "Selects sources and data combinations for shared signal set index.\n\nYou can [`read`](crate::Reg::read) this register and get [`sharedctrlset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharedctrlset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedctrlsetSpec;
impl crate::RegisterSpec for SharedctrlsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharedctrlset::R`](R) reader structure"]
impl crate::Readable for SharedctrlsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sharedctrlset::W`](W) writer structure"]
impl crate::Writable for SharedctrlsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHAREDCTRLSET%s to value 0"]
impl crate::Resettable for SharedctrlsetSpec {}

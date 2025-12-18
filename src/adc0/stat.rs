#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Result FIFO 0 Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy0 {
    #[doc = "0: Result FIFO 0 data level not above watermark level."]
    Rdy0_0 = 0,
    #[doc = "1: Result FIFO 0 holding data above watermark level."]
    Rdy0_1 = 1,
}
impl From<Rdy0> for bool {
    #[inline(always)]
    fn from(variant: Rdy0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY0` reader - Result FIFO 0 Ready Flag"]
pub type Rdy0R = crate::BitReader<Rdy0>;
impl Rdy0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy0 {
        match self.bits {
            false => Rdy0::Rdy0_0,
            true => Rdy0::Rdy0_1,
        }
    }
    #[doc = "Result FIFO 0 data level not above watermark level."]
    #[inline(always)]
    pub fn is_rdy0_0(&self) -> bool {
        *self == Rdy0::Rdy0_0
    }
    #[doc = "Result FIFO 0 holding data above watermark level."]
    #[inline(always)]
    pub fn is_rdy0_1(&self) -> bool {
        *self == Rdy0::Rdy0_1
    }
}
#[doc = "Result FIFO 0 Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fof0 {
    #[doc = "0: No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    Fof0_0 = 0,
    #[doc = "1: At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    Fof0_1 = 1,
}
impl From<Fof0> for bool {
    #[inline(always)]
    fn from(variant: Fof0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOF0` reader - Result FIFO 0 Overflow Flag"]
pub type Fof0R = crate::BitReader<Fof0>;
impl Fof0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fof0 {
        match self.bits {
            false => Fof0::Fof0_0,
            true => Fof0::Fof0_1,
        }
    }
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof0_0(&self) -> bool {
        *self == Fof0::Fof0_0
    }
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof0_1(&self) -> bool {
        *self == Fof0::Fof0_1
    }
}
#[doc = "Field `FOF0` writer - Result FIFO 0 Overflow Flag"]
pub type Fof0W<'a, REG> = crate::BitWriter1C<'a, REG, Fof0>;
impl<'a, REG> Fof0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fof0::Fof0_0)
    }
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fof0::Fof0_1)
    }
}
#[doc = "Result FIFO1 Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy1 {
    #[doc = "0: Result FIFO1 data level not above watermark level."]
    Rdy1_0 = 0,
    #[doc = "1: Result FIFO1 holding data above watermark level."]
    Rdy1_1 = 1,
}
impl From<Rdy1> for bool {
    #[inline(always)]
    fn from(variant: Rdy1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY1` reader - Result FIFO1 Ready Flag"]
pub type Rdy1R = crate::BitReader<Rdy1>;
impl Rdy1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy1 {
        match self.bits {
            false => Rdy1::Rdy1_0,
            true => Rdy1::Rdy1_1,
        }
    }
    #[doc = "Result FIFO1 data level not above watermark level."]
    #[inline(always)]
    pub fn is_rdy1_0(&self) -> bool {
        *self == Rdy1::Rdy1_0
    }
    #[doc = "Result FIFO1 holding data above watermark level."]
    #[inline(always)]
    pub fn is_rdy1_1(&self) -> bool {
        *self == Rdy1::Rdy1_1
    }
}
#[doc = "Result FIFO1 Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fof1 {
    #[doc = "0: No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    Fof1_0 = 0,
    #[doc = "1: At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    Fof1_1 = 1,
}
impl From<Fof1> for bool {
    #[inline(always)]
    fn from(variant: Fof1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOF1` reader - Result FIFO1 Overflow Flag"]
pub type Fof1R = crate::BitReader<Fof1>;
impl Fof1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fof1 {
        match self.bits {
            false => Fof1::Fof1_0,
            true => Fof1::Fof1_1,
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof1_0(&self) -> bool {
        *self == Fof1::Fof1_0
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_fof1_1(&self) -> bool {
        *self == Fof1::Fof1_1
    }
}
#[doc = "Field `FOF1` writer - Result FIFO1 Overflow Flag"]
pub type Fof1W<'a, REG> = crate::BitWriter1C<'a, REG, Fof1>;
impl<'a, REG> Fof1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fof1::Fof1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fof1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fof1::Fof1_1)
    }
}
#[doc = "Interrupt Flag For High Priority Trigger Exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TexcInt {
    #[doc = "0: No trigger exceptions have occurred."]
    TexcInt0 = 0,
    #[doc = "1: A trigger exception has occurred and is pending acknowledgement."]
    TexcInt1 = 1,
}
impl From<TexcInt> for bool {
    #[inline(always)]
    fn from(variant: TexcInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXC_INT` reader - Interrupt Flag For High Priority Trigger Exception"]
pub type TexcIntR = crate::BitReader<TexcInt>;
impl TexcIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TexcInt {
        match self.bits {
            false => TexcInt::TexcInt0,
            true => TexcInt::TexcInt1,
        }
    }
    #[doc = "No trigger exceptions have occurred."]
    #[inline(always)]
    pub fn is_texc_int_0(&self) -> bool {
        *self == TexcInt::TexcInt0
    }
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    #[inline(always)]
    pub fn is_texc_int_1(&self) -> bool {
        *self == TexcInt::TexcInt1
    }
}
#[doc = "Field `TEXC_INT` writer - Interrupt Flag For High Priority Trigger Exception"]
pub type TexcIntW<'a, REG> = crate::BitWriter1C<'a, REG, TexcInt>;
impl<'a, REG> TexcIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger exceptions have occurred."]
    #[inline(always)]
    pub fn texc_int_0(self) -> &'a mut crate::W<REG> {
        self.variant(TexcInt::TexcInt0)
    }
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    #[inline(always)]
    pub fn texc_int_1(self) -> &'a mut crate::W<REG> {
        self.variant(TexcInt::TexcInt1)
    }
}
#[doc = "Interrupt Flag For Trigger Completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcompInt {
    #[doc = "0: Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TcompInt0 = 0,
    #[doc = "1: Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TcompInt1 = 1,
}
impl From<TcompInt> for bool {
    #[inline(always)]
    fn from(variant: TcompInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCOMP_INT` reader - Interrupt Flag For Trigger Completion"]
pub type TcompIntR = crate::BitReader<TcompInt>;
impl TcompIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TcompInt {
        match self.bits {
            false => TcompInt::TcompInt0,
            true => TcompInt::TcompInt1,
        }
    }
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    #[inline(always)]
    pub fn is_tcomp_int_0(&self) -> bool {
        *self == TcompInt::TcompInt0
    }
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    #[inline(always)]
    pub fn is_tcomp_int_1(&self) -> bool {
        *self == TcompInt::TcompInt1
    }
}
#[doc = "Field `TCOMP_INT` writer - Interrupt Flag For Trigger Completion"]
pub type TcompIntW<'a, REG> = crate::BitWriter1C<'a, REG, TcompInt>;
impl<'a, REG> TcompIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    #[inline(always)]
    pub fn tcomp_int_0(self) -> &'a mut crate::W<REG> {
        self.variant(TcompInt::TcompInt0)
    }
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    #[inline(always)]
    pub fn tcomp_int_1(self) -> &'a mut crate::W<REG> {
        self.variant(TcompInt::TcompInt1)
    }
}
#[doc = "Calibration Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalRdy {
    #[doc = "0: Calibration is incomplete or hasn't been ran."]
    CalRdy0 = 0,
    #[doc = "1: The ADC is calibrated."]
    CalRdy1 = 1,
}
impl From<CalRdy> for bool {
    #[inline(always)]
    fn from(variant: CalRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_RDY` reader - Calibration Ready"]
pub type CalRdyR = crate::BitReader<CalRdy>;
impl CalRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalRdy {
        match self.bits {
            false => CalRdy::CalRdy0,
            true => CalRdy::CalRdy1,
        }
    }
    #[doc = "Calibration is incomplete or hasn't been ran."]
    #[inline(always)]
    pub fn is_cal_rdy_0(&self) -> bool {
        *self == CalRdy::CalRdy0
    }
    #[doc = "The ADC is calibrated."]
    #[inline(always)]
    pub fn is_cal_rdy_1(&self) -> bool {
        *self == CalRdy::CalRdy1
    }
}
#[doc = "ADC Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcActive {
    #[doc = "0: The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    AdcActive0 = 0,
    #[doc = "1: The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    AdcActive1 = 1,
}
impl From<AdcActive> for bool {
    #[inline(always)]
    fn from(variant: AdcActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_ACTIVE` reader - ADC Active"]
pub type AdcActiveR = crate::BitReader<AdcActive>;
impl AdcActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcActive {
        match self.bits {
            false => AdcActive::AdcActive0,
            true => AdcActive::AdcActive1,
        }
    }
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    #[inline(always)]
    pub fn is_adc_active_0(&self) -> bool {
        *self == AdcActive::AdcActive0
    }
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    #[inline(always)]
    pub fn is_adc_active_1(&self) -> bool {
        *self == AdcActive::AdcActive1
    }
}
#[doc = "Trigger Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgact {
    #[doc = "0: Command (sequence) associated with Trigger 0 currently being executed."]
    Trgact0 = 0,
    #[doc = "1: Command (sequence) associated with Trigger 1 currently being executed."]
    Trgact1 = 1,
    #[doc = "2: Command (sequence) associated with Trigger 2 currently being executed."]
    Trgact2 = 2,
    #[doc = "3: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact3 = 3,
    #[doc = "4: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact4 = 4,
    #[doc = "5: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact5 = 5,
    #[doc = "6: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact6 = 6,
    #[doc = "7: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact7 = 7,
    #[doc = "8: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact8 = 8,
    #[doc = "9: Command (sequence) from the associated Trigger number is currently being executed."]
    Trgact9 = 9,
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(variant: Trgact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgact {
    type Ux = u8;
}
impl crate::IsEnum for Trgact {}
#[doc = "Field `TRGACT` reader - Trigger Active"]
pub type TrgactR = crate::FieldReader<Trgact>;
impl TrgactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgact> {
        match self.bits {
            0 => Some(Trgact::Trgact0),
            1 => Some(Trgact::Trgact1),
            2 => Some(Trgact::Trgact2),
            3 => Some(Trgact::Trgact3),
            4 => Some(Trgact::Trgact4),
            5 => Some(Trgact::Trgact5),
            6 => Some(Trgact::Trgact6),
            7 => Some(Trgact::Trgact7),
            8 => Some(Trgact::Trgact8),
            9 => Some(Trgact::Trgact9),
            _ => None,
        }
    }
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_0(&self) -> bool {
        *self == Trgact::Trgact0
    }
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_1(&self) -> bool {
        *self == Trgact::Trgact1
    }
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    #[inline(always)]
    pub fn is_trgact_2(&self) -> bool {
        *self == Trgact::Trgact2
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_3(&self) -> bool {
        *self == Trgact::Trgact3
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_4(&self) -> bool {
        *self == Trgact::Trgact4
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_5(&self) -> bool {
        *self == Trgact::Trgact5
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_6(&self) -> bool {
        *self == Trgact::Trgact6
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_7(&self) -> bool {
        *self == Trgact::Trgact7
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_8(&self) -> bool {
        *self == Trgact::Trgact8
    }
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    #[inline(always)]
    pub fn is_trgact_9(&self) -> bool {
        *self == Trgact::Trgact9
    }
}
#[doc = "Command Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdact {
    #[doc = "0: No command is currently in progress."]
    Cmdact0 = 0,
    #[doc = "1: Command 1 currently being executed."]
    Cmdact1 = 1,
    #[doc = "2: Command 2 currently being executed."]
    Cmdact2 = 2,
    #[doc = "3: Associated command number is currently being executed."]
    Cmdact3 = 3,
    #[doc = "4: Associated command number is currently being executed."]
    Cmdact4 = 4,
    #[doc = "5: Associated command number is currently being executed."]
    Cmdact5 = 5,
    #[doc = "6: Associated command number is currently being executed."]
    Cmdact6 = 6,
    #[doc = "7: Associated command number is currently being executed."]
    Cmdact7 = 7,
    #[doc = "8: Associated command number is currently being executed."]
    Cmdact8 = 8,
    #[doc = "9: Associated command number is currently being executed."]
    Cmdact9 = 9,
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(variant: Cmdact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdact {
    type Ux = u8;
}
impl crate::IsEnum for Cmdact {}
#[doc = "Field `CMDACT` reader - Command Active"]
pub type CmdactR = crate::FieldReader<Cmdact>;
impl CmdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdact> {
        match self.bits {
            0 => Some(Cmdact::Cmdact0),
            1 => Some(Cmdact::Cmdact1),
            2 => Some(Cmdact::Cmdact2),
            3 => Some(Cmdact::Cmdact3),
            4 => Some(Cmdact::Cmdact4),
            5 => Some(Cmdact::Cmdact5),
            6 => Some(Cmdact::Cmdact6),
            7 => Some(Cmdact::Cmdact7),
            8 => Some(Cmdact::Cmdact8),
            9 => Some(Cmdact::Cmdact9),
            _ => None,
        }
    }
    #[doc = "No command is currently in progress."]
    #[inline(always)]
    pub fn is_cmdact_0(&self) -> bool {
        *self == Cmdact::Cmdact0
    }
    #[doc = "Command 1 currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_1(&self) -> bool {
        *self == Cmdact::Cmdact1
    }
    #[doc = "Command 2 currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_2(&self) -> bool {
        *self == Cmdact::Cmdact2
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_3(&self) -> bool {
        *self == Cmdact::Cmdact3
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_4(&self) -> bool {
        *self == Cmdact::Cmdact4
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_5(&self) -> bool {
        *self == Cmdact::Cmdact5
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_6(&self) -> bool {
        *self == Cmdact::Cmdact6
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_7(&self) -> bool {
        *self == Cmdact::Cmdact7
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_8(&self) -> bool {
        *self == Cmdact::Cmdact8
    }
    #[doc = "Associated command number is currently being executed."]
    #[inline(always)]
    pub fn is_cmdact_9(&self) -> bool {
        *self == Cmdact::Cmdact9
    }
}
impl R {
    #[doc = "Bit 0 - Result FIFO 0 Ready Flag"]
    #[inline(always)]
    pub fn rdy0(&self) -> Rdy0R {
        Rdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub fn fof0(&self) -> Fof0R {
        Fof0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result FIFO1 Ready Flag"]
    #[inline(always)]
    pub fn rdy1(&self) -> Rdy1R {
        Rdy1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub fn fof1(&self) -> Fof1R {
        Fof1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline(always)]
    pub fn texc_int(&self) -> TexcIntR {
        TexcIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub fn tcomp_int(&self) -> TcompIntR {
        TcompIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Calibration Ready"]
    #[inline(always)]
    pub fn cal_rdy(&self) -> CalRdyR {
        CalRdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Active"]
    #[inline(always)]
    pub fn adc_active(&self) -> AdcActiveR {
        AdcActiveR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger Active"]
    #[inline(always)]
    pub fn trgact(&self) -> TrgactR {
        TrgactR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Active"]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub fn fof0(&mut self) -> Fof0W<'_, StatSpec> {
        Fof0W::new(self, 1)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub fn fof1(&mut self) -> Fof1W<'_, StatSpec> {
        Fof1W::new(self, 3)
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline(always)]
    pub fn texc_int(&mut self) -> TexcIntW<'_, StatSpec> {
        TexcIntW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub fn tcomp_int(&mut self) -> TcompIntW<'_, StatSpec> {
        TcompIntW::new(self, 9)
    }
}
#[doc = "ADC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x030a;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}

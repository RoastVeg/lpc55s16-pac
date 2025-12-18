#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "ADC Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcen {
    #[doc = "0: ADC is disabled."]
    Adcen0 = 0,
    #[doc = "1: ADC is enabled."]
    Adcen1 = 1,
}
impl From<Adcen> for bool {
    #[inline(always)]
    fn from(variant: Adcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC Enable"]
pub type AdcenR = crate::BitReader<Adcen>;
impl AdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcen {
        match self.bits {
            false => Adcen::Adcen0,
            true => Adcen::Adcen1,
        }
    }
    #[doc = "ADC is disabled."]
    #[inline(always)]
    pub fn is_adcen_0(&self) -> bool {
        *self == Adcen::Adcen0
    }
    #[doc = "ADC is enabled."]
    #[inline(always)]
    pub fn is_adcen_1(&self) -> bool {
        *self == Adcen::Adcen1
    }
}
#[doc = "Field `ADCEN` writer - ADC Enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG, Adcen>;
impl<'a, REG> AdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is disabled."]
    #[inline(always)]
    pub fn adcen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Adcen0)
    }
    #[doc = "ADC is enabled."]
    #[inline(always)]
    pub fn adcen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Adcen1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "0: ADC logic is not reset."]
    Rst0 = 0,
    #[doc = "1: ADC logic is reset."]
    Rst1 = 1,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RstR = crate::BitReader<Rst>;
impl RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rst {
        match self.bits {
            false => Rst::Rst0,
            true => Rst::Rst1,
        }
    }
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn is_rst_0(&self) -> bool {
        *self == Rst::Rst0
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn is_rst_1(&self) -> bool {
        *self == Rst::Rst1
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Rst0)
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn rst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Rst1)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dozen {
    #[doc = "0: ADC is enabled in Doze mode."]
    Dozen0 = 0,
    #[doc = "1: ADC is disabled in Doze mode."]
    Dozen1 = 1,
}
impl From<Dozen> for bool {
    #[inline(always)]
    fn from(variant: Dozen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub type DozenR = crate::BitReader<Dozen>;
impl DozenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dozen {
        match self.bits {
            false => Dozen::Dozen0,
            true => Dozen::Dozen1,
        }
    }
    #[doc = "ADC is enabled in Doze mode."]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        *self == Dozen::Dozen0
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        *self == Dozen::Dozen1
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub type DozenW<'a, REG> = crate::BitWriter<'a, REG, Dozen>;
impl<'a, REG> DozenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is enabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::Dozen0)
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::Dozen1)
    }
}
#[doc = "Auto-Calibration Request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalReq {
    #[doc = "0: No request for auto-calibration has been made."]
    CalReq0 = 0,
    #[doc = "1: A request for auto-calibration has been made"]
    CalReq1 = 1,
}
impl From<CalReq> for bool {
    #[inline(always)]
    fn from(variant: CalReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_REQ` reader - Auto-Calibration Request"]
pub type CalReqR = crate::BitReader<CalReq>;
impl CalReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalReq {
        match self.bits {
            false => CalReq::CalReq0,
            true => CalReq::CalReq1,
        }
    }
    #[doc = "No request for auto-calibration has been made."]
    #[inline(always)]
    pub fn is_cal_req_0(&self) -> bool {
        *self == CalReq::CalReq0
    }
    #[doc = "A request for auto-calibration has been made"]
    #[inline(always)]
    pub fn is_cal_req_1(&self) -> bool {
        *self == CalReq::CalReq1
    }
}
#[doc = "Field `CAL_REQ` writer - Auto-Calibration Request"]
pub type CalReqW<'a, REG> = crate::BitWriter<'a, REG, CalReq>;
impl<'a, REG> CalReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request for auto-calibration has been made."]
    #[inline(always)]
    pub fn cal_req_0(self) -> &'a mut crate::W<REG> {
        self.variant(CalReq::CalReq0)
    }
    #[doc = "A request for auto-calibration has been made"]
    #[inline(always)]
    pub fn cal_req_1(self) -> &'a mut crate::W<REG> {
        self.variant(CalReq::CalReq1)
    }
}
#[doc = "Configure for offset calibration function\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calofs {
    #[doc = "0: Calibration function disabled"]
    Calofs0 = 0,
    #[doc = "1: Request for offset calibration function"]
    Calofs1 = 1,
}
impl From<Calofs> for bool {
    #[inline(always)]
    fn from(variant: Calofs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOFS` reader - Configure for offset calibration function"]
pub type CalofsR = crate::BitReader<Calofs>;
impl CalofsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calofs {
        match self.bits {
            false => Calofs::Calofs0,
            true => Calofs::Calofs1,
        }
    }
    #[doc = "Calibration function disabled"]
    #[inline(always)]
    pub fn is_calofs_0(&self) -> bool {
        *self == Calofs::Calofs0
    }
    #[doc = "Request for offset calibration function"]
    #[inline(always)]
    pub fn is_calofs_1(&self) -> bool {
        *self == Calofs::Calofs1
    }
}
#[doc = "Field `CALOFS` writer - Configure for offset calibration function"]
pub type CalofsW<'a, REG> = crate::BitWriter<'a, REG, Calofs>;
impl<'a, REG> CalofsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration function disabled"]
    #[inline(always)]
    pub fn calofs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Calofs::Calofs0)
    }
    #[doc = "Request for offset calibration function"]
    #[inline(always)]
    pub fn calofs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Calofs::Calofs1)
    }
}
#[doc = "Reset FIFO 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfifo0 {
    #[doc = "0: No effect."]
    Rstfifo0_0 = 0,
    #[doc = "1: FIFO 0 is reset."]
    Rstfifo0_1 = 1,
}
impl From<Rstfifo0> for bool {
    #[inline(always)]
    fn from(variant: Rstfifo0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFIFO0` reader - Reset FIFO 0"]
pub type Rstfifo0R = crate::BitReader<Rstfifo0>;
impl Rstfifo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfifo0 {
        match self.bits {
            false => Rstfifo0::Rstfifo0_0,
            true => Rstfifo0::Rstfifo0_1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_rstfifo0_0(&self) -> bool {
        *self == Rstfifo0::Rstfifo0_0
    }
    #[doc = "FIFO 0 is reset."]
    #[inline(always)]
    pub fn is_rstfifo0_1(&self) -> bool {
        *self == Rstfifo0::Rstfifo0_1
    }
}
#[doc = "Field `RSTFIFO0` writer - Reset FIFO 0"]
pub type Rstfifo0W<'a, REG> = crate::BitWriter<'a, REG, Rstfifo0>;
impl<'a, REG> Rstfifo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn rstfifo0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo0::Rstfifo0_0)
    }
    #[doc = "FIFO 0 is reset."]
    #[inline(always)]
    pub fn rstfifo0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo0::Rstfifo0_1)
    }
}
#[doc = "Reset FIFO 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfifo1 {
    #[doc = "0: No effect."]
    Rstfifo1_0 = 0,
    #[doc = "1: FIFO 1 is reset."]
    Rstfifo1_1 = 1,
}
impl From<Rstfifo1> for bool {
    #[inline(always)]
    fn from(variant: Rstfifo1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFIFO1` reader - Reset FIFO 1"]
pub type Rstfifo1R = crate::BitReader<Rstfifo1>;
impl Rstfifo1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfifo1 {
        match self.bits {
            false => Rstfifo1::Rstfifo1_0,
            true => Rstfifo1::Rstfifo1_1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_rstfifo1_0(&self) -> bool {
        *self == Rstfifo1::Rstfifo1_0
    }
    #[doc = "FIFO 1 is reset."]
    #[inline(always)]
    pub fn is_rstfifo1_1(&self) -> bool {
        *self == Rstfifo1::Rstfifo1_1
    }
}
#[doc = "Field `RSTFIFO1` writer - Reset FIFO 1"]
pub type Rstfifo1W<'a, REG> = crate::BitWriter<'a, REG, Rstfifo1>;
impl<'a, REG> Rstfifo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn rstfifo1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo1::Rstfifo1_0)
    }
    #[doc = "FIFO 1 is reset."]
    #[inline(always)]
    pub fn rstfifo1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfifo1::Rstfifo1_1)
    }
}
#[doc = "Auto-Calibration Averages\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CalAvgs {
    #[doc = "0: Single conversion."]
    CalAvgs0 = 0,
    #[doc = "1: 2 conversions averaged."]
    CalAvgs1 = 1,
    #[doc = "2: 4 conversions averaged."]
    CalAvgs2 = 2,
    #[doc = "3: 8 conversions averaged."]
    CalAvgs3 = 3,
    #[doc = "4: 16 conversions averaged."]
    CalAvgs4 = 4,
    #[doc = "5: 32 conversions averaged."]
    CalAvgs5 = 5,
    #[doc = "6: 64 conversions averaged."]
    CalAvgs6 = 6,
    #[doc = "7: 128 conversions averaged."]
    CalAvgs7 = 7,
}
impl From<CalAvgs> for u8 {
    #[inline(always)]
    fn from(variant: CalAvgs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CalAvgs {
    type Ux = u8;
}
impl crate::IsEnum for CalAvgs {}
#[doc = "Field `CAL_AVGS` reader - Auto-Calibration Averages"]
pub type CalAvgsR = crate::FieldReader<CalAvgs>;
impl CalAvgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalAvgs {
        match self.bits {
            0 => CalAvgs::CalAvgs0,
            1 => CalAvgs::CalAvgs1,
            2 => CalAvgs::CalAvgs2,
            3 => CalAvgs::CalAvgs3,
            4 => CalAvgs::CalAvgs4,
            5 => CalAvgs::CalAvgs5,
            6 => CalAvgs::CalAvgs6,
            7 => CalAvgs::CalAvgs7,
            _ => unreachable!(),
        }
    }
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn is_cal_avgs_0(&self) -> bool {
        *self == CalAvgs::CalAvgs0
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_1(&self) -> bool {
        *self == CalAvgs::CalAvgs1
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_2(&self) -> bool {
        *self == CalAvgs::CalAvgs2
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_3(&self) -> bool {
        *self == CalAvgs::CalAvgs3
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_4(&self) -> bool {
        *self == CalAvgs::CalAvgs4
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_5(&self) -> bool {
        *self == CalAvgs::CalAvgs5
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_6(&self) -> bool {
        *self == CalAvgs::CalAvgs6
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn is_cal_avgs_7(&self) -> bool {
        *self == CalAvgs::CalAvgs7
    }
}
#[doc = "Field `CAL_AVGS` writer - Auto-Calibration Averages"]
pub type CalAvgsW<'a, REG> = crate::FieldWriter<'a, REG, 3, CalAvgs, crate::Safe>;
impl<'a, REG> CalAvgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn cal_avgs_0(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs0)
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_1(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs1)
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_2(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs2)
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_3(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs3)
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_4(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs4)
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_5(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs5)
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_6(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs6)
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_7(self) -> &'a mut crate::W<REG> {
        self.variant(CalAvgs::CalAvgs7)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DozenR {
        DozenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline(always)]
    pub fn cal_req(&self) -> CalReqR {
        CalReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline(always)]
    pub fn calofs(&self) -> CalofsR {
        CalofsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    pub fn rstfifo0(&self) -> Rstfifo0R {
        Rstfifo0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    pub fn rstfifo1(&self) -> Rstfifo1R {
        Rstfifo1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline(always)]
    pub fn cal_avgs(&self) -> CalAvgsR {
        CalAvgsR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("adcen", &self.adcen())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("cal_req", &self.cal_req())
            .field("calofs", &self.calofs())
            .field("rstfifo0", &self.rstfifo0())
            .field("rstfifo1", &self.rstfifo1())
            .field("cal_avgs", &self.cal_avgs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, CtrlSpec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, CtrlSpec> {
        RstW::new(self, 1)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DozenW<'_, CtrlSpec> {
        DozenW::new(self, 2)
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline(always)]
    pub fn cal_req(&mut self) -> CalReqW<'_, CtrlSpec> {
        CalReqW::new(self, 3)
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline(always)]
    pub fn calofs(&mut self) -> CalofsW<'_, CtrlSpec> {
        CalofsW::new(self, 4)
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    pub fn rstfifo0(&mut self) -> Rstfifo0W<'_, CtrlSpec> {
        Rstfifo0W::new(self, 8)
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    pub fn rstfifo1(&mut self) -> Rstfifo1W<'_, CtrlSpec> {
        Rstfifo1W::new(self, 9)
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline(always)]
    pub fn cal_avgs(&mut self) -> CalAvgsW<'_, CtrlSpec> {
        CalAvgsW::new(self, 16)
    }
}
#[doc = "ADC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

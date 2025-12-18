#[doc = "Register `FMCCR` reader"]
pub type R = crate::R<FmccrSpec>;
#[doc = "Register `FMCCR` writer"]
pub type W = crate::W<FmccrSpec>;
#[doc = "Instruction fetch configuration.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fetchcfg {
    #[doc = "0: Instruction fetches from flash are not buffered."]
    Nobuf = 0,
    #[doc = "1: One buffer is used for all instruction fetches."]
    Onebuf = 1,
    #[doc = "2: All buffers may be used for instruction fetches."]
    Allbuf = 2,
}
impl From<Fetchcfg> for u8 {
    #[inline(always)]
    fn from(variant: Fetchcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fetchcfg {
    type Ux = u8;
}
impl crate::IsEnum for Fetchcfg {}
#[doc = "Field `FETCHCFG` reader - Instruction fetch configuration."]
pub type FetchcfgR = crate::FieldReader<Fetchcfg>;
impl FetchcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fetchcfg> {
        match self.bits {
            0 => Some(Fetchcfg::Nobuf),
            1 => Some(Fetchcfg::Onebuf),
            2 => Some(Fetchcfg::Allbuf),
            _ => None,
        }
    }
    #[doc = "Instruction fetches from flash are not buffered."]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == Fetchcfg::Nobuf
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == Fetchcfg::Onebuf
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == Fetchcfg::Allbuf
    }
}
#[doc = "Field `FETCHCFG` writer - Instruction fetch configuration."]
pub type FetchcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fetchcfg>;
impl<'a, REG> FetchcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Instruction fetches from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut crate::W<REG> {
        self.variant(Fetchcfg::Nobuf)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut crate::W<REG> {
        self.variant(Fetchcfg::Onebuf)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut crate::W<REG> {
        self.variant(Fetchcfg::Allbuf)
    }
}
#[doc = "Data read configuration.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datacfg {
    #[doc = "0: Data accesses from flash are not buffered."]
    Nobuf = 0,
    #[doc = "1: One buffer is used for all data accesses."]
    Onebuf = 1,
    #[doc = "2: All buffers can be used for data accesses."]
    Allbuf = 2,
}
impl From<Datacfg> for u8 {
    #[inline(always)]
    fn from(variant: Datacfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datacfg {
    type Ux = u8;
}
impl crate::IsEnum for Datacfg {}
#[doc = "Field `DATACFG` reader - Data read configuration."]
pub type DatacfgR = crate::FieldReader<Datacfg>;
impl DatacfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datacfg> {
        match self.bits {
            0 => Some(Datacfg::Nobuf),
            1 => Some(Datacfg::Onebuf),
            2 => Some(Datacfg::Allbuf),
            _ => None,
        }
    }
    #[doc = "Data accesses from flash are not buffered."]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == Datacfg::Nobuf
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == Datacfg::Onebuf
    }
    #[doc = "All buffers can be used for data accesses."]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == Datacfg::Allbuf
    }
}
#[doc = "Field `DATACFG` writer - Data read configuration."]
pub type DatacfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datacfg>;
impl<'a, REG> DatacfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data accesses from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut crate::W<REG> {
        self.variant(Datacfg::Nobuf)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut crate::W<REG> {
        self.variant(Datacfg::Onebuf)
    }
    #[doc = "All buffers can be used for data accesses."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut crate::W<REG> {
        self.variant(Datacfg::Allbuf)
    }
}
#[doc = "Acceleration enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accel {
    #[doc = "0: Flash acceleration is disabled."]
    Disable = 0,
    #[doc = "1: Flash acceleration is enabled."]
    Enable = 1,
}
impl From<Accel> for bool {
    #[inline(always)]
    fn from(variant: Accel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCEL` reader - Acceleration enable."]
pub type AccelR = crate::BitReader<Accel>;
impl AccelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accel {
        match self.bits {
            false => Accel::Disable,
            true => Accel::Enable,
        }
    }
    #[doc = "Flash acceleration is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Accel::Disable
    }
    #[doc = "Flash acceleration is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Accel::Enable
    }
}
#[doc = "Field `ACCEL` writer - Acceleration enable."]
pub type AccelW<'a, REG> = crate::BitWriter<'a, REG, Accel>;
impl<'a, REG> AccelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash acceleration is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Accel::Disable)
    }
    #[doc = "Flash acceleration is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Accel::Enable)
    }
}
#[doc = "Prefetch enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefen {
    #[doc = "0: No instruction prefetch is performed."]
    Disable = 0,
    #[doc = "1: Instruction prefetch is enabled."]
    Enable = 1,
}
impl From<Prefen> for bool {
    #[inline(always)]
    fn from(variant: Prefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFEN` reader - Prefetch enable."]
pub type PrefenR = crate::BitReader<Prefen>;
impl PrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prefen {
        match self.bits {
            false => Prefen::Disable,
            true => Prefen::Enable,
        }
    }
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prefen::Disable
    }
    #[doc = "Instruction prefetch is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prefen::Enable
    }
}
#[doc = "Field `PREFEN` writer - Prefetch enable."]
pub type PrefenW<'a, REG> = crate::BitWriter<'a, REG, Prefen>;
impl<'a, REG> PrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefen::Disable)
    }
    #[doc = "Instruction prefetch is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefen::Enable)
    }
}
#[doc = "Prefetch override.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefovr {
    #[doc = "0: Any previously initiated prefetch will be completed."]
    Normal = 0,
    #[doc = "1: Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    Override = 1,
}
impl From<Prefovr> for bool {
    #[inline(always)]
    fn from(variant: Prefovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFOVR` reader - Prefetch override."]
pub type PrefovrR = crate::BitReader<Prefovr>;
impl PrefovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prefovr {
        match self.bits {
            false => Prefovr::Normal,
            true => Prefovr::Override,
        }
    }
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Prefovr::Normal
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Prefovr::Override
    }
}
#[doc = "Field `PREFOVR` writer - Prefetch override."]
pub type PrefovrW<'a, REG> = crate::BitWriter<'a, REG, Prefovr>;
impl<'a, REG> PrefovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Prefovr::Normal)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Prefovr::Override)
    }
}
#[doc = "Flash memory access time.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashtim {
    #[doc = "0: 1 system clock flash access time (for system clock rates up to 11 MHz)."]
    Flashtim0 = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    Flashtim1 = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    Flashtim2 = 2,
    #[doc = "3: 4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    Flashtim3 = 3,
    #[doc = "4: 5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    Flashtim4 = 4,
    #[doc = "5: 6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    Flashtim5 = 5,
    #[doc = "6: 7 system clocks flash access time (for system clock rates up to 84 MHz)."]
    Flashtim6 = 6,
    #[doc = "7: 8 system clocks flash access time (for system clock rates up to 104 MHz)."]
    Flashtim7 = 7,
    #[doc = "8: 9 system clocks flash access time (for system clock rates up to 119 MHz)."]
    Flashtim8 = 8,
    #[doc = "9: 10 system clocks flash access time (for system clock rates up to 129 MHz)."]
    Flashtim9 = 9,
    #[doc = "10: 11 system clocks flash access time (for system clock rates up to 144 MHz)."]
    Flashtim10 = 10,
    #[doc = "11: 12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    Flashtim11 = 11,
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(variant: Flashtim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashtim {
    type Ux = u8;
}
impl crate::IsEnum for Flashtim {}
#[doc = "Field `FLASHTIM` reader - Flash memory access time."]
pub type FlashtimR = crate::FieldReader<Flashtim>;
impl FlashtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flashtim> {
        match self.bits {
            0 => Some(Flashtim::Flashtim0),
            1 => Some(Flashtim::Flashtim1),
            2 => Some(Flashtim::Flashtim2),
            3 => Some(Flashtim::Flashtim3),
            4 => Some(Flashtim::Flashtim4),
            5 => Some(Flashtim::Flashtim5),
            6 => Some(Flashtim::Flashtim6),
            7 => Some(Flashtim::Flashtim7),
            8 => Some(Flashtim::Flashtim8),
            9 => Some(Flashtim::Flashtim9),
            10 => Some(Flashtim::Flashtim10),
            11 => Some(Flashtim::Flashtim11),
            _ => None,
        }
    }
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    #[inline(always)]
    pub fn is_flashtim0(&self) -> bool {
        *self == Flashtim::Flashtim0
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    #[inline(always)]
    pub fn is_flashtim1(&self) -> bool {
        *self == Flashtim::Flashtim1
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    #[inline(always)]
    pub fn is_flashtim2(&self) -> bool {
        *self == Flashtim::Flashtim2
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    #[inline(always)]
    pub fn is_flashtim3(&self) -> bool {
        *self == Flashtim::Flashtim3
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    #[inline(always)]
    pub fn is_flashtim4(&self) -> bool {
        *self == Flashtim::Flashtim4
    }
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    #[inline(always)]
    pub fn is_flashtim5(&self) -> bool {
        *self == Flashtim::Flashtim5
    }
    #[doc = "7 system clocks flash access time (for system clock rates up to 84 MHz)."]
    #[inline(always)]
    pub fn is_flashtim6(&self) -> bool {
        *self == Flashtim::Flashtim6
    }
    #[doc = "8 system clocks flash access time (for system clock rates up to 104 MHz)."]
    #[inline(always)]
    pub fn is_flashtim7(&self) -> bool {
        *self == Flashtim::Flashtim7
    }
    #[doc = "9 system clocks flash access time (for system clock rates up to 119 MHz)."]
    #[inline(always)]
    pub fn is_flashtim8(&self) -> bool {
        *self == Flashtim::Flashtim8
    }
    #[doc = "10 system clocks flash access time (for system clock rates up to 129 MHz)."]
    #[inline(always)]
    pub fn is_flashtim9(&self) -> bool {
        *self == Flashtim::Flashtim9
    }
    #[doc = "11 system clocks flash access time (for system clock rates up to 144 MHz)."]
    #[inline(always)]
    pub fn is_flashtim10(&self) -> bool {
        *self == Flashtim::Flashtim10
    }
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    #[inline(always)]
    pub fn is_flashtim11(&self) -> bool {
        *self == Flashtim::Flashtim11
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time."]
pub type FlashtimW<'a, REG> = crate::FieldWriter<'a, REG, 4, Flashtim>;
impl<'a, REG> FlashtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    #[inline(always)]
    pub fn flashtim0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim0)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    #[inline(always)]
    pub fn flashtim1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim1)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    #[inline(always)]
    pub fn flashtim2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim2)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    #[inline(always)]
    pub fn flashtim3(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim3)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    #[inline(always)]
    pub fn flashtim4(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim4)
    }
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    #[inline(always)]
    pub fn flashtim5(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim5)
    }
    #[doc = "7 system clocks flash access time (for system clock rates up to 84 MHz)."]
    #[inline(always)]
    pub fn flashtim6(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim6)
    }
    #[doc = "8 system clocks flash access time (for system clock rates up to 104 MHz)."]
    #[inline(always)]
    pub fn flashtim7(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim7)
    }
    #[doc = "9 system clocks flash access time (for system clock rates up to 119 MHz)."]
    #[inline(always)]
    pub fn flashtim8(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim8)
    }
    #[doc = "10 system clocks flash access time (for system clock rates up to 129 MHz)."]
    #[inline(always)]
    pub fn flashtim9(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim9)
    }
    #[doc = "11 system clocks flash access time (for system clock rates up to 144 MHz)."]
    #[inline(always)]
    pub fn flashtim10(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim10)
    }
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    #[inline(always)]
    pub fn flashtim11(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::Flashtim11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FetchcfgR {
        FetchcfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    pub fn datacfg(&self) -> DatacfgR {
        DatacfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> AccelR {
        AccelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PrefenR {
        PrefenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    pub fn prefovr(&self) -> PrefovrR {
        PrefovrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&self) -> FlashtimR {
        FlashtimR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCCR")
            .field("fetchcfg", &self.fetchcfg())
            .field("datacfg", &self.datacfg())
            .field("accel", &self.accel())
            .field("prefen", &self.prefen())
            .field("prefovr", &self.prefovr())
            .field("flashtim", &self.flashtim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    pub fn fetchcfg(&mut self) -> FetchcfgW<'_, FmccrSpec> {
        FetchcfgW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    pub fn datacfg(&mut self) -> DatacfgW<'_, FmccrSpec> {
        DatacfgW::new(self, 2)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&mut self) -> AccelW<'_, FmccrSpec> {
        AccelW::new(self, 4)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&mut self) -> PrefenW<'_, FmccrSpec> {
        PrefenW::new(self, 5)
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    pub fn prefovr(&mut self) -> PrefovrW<'_, FmccrSpec> {
        PrefovrW::new(self, 6)
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FlashtimW<'_, FmccrSpec> {
        FlashtimW::new(self, 12)
    }
}
#[doc = "FMC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmccrSpec;
impl crate::RegisterSpec for FmccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmccr::R`](R) reader structure"]
impl crate::Readable for FmccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fmccr::W`](W) writer structure"]
impl crate::Writable for FmccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMCCR to value 0x2000"]
impl crate::Resettable for FmccrSpec {
    const RESET_VALUE: u32 = 0x2000;
}

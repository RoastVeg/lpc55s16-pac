#[doc = "Register `SRAMCTRL` reader"]
pub type R = crate::R<SramctrlSpec>;
#[doc = "Register `SRAMCTRL` writer"]
pub type W = crate::W<SramctrlSpec>;
#[doc = "Source Biasing voltage.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smb {
    #[doc = "0: Low leakage."]
    Low = 0,
    #[doc = "1: Medium leakage."]
    Medium = 1,
    #[doc = "2: Highest leakage."]
    Highest = 2,
    #[doc = "3: Disable."]
    Disable = 3,
}
impl From<Smb> for u8 {
    #[inline(always)]
    fn from(variant: Smb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smb {
    type Ux = u8;
}
impl crate::IsEnum for Smb {}
#[doc = "Field `SMB` reader - Source Biasing voltage."]
pub type SmbR = crate::FieldReader<Smb>;
impl SmbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smb {
        match self.bits {
            0 => Smb::Low,
            1 => Smb::Medium,
            2 => Smb::Highest,
            3 => Smb::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Smb::Low
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Smb::Medium
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == Smb::Highest
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Smb::Disable
    }
}
#[doc = "Field `SMB` writer - Source Biasing voltage."]
pub type SmbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Smb, crate::Safe>;
impl<'a, REG> SmbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Low)
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Medium)
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Highest)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Smb::Disable)
    }
}
#[doc = "Field `RM` reader - Read Margin control settings."]
pub type RmR = crate::FieldReader;
#[doc = "Field `RM` writer - Read Margin control settings."]
pub type RmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WM` reader - Write Margin control settings."]
pub type WmR = crate::FieldReader;
#[doc = "Field `WM` writer - Write Margin control settings."]
pub type WmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRME` reader - Write read margin enable."]
pub type WrmeR = crate::BitReader;
#[doc = "Field `WRME` writer - Write read margin enable."]
pub type WrmeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&self) -> SmbR {
        SmbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&self) -> WrmeR {
        WrmeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&mut self) -> SmbW<'_, SramctrlSpec> {
        SmbW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<'_, SramctrlSpec> {
        RmW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&mut self) -> WmW<'_, SramctrlSpec> {
        WmW::new(self, 5)
    }
    #[doc = "Bit 8 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&mut self) -> WrmeW<'_, SramctrlSpec> {
        WrmeW::new(self, 8)
    }
}
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramctrlSpec;
impl crate::RegisterSpec for SramctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramctrl::R`](R) reader structure"]
impl crate::Readable for SramctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sramctrl::W`](W) writer structure"]
impl crate::Writable for SramctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAMCTRL to value 0x01"]
impl crate::Resettable for SramctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}

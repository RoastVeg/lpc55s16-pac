#[doc = "Register `XTAL_32KHZ_CAPABANK_TRIM` reader"]
pub type R = crate::R<Xtal32khzCapabankTrimSpec>;
#[doc = "Register `XTAL_32KHZ_CAPABANK_TRIM` writer"]
pub type W = crate::W<Xtal32khzCapabankTrimSpec>;
#[doc = "XTAL 32kHz capa bank trimmings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrimValid {
    #[doc = "0: Capa Bank trimmings not valid. Default trimmings value are used"]
    NotTrim = 0,
    #[doc = "1: Capa Bank trimmings valid"]
    Valid = 1,
}
impl From<TrimValid> for bool {
    #[inline(always)]
    fn from(variant: TrimValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIM_VALID` reader - XTAL 32kHz capa bank trimmings"]
pub type TrimValidR = crate::BitReader<TrimValid>;
impl TrimValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrimValid {
        match self.bits {
            false => TrimValid::NotTrim,
            true => TrimValid::Valid,
        }
    }
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used"]
    #[inline(always)]
    pub fn is_not_trim(&self) -> bool {
        *self == TrimValid::NotTrim
    }
    #[doc = "Capa Bank trimmings valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == TrimValid::Valid
    }
}
#[doc = "Field `TRIM_VALID` writer - XTAL 32kHz capa bank trimmings"]
pub type TrimValidW<'a, REG> = crate::BitWriter<'a, REG, TrimValid>;
impl<'a, REG> TrimValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used"]
    #[inline(always)]
    pub fn not_trim(self) -> &'a mut crate::W<REG> {
        self.variant(TrimValid::NotTrim)
    }
    #[doc = "Capa Bank trimmings valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(TrimValid::Valid)
    }
}
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` reader - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub type XtalLoadCapIecPfX100R = crate::FieldReader<u16>;
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` writer - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub type XtalLoadCapIecPfX100W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` reader - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PcbXinParaCapPfX100R = crate::FieldReader<u16>;
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` writer - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PcbXinParaCapPfX100W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` reader - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PcbXoutParaCapPfX100R = crate::FieldReader<u16>;
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` writer - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PcbXoutParaCapPfX100W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - XTAL 32kHz capa bank trimmings"]
    #[inline(always)]
    pub fn trim_valid(&self) -> TrimValidR {
        TrimValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn xtal_load_cap_iec_pf_x100(&self) -> XtalLoadCapIecPfX100R {
        XtalLoadCapIecPfX100R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xin_para_cap_pf_x100(&self) -> PcbXinParaCapPfX100R {
        PcbXinParaCapPfX100R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xout_para_cap_pf_x100(&self) -> PcbXoutParaCapPfX100R {
        PcbXoutParaCapPfX100R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL 32kHz capa bank trimmings"]
    #[inline(always)]
    pub fn trim_valid(&mut self) -> TrimValidW<'_, Xtal32khzCapabankTrimSpec> {
        TrimValidW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn xtal_load_cap_iec_pf_x100(
        &mut self,
    ) -> XtalLoadCapIecPfX100W<'_, Xtal32khzCapabankTrimSpec> {
        XtalLoadCapIecPfX100W::new(self, 1)
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xin_para_cap_pf_x100(
        &mut self,
    ) -> PcbXinParaCapPfX100W<'_, Xtal32khzCapabankTrimSpec> {
        PcbXinParaCapPfX100W::new(self, 11)
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xout_para_cap_pf_x100(
        &mut self,
    ) -> PcbXoutParaCapPfX100W<'_, Xtal32khzCapabankTrimSpec> {
        PcbXoutParaCapPfX100W::new(self, 21)
    }
}
#[doc = "Xtal 32kHz capabank triming.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_32khz_capabank_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_32khz_capabank_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32khzCapabankTrimSpec;
impl crate::RegisterSpec for Xtal32khzCapabankTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_32khz_capabank_trim::R`](R) reader structure"]
impl crate::Readable for Xtal32khzCapabankTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_32khz_capabank_trim::W`](W) writer structure"]
impl crate::Writable for Xtal32khzCapabankTrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_32KHZ_CAPABANK_TRIM to value 0"]
impl crate::Resettable for Xtal32khzCapabankTrimSpec {}

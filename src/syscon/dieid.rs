#[doc = "Register `DIEID` reader"]
pub type R = crate::R<DieidSpec>;
#[doc = "Field `REV_ID` reader - Chip Metal Revision ID."]
pub type RevIdR = crate::FieldReader;
#[doc = "Field `MCO_NUM_IN_DIE_ID` reader - Chip Number 0x426B."]
pub type McoNumInDieIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Chip Metal Revision ID."]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Chip Number 0x426B."]
    #[inline(always)]
    pub fn mco_num_in_die_id(&self) -> McoNumInDieIdR {
        McoNumInDieIdR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
#[doc = "Chip revision ID and Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dieid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieidSpec;
impl crate::RegisterSpec for DieidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieid::R`](R) reader structure"]
impl crate::Readable for DieidSpec {}
#[doc = "`reset()` method sets DIEID to value 0x0004_26b0"]
impl crate::Resettable for DieidSpec {
    const RESET_VALUE: u32 = 0x0004_26b0;
}

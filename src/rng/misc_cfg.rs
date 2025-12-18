#[doc = "Register `MISC_CFG` reader"]
pub type R = crate::R<MiscCfgSpec>;
#[doc = "Register `MISC_CFG` writer"]
pub type W = crate::W<MiscCfgSpec>;
#[doc = "Field `AES_RESEED` reader - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
pub type AesReseedR = crate::BitReader;
#[doc = "Field `AES_RESEED` writer - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
pub type AesReseedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_DT_CFG` reader - Set this bit to re-seed AES."]
pub type AesDtCfgR = crate::BitReader;
#[doc = "Field `AES_DT_CFG` writer - Set this bit to re-seed AES."]
pub type AesDtCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&self) -> AesReseedR {
        AesReseedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&self) -> AesDtCfgR {
        AesDtCfgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&mut self) -> AesReseedW<'_, MiscCfgSpec> {
        AesReseedW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&mut self) -> AesDtCfgW<'_, MiscCfgSpec> {
        AesDtCfgW::new(self, 1)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscCfgSpec;
impl crate::RegisterSpec for MiscCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_cfg::R`](R) reader structure"]
impl crate::Readable for MiscCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_cfg::W`](W) writer structure"]
impl crate::Writable for MiscCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CFG to value 0"]
impl crate::Resettable for MiscCfgSpec {}

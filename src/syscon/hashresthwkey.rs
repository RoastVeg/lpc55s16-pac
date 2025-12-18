#[doc = "Register `HASHRESTHWKEY` reader"]
pub type R = crate::R<HashresthwkeySpec>;
#[doc = "Register `HASHRESTHWKEY` writer"]
pub type W = crate::W<HashresthwkeySpec>;
#[doc = "Code value that controls whether HASH AES hardware secret key is unlocked\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Unlockcode {
    #[doc = "3275531610: HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    Unlock = 3275531610,
}
impl From<Unlockcode> for u32 {
    #[inline(always)]
    fn from(variant: Unlockcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Unlockcode {
    type Ux = u32;
}
impl crate::IsEnum for Unlockcode {}
#[doc = "Field `UNLOCKCODE` reader - Code value that controls whether HASH AES hardware secret key is unlocked"]
pub type UnlockcodeR = crate::FieldReader<Unlockcode>;
impl UnlockcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Unlockcode> {
        match self.bits {
            3275531610 => Some(Unlockcode::Unlock),
            _ => None,
        }
    }
    #[doc = "HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Unlockcode::Unlock
    }
}
#[doc = "Field `UNLOCKCODE` writer - Code value that controls whether HASH AES hardware secret key is unlocked"]
pub type UnlockcodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, Unlockcode>;
impl<'a, REG> UnlockcodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Unlockcode::Unlock)
    }
}
impl R {
    #[doc = "Bits 0:31 - Code value that controls whether HASH AES hardware secret key is unlocked"]
    #[inline(always)]
    pub fn unlockcode(&self) -> UnlockcodeR {
        UnlockcodeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASHRESTHWKEY")
            .field("unlockcode", &self.unlockcode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Code value that controls whether HASH AES hardware secret key is unlocked"]
    #[inline(always)]
    pub fn unlockcode(&mut self) -> UnlockcodeW<'_, HashresthwkeySpec> {
        UnlockcodeW::new(self, 0)
    }
}
#[doc = "Controls whether the HASH AES hardware secret key is restricted to use by secure code\n\nYou can [`read`](crate::Reg::read) this register and get [`hashresthwkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashresthwkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashresthwkeySpec;
impl crate::RegisterSpec for HashresthwkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashresthwkey::R`](R) reader structure"]
impl crate::Readable for HashresthwkeySpec {}
#[doc = "`write(|w| ..)` method takes [`hashresthwkey::W`](W) writer structure"]
impl crate::Writable for HashresthwkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASHRESTHWKEY to value 0"]
impl crate::Resettable for HashresthwkeySpec {}

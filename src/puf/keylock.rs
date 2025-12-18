#[doc = "Register `KEYLOCK` reader"]
pub type R = crate::R<KeylockSpec>;
#[doc = "Register `KEYLOCK` writer"]
pub type W = crate::W<KeylockSpec>;
#[doc = "Field `KEY0` reader - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key0R = crate::FieldReader;
#[doc = "Field `KEY0` writer - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY1` reader - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key1R = crate::FieldReader;
#[doc = "Field `KEY1` writer - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY2` reader - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key2R = crate::FieldReader;
#[doc = "Field `KEY2` writer - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY3` reader - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key3R = crate::FieldReader;
#[doc = "Field `KEY3` writer - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYLOCK")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<'_, KeylockSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<'_, KeylockSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, KeylockSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, KeylockSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "Only reset in case of full IC reset\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeylockSpec;
impl crate::RegisterSpec for KeylockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylock::R`](R) reader structure"]
impl crate::Readable for KeylockSpec {}
#[doc = "`write(|w| ..)` method takes [`keylock::W`](W) writer structure"]
impl crate::Writable for KeylockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYLOCK to value 0xaa"]
impl crate::Resettable for KeylockSpec {
    const RESET_VALUE: u32 = 0xaa;
}

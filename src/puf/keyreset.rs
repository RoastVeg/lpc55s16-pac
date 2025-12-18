#[doc = "Register `KEYRESET` writer"]
pub type W = crate::W<KeyresetSpec>;
#[doc = "Field `KEY0` writer - 10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY1` writer - 10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY2` writer - 10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY3` writer - 10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bits 0:1 - 10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<'_, KeyresetSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<'_, KeyresetSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, KeyresetSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, KeyresetSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "Reinitialize Keys shift registers counters\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyreset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyresetSpec;
impl crate::RegisterSpec for KeyresetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyreset::W`](W) writer structure"]
impl crate::Writable for KeyresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYRESET to value 0"]
impl crate::Resettable for KeyresetSpec {}

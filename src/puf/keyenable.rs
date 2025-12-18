#[doc = "Register `KEYENABLE` reader"]
pub type R = crate::R<KeyenableSpec>;
#[doc = "Register `KEYENABLE` writer"]
pub type W = crate::W<KeyenableSpec>;
#[doc = "Field `KEY0` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type Key0R = crate::FieldReader;
#[doc = "Field `KEY0` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY1` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type Key1R = crate::FieldReader;
#[doc = "Field `KEY1` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY2` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type Key2R = crate::FieldReader;
#[doc = "Field `KEY2` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY3` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type Key3R = crate::FieldReader;
#[doc = "Field `KEY3` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYENABLE")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<'_, KeyenableSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<'_, KeyenableSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, KeyenableSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, KeyenableSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`keyenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyenableSpec;
impl crate::RegisterSpec for KeyenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyenable::R`](R) reader structure"]
impl crate::Readable for KeyenableSpec {}
#[doc = "`write(|w| ..)` method takes [`keyenable::W`](W) writer structure"]
impl crate::Writable for KeyenableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KeyenableSpec {
    const RESET_VALUE: u32 = 0x55;
}

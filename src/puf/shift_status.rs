#[doc = "Register `SHIFT_STATUS` reader"]
pub type R = crate::R<ShiftStatusSpec>;
#[doc = "Field `KEY0` reader - Index counter from key 0 shift register"]
pub type Key0R = crate::FieldReader;
#[doc = "Field `KEY1` reader - Index counter from key 1 shift register"]
pub type Key1R = crate::FieldReader;
#[doc = "Field `KEY2` reader - Index counter from key 2 shift register"]
pub type Key2R = crate::FieldReader;
#[doc = "Field `KEY3` reader - Index counter from key 3 shift register"]
pub type Key3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Index counter from key 0 shift register"]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Index counter from key 1 shift register"]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Index counter from key 2 shift register"]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Index counter from key 3 shift register"]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHIFT_STATUS")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`shift_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftStatusSpec;
impl crate::RegisterSpec for ShiftStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shift_status::R`](R) reader structure"]
impl crate::Readable for ShiftStatusSpec {}
#[doc = "`reset()` method sets SHIFT_STATUS to value 0"]
impl crate::Resettable for ShiftStatusSpec {}

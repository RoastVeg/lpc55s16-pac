#[doc = "Register `GPO0_2` reader"]
pub type R = crate::R<Gpo0Gpo0_2Spec>;
#[doc = "Register `GPO0_2` writer"]
pub type W = crate::W<Gpo0Gpo0_2Spec>;
#[doc = "Field `SYSTEM_SPEED_CODE` reader - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
pub type SystemSpeedCodeR = crate::FieldReader;
#[doc = "Field `SYSTEM_SPEED_CODE` writer - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
pub type SystemSpeedCodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_CTRL_OPMODE` reader - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
pub type FlashCtrlOpmodeR = crate::FieldReader;
#[doc = "Field `FLASH_CTRL_OPMODE` writer - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
pub type FlashCtrlOpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:1 - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
    #[inline(always)]
    pub fn system_speed_code(&self) -> SystemSpeedCodeR {
        SystemSpeedCodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[inline(always)]
    pub fn flash_ctrl_opmode(&self) -> FlashCtrlOpmodeR {
        FlashCtrlOpmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_GPO0_2")
            .field("system_speed_code", &self.system_speed_code())
            .field("flash_ctrl_opmode", &self.flash_ctrl_opmode())
            .field("field", &self.field())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
    #[inline(always)]
    pub fn system_speed_code(&mut self) -> SystemSpeedCodeW<'_, Gpo0Gpo0_2Spec> {
        SystemSpeedCodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[inline(always)]
    pub fn flash_ctrl_opmode(&mut self) -> FlashCtrlOpmodeW<'_, Gpo0Gpo0_2Spec> {
        FlashCtrlOpmodeW::new(self, 2)
    }
    #[doc = "Bits 4:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, Gpo0Gpo0_2Spec> {
        FieldW::new(self, 4)
    }
}
#[doc = "GPO0 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpo0Gpo0_2Spec;
impl crate::RegisterSpec for Gpo0Gpo0_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo0_gpo0_2::R`](R) reader structure"]
impl crate::Readable for Gpo0Gpo0_2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo0_gpo0_2::W`](W) writer structure"]
impl crate::Writable for Gpo0Gpo0_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPO0_2 to value 0"]
impl crate::Resettable for Gpo0Gpo0_2Spec {}

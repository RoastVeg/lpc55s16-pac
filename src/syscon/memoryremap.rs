#[doc = "Register `MEMORYREMAP` reader"]
pub type R = crate::R<MemoryremapSpec>;
#[doc = "Register `MEMORYREMAP` writer"]
pub type W = crate::W<MemoryremapSpec>;
#[doc = "Select the location of the vector table :.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Map {
    #[doc = "0: Vector Table in ROM."]
    Rom0 = 0,
    #[doc = "1: Vector Table in RAM."]
    Ram1 = 1,
    #[doc = "2: Vector Table in Flash."]
    Flash0 = 2,
    #[doc = "3: Vector Table in Flash."]
    Flash1 = 3,
}
impl From<Map> for u8 {
    #[inline(always)]
    fn from(variant: Map) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Map {
    type Ux = u8;
}
impl crate::IsEnum for Map {}
#[doc = "Field `MAP` reader - Select the location of the vector table :."]
pub type MapR = crate::FieldReader<Map>;
impl MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Map {
        match self.bits {
            0 => Map::Rom0,
            1 => Map::Ram1,
            2 => Map::Flash0,
            3 => Map::Flash1,
            _ => unreachable!(),
        }
    }
    #[doc = "Vector Table in ROM."]
    #[inline(always)]
    pub fn is_rom0(&self) -> bool {
        *self == Map::Rom0
    }
    #[doc = "Vector Table in RAM."]
    #[inline(always)]
    pub fn is_ram1(&self) -> bool {
        *self == Map::Ram1
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn is_flash0(&self) -> bool {
        *self == Map::Flash0
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn is_flash1(&self) -> bool {
        *self == Map::Flash1
    }
}
#[doc = "Field `MAP` writer - Select the location of the vector table :."]
pub type MapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Map, crate::Safe>;
impl<'a, REG> MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Vector Table in ROM."]
    #[inline(always)]
    pub fn rom0(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Rom0)
    }
    #[doc = "Vector Table in RAM."]
    #[inline(always)]
    pub fn ram1(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Ram1)
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn flash0(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Flash0)
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn flash1(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Flash1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline(always)]
    pub fn map(&self) -> MapR {
        MapR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMORYREMAP")
            .field("map", &self.map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline(always)]
    pub fn map(&mut self) -> MapW<'_, MemoryremapSpec> {
        MapW::new(self, 0)
    }
}
#[doc = "Memory Remap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`memoryremap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memoryremap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemoryremapSpec;
impl crate::RegisterSpec for MemoryremapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memoryremap::R`](R) reader structure"]
impl crate::Readable for MemoryremapSpec {}
#[doc = "`write(|w| ..)` method takes [`memoryremap::W`](W) writer structure"]
impl crate::Writable for MemoryremapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMORYREMAP to value 0"]
impl crate::Resettable for MemoryremapSpec {}

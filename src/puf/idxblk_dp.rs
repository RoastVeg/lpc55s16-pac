#[doc = "Register `IDXBLK_DP` writer"]
pub type W = crate::W<IdxblkDpSpec>;
#[doc = "Field `IDX0` writer - Use to block PUF index 0"]
pub type Idx0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX1` writer - Use to block PUF index 1"]
pub type Idx1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX2` writer - Use to block PUF index 2"]
pub type Idx2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX3` writer - Use to block PUF index 3"]
pub type Idx3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX4` writer - Use to block PUF index 4"]
pub type Idx4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX5` writer - Use to block PUF index 5"]
pub type Idx5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX6` writer - Use to block PUF index 6"]
pub type Idx6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX7` writer - Use to block PUF index 7"]
pub type Idx7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX8` writer - Use to block PUF index 8"]
pub type Idx8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX9` writer - Use to block PUF index 9"]
pub type Idx9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX10` writer - Use to block PUF index 10"]
pub type Idx10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX11` writer - Use to block PUF index 11"]
pub type Idx11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX12` writer - Use to block PUF index 12"]
pub type Idx12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX13` writer - Use to block PUF index 13"]
pub type Idx13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX14` writer - Use to block PUF index 14"]
pub type Idx14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX15` writer - Use to block PUF index 15"]
pub type Idx15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for crate::generic::Reg<IdxblkDpSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:1 - Use to block PUF index 0"]
    #[inline(always)]
    pub fn idx0(&mut self) -> Idx0W<'_, IdxblkDpSpec> {
        Idx0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&mut self) -> Idx1W<'_, IdxblkDpSpec> {
        Idx1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&mut self) -> Idx2W<'_, IdxblkDpSpec> {
        Idx2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&mut self) -> Idx3W<'_, IdxblkDpSpec> {
        Idx3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&mut self) -> Idx4W<'_, IdxblkDpSpec> {
        Idx4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&mut self) -> Idx5W<'_, IdxblkDpSpec> {
        Idx5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&mut self) -> Idx6W<'_, IdxblkDpSpec> {
        Idx6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&mut self) -> Idx7W<'_, IdxblkDpSpec> {
        Idx7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&mut self) -> Idx8W<'_, IdxblkDpSpec> {
        Idx8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&mut self) -> Idx9W<'_, IdxblkDpSpec> {
        Idx9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&mut self) -> Idx10W<'_, IdxblkDpSpec> {
        Idx10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&mut self) -> Idx11W<'_, IdxblkDpSpec> {
        Idx11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&mut self) -> Idx12W<'_, IdxblkDpSpec> {
        Idx12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&mut self) -> Idx13W<'_, IdxblkDpSpec> {
        Idx13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&mut self) -> Idx14W<'_, IdxblkDpSpec> {
        Idx14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&mut self) -> Idx15W<'_, IdxblkDpSpec> {
        Idx15W::new(self, 30)
    }
}
#[doc = "no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_dp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdxblkDpSpec;
impl crate::RegisterSpec for IdxblkDpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idxblk_dp::W`](W) writer structure"]
impl crate::Writable for IdxblkDpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDXBLK_DP to value 0xaaaa_aaaa"]
impl crate::Resettable for IdxblkDpSpec {
    const RESET_VALUE: u32 = 0xaaaa_aaaa;
}

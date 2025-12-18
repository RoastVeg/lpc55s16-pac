#[doc = "Register `IDXBLK_STATUS` reader"]
pub type R = crate::R<IdxblkStatusSpec>;
#[doc = "Field `IDX0` reader - Status block index 0"]
pub type Idx0R = crate::FieldReader;
#[doc = "Field `IDX1` reader - Status block index 1"]
pub type Idx1R = crate::FieldReader;
#[doc = "Field `IDX2` reader - Status block index 2"]
pub type Idx2R = crate::FieldReader;
#[doc = "Field `IDX3` reader - Status block index 3"]
pub type Idx3R = crate::FieldReader;
#[doc = "Field `IDX4` reader - Status block index 4"]
pub type Idx4R = crate::FieldReader;
#[doc = "Field `IDX5` reader - Status block index 5"]
pub type Idx5R = crate::FieldReader;
#[doc = "Field `IDX6` reader - Status block index 6"]
pub type Idx6R = crate::FieldReader;
#[doc = "Field `IDX7` reader - Status block index 7"]
pub type Idx7R = crate::FieldReader;
#[doc = "Field `IDX8` reader - Status block index 8"]
pub type Idx8R = crate::FieldReader;
#[doc = "Field `IDX9` reader - Status block index 9"]
pub type Idx9R = crate::FieldReader;
#[doc = "Field `IDX10` reader - Status block index 10"]
pub type Idx10R = crate::FieldReader;
#[doc = "Field `IDX11` reader - Status block index 11"]
pub type Idx11R = crate::FieldReader;
#[doc = "Field `IDX12` reader - Status block index 12"]
pub type Idx12R = crate::FieldReader;
#[doc = "Field `IDX13` reader - Status block index 13"]
pub type Idx13R = crate::FieldReader;
#[doc = "Field `IDX14` reader - Status block index 14"]
pub type Idx14R = crate::FieldReader;
#[doc = "Field `IDX15` reader - Status block index 15"]
pub type Idx15R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Status block index 0"]
    #[inline(always)]
    pub fn idx0(&self) -> Idx0R {
        Idx0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Status block index 1"]
    #[inline(always)]
    pub fn idx1(&self) -> Idx1R {
        Idx1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Status block index 2"]
    #[inline(always)]
    pub fn idx2(&self) -> Idx2R {
        Idx2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Status block index 3"]
    #[inline(always)]
    pub fn idx3(&self) -> Idx3R {
        Idx3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Status block index 4"]
    #[inline(always)]
    pub fn idx4(&self) -> Idx4R {
        Idx4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Status block index 5"]
    #[inline(always)]
    pub fn idx5(&self) -> Idx5R {
        Idx5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Status block index 6"]
    #[inline(always)]
    pub fn idx6(&self) -> Idx6R {
        Idx6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Status block index 7"]
    #[inline(always)]
    pub fn idx7(&self) -> Idx7R {
        Idx7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Status block index 8"]
    #[inline(always)]
    pub fn idx8(&self) -> Idx8R {
        Idx8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Status block index 9"]
    #[inline(always)]
    pub fn idx9(&self) -> Idx9R {
        Idx9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Status block index 10"]
    #[inline(always)]
    pub fn idx10(&self) -> Idx10R {
        Idx10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Status block index 11"]
    #[inline(always)]
    pub fn idx11(&self) -> Idx11R {
        Idx11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Status block index 12"]
    #[inline(always)]
    pub fn idx12(&self) -> Idx12R {
        Idx12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Status block index 13"]
    #[inline(always)]
    pub fn idx13(&self) -> Idx13R {
        Idx13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Status block index 14"]
    #[inline(always)]
    pub fn idx14(&self) -> Idx14R {
        Idx14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Status block index 15"]
    #[inline(always)]
    pub fn idx15(&self) -> Idx15R {
        Idx15R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Index block status\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdxblkStatusSpec;
impl crate::RegisterSpec for IdxblkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idxblk_status::R`](R) reader structure"]
impl crate::Readable for IdxblkStatusSpec {}
#[doc = "`reset()` method sets IDXBLK_STATUS to value 0"]
impl crate::Resettable for IdxblkStatusSpec {}

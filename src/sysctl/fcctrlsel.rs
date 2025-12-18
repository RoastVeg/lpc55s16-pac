#[doc = "Register `FCCTRLSEL%s` reader"]
pub type R = crate::R<FcctrlselSpec>;
#[doc = "Register `FCCTRLSEL%s` writer"]
pub type W = crate::W<FcctrlselSpec>;
#[doc = "Selects the source for SCK going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sckinsel {
    #[doc = "0: Selects the dedicated FCn_SCK function for this Flexcomm."]
    OrigFlexI2sSignals = 0,
    #[doc = "1: SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SharedSet1I2sSignals = 2,
}
impl From<Sckinsel> for u8 {
    #[inline(always)]
    fn from(variant: Sckinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sckinsel {
    type Ux = u8;
}
impl crate::IsEnum for Sckinsel {}
#[doc = "Field `SCKINSEL` reader - Selects the source for SCK going into this Flexcomm."]
pub type SckinselR = crate::FieldReader<Sckinsel>;
impl SckinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sckinsel> {
        match self.bits {
            0 => Some(Sckinsel::OrigFlexI2sSignals),
            1 => Some(Sckinsel::SharedSet0I2sSignals),
            2 => Some(Sckinsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Sckinsel::OrigFlexI2sSignals
    }
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Sckinsel::SharedSet0I2sSignals
    }
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Sckinsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `SCKINSEL` writer - Selects the source for SCK going into this Flexcomm."]
pub type SckinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sckinsel>;
impl<'a, REG> SckinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::OrigFlexI2sSignals)
    }
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::SharedSet0I2sSignals)
    }
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::SharedSet1I2sSignals)
    }
}
#[doc = "Selects the source for WS going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wsinsel {
    #[doc = "0: Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    OrigFlexI2sSignals = 0,
    #[doc = "1: WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SharedSet1I2sSignals = 2,
}
impl From<Wsinsel> for u8 {
    #[inline(always)]
    fn from(variant: Wsinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wsinsel {
    type Ux = u8;
}
impl crate::IsEnum for Wsinsel {}
#[doc = "Field `WSINSEL` reader - Selects the source for WS going into this Flexcomm."]
pub type WsinselR = crate::FieldReader<Wsinsel>;
impl WsinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wsinsel> {
        match self.bits {
            0 => Some(Wsinsel::OrigFlexI2sSignals),
            1 => Some(Wsinsel::SharedSet0I2sSignals),
            2 => Some(Wsinsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Wsinsel::OrigFlexI2sSignals
    }
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Wsinsel::SharedSet0I2sSignals
    }
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Wsinsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `WSINSEL` writer - Selects the source for WS going into this Flexcomm."]
pub type WsinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wsinsel>;
impl<'a, REG> WsinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::OrigFlexI2sSignals)
    }
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::SharedSet0I2sSignals)
    }
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::SharedSet1I2sSignals)
    }
}
#[doc = "Selects the source for DATA input to this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datainsel {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SharedSet1I2sSignals = 2,
}
impl From<Datainsel> for u8 {
    #[inline(always)]
    fn from(variant: Datainsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datainsel {
    type Ux = u8;
}
impl crate::IsEnum for Datainsel {}
#[doc = "Field `DATAINSEL` reader - Selects the source for DATA input to this Flexcomm."]
pub type DatainselR = crate::FieldReader<Datainsel>;
impl DatainselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datainsel> {
        match self.bits {
            0 => Some(Datainsel::OrigFlexI2sSignals),
            1 => Some(Datainsel::SharedSet0I2sSignals),
            2 => Some(Datainsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Datainsel::OrigFlexI2sSignals
    }
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Datainsel::SharedSet0I2sSignals
    }
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Datainsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `DATAINSEL` writer - Selects the source for DATA input to this Flexcomm."]
pub type DatainselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datainsel>;
impl<'a, REG> DatainselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::OrigFlexI2sSignals)
    }
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::SharedSet0I2sSignals)
    }
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::SharedSet1I2sSignals)
    }
}
#[doc = "Selects the source for DATA output from this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dataoutsel {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SharedSet1I2sSignals = 2,
}
impl From<Dataoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Dataoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dataoutsel {
    type Ux = u8;
}
impl crate::IsEnum for Dataoutsel {}
#[doc = "Field `DATAOUTSEL` reader - Selects the source for DATA output from this Flexcomm."]
pub type DataoutselR = crate::FieldReader<Dataoutsel>;
impl DataoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dataoutsel> {
        match self.bits {
            0 => Some(Dataoutsel::OrigFlexI2sSignals),
            1 => Some(Dataoutsel::SharedSet0I2sSignals),
            2 => Some(Dataoutsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Dataoutsel::OrigFlexI2sSignals
    }
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Dataoutsel::SharedSet0I2sSignals
    }
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Dataoutsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `DATAOUTSEL` writer - Selects the source for DATA output from this Flexcomm."]
pub type DataoutselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dataoutsel>;
impl<'a, REG> DataoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::OrigFlexI2sSignals)
    }
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::SharedSet0I2sSignals)
    }
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::SharedSet1I2sSignals)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub fn sckinsel(&self) -> SckinselR {
        SckinselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub fn wsinsel(&self) -> WsinselR {
        WsinselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub fn datainsel(&self) -> DatainselR {
        DatainselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn dataoutsel(&self) -> DataoutselR {
        DataoutselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub fn sckinsel(&mut self) -> SckinselW<'_, FcctrlselSpec> {
        SckinselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub fn wsinsel(&mut self) -> WsinselW<'_, FcctrlselSpec> {
        WsinselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub fn datainsel(&mut self) -> DatainselW<'_, FcctrlselSpec> {
        DatainselW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn dataoutsel(&mut self) -> DataoutselW<'_, FcctrlselSpec> {
        DataoutselW::new(self, 24)
    }
}
#[doc = "Selects the source for SCK going into Flexcomm index\n\nYou can [`read`](crate::Reg::read) this register and get [`fcctrlsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcctrlsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcctrlselSpec;
impl crate::RegisterSpec for FcctrlselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcctrlsel::R`](R) reader structure"]
impl crate::Readable for FcctrlselSpec {}
#[doc = "`write(|w| ..)` method takes [`fcctrlsel::W`](W) writer structure"]
impl crate::Writable for FcctrlselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCTRLSEL%s to value 0"]
impl crate::Resettable for FcctrlselSpec {}

#[doc = "Register `PWD_TOG` reader"]
pub type R = crate::R<PwdTogSpec>;
#[doc = "Register `PWD_TOG` writer"]
pub type W = crate::W<PwdTogSpec>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdfs {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    Value1 = 1,
}
impl From<Txpwdfs> for bool {
    #[inline(always)]
    fn from(variant: Txpwdfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDFS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdfsR = crate::BitReader<Txpwdfs>;
impl TxpwdfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdfs {
        match self.bits {
            false => Txpwdfs::Value0,
            true => Txpwdfs::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Txpwdfs::Value0
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txpwdfs::Value1
    }
}
#[doc = "Field `TXPWDFS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdfsW<'a, REG> = crate::BitWriter<'a, REG, Txpwdfs>;
impl<'a, REG> TxpwdfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdfs::Value0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdfs::Value1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdibias {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    Value1 = 1,
}
impl From<Txpwdibias> for bool {
    #[inline(always)]
    fn from(variant: Txpwdibias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDIBIAS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdibiasR = crate::BitReader<Txpwdibias>;
impl TxpwdibiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdibias {
        match self.bits {
            false => Txpwdibias::Value0,
            true => Txpwdibias::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Txpwdibias::Value0
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txpwdibias::Value1
    }
}
#[doc = "Field `TXPWDIBIAS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdibiasW<'a, REG> = crate::BitWriter<'a, REG, Txpwdibias>;
impl<'a, REG> TxpwdibiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdibias::Value0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdibias::Value1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdv2i {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    Value1 = 1,
}
impl From<Txpwdv2i> for bool {
    #[inline(always)]
    fn from(variant: Txpwdv2i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDV2I` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Txpwdv2iR = crate::BitReader<Txpwdv2i>;
impl Txpwdv2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdv2i {
        match self.bits {
            false => Txpwdv2i::Value0,
            true => Txpwdv2i::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Txpwdv2i::Value0
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txpwdv2i::Value1
    }
}
#[doc = "Field `TXPWDV2I` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Txpwdv2iW<'a, REG> = crate::BitWriter<'a, REG, Txpwdv2i>;
impl<'a, REG> Txpwdv2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdv2i::Value0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdv2i::Value1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwdenv {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    Value1 = 1,
}
impl From<Rxpwdenv> for bool {
    #[inline(always)]
    fn from(variant: Rxpwdenv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDENV` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdenvR = crate::BitReader<Rxpwdenv>;
impl RxpwdenvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwdenv {
        match self.bits {
            false => Rxpwdenv::Value0,
            true => Rxpwdenv::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Rxpwdenv::Value0
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxpwdenv::Value1
    }
}
#[doc = "Field `RXPWDENV` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdenvW<'a, REG> = crate::BitWriter<'a, REG, Rxpwdenv>;
impl<'a, REG> RxpwdenvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdenv::Value0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdenv::Value1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwd1pt1 {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB full-speed differential receiver."]
    Value1 = 1,
}
impl From<Rxpwd1pt1> for bool {
    #[inline(always)]
    fn from(variant: Rxpwd1pt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWD1PT1` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Rxpwd1pt1R = crate::BitReader<Rxpwd1pt1>;
impl Rxpwd1pt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwd1pt1 {
        match self.bits {
            false => Rxpwd1pt1::Value0,
            true => Rxpwd1pt1::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Rxpwd1pt1::Value0
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxpwd1pt1::Value1
    }
}
#[doc = "Field `RXPWD1PT1` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Rxpwd1pt1W<'a, REG> = crate::BitWriter<'a, REG, Rxpwd1pt1>;
impl<'a, REG> Rxpwd1pt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwd1pt1::Value0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwd1pt1::Value1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwddiff {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the USB high-speed differential receive"]
    Value1 = 1,
}
impl From<Rxpwddiff> for bool {
    #[inline(always)]
    fn from(variant: Rxpwddiff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDDIFF` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwddiffR = crate::BitReader<Rxpwddiff>;
impl RxpwddiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwddiff {
        match self.bits {
            false => Rxpwddiff::Value0,
            true => Rxpwddiff::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Rxpwddiff::Value0
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxpwddiff::Value1
    }
}
#[doc = "Field `RXPWDDIFF` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwddiffW<'a, REG> = crate::BitWriter<'a, REG, Rxpwddiff>;
impl<'a, REG> RxpwddiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwddiff::Value0)
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwddiff::Value1)
    }
}
#[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwdrx {
    #[doc = "0: Normal operation."]
    Value0 = 0,
    #[doc = "1: Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    Value1 = 1,
}
impl From<Rxpwdrx> for bool {
    #[inline(always)]
    fn from(variant: Rxpwdrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDRX` reader - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdrxR = crate::BitReader<Rxpwdrx>;
impl RxpwdrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwdrx {
        match self.bits {
            false => Rxpwdrx::Value0,
            true => Rxpwdrx::Value1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == Rxpwdrx::Value0
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxpwdrx::Value1
    }
}
#[doc = "Field `RXPWDRX` writer - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdrxW<'a, REG> = crate::BitWriter<'a, REG, Rxpwdrx>;
impl<'a, REG> RxpwdrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdrx::Value0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdrx::Value1)
    }
}
impl R {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TxpwdfsR {
        TxpwdfsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TxpwdibiasR {
        TxpwdibiasR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> Txpwdv2iR {
        Txpwdv2iR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RxpwdenvR {
        RxpwdenvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> Rxpwd1pt1R {
        Rxpwd1pt1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RxpwddiffR {
        RxpwddiffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RxpwdrxR {
        RxpwdrxR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&mut self) -> TxpwdfsW<'_, PwdTogSpec> {
        TxpwdfsW::new(self, 10)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&mut self) -> TxpwdibiasW<'_, PwdTogSpec> {
        TxpwdibiasW::new(self, 11)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&mut self) -> Txpwdv2iW<'_, PwdTogSpec> {
        Txpwdv2iW::new(self, 12)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&mut self) -> RxpwdenvW<'_, PwdTogSpec> {
        RxpwdenvW::new(self, 17)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&mut self) -> Rxpwd1pt1W<'_, PwdTogSpec> {
        Rxpwd1pt1W::new(self, 18)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&mut self) -> RxpwddiffW<'_, PwdTogSpec> {
        RxpwddiffW::new(self, 19)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&mut self) -> RxpwdrxW<'_, PwdTogSpec> {
        RxpwdrxW::new(self, 20)
    }
}
#[doc = "USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd_tog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_tog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdTogSpec;
impl crate::RegisterSpec for PwdTogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwd_tog::R`](R) reader structure"]
impl crate::Readable for PwdTogSpec {}
#[doc = "`write(|w| ..)` method takes [`pwd_tog::W`](W) writer structure"]
impl crate::Writable for PwdTogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWD_TOG to value 0x001e_1c00"]
impl crate::Resettable for PwdTogSpec {
    const RESET_VALUE: u32 = 0x001e_1c00;
}

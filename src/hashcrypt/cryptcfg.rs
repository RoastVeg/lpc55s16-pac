#[doc = "Register `CRYPTCFG` reader"]
pub type R = crate::R<CryptcfgSpec>;
#[doc = "Register `CRYPTCFG` writer"]
pub type W = crate::W<CryptcfgSpec>;
#[doc = "Field `MSW1ST_OUT` reader - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
pub type Msw1stOutR = crate::BitReader;
#[doc = "Field `MSW1ST_OUT` writer - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
pub type Msw1stOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPKEY` reader - If 1, will Swap the key input (bytes in each word)."]
pub type SwapkeyR = crate::BitReader;
#[doc = "Field `SWAPKEY` writer - If 1, will Swap the key input (bytes in each word)."]
pub type SwapkeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPDAT` reader - If 1, will SWAP the data and IV inputs (bytes in each word)."]
pub type SwapdatR = crate::BitReader;
#[doc = "Field `SWAPDAT` writer - If 1, will SWAP the data and IV inputs (bytes in each word)."]
pub type SwapdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSW1ST` reader - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
pub type Msw1stR = crate::BitReader;
#[doc = "Field `MSW1ST` writer - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
pub type Msw1stW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AES Cipher mode to use if plain AES\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesmode {
    #[doc = "0: ECB - used as is"]
    Ecb = 0,
    #[doc = "1: CBC mode (see details on IV/nonce)"]
    Cbc = 1,
    #[doc = "2: CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    Ctr = 2,
}
impl From<Aesmode> for u8 {
    #[inline(always)]
    fn from(variant: Aesmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesmode {
    type Ux = u8;
}
impl crate::IsEnum for Aesmode {}
#[doc = "Field `AESMODE` reader - AES Cipher mode to use if plain AES"]
pub type AesmodeR = crate::FieldReader<Aesmode>;
impl AesmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aesmode> {
        match self.bits {
            0 => Some(Aesmode::Ecb),
            1 => Some(Aesmode::Cbc),
            2 => Some(Aesmode::Ctr),
            _ => None,
        }
    }
    #[doc = "ECB - used as is"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Aesmode::Ecb
    }
    #[doc = "CBC mode (see details on IV/nonce)"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Aesmode::Cbc
    }
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Aesmode::Ctr
    }
}
#[doc = "Field `AESMODE` writer - AES Cipher mode to use if plain AES"]
pub type AesmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesmode>;
impl<'a, REG> AesmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB - used as is"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmode::Ecb)
    }
    #[doc = "CBC mode (see details on IV/nonce)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmode::Cbc)
    }
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmode::Ctr)
    }
}
#[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesdecrypt {
    #[doc = "0: Encrypt"]
    Encrypt = 0,
    #[doc = "1: Decrypt"]
    Decrypt = 1,
}
impl From<Aesdecrypt> for bool {
    #[inline(always)]
    fn from(variant: Aesdecrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESDECRYPT` reader - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
pub type AesdecryptR = crate::BitReader<Aesdecrypt>;
impl AesdecryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesdecrypt {
        match self.bits {
            false => Aesdecrypt::Encrypt,
            true => Aesdecrypt::Decrypt,
        }
    }
    #[doc = "Encrypt"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == Aesdecrypt::Encrypt
    }
    #[doc = "Decrypt"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == Aesdecrypt::Decrypt
    }
}
#[doc = "Field `AESDECRYPT` writer - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
pub type AesdecryptW<'a, REG> = crate::BitWriter<'a, REG, Aesdecrypt>;
impl<'a, REG> AesdecryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encrypt"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Aesdecrypt::Encrypt)
    }
    #[doc = "Decrypt"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Aesdecrypt::Decrypt)
    }
}
#[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aessecret {
    #[doc = "0: User key provided in normal way"]
    NormalWay = 0,
    #[doc = "1: Secret key provided in hidden way by HW"]
    HiddenWay = 1,
}
impl From<Aessecret> for bool {
    #[inline(always)]
    fn from(variant: Aessecret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSECRET` reader - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
pub type AessecretR = crate::BitReader<Aessecret>;
impl AessecretR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aessecret {
        match self.bits {
            false => Aessecret::NormalWay,
            true => Aessecret::HiddenWay,
        }
    }
    #[doc = "User key provided in normal way"]
    #[inline(always)]
    pub fn is_normal_way(&self) -> bool {
        *self == Aessecret::NormalWay
    }
    #[doc = "Secret key provided in hidden way by HW"]
    #[inline(always)]
    pub fn is_hidden_way(&self) -> bool {
        *self == Aessecret::HiddenWay
    }
}
#[doc = "Field `AESSECRET` writer - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
pub type AessecretW<'a, REG> = crate::BitWriter<'a, REG, Aessecret>;
impl<'a, REG> AessecretW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User key provided in normal way"]
    #[inline(always)]
    pub fn normal_way(self) -> &'a mut crate::W<REG> {
        self.variant(Aessecret::NormalWay)
    }
    #[doc = "Secret key provided in hidden way by HW"]
    #[inline(always)]
    pub fn hidden_way(self) -> &'a mut crate::W<REG> {
        self.variant(Aessecret::HiddenWay)
    }
}
#[doc = "Sets the AES key size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeskeysz {
    #[doc = "0: 128 bit key"]
    Bits128 = 0,
    #[doc = "1: 192 bit key"]
    Bits192 = 1,
    #[doc = "2: 256 bit key"]
    Bits256 = 2,
}
impl From<Aeskeysz> for u8 {
    #[inline(always)]
    fn from(variant: Aeskeysz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeskeysz {
    type Ux = u8;
}
impl crate::IsEnum for Aeskeysz {}
#[doc = "Field `AESKEYSZ` reader - Sets the AES key size"]
pub type AeskeyszR = crate::FieldReader<Aeskeysz>;
impl AeskeyszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aeskeysz> {
        match self.bits {
            0 => Some(Aeskeysz::Bits128),
            1 => Some(Aeskeysz::Bits192),
            2 => Some(Aeskeysz::Bits256),
            _ => None,
        }
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn is_bits_128(&self) -> bool {
        *self == Aeskeysz::Bits128
    }
    #[doc = "192 bit key"]
    #[inline(always)]
    pub fn is_bits_192(&self) -> bool {
        *self == Aeskeysz::Bits192
    }
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn is_bits_256(&self) -> bool {
        *self == Aeskeysz::Bits256
    }
}
#[doc = "Field `AESKEYSZ` writer - Sets the AES key size"]
pub type AeskeyszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aeskeysz>;
impl<'a, REG> AeskeyszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn bits_128(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskeysz::Bits128)
    }
    #[doc = "192 bit key"]
    #[inline(always)]
    pub fn bits_192(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskeysz::Bits192)
    }
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn bits_256(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskeysz::Bits256)
    }
}
#[doc = "Field `AESCTRPOS` reader - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
pub type AesctrposR = crate::FieldReader;
#[doc = "Field `AESCTRPOS` writer - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
pub type AesctrposW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STREAMLAST` reader - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
pub type StreamlastR = crate::BitReader;
#[doc = "Field `STREAMLAST` writer - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
pub type StreamlastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st_out(&self) -> Msw1stOutR {
        Msw1stOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub fn swapkey(&self) -> SwapkeyR {
        SwapkeyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub fn swapdat(&self) -> SwapdatR {
        SwapdatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st(&self) -> Msw1stR {
        Msw1stR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline(always)]
    pub fn aesmode(&self) -> AesmodeR {
        AesmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline(always)]
    pub fn aesdecrypt(&self) -> AesdecryptR {
        AesdecryptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub fn aessecret(&self) -> AessecretR {
        AessecretR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline(always)]
    pub fn aeskeysz(&self) -> AeskeyszR {
        AeskeyszR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub fn aesctrpos(&self) -> AesctrposR {
        AesctrposR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub fn streamlast(&self) -> StreamlastR {
        StreamlastR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTCFG")
            .field("msw1st_out", &self.msw1st_out())
            .field("swapkey", &self.swapkey())
            .field("swapdat", &self.swapdat())
            .field("msw1st", &self.msw1st())
            .field("aesmode", &self.aesmode())
            .field("aesdecrypt", &self.aesdecrypt())
            .field("aessecret", &self.aessecret())
            .field("aeskeysz", &self.aeskeysz())
            .field("aesctrpos", &self.aesctrpos())
            .field("streamlast", &self.streamlast())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st_out(&mut self) -> Msw1stOutW<'_, CryptcfgSpec> {
        Msw1stOutW::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub fn swapkey(&mut self) -> SwapkeyW<'_, CryptcfgSpec> {
        SwapkeyW::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub fn swapdat(&mut self) -> SwapdatW<'_, CryptcfgSpec> {
        SwapdatW::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st(&mut self) -> Msw1stW<'_, CryptcfgSpec> {
        Msw1stW::new(self, 3)
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline(always)]
    pub fn aesmode(&mut self) -> AesmodeW<'_, CryptcfgSpec> {
        AesmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline(always)]
    pub fn aesdecrypt(&mut self) -> AesdecryptW<'_, CryptcfgSpec> {
        AesdecryptW::new(self, 6)
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub fn aessecret(&mut self) -> AessecretW<'_, CryptcfgSpec> {
        AessecretW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline(always)]
    pub fn aeskeysz(&mut self) -> AeskeyszW<'_, CryptcfgSpec> {
        AeskeyszW::new(self, 8)
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub fn aesctrpos(&mut self) -> AesctrposW<'_, CryptcfgSpec> {
        AesctrposW::new(self, 10)
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub fn streamlast(&mut self) -> StreamlastW<'_, CryptcfgSpec> {
        StreamlastW::new(self, 16)
    }
}
#[doc = "Crypto settings for AES and Salsa and ChaCha\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptcfgSpec;
impl crate::RegisterSpec for CryptcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryptcfg::R`](R) reader structure"]
impl crate::Readable for CryptcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cryptcfg::W`](W) writer structure"]
impl crate::Writable for CryptcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRYPTCFG to value 0"]
impl crate::Resettable for CryptcfgSpec {}

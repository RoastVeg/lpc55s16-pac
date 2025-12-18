#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    random_number: RandomNumber,
    encrypted_number: EncryptedNumber,
    counter_val: CounterVal,
    counter_cfg: CounterCfg,
    online_test_cfg: OnlineTestCfg,
    online_test_val: OnlineTestVal,
    entropy_inject: EntropyInject,
    misc_cfg: MiscCfg,
    _reserved8: [u8; 0x0fd4],
    powerdown: Powerdown,
    _reserved9: [u8; 0x04],
    moduleid: Moduleid,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    #[inline(always)]
    pub const fn random_number(&self) -> &RandomNumber {
        &self.random_number
    }
    #[doc = "0x04 - This register contains a random 32 bit number which is pre-computed"]
    #[inline(always)]
    pub const fn encrypted_number(&self) -> &EncryptedNumber {
        &self.encrypted_number
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn counter_val(&self) -> &CounterVal {
        &self.counter_val
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn counter_cfg(&self) -> &CounterCfg {
        &self.counter_cfg
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn online_test_cfg(&self) -> &OnlineTestCfg {
        &self.online_test_cfg
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn online_test_val(&self) -> &OnlineTestVal {
        &self.online_test_val
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn entropy_inject(&self) -> &EntropyInject {
        &self.entropy_inject
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn misc_cfg(&self) -> &MiscCfg {
        &self.misc_cfg
    }
    #[doc = "0xff4 - Powerdown mode (standard but certainly useless here)"]
    #[inline(always)]
    pub const fn powerdown(&self) -> &Powerdown {
        &self.powerdown
    }
    #[doc = "0xffc - IP identifier"]
    #[inline(always)]
    pub const fn moduleid(&self) -> &Moduleid {
        &self.moduleid
    }
}
#[doc = "RANDOM_NUMBER (r) register accessor: This register contains a random 32 bit number which is computed on demand, at each time it is read\n\nYou can [`read`](crate::Reg::read) this register and get [`random_number::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_number`] module"]
#[doc(alias = "RANDOM_NUMBER")]
pub type RandomNumber = crate::Reg<random_number::RandomNumberSpec>;
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub mod random_number;
#[doc = "ENCRYPTED_NUMBER (rw) register accessor: This register contains a random 32 bit number which is pre-computed\n\nYou can [`read`](crate::Reg::read) this register and get [`encrypted_number::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encrypted_number::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@encrypted_number`] module"]
#[doc(alias = "ENCRYPTED_NUMBER")]
pub type EncryptedNumber = crate::Reg<encrypted_number::EncryptedNumberSpec>;
#[doc = "This register contains a random 32 bit number which is pre-computed"]
pub mod encrypted_number;
#[doc = "COUNTER_VAL (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_val`] module"]
#[doc(alias = "COUNTER_VAL")]
pub type CounterVal = crate::Reg<counter_val::CounterValSpec>;
#[doc = "no description available"]
pub mod counter_val;
#[doc = "COUNTER_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_cfg`] module"]
#[doc(alias = "COUNTER_CFG")]
pub type CounterCfg = crate::Reg<counter_cfg::CounterCfgSpec>;
#[doc = "no description available"]
pub mod counter_cfg;
#[doc = "ONLINE_TEST_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@online_test_cfg`] module"]
#[doc(alias = "ONLINE_TEST_CFG")]
pub type OnlineTestCfg = crate::Reg<online_test_cfg::OnlineTestCfgSpec>;
#[doc = "no description available"]
pub mod online_test_cfg;
#[doc = "ONLINE_TEST_VAL (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@online_test_val`] module"]
#[doc(alias = "ONLINE_TEST_VAL")]
pub type OnlineTestVal = crate::Reg<online_test_val::OnlineTestValSpec>;
#[doc = "no description available"]
pub mod online_test_val;
#[doc = "ENTROPY_INJECT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`entropy_inject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entropy_inject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entropy_inject`] module"]
#[doc(alias = "ENTROPY_INJECT")]
pub type EntropyInject = crate::Reg<entropy_inject::EntropyInjectSpec>;
#[doc = "no description available"]
pub mod entropy_inject;
#[doc = "MISC_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_cfg`] module"]
#[doc(alias = "MISC_CFG")]
pub type MiscCfg = crate::Reg<misc_cfg::MiscCfgSpec>;
#[doc = "no description available"]
pub mod misc_cfg;
#[doc = "POWERDOWN (rw) register accessor: Powerdown mode (standard but certainly useless here)\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerdown`] module"]
#[doc(alias = "POWERDOWN")]
pub type Powerdown = crate::Reg<powerdown::PowerdownSpec>;
#[doc = "Powerdown mode (standard but certainly useless here)"]
pub mod powerdown;
#[doc = "MODULEID (r) register accessor: IP identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`moduleid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moduleid`] module"]
#[doc(alias = "MODULEID")]
pub type Moduleid = crate::Reg<moduleid::ModuleidSpec>;
#[doc = "IP identifier"]
pub mod moduleid;

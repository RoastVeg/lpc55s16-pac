#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    reload: Reload,
    instruction_timer: InstructionTimer,
    secure_counter: SecureCounter,
    status: Status,
    status2: Status2,
    flags: Flags,
    persistent: Persistent,
    start: Start,
    stop: Stop,
    restart: Restart,
    add: Add,
    add1: Add1,
    add16: Add16,
    add256: Add256,
    sub: Sub,
    sub1: Sub1,
    sub16: Sub16,
    sub256: Sub256,
}
impl RegisterBlock {
    #[doc = "0x00 - The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Instruction timer reload"]
    #[inline(always)]
    pub const fn reload(&self) -> &Reload {
        &self.reload
    }
    #[doc = "0x08 - The INSTRUCTION TIMER itself"]
    #[inline(always)]
    pub const fn instruction_timer(&self) -> &InstructionTimer {
        &self.instruction_timer
    }
    #[doc = "0x0c - Also known as SEC_CNT"]
    #[inline(always)]
    pub const fn secure_counter(&self) -> &SecureCounter {
        &self.secure_counter
    }
    #[doc = "0x10 - Status register (1 of 2)"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - STATUS register (2 of 2)"]
    #[inline(always)]
    pub const fn status2(&self) -> &Status2 {
        &self.status2
    }
    #[doc = "0x18 - Hardware flags"]
    #[inline(always)]
    pub const fn flags(&self) -> &Flags {
        &self.flags
    }
    #[doc = "0x1c - Persistent (Ad. Hoc., quasi-NV) data storage"]
    #[inline(always)]
    pub const fn persistent(&self) -> &Persistent {
        &self.persistent
    }
    #[doc = "0x20 - Write address for issuing the START command."]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x24 - Write address for issuing the STOP command."]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x28 - Write address for issuing the RESTART command."]
    #[inline(always)]
    pub const fn restart(&self) -> &Restart {
        &self.restart
    }
    #[doc = "0x2c - Write address for issuing the ADD command."]
    #[inline(always)]
    pub const fn add(&self) -> &Add {
        &self.add
    }
    #[doc = "0x30 - Write address for issuing the ADD1 command."]
    #[inline(always)]
    pub const fn add1(&self) -> &Add1 {
        &self.add1
    }
    #[doc = "0x34 - Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn add16(&self) -> &Add16 {
        &self.add16
    }
    #[doc = "0x38 - Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn add256(&self) -> &Add256 {
        &self.add256
    }
    #[doc = "0x3c - Write address for issuing the SUB command."]
    #[inline(always)]
    pub const fn sub(&self) -> &Sub {
        &self.sub
    }
    #[doc = "0x40 - Write address for issuing the SUB1 command."]
    #[inline(always)]
    pub const fn sub1(&self) -> &Sub1 {
        &self.sub1
    }
    #[doc = "0x44 - Write address for issuing the SUB16 command."]
    #[inline(always)]
    pub const fn sub16(&self) -> &Sub16 {
        &self.sub16
    }
    #[doc = "0x48 - Write address for issuing the SUB256 command."]
    #[inline(always)]
    pub const fn sub256(&self) -> &Sub256 {
        &self.sub256
    }
}
#[doc = "CONTROL (rw) register accessor: The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
pub mod control;
#[doc = "RELOAD (rw) register accessor: Instruction timer reload\n\nYou can [`read`](crate::Reg::read) this register and get [`reload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reload`] module"]
#[doc(alias = "RELOAD")]
pub type Reload = crate::Reg<reload::ReloadSpec>;
#[doc = "Instruction timer reload"]
pub mod reload;
#[doc = "INSTRUCTION_TIMER (rw) register accessor: The INSTRUCTION TIMER itself\n\nYou can [`read`](crate::Reg::read) this register and get [`instruction_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instruction_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instruction_timer`] module"]
#[doc(alias = "INSTRUCTION_TIMER")]
pub type InstructionTimer = crate::Reg<instruction_timer::InstructionTimerSpec>;
#[doc = "The INSTRUCTION TIMER itself"]
pub mod instruction_timer;
#[doc = "SECURE_COUNTER (rw) register accessor: Also known as SEC_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_counter`] module"]
#[doc(alias = "SECURE_COUNTER")]
pub type SecureCounter = crate::Reg<secure_counter::SecureCounterSpec>;
#[doc = "Also known as SEC_CNT"]
pub mod secure_counter;
#[doc = "STATUS (r) register accessor: Status register (1 of 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register (1 of 2)"]
pub mod status;
#[doc = "STATUS2 (r) register accessor: STATUS register (2 of 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2`] module"]
#[doc(alias = "STATUS2")]
pub type Status2 = crate::Reg<status2::Status2Spec>;
#[doc = "STATUS register (2 of 2)"]
pub mod status2;
#[doc = "FLAGS (rw) register accessor: Hardware flags\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flags`] module"]
#[doc(alias = "FLAGS")]
pub type Flags = crate::Reg<flags::FlagsSpec>;
#[doc = "Hardware flags"]
pub mod flags;
#[doc = "PERSISTENT (rw) register accessor: Persistent (Ad. Hoc., quasi-NV) data storage\n\nYou can [`read`](crate::Reg::read) this register and get [`persistent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`persistent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@persistent`] module"]
#[doc(alias = "PERSISTENT")]
pub type Persistent = crate::Reg<persistent::PersistentSpec>;
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage"]
pub mod persistent;
#[doc = "START (w) register accessor: Write address for issuing the START command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Write address for issuing the START command."]
pub mod start;
#[doc = "STOP (w) register accessor: Write address for issuing the STOP command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`] module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Write address for issuing the STOP command."]
pub mod stop;
#[doc = "RESTART (w) register accessor: Write address for issuing the RESTART command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restart`] module"]
#[doc(alias = "RESTART")]
pub type Restart = crate::Reg<restart::RestartSpec>;
#[doc = "Write address for issuing the RESTART command."]
pub mod restart;
#[doc = "ADD (w) register accessor: Write address for issuing the ADD command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`add::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@add`] module"]
#[doc(alias = "ADD")]
pub type Add = crate::Reg<add::AddSpec>;
#[doc = "Write address for issuing the ADD command."]
pub mod add;
#[doc = "ADD1 (w) register accessor: Write address for issuing the ADD1 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`add1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@add1`] module"]
#[doc(alias = "ADD1")]
pub type Add1 = crate::Reg<add1::Add1Spec>;
#[doc = "Write address for issuing the ADD1 command."]
pub mod add1;
#[doc = "ADD16 (w) register accessor: Write address for issuing the ADD16 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`add16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@add16`] module"]
#[doc(alias = "ADD16")]
pub type Add16 = crate::Reg<add16::Add16Spec>;
#[doc = "Write address for issuing the ADD16 command."]
pub mod add16;
#[doc = "ADD256 (w) register accessor: Write address for issuing the ADD16 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`add256::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@add256`] module"]
#[doc(alias = "ADD256")]
pub type Add256 = crate::Reg<add256::Add256Spec>;
#[doc = "Write address for issuing the ADD16 command."]
pub mod add256;
#[doc = "SUB (w) register accessor: Write address for issuing the SUB command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub`] module"]
#[doc(alias = "SUB")]
pub type Sub = crate::Reg<sub::SubSpec>;
#[doc = "Write address for issuing the SUB command."]
pub mod sub;
#[doc = "SUB1 (w) register accessor: Write address for issuing the SUB1 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub1`] module"]
#[doc(alias = "SUB1")]
pub type Sub1 = crate::Reg<sub1::Sub1Spec>;
#[doc = "Write address for issuing the SUB1 command."]
pub mod sub1;
#[doc = "SUB16 (w) register accessor: Write address for issuing the SUB16 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub16`] module"]
#[doc(alias = "SUB16")]
pub type Sub16 = crate::Reg<sub16::Sub16Spec>;
#[doc = "Write address for issuing the SUB16 command."]
pub mod sub16;
#[doc = "SUB256 (w) register accessor: Write address for issuing the SUB256 command.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub256::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub256`] module"]
#[doc(alias = "SUB256")]
pub type Sub256 = crate::Reg<sub256::Sub256Spec>;
#[doc = "Write address for issuing the SUB256 command."]
pub mod sub256;

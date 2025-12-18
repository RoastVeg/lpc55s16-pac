#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcrevision: Hcrevision,
    hccontrol: Hccontrol,
    hccommandstatus: Hccommandstatus,
    hcinterruptstatus: Hcinterruptstatus,
    hcinterruptenable: Hcinterruptenable,
    hcinterruptdisable: Hcinterruptdisable,
    hchcca: Hchcca,
    hcperiodcurrented: Hcperiodcurrented,
    hccontrolheaded: Hccontrolheaded,
    hccontrolcurrented: Hccontrolcurrented,
    hcbulkheaded: Hcbulkheaded,
    hcbulkcurrented: Hcbulkcurrented,
    hcdonehead: Hcdonehead,
    hcfminterval: Hcfminterval,
    hcfmremaining: Hcfmremaining,
    hcfmnumber: Hcfmnumber,
    hcperiodicstart: Hcperiodicstart,
    hclsthreshold: Hclsthreshold,
    hcrhdescriptora: Hcrhdescriptora,
    hcrhdescriptorb: Hcrhdescriptorb,
    hcrhstatus: Hcrhstatus,
    hcrhportstatus: Hcrhportstatus,
    _reserved22: [u8; 0x04],
    portmode: Portmode,
}
impl RegisterBlock {
    #[doc = "0x00 - BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
    #[inline(always)]
    pub const fn hcrevision(&self) -> &Hcrevision {
        &self.hcrevision
    }
    #[doc = "0x04 - Defines the operating modes of the HC"]
    #[inline(always)]
    pub const fn hccontrol(&self) -> &Hccontrol {
        &self.hccontrol
    }
    #[doc = "0x08 - This register is used to receive the commands from the Host Controller Driver (HCD)"]
    #[inline(always)]
    pub const fn hccommandstatus(&self) -> &Hccommandstatus {
        &self.hccommandstatus
    }
    #[doc = "0x0c - Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
    #[inline(always)]
    pub const fn hcinterruptstatus(&self) -> &Hcinterruptstatus {
        &self.hcinterruptstatus
    }
    #[doc = "0x10 - Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
    #[inline(always)]
    pub const fn hcinterruptenable(&self) -> &Hcinterruptenable {
        &self.hcinterruptenable
    }
    #[doc = "0x14 - The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
    #[inline(always)]
    pub const fn hcinterruptdisable(&self) -> &Hcinterruptdisable {
        &self.hcinterruptdisable
    }
    #[doc = "0x18 - Contains the physical address of the host controller communication area"]
    #[inline(always)]
    pub const fn hchcca(&self) -> &Hchcca {
        &self.hchcca
    }
    #[doc = "0x1c - Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
    #[inline(always)]
    pub const fn hcperiodcurrented(&self) -> &Hcperiodcurrented {
        &self.hcperiodcurrented
    }
    #[doc = "0x20 - Contains the physical address of the first endpoint descriptor of the control list"]
    #[inline(always)]
    pub const fn hccontrolheaded(&self) -> &Hccontrolheaded {
        &self.hccontrolheaded
    }
    #[doc = "0x24 - Contains the physical address of the current endpoint descriptor of the control list"]
    #[inline(always)]
    pub const fn hccontrolcurrented(&self) -> &Hccontrolcurrented {
        &self.hccontrolcurrented
    }
    #[doc = "0x28 - Contains the physical address of the first endpoint descriptor of the bulk list"]
    #[inline(always)]
    pub const fn hcbulkheaded(&self) -> &Hcbulkheaded {
        &self.hcbulkheaded
    }
    #[doc = "0x2c - Contains the physical address of the current endpoint descriptor of the bulk list"]
    #[inline(always)]
    pub const fn hcbulkcurrented(&self) -> &Hcbulkcurrented {
        &self.hcbulkcurrented
    }
    #[doc = "0x30 - Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
    #[inline(always)]
    pub const fn hcdonehead(&self) -> &Hcdonehead {
        &self.hcdonehead
    }
    #[doc = "0x34 - Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
    #[inline(always)]
    pub const fn hcfminterval(&self) -> &Hcfminterval {
        &self.hcfminterval
    }
    #[doc = "0x38 - A 14-bit counter showing the bit time remaining in the current frame"]
    #[inline(always)]
    pub const fn hcfmremaining(&self) -> &Hcfmremaining {
        &self.hcfmremaining
    }
    #[doc = "0x3c - Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
    #[inline(always)]
    pub const fn hcfmnumber(&self) -> &Hcfmnumber {
        &self.hcfmnumber
    }
    #[doc = "0x40 - Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
    #[inline(always)]
    pub const fn hcperiodicstart(&self) -> &Hcperiodicstart {
        &self.hcperiodicstart
    }
    #[doc = "0x44 - Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
    #[inline(always)]
    pub const fn hclsthreshold(&self) -> &Hclsthreshold {
        &self.hclsthreshold
    }
    #[doc = "0x48 - First of the two registers which describes the characteristics of the root hub"]
    #[inline(always)]
    pub const fn hcrhdescriptora(&self) -> &Hcrhdescriptora {
        &self.hcrhdescriptora
    }
    #[doc = "0x4c - Second of the two registers which describes the characteristics of the Root Hub"]
    #[inline(always)]
    pub const fn hcrhdescriptorb(&self) -> &Hcrhdescriptorb {
        &self.hcrhdescriptorb
    }
    #[doc = "0x50 - This register is divided into two parts"]
    #[inline(always)]
    pub const fn hcrhstatus(&self) -> &Hcrhstatus {
        &self.hcrhstatus
    }
    #[doc = "0x54 - Controls and reports the port events on a per-port basis"]
    #[inline(always)]
    pub const fn hcrhportstatus(&self) -> &Hcrhportstatus {
        &self.hcrhportstatus
    }
    #[doc = "0x5c - Controls the port if it is attached to the host block or the device block"]
    #[inline(always)]
    pub const fn portmode(&self) -> &Portmode {
        &self.portmode
    }
}
#[doc = "HCREVISION (r) register accessor: BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrevision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrevision`] module"]
#[doc(alias = "HCREVISION")]
pub type Hcrevision = crate::Reg<hcrevision::HcrevisionSpec>;
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
pub mod hcrevision;
#[doc = "HCCONTROL (rw) register accessor: Defines the operating modes of the HC\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrol`] module"]
#[doc(alias = "HCCONTROL")]
pub type Hccontrol = crate::Reg<hccontrol::HccontrolSpec>;
#[doc = "Defines the operating modes of the HC"]
pub mod hccontrol;
#[doc = "HCCOMMANDSTATUS (rw) register accessor: This register is used to receive the commands from the Host Controller Driver (HCD)\n\nYou can [`read`](crate::Reg::read) this register and get [`hccommandstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccommandstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccommandstatus`] module"]
#[doc(alias = "HCCOMMANDSTATUS")]
pub type Hccommandstatus = crate::Reg<hccommandstatus::HccommandstatusSpec>;
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
pub mod hccommandstatus;
#[doc = "HCINTERRUPTSTATUS (rw) register accessor: Indicates the status on various events that cause hardware interrupts by setting the appropriate bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptstatus`] module"]
#[doc(alias = "HCINTERRUPTSTATUS")]
pub type Hcinterruptstatus = crate::Reg<hcinterruptstatus::HcinterruptstatusSpec>;
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
pub mod hcinterruptstatus;
#[doc = "HCINTERRUPTENABLE (rw) register accessor: Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptenable`] module"]
#[doc(alias = "HCINTERRUPTENABLE")]
pub type Hcinterruptenable = crate::Reg<hcinterruptenable::HcinterruptenableSpec>;
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
pub mod hcinterruptenable;
#[doc = "HCINTERRUPTDISABLE (rw) register accessor: The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptdisable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptdisable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptdisable`] module"]
#[doc(alias = "HCINTERRUPTDISABLE")]
pub type Hcinterruptdisable = crate::Reg<hcinterruptdisable::HcinterruptdisableSpec>;
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
pub mod hcinterruptdisable;
#[doc = "HCHCCA (rw) register accessor: Contains the physical address of the host controller communication area\n\nYou can [`read`](crate::Reg::read) this register and get [`hchcca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hchcca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hchcca`] module"]
#[doc(alias = "HCHCCA")]
pub type Hchcca = crate::Reg<hchcca::HchccaSpec>;
#[doc = "Contains the physical address of the host controller communication area"]
pub mod hchcca;
#[doc = "HCPERIODCURRENTED (rw) register accessor: Contains the physical address of the current isochronous or interrupt endpoint descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodcurrented::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcperiodcurrented::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcperiodcurrented`] module"]
#[doc(alias = "HCPERIODCURRENTED")]
pub type Hcperiodcurrented = crate::Reg<hcperiodcurrented::HcperiodcurrentedSpec>;
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
pub mod hcperiodcurrented;
#[doc = "HCCONTROLHEADED (rw) register accessor: Contains the physical address of the first endpoint descriptor of the control list\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolheaded::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolheaded::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrolheaded`] module"]
#[doc(alias = "HCCONTROLHEADED")]
pub type Hccontrolheaded = crate::Reg<hccontrolheaded::HccontrolheadedSpec>;
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
pub mod hccontrolheaded;
#[doc = "HCCONTROLCURRENTED (rw) register accessor: Contains the physical address of the current endpoint descriptor of the control list\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolcurrented::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolcurrented::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrolcurrented`] module"]
#[doc(alias = "HCCONTROLCURRENTED")]
pub type Hccontrolcurrented = crate::Reg<hccontrolcurrented::HccontrolcurrentedSpec>;
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
pub mod hccontrolcurrented;
#[doc = "HCBULKHEADED (rw) register accessor: Contains the physical address of the first endpoint descriptor of the bulk list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkheaded::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkheaded::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbulkheaded`] module"]
#[doc(alias = "HCBULKHEADED")]
pub type Hcbulkheaded = crate::Reg<hcbulkheaded::HcbulkheadedSpec>;
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
pub mod hcbulkheaded;
#[doc = "HCBULKCURRENTED (rw) register accessor: Contains the physical address of the current endpoint descriptor of the bulk list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkcurrented::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkcurrented::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbulkcurrented`] module"]
#[doc(alias = "HCBULKCURRENTED")]
pub type Hcbulkcurrented = crate::Reg<hcbulkcurrented::HcbulkcurrentedSpec>;
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
pub mod hcbulkcurrented;
#[doc = "HCDONEHEAD (rw) register accessor: Contains the physical address of the last transfer descriptor added to the 'Done' queue\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdonehead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdonehead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdonehead`] module"]
#[doc(alias = "HCDONEHEAD")]
pub type Hcdonehead = crate::Reg<hcdonehead::HcdoneheadSpec>;
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
pub mod hcdonehead;
#[doc = "HCFMINTERVAL (rw) register accessor: Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfminterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfminterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfminterval`] module"]
#[doc(alias = "HCFMINTERVAL")]
pub type Hcfminterval = crate::Reg<hcfminterval::HcfmintervalSpec>;
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
pub mod hcfminterval;
#[doc = "HCFMREMAINING (rw) register accessor: A 14-bit counter showing the bit time remaining in the current frame\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmremaining::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfmremaining::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfmremaining`] module"]
#[doc(alias = "HCFMREMAINING")]
pub type Hcfmremaining = crate::Reg<hcfmremaining::HcfmremainingSpec>;
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
pub mod hcfmremaining;
#[doc = "HCFMNUMBER (rw) register accessor: Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmnumber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfmnumber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfmnumber`] module"]
#[doc(alias = "HCFMNUMBER")]
pub type Hcfmnumber = crate::Reg<hcfmnumber::HcfmnumberSpec>;
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
pub mod hcfmnumber;
#[doc = "HCPERIODICSTART (rw) register accessor: Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodicstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcperiodicstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcperiodicstart`] module"]
#[doc(alias = "HCPERIODICSTART")]
pub type Hcperiodicstart = crate::Reg<hcperiodicstart::HcperiodicstartSpec>;
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
pub mod hcperiodicstart;
#[doc = "HCLSTHRESHOLD (rw) register accessor: Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF\n\nYou can [`read`](crate::Reg::read) this register and get [`hclsthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclsthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hclsthreshold`] module"]
#[doc(alias = "HCLSTHRESHOLD")]
pub type Hclsthreshold = crate::Reg<hclsthreshold::HclsthresholdSpec>;
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
pub mod hclsthreshold;
#[doc = "HCRHDESCRIPTORA (rw) register accessor: First of the two registers which describes the characteristics of the root hub\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptora::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptora::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhdescriptora`] module"]
#[doc(alias = "HCRHDESCRIPTORA")]
pub type Hcrhdescriptora = crate::Reg<hcrhdescriptora::HcrhdescriptoraSpec>;
#[doc = "First of the two registers which describes the characteristics of the root hub"]
pub mod hcrhdescriptora;
#[doc = "HCRHDESCRIPTORB (rw) register accessor: Second of the two registers which describes the characteristics of the Root Hub\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptorb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptorb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhdescriptorb`] module"]
#[doc(alias = "HCRHDESCRIPTORB")]
pub type Hcrhdescriptorb = crate::Reg<hcrhdescriptorb::HcrhdescriptorbSpec>;
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
pub mod hcrhdescriptorb;
#[doc = "HCRHSTATUS (rw) register accessor: This register is divided into two parts\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhstatus`] module"]
#[doc(alias = "HCRHSTATUS")]
pub type Hcrhstatus = crate::Reg<hcrhstatus::HcrhstatusSpec>;
#[doc = "This register is divided into two parts"]
pub mod hcrhstatus;
#[doc = "HCRHPORTSTATUS (rw) register accessor: Controls and reports the port events on a per-port basis\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhportstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhportstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhportstatus`] module"]
#[doc(alias = "HCRHPORTSTATUS")]
pub type Hcrhportstatus = crate::Reg<hcrhportstatus::HcrhportstatusSpec>;
#[doc = "Controls and reports the port events on a per-port basis"]
pub mod hcrhportstatus;
#[doc = "PORTMODE (rw) register accessor: Controls the port if it is attached to the host block or the device block\n\nYou can [`read`](crate::Reg::read) this register and get [`portmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portmode`] module"]
#[doc(alias = "PORTMODE")]
pub type Portmode = crate::Reg<portmode::PortmodeSpec>;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;

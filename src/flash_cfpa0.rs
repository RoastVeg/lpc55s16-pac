#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    header: Header,
    version: Version,
    s_fw_version: SFwVersion,
    ns_fw_version: NsFwVersion,
    image_key_revoke: ImageKeyRevoke,
    _reserved5: [u8; 0x04],
    rotkh_revoke: RotkhRevoke,
    vendor_usage: VendorUsage,
    dcfg_cc_socu_pin: DcfgCcSocuPin,
    dcfg_cc_socu_dflt: DcfgCcSocuDflt,
    enable_fa_mode: EnableFaMode,
    cmpa_prog_in_progress: CmpaProgInProgress,
    _reserved_11_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_12_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_13_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_14_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_15_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_16_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_17_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_18_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_19_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_20_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_21_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_22_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_23_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_24_prince_region0_iv_code_prince_region0_iv: [u8; 0x04],
    _reserved_25_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_26_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_27_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_28_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_29_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_30_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_31_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_32_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_33_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_34_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_35_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_36_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_37_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_38_prince_region1_iv_code_prince_region1_iv: [u8; 0x04],
    _reserved_39_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_40_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_41_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_42_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_43_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_44_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_45_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_46_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_47_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_48_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_49_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_50_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_51_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved_52_prince_region2_iv_code_prince_region2_iv: [u8; 0x04],
    _reserved53: [u8; 0x28],
    customer_defined: [CustomerDefined; 56],
    sha256_digest: [Sha256Digest; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn header(&self) -> &Header {
        &self.header
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x08 - Secure firmware version (Monotonic counter)"]
    #[inline(always)]
    pub const fn s_fw_version(&self) -> &SFwVersion {
        &self.s_fw_version
    }
    #[doc = "0x0c - Non-Secure firmware version (Monotonic counter)"]
    #[inline(always)]
    pub const fn ns_fw_version(&self) -> &NsFwVersion {
        &self.ns_fw_version
    }
    #[doc = "0x10 - Image key revocation ID (Monotonic counter)"]
    #[inline(always)]
    pub const fn image_key_revoke(&self) -> &ImageKeyRevoke {
        &self.image_key_revoke
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn rotkh_revoke(&self) -> &RotkhRevoke {
        &self.rotkh_revoke
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn vendor_usage(&self) -> &VendorUsage {
        &self.vendor_usage
    }
    #[doc = "0x20 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn dcfg_cc_socu_pin(&self) -> &DcfgCcSocuPin {
        &self.dcfg_cc_socu_pin
    }
    #[doc = "0x24 - With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn dcfg_cc_socu_dflt(&self) -> &DcfgCcSocuDflt {
        &self.dcfg_cc_socu_dflt
    }
    #[doc = "0x28 - Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
    #[inline(always)]
    pub const fn enable_fa_mode(&self) -> &EnableFaMode {
        &self.enable_fa_mode
    }
    #[doc = "0x2c - CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
    #[inline(always)]
    pub const fn cmpa_prog_in_progress(&self) -> &CmpaProgInProgress {
        &self.cmpa_prog_in_progress
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_header0(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code0(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_header1(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code1(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code2(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body0(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code3(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body1(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code4(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body2(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code5(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body3(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code6(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body4(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code7(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body5(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code8(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body6(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code9(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body7(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code10(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body8(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code11(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body9(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code12(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body10(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_code13(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code_prince_region0_iv_body11(
        &self,
    ) -> &PrinceRegion0IvCodePrinceRegion0IvBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_header0(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code0(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_header1(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code1(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code2(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body0(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code3(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body1(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code4(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body2(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code5(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body3(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code6(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body4(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code7(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body5(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code8(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body6(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code9(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body7(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code10(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body8(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code11(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body9(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code12(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body10(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_code13(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code_prince_region1_iv_body11(
        &self,
    ) -> &PrinceRegion1IvCodePrinceRegion1IvBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_header0(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code0(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_header1(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code1(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code2(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body0(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xac - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code3(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xac - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body1(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code4(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body2(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code5(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body3(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code6(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body4(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xbc - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code7(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbc - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body5(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code8(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body6(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code9(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body7(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code10(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body8(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code11(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body9(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code12(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body10(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_code13(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code_prince_region2_iv_body11(
        &self,
    ) -> &PrinceRegion2IvCodePrinceRegion2IvBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub const fn customer_defined(&self, n: usize) -> &CustomerDefined {
        &self.customer_defined[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub fn customer_defined_iter(&self) -> impl Iterator<Item = &CustomerDefined> {
        self.customer_defined.iter()
    }
    #[doc = "0x1e0..0x200 - SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(&self, n: usize) -> &Sha256Digest {
        &self.sha256_digest[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1e0..0x200 - SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub fn sha256_digest_iter(&self) -> impl Iterator<Item = &Sha256Digest> {
        self.sha256_digest.iter()
    }
}
#[doc = "HEADER (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header`] module"]
#[doc(alias = "HEADER")]
pub type Header = crate::Reg<header::HeaderSpec>;
#[doc = "no description available"]
pub mod header;
#[doc = "VERSION (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "no description available"]
pub mod version;
#[doc = "S_FW_Version (rw) register accessor: Secure firmware version (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_fw_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_fw_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_fw_version`] module"]
#[doc(alias = "S_FW_Version")]
pub type SFwVersion = crate::Reg<s_fw_version::SFwVersionSpec>;
#[doc = "Secure firmware version (Monotonic counter)"]
pub mod s_fw_version;
#[doc = "NS_FW_Version (rw) register accessor: Non-Secure firmware version (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`ns_fw_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ns_fw_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ns_fw_version`] module"]
#[doc(alias = "NS_FW_Version")]
pub type NsFwVersion = crate::Reg<ns_fw_version::NsFwVersionSpec>;
#[doc = "Non-Secure firmware version (Monotonic counter)"]
pub mod ns_fw_version;
#[doc = "IMAGE_KEY_REVOKE (rw) register accessor: Image key revocation ID (Monotonic counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`image_key_revoke::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`image_key_revoke::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@image_key_revoke`] module"]
#[doc(alias = "IMAGE_KEY_REVOKE")]
pub type ImageKeyRevoke = crate::Reg<image_key_revoke::ImageKeyRevokeSpec>;
#[doc = "Image key revocation ID (Monotonic counter)"]
pub mod image_key_revoke;
#[doc = "ROTKH_REVOKE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`rotkh_revoke::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotkh_revoke::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rotkh_revoke`] module"]
#[doc(alias = "ROTKH_REVOKE")]
pub type RotkhRevoke = crate::Reg<rotkh_revoke::RotkhRevokeSpec>;
#[doc = "no description available"]
pub mod rotkh_revoke;
#[doc = "VENDOR_USAGE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`vendor_usage::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendor_usage::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_usage`] module"]
#[doc(alias = "VENDOR_USAGE")]
pub type VendorUsage = crate::Reg<vendor_usage::VendorUsageSpec>;
#[doc = "no description available"]
pub mod vendor_usage;
#[doc = "DCFG_CC_SOCU_PIN (rw) register accessor: With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg_cc_socu_pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg_cc_socu_pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg_cc_socu_pin`] module"]
#[doc(alias = "DCFG_CC_SOCU_PIN")]
pub type DcfgCcSocuPin = crate::Reg<dcfg_cc_socu_pin::DcfgCcSocuPinSpec>;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_pin;
#[doc = "DCFG_CC_SOCU_DFLT (rw) register accessor: With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg_cc_socu_dflt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg_cc_socu_dflt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg_cc_socu_dflt`] module"]
#[doc(alias = "DCFG_CC_SOCU_DFLT")]
pub type DcfgCcSocuDflt = crate::Reg<dcfg_cc_socu_dflt::DcfgCcSocuDfltSpec>;
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
pub mod dcfg_cc_socu_dflt;
#[doc = "ENABLE_FA_MODE (rw) register accessor: Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_fa_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_fa_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_fa_mode`] module"]
#[doc(alias = "ENABLE_FA_MODE")]
pub type EnableFaMode = crate::Reg<enable_fa_mode::EnableFaModeSpec>;
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
pub mod enable_fa_mode;
#[doc = "CMPA_PROG_IN_PROGRESS (rw) register accessor: CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpa_prog_in_progress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpa_prog_in_progress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpa_prog_in_progress`] module"]
#[doc(alias = "CMPA_PROG_IN_PROGRESS")]
pub type CmpaProgInProgress = crate::Reg<cmpa_prog_in_progress::CmpaProgInProgressSpec>;
#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
pub mod cmpa_prog_in_progress;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code0`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE0")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode0 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code0::PrinceRegion0IvCodePrinceRegion0IvCode0Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code0;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_HEADER0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_header0`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_HEADER0")]
pub type PrinceRegion0IvCodePrinceRegion0IvHeader0 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_header0::PrinceRegion0IvCodePrinceRegion0IvHeader0Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_header0;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code1`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE1")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode1 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code1::PrinceRegion0IvCodePrinceRegion0IvCode1Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code1;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_HEADER1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_header1`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_HEADER1")]
pub type PrinceRegion0IvCodePrinceRegion0IvHeader1 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_header1::PrinceRegion0IvCodePrinceRegion0IvHeader1Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_header1;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body0`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY0")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody0 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body0::PrinceRegion0IvCodePrinceRegion0IvBody0Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body0;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code2`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE2")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode2 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code2::PrinceRegion0IvCodePrinceRegion0IvCode2Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code2;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body1`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY1")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody1 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body1::PrinceRegion0IvCodePrinceRegion0IvBody1Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body1;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code3`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE3")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode3 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code3::PrinceRegion0IvCodePrinceRegion0IvCode3Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code3;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body2`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY2")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody2 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body2::PrinceRegion0IvCodePrinceRegion0IvBody2Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body2;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code4`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE4")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode4 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code4::PrinceRegion0IvCodePrinceRegion0IvCode4Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code4;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body3`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY3")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody3 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body3::PrinceRegion0IvCodePrinceRegion0IvBody3Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body3;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code5`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE5")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode5 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code5::PrinceRegion0IvCodePrinceRegion0IvCode5Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code5;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body4`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY4")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody4 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body4::PrinceRegion0IvCodePrinceRegion0IvBody4Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body4;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code6`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE6")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode6 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code6::PrinceRegion0IvCodePrinceRegion0IvCode6Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code6;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body5`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY5")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody5 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body5::PrinceRegion0IvCodePrinceRegion0IvBody5Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body5;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code7`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE7")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode7 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code7::PrinceRegion0IvCodePrinceRegion0IvCode7Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code7;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body6`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY6")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody6 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body6::PrinceRegion0IvCodePrinceRegion0IvBody6Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body6;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code8`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE8")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode8 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code8::PrinceRegion0IvCodePrinceRegion0IvCode8Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code8;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body7`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY7")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody7 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body7::PrinceRegion0IvCodePrinceRegion0IvBody7Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body7;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code9`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE9")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode9 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code9::PrinceRegion0IvCodePrinceRegion0IvCode9Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code9;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body8`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY8")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody8 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body8::PrinceRegion0IvCodePrinceRegion0IvBody8Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body8;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code10`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE10")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode10 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code10::PrinceRegion0IvCodePrinceRegion0IvCode10Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code10;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body9`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY9")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody9 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body9::PrinceRegion0IvCodePrinceRegion0IvBody9Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body9;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code11`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE11")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode11 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code11::PrinceRegion0IvCodePrinceRegion0IvCode11Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code11;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body10`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY10")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody10 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body10::PrinceRegion0IvCodePrinceRegion0IvBody10Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body10;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE12 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code12`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE12")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode12 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code12::PrinceRegion0IvCodePrinceRegion0IvCode12Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code12;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_body11`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_BODY11")]
pub type PrinceRegion0IvCodePrinceRegion0IvBody11 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_body11::PrinceRegion0IvCodePrinceRegion0IvBody11Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_body11;
#[doc = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE13 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_iv_code_prince_region0_iv_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_iv_code_prince_region0_iv_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_iv_code_prince_region0_iv_code13`] module"]
#[doc(alias = "PRINCE_REGION0_IV_CODE_PRINCE_REGION0_IV_CODE13")]
pub type PrinceRegion0IvCodePrinceRegion0IvCode13 = crate::Reg<
    prince_region0_iv_code_prince_region0_iv_code13::PrinceRegion0IvCodePrinceRegion0IvCode13Spec,
>;
#[doc = "no description available"]
pub mod prince_region0_iv_code_prince_region0_iv_code13;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code0`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE0")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode0 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code0::PrinceRegion1IvCodePrinceRegion1IvCode0Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code0;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_HEADER0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_header0`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_HEADER0")]
pub type PrinceRegion1IvCodePrinceRegion1IvHeader0 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_header0::PrinceRegion1IvCodePrinceRegion1IvHeader0Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_header0;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code1`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE1")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode1 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code1::PrinceRegion1IvCodePrinceRegion1IvCode1Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code1;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_HEADER1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_header1`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_HEADER1")]
pub type PrinceRegion1IvCodePrinceRegion1IvHeader1 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_header1::PrinceRegion1IvCodePrinceRegion1IvHeader1Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_header1;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body0`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY0")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody0 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body0::PrinceRegion1IvCodePrinceRegion1IvBody0Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body0;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code2`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE2")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode2 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code2::PrinceRegion1IvCodePrinceRegion1IvCode2Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code2;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body1`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY1")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody1 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body1::PrinceRegion1IvCodePrinceRegion1IvBody1Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body1;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code3`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE3")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode3 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code3::PrinceRegion1IvCodePrinceRegion1IvCode3Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code3;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body2`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY2")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody2 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body2::PrinceRegion1IvCodePrinceRegion1IvBody2Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body2;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code4`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE4")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode4 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code4::PrinceRegion1IvCodePrinceRegion1IvCode4Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code4;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body3`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY3")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody3 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body3::PrinceRegion1IvCodePrinceRegion1IvBody3Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body3;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code5`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE5")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode5 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code5::PrinceRegion1IvCodePrinceRegion1IvCode5Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code5;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body4`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY4")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody4 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body4::PrinceRegion1IvCodePrinceRegion1IvBody4Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body4;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code6`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE6")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode6 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code6::PrinceRegion1IvCodePrinceRegion1IvCode6Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code6;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body5`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY5")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody5 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body5::PrinceRegion1IvCodePrinceRegion1IvBody5Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body5;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code7`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE7")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode7 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code7::PrinceRegion1IvCodePrinceRegion1IvCode7Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code7;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body6`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY6")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody6 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body6::PrinceRegion1IvCodePrinceRegion1IvBody6Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body6;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code8`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE8")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode8 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code8::PrinceRegion1IvCodePrinceRegion1IvCode8Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code8;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body7`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY7")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody7 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body7::PrinceRegion1IvCodePrinceRegion1IvBody7Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body7;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code9`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE9")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode9 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code9::PrinceRegion1IvCodePrinceRegion1IvCode9Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code9;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body8`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY8")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody8 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body8::PrinceRegion1IvCodePrinceRegion1IvBody8Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body8;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code10`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE10")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode10 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code10::PrinceRegion1IvCodePrinceRegion1IvCode10Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code10;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body9`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY9")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody9 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body9::PrinceRegion1IvCodePrinceRegion1IvBody9Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body9;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code11`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE11")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode11 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code11::PrinceRegion1IvCodePrinceRegion1IvCode11Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code11;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body10`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY10")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody10 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body10::PrinceRegion1IvCodePrinceRegion1IvBody10Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body10;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE12 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code12`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE12")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode12 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code12::PrinceRegion1IvCodePrinceRegion1IvCode12Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code12;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_body11`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_BODY11")]
pub type PrinceRegion1IvCodePrinceRegion1IvBody11 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_body11::PrinceRegion1IvCodePrinceRegion1IvBody11Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_body11;
#[doc = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE13 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_iv_code_prince_region1_iv_code13`] module"]
#[doc(alias = "PRINCE_REGION1_IV_CODE_PRINCE_REGION1_IV_CODE13")]
pub type PrinceRegion1IvCodePrinceRegion1IvCode13 = crate::Reg<
    prince_region1_iv_code_prince_region1_iv_code13::PrinceRegion1IvCodePrinceRegion1IvCode13Spec,
>;
#[doc = "no description available"]
pub mod prince_region1_iv_code_prince_region1_iv_code13;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code0`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE0")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode0 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code0::PrinceRegion2IvCodePrinceRegion2IvCode0Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code0;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_HEADER0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_header0`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_HEADER0")]
pub type PrinceRegion2IvCodePrinceRegion2IvHeader0 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_header0::PrinceRegion2IvCodePrinceRegion2IvHeader0Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_header0;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code1`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE1")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode1 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code1::PrinceRegion2IvCodePrinceRegion2IvCode1Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code1;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_HEADER1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_header1`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_HEADER1")]
pub type PrinceRegion2IvCodePrinceRegion2IvHeader1 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_header1::PrinceRegion2IvCodePrinceRegion2IvHeader1Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_header1;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body0`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY0")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody0 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body0::PrinceRegion2IvCodePrinceRegion2IvBody0Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body0;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code2`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE2")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode2 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code2::PrinceRegion2IvCodePrinceRegion2IvCode2Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code2;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body1`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY1")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody1 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body1::PrinceRegion2IvCodePrinceRegion2IvBody1Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body1;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code3`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE3")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode3 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code3::PrinceRegion2IvCodePrinceRegion2IvCode3Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code3;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body2`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY2")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody2 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body2::PrinceRegion2IvCodePrinceRegion2IvBody2Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body2;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code4`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE4")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode4 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code4::PrinceRegion2IvCodePrinceRegion2IvCode4Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code4;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body3`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY3")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody3 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body3::PrinceRegion2IvCodePrinceRegion2IvBody3Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body3;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code5`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE5")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode5 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code5::PrinceRegion2IvCodePrinceRegion2IvCode5Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code5;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY4 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body4`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY4")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody4 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body4::PrinceRegion2IvCodePrinceRegion2IvBody4Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body4;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code6`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE6")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode6 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code6::PrinceRegion2IvCodePrinceRegion2IvCode6Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code6;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY5 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body5`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY5")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody5 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body5::PrinceRegion2IvCodePrinceRegion2IvBody5Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body5;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code7`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE7")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode7 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code7::PrinceRegion2IvCodePrinceRegion2IvCode7Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code7;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY6 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body6`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY6")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody6 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body6::PrinceRegion2IvCodePrinceRegion2IvBody6Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body6;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code8`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE8")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode8 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code8::PrinceRegion2IvCodePrinceRegion2IvCode8Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code8;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY7 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body7`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY7")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody7 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body7::PrinceRegion2IvCodePrinceRegion2IvBody7Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body7;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code9`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE9")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode9 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code9::PrinceRegion2IvCodePrinceRegion2IvCode9Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code9;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY8 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body8`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY8")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody8 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body8::PrinceRegion2IvCodePrinceRegion2IvBody8Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body8;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code10`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE10")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode10 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code10::PrinceRegion2IvCodePrinceRegion2IvCode10Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code10;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY9 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body9`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY9")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody9 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body9::PrinceRegion2IvCodePrinceRegion2IvBody9Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body9;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code11`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE11")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode11 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code11::PrinceRegion2IvCodePrinceRegion2IvCode11Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code11;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY10 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body10`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY10")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody10 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body10::PrinceRegion2IvCodePrinceRegion2IvBody10Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body10;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE12 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code12`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE12")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode12 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code12::PrinceRegion2IvCodePrinceRegion2IvCode12Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code12;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY11 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_body11`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_BODY11")]
pub type PrinceRegion2IvCodePrinceRegion2IvBody11 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_body11::PrinceRegion2IvCodePrinceRegion2IvBody11Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_body11;
#[doc = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE13 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_iv_code_prince_region2_iv_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_iv_code_prince_region2_iv_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_iv_code_prince_region2_iv_code13`] module"]
#[doc(alias = "PRINCE_REGION2_IV_CODE_PRINCE_REGION2_IV_CODE13")]
pub type PrinceRegion2IvCodePrinceRegion2IvCode13 = crate::Reg<
    prince_region2_iv_code_prince_region2_iv_code13::PrinceRegion2IvCodePrinceRegion2IvCode13Spec,
>;
#[doc = "no description available"]
pub mod prince_region2_iv_code_prince_region2_iv_code13;
#[doc = "CUSTOMER_DEFINED (rw) register accessor: Customer Defined (Programable through ROM API)\n\nYou can [`read`](crate::Reg::read) this register and get [`customer_defined::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer_defined::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer_defined`] module"]
#[doc(alias = "CUSTOMER_DEFINED")]
pub type CustomerDefined = crate::Reg<customer_defined::CustomerDefinedSpec>;
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST (rw) register accessor: SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha256_digest`] module"]
#[doc(alias = "SHA256_DIGEST")]
pub type Sha256Digest = crate::Reg<sha256_digest::Sha256DigestSpec>;
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
pub mod sha256_digest;

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    header: Header,
    puf_discharge_time_in_ms: PufDischargeTimeInMs,
    activation_code: [ActivationCode; 298],
    _reserved_3_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_4_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_5_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_6_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_7_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_8_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_9_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_10_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_11_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_12_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_13_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_14_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_15_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_16_sbkey_key_code_sbkey: [u8; 0x04],
    _reserved_17_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_18_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_19_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_20_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_21_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_22_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_23_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_24_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_25_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_26_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_27_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_28_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_29_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_30_user_kek_key_code_user_kek: [u8; 0x04],
    _reserved_31_uds_key_code_uds: [u8; 0x04],
    _reserved_32_uds_key_code_uds: [u8; 0x04],
    _reserved_33_uds_key_code_uds: [u8; 0x04],
    _reserved_34_uds_key_code_uds: [u8; 0x04],
    _reserved_35_uds_key_code_uds: [u8; 0x04],
    _reserved_36_uds_key_code_uds: [u8; 0x04],
    _reserved_37_uds_key_code_uds: [u8; 0x04],
    _reserved_38_uds_key_code_uds: [u8; 0x04],
    _reserved_39_uds_key_code_uds: [u8; 0x04],
    _reserved_40_uds_key_code_uds: [u8; 0x04],
    _reserved_41_uds_key_code_uds: [u8; 0x04],
    _reserved_42_uds_key_code_uds: [u8; 0x04],
    _reserved_43_uds_key_code_uds: [u8; 0x04],
    _reserved_44_uds_key_code_uds: [u8; 0x04],
    _reserved_45_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_46_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_47_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_48_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_49_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_50_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_51_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_52_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_53_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_54_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_55_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_56_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_57_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_58_prince_region0_key_code_prince_region0: [u8; 0x04],
    _reserved_59_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_60_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_61_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_62_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_63_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_64_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_65_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_66_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_67_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_68_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_69_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_70_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_71_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_72_prince_region1_key_code_prince_region1: [u8; 0x04],
    _reserved_73_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_74_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_75_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_76_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_77_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_78_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_79_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_80_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_81_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_82_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_83_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_84_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_85_prince_region2_key_code_prince_region2: [u8; 0x04],
    _reserved_86_prince_region2_key_code_prince_region2: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Valid Key Sore Header : 0x95959595"]
    #[inline(always)]
    pub const fn header(&self) -> &Header {
        &self.header
    }
    #[doc = "0x04 - puf discharge time in ms."]
    #[inline(always)]
    pub const fn puf_discharge_time_in_ms(&self) -> &PufDischargeTimeInMs {
        &self.puf_discharge_time_in_ms
    }
    #[doc = "0x08..0x4b0 - ."]
    #[inline(always)]
    pub const fn activation_code(&self, n: usize) -> &ActivationCode {
        &self.activation_code[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x4b0 - ."]
    #[inline(always)]
    pub fn activation_code_iter(&self) -> impl Iterator<Item = &ActivationCode> {
        self.activation_code.iter()
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code0(&self) -> &SbkeyKeyCodeSbkeyKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1200).cast() }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_header0(&self) -> &SbkeyKeyCodeSbkeyHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1200).cast() }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code1(&self) -> &SbkeyKeyCodeSbkeyKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1204).cast() }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_header1(&self) -> &SbkeyKeyCodeSbkeyHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1204).cast() }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code2(&self) -> &SbkeyKeyCodeSbkeyKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1208).cast() }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body0(&self) -> &SbkeyKeyCodeSbkeyBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1208).cast() }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code3(&self) -> &SbkeyKeyCodeSbkeyKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1212).cast() }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body1(&self) -> &SbkeyKeyCodeSbkeyBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1212).cast() }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code4(&self) -> &SbkeyKeyCodeSbkeyKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1216).cast() }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body2(&self) -> &SbkeyKeyCodeSbkeyBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1216).cast() }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code5(&self) -> &SbkeyKeyCodeSbkeyKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1220).cast() }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body3(&self) -> &SbkeyKeyCodeSbkeyBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1220).cast() }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code6(&self) -> &SbkeyKeyCodeSbkeyKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1224).cast() }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body4(&self) -> &SbkeyKeyCodeSbkeyBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1224).cast() }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code7(&self) -> &SbkeyKeyCodeSbkeyKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1228).cast() }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body5(&self) -> &SbkeyKeyCodeSbkeyBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1228).cast() }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code8(&self) -> &SbkeyKeyCodeSbkeyKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1232).cast() }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body6(&self) -> &SbkeyKeyCodeSbkeyBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1232).cast() }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code9(&self) -> &SbkeyKeyCodeSbkeyKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1236).cast() }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body7(&self) -> &SbkeyKeyCodeSbkeyBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1236).cast() }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code10(&self) -> &SbkeyKeyCodeSbkeyKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1240).cast() }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body8(&self) -> &SbkeyKeyCodeSbkeyBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1240).cast() }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code11(&self) -> &SbkeyKeyCodeSbkeyKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1244).cast() }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body9(&self) -> &SbkeyKeyCodeSbkeyBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1244).cast() }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code12(&self) -> &SbkeyKeyCodeSbkeyKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1248).cast() }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body10(&self) -> &SbkeyKeyCodeSbkeyBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1248).cast() }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_key_code13(&self) -> &SbkeyKeyCodeSbkeyKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1252).cast() }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code_sbkey_body11(&self) -> &SbkeyKeyCodeSbkeyBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1252).cast() }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code0(&self) -> &UserKekKeyCodeUserKekKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1256).cast() }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_header0(&self) -> &UserKekKeyCodeUserKekHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1256).cast() }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code1(&self) -> &UserKekKeyCodeUserKekKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1260).cast() }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_header1(&self) -> &UserKekKeyCodeUserKekHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1260).cast() }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code2(&self) -> &UserKekKeyCodeUserKekKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1264).cast() }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body0(&self) -> &UserKekKeyCodeUserKekBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1264).cast() }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code3(&self) -> &UserKekKeyCodeUserKekKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1268).cast() }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body1(&self) -> &UserKekKeyCodeUserKekBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1268).cast() }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code4(&self) -> &UserKekKeyCodeUserKekKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1272).cast() }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body2(&self) -> &UserKekKeyCodeUserKekBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1272).cast() }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code5(&self) -> &UserKekKeyCodeUserKekKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1276).cast() }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body3(&self) -> &UserKekKeyCodeUserKekBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1276).cast() }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code6(&self) -> &UserKekKeyCodeUserKekKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body4(&self) -> &UserKekKeyCodeUserKekBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code7(&self) -> &UserKekKeyCodeUserKekKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body5(&self) -> &UserKekKeyCodeUserKekBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code8(&self) -> &UserKekKeyCodeUserKekKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body6(&self) -> &UserKekKeyCodeUserKekBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code9(&self) -> &UserKekKeyCodeUserKekKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1292).cast() }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body7(&self) -> &UserKekKeyCodeUserKekBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1292).cast() }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code10(&self) -> &UserKekKeyCodeUserKekKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1296).cast() }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body8(&self) -> &UserKekKeyCodeUserKekBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1296).cast() }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code11(&self) -> &UserKekKeyCodeUserKekKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1300).cast() }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body9(&self) -> &UserKekKeyCodeUserKekBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1300).cast() }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code12(&self) -> &UserKekKeyCodeUserKekKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1304).cast() }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body10(&self) -> &UserKekKeyCodeUserKekBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1304).cast() }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_key_code13(&self) -> &UserKekKeyCodeUserKekKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1308).cast() }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code_user_kek_body11(&self) -> &UserKekKeyCodeUserKekBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1308).cast() }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code0(&self) -> &UdsKeyCodeUdsKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_header0(&self) -> &UdsKeyCodeUdsHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code1(&self) -> &UdsKeyCodeUdsKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_header1(&self) -> &UdsKeyCodeUdsHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code2(&self) -> &UdsKeyCodeUdsKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body0(&self) -> &UdsKeyCodeUdsBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code3(&self) -> &UdsKeyCodeUdsKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1324).cast() }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body1(&self) -> &UdsKeyCodeUdsBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1324).cast() }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code4(&self) -> &UdsKeyCodeUdsKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body2(&self) -> &UdsKeyCodeUdsBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code5(&self) -> &UdsKeyCodeUdsKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1332).cast() }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body3(&self) -> &UdsKeyCodeUdsBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1332).cast() }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code6(&self) -> &UdsKeyCodeUdsKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1336).cast() }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body4(&self) -> &UdsKeyCodeUdsBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1336).cast() }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code7(&self) -> &UdsKeyCodeUdsKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1340).cast() }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body5(&self) -> &UdsKeyCodeUdsBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1340).cast() }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code8(&self) -> &UdsKeyCodeUdsKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body6(&self) -> &UdsKeyCodeUdsBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code9(&self) -> &UdsKeyCodeUdsKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body7(&self) -> &UdsKeyCodeUdsBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code10(&self) -> &UdsKeyCodeUdsKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1352).cast() }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body8(&self) -> &UdsKeyCodeUdsBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1352).cast() }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code11(&self) -> &UdsKeyCodeUdsKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1356).cast() }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body9(&self) -> &UdsKeyCodeUdsBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1356).cast() }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code12(&self) -> &UdsKeyCodeUdsKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1360).cast() }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body10(&self) -> &UdsKeyCodeUdsBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1360).cast() }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_key_code13(&self) -> &UdsKeyCodeUdsKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1364).cast() }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub const fn uds_key_code_uds_body11(&self) -> &UdsKeyCodeUdsBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1364).cast() }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code0(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1368).cast() }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_header0(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1368).cast() }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code1(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1372).cast() }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_header1(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1372).cast() }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code2(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body0(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code3(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body1(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code4(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body2(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code5(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body3(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code6(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1392).cast() }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body4(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1392).cast() }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code7(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1396).cast() }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body5(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1396).cast() }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code8(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1400).cast() }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body6(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1400).cast() }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code9(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1404).cast() }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body7(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1404).cast() }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code10(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1408).cast() }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body8(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1408).cast() }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code11(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1412).cast() }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body9(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1412).cast() }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code12(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1416).cast() }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body10(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1416).cast() }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_key_code13(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1420).cast() }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code_prince_region0_body11(
        &self,
    ) -> &PrinceRegion0KeyCodePrinceRegion0Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1420).cast() }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code0(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_header0(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code1(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1428).cast() }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_header1(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1428).cast() }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code2(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1432).cast() }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body0(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1432).cast() }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code3(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1436).cast() }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body1(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1436).cast() }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code4(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1440).cast() }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body2(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1440).cast() }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code5(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1444).cast() }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body3(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1444).cast() }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code6(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1448).cast() }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body4(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1448).cast() }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code7(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1452).cast() }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body5(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1452).cast() }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code8(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1456).cast() }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body6(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1456).cast() }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code9(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1460).cast() }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body7(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1460).cast() }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code10(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1464).cast() }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body8(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1464).cast() }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code11(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1468).cast() }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body9(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1468).cast() }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code12(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body10(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_key_code13(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1476).cast() }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code_prince_region1_body11(
        &self,
    ) -> &PrinceRegion1KeyCodePrinceRegion1Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1476).cast() }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code0(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1480).cast() }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_header0(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1480).cast() }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code1(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1484).cast() }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_header1(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1484).cast() }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code2(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1488).cast() }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body0(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1488).cast() }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code3(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1492).cast() }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body1(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1492).cast() }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code4(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1496).cast() }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body2(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1496).cast() }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code5(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1500).cast() }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body3(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1500).cast() }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code6(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1504).cast() }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body4(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1504).cast() }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code7(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1508).cast() }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body5(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1508).cast() }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code8(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1512).cast() }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body6(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1512).cast() }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code9(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1516).cast() }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body7(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1516).cast() }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code10(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body8(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code11(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1524).cast() }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body9(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1524).cast() }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code12(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1528).cast() }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body10(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1528).cast() }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_key_code13(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1532).cast() }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code_prince_region2_body11(
        &self,
    ) -> &PrinceRegion2KeyCodePrinceRegion2Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1532).cast() }
    }
}
#[doc = "HEADER (rw) register accessor: Valid Key Sore Header : 0x95959595\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header`] module"]
#[doc(alias = "HEADER")]
pub type Header = crate::Reg<header::HeaderSpec>;
#[doc = "Valid Key Sore Header : 0x95959595"]
pub mod header;
#[doc = "puf_discharge_time_in_ms (rw) register accessor: puf discharge time in ms.\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_discharge_time_in_ms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_discharge_time_in_ms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@puf_discharge_time_in_ms`] module"]
#[doc(alias = "puf_discharge_time_in_ms")]
pub type PufDischargeTimeInMs = crate::Reg<puf_discharge_time_in_ms::PufDischargeTimeInMsSpec>;
#[doc = "puf discharge time in ms."]
pub mod puf_discharge_time_in_ms;
#[doc = "ACTIVATION_CODE (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`activation_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`activation_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@activation_code`] module"]
#[doc(alias = "ACTIVATION_CODE")]
pub type ActivationCode = crate::Reg<activation_code::ActivationCodeSpec>;
#[doc = "."]
pub mod activation_code;
#[doc = "SBKEY_KEY_CODE_SBKEY_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_header0`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_HEADER0")]
pub type SbkeyKeyCodeSbkeyHeader0 =
    crate::Reg<sbkey_key_code_sbkey_header0::SbkeyKeyCodeSbkeyHeader0Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_header0;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code0`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE0")]
pub type SbkeyKeyCodeSbkeyKeyCode0 =
    crate::Reg<sbkey_key_code_sbkey_key_code0::SbkeyKeyCodeSbkeyKeyCode0Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code0;
#[doc = "SBKEY_KEY_CODE_SBKEY_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_header1`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_HEADER1")]
pub type SbkeyKeyCodeSbkeyHeader1 =
    crate::Reg<sbkey_key_code_sbkey_header1::SbkeyKeyCodeSbkeyHeader1Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_header1;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code1`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE1")]
pub type SbkeyKeyCodeSbkeyKeyCode1 =
    crate::Reg<sbkey_key_code_sbkey_key_code1::SbkeyKeyCodeSbkeyKeyCode1Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code1;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body0`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY0")]
pub type SbkeyKeyCodeSbkeyBody0 =
    crate::Reg<sbkey_key_code_sbkey_body0::SbkeyKeyCodeSbkeyBody0Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body0;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code2`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE2")]
pub type SbkeyKeyCodeSbkeyKeyCode2 =
    crate::Reg<sbkey_key_code_sbkey_key_code2::SbkeyKeyCodeSbkeyKeyCode2Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code2;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body1`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY1")]
pub type SbkeyKeyCodeSbkeyBody1 =
    crate::Reg<sbkey_key_code_sbkey_body1::SbkeyKeyCodeSbkeyBody1Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body1;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code3`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE3")]
pub type SbkeyKeyCodeSbkeyKeyCode3 =
    crate::Reg<sbkey_key_code_sbkey_key_code3::SbkeyKeyCodeSbkeyKeyCode3Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code3;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body2`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY2")]
pub type SbkeyKeyCodeSbkeyBody2 =
    crate::Reg<sbkey_key_code_sbkey_body2::SbkeyKeyCodeSbkeyBody2Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body2;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code4`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE4")]
pub type SbkeyKeyCodeSbkeyKeyCode4 =
    crate::Reg<sbkey_key_code_sbkey_key_code4::SbkeyKeyCodeSbkeyKeyCode4Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code4;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body3`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY3")]
pub type SbkeyKeyCodeSbkeyBody3 =
    crate::Reg<sbkey_key_code_sbkey_body3::SbkeyKeyCodeSbkeyBody3Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body3;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code5`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE5")]
pub type SbkeyKeyCodeSbkeyKeyCode5 =
    crate::Reg<sbkey_key_code_sbkey_key_code5::SbkeyKeyCodeSbkeyKeyCode5Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code5;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body4`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY4")]
pub type SbkeyKeyCodeSbkeyBody4 =
    crate::Reg<sbkey_key_code_sbkey_body4::SbkeyKeyCodeSbkeyBody4Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body4;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code6`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE6")]
pub type SbkeyKeyCodeSbkeyKeyCode6 =
    crate::Reg<sbkey_key_code_sbkey_key_code6::SbkeyKeyCodeSbkeyKeyCode6Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code6;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body5`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY5")]
pub type SbkeyKeyCodeSbkeyBody5 =
    crate::Reg<sbkey_key_code_sbkey_body5::SbkeyKeyCodeSbkeyBody5Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body5;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code7`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE7")]
pub type SbkeyKeyCodeSbkeyKeyCode7 =
    crate::Reg<sbkey_key_code_sbkey_key_code7::SbkeyKeyCodeSbkeyKeyCode7Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code7;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body6`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY6")]
pub type SbkeyKeyCodeSbkeyBody6 =
    crate::Reg<sbkey_key_code_sbkey_body6::SbkeyKeyCodeSbkeyBody6Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body6;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code8`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE8")]
pub type SbkeyKeyCodeSbkeyKeyCode8 =
    crate::Reg<sbkey_key_code_sbkey_key_code8::SbkeyKeyCodeSbkeyKeyCode8Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code8;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body7`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY7")]
pub type SbkeyKeyCodeSbkeyBody7 =
    crate::Reg<sbkey_key_code_sbkey_body7::SbkeyKeyCodeSbkeyBody7Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body7;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code9`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE9")]
pub type SbkeyKeyCodeSbkeyKeyCode9 =
    crate::Reg<sbkey_key_code_sbkey_key_code9::SbkeyKeyCodeSbkeyKeyCode9Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code9;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body8`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY8")]
pub type SbkeyKeyCodeSbkeyBody8 =
    crate::Reg<sbkey_key_code_sbkey_body8::SbkeyKeyCodeSbkeyBody8Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body8;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code10`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE10")]
pub type SbkeyKeyCodeSbkeyKeyCode10 =
    crate::Reg<sbkey_key_code_sbkey_key_code10::SbkeyKeyCodeSbkeyKeyCode10Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code10;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body9`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY9")]
pub type SbkeyKeyCodeSbkeyBody9 =
    crate::Reg<sbkey_key_code_sbkey_body9::SbkeyKeyCodeSbkeyBody9Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body9;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code11`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE11")]
pub type SbkeyKeyCodeSbkeyKeyCode11 =
    crate::Reg<sbkey_key_code_sbkey_key_code11::SbkeyKeyCodeSbkeyKeyCode11Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code11;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body10`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY10")]
pub type SbkeyKeyCodeSbkeyBody10 =
    crate::Reg<sbkey_key_code_sbkey_body10::SbkeyKeyCodeSbkeyBody10Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body10;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code12`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE12")]
pub type SbkeyKeyCodeSbkeyKeyCode12 =
    crate::Reg<sbkey_key_code_sbkey_key_code12::SbkeyKeyCodeSbkeyKeyCode12Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code12;
#[doc = "SBKEY_KEY_CODE_SBKEY_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_body11`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_BODY11")]
pub type SbkeyKeyCodeSbkeyBody11 =
    crate::Reg<sbkey_key_code_sbkey_body11::SbkeyKeyCodeSbkeyBody11Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_body11;
#[doc = "SBKEY_KEY_CODE_SBKEY_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code_sbkey_key_code13`] module"]
#[doc(alias = "SBKEY_KEY_CODE_SBKEY_KEY_CODE13")]
pub type SbkeyKeyCodeSbkeyKeyCode13 =
    crate::Reg<sbkey_key_code_sbkey_key_code13::SbkeyKeyCodeSbkeyKeyCode13Spec>;
#[doc = "."]
pub mod sbkey_key_code_sbkey_key_code13;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_header0`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_HEADER0")]
pub type UserKekKeyCodeUserKekHeader0 =
    crate::Reg<user_kek_key_code_user_kek_header0::UserKekKeyCodeUserKekHeader0Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_header0;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code0`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE0")]
pub type UserKekKeyCodeUserKekKeyCode0 =
    crate::Reg<user_kek_key_code_user_kek_key_code0::UserKekKeyCodeUserKekKeyCode0Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code0;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_header1`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_HEADER1")]
pub type UserKekKeyCodeUserKekHeader1 =
    crate::Reg<user_kek_key_code_user_kek_header1::UserKekKeyCodeUserKekHeader1Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_header1;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code1`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE1")]
pub type UserKekKeyCodeUserKekKeyCode1 =
    crate::Reg<user_kek_key_code_user_kek_key_code1::UserKekKeyCodeUserKekKeyCode1Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code1;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body0`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY0")]
pub type UserKekKeyCodeUserKekBody0 =
    crate::Reg<user_kek_key_code_user_kek_body0::UserKekKeyCodeUserKekBody0Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body0;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code2`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE2")]
pub type UserKekKeyCodeUserKekKeyCode2 =
    crate::Reg<user_kek_key_code_user_kek_key_code2::UserKekKeyCodeUserKekKeyCode2Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code2;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body1`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY1")]
pub type UserKekKeyCodeUserKekBody1 =
    crate::Reg<user_kek_key_code_user_kek_body1::UserKekKeyCodeUserKekBody1Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body1;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code3`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE3")]
pub type UserKekKeyCodeUserKekKeyCode3 =
    crate::Reg<user_kek_key_code_user_kek_key_code3::UserKekKeyCodeUserKekKeyCode3Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code3;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body2`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY2")]
pub type UserKekKeyCodeUserKekBody2 =
    crate::Reg<user_kek_key_code_user_kek_body2::UserKekKeyCodeUserKekBody2Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body2;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code4`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE4")]
pub type UserKekKeyCodeUserKekKeyCode4 =
    crate::Reg<user_kek_key_code_user_kek_key_code4::UserKekKeyCodeUserKekKeyCode4Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code4;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body3`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY3")]
pub type UserKekKeyCodeUserKekBody3 =
    crate::Reg<user_kek_key_code_user_kek_body3::UserKekKeyCodeUserKekBody3Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body3;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code5`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE5")]
pub type UserKekKeyCodeUserKekKeyCode5 =
    crate::Reg<user_kek_key_code_user_kek_key_code5::UserKekKeyCodeUserKekKeyCode5Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code5;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body4`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY4")]
pub type UserKekKeyCodeUserKekBody4 =
    crate::Reg<user_kek_key_code_user_kek_body4::UserKekKeyCodeUserKekBody4Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body4;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code6`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE6")]
pub type UserKekKeyCodeUserKekKeyCode6 =
    crate::Reg<user_kek_key_code_user_kek_key_code6::UserKekKeyCodeUserKekKeyCode6Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code6;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body5`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY5")]
pub type UserKekKeyCodeUserKekBody5 =
    crate::Reg<user_kek_key_code_user_kek_body5::UserKekKeyCodeUserKekBody5Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body5;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code7`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE7")]
pub type UserKekKeyCodeUserKekKeyCode7 =
    crate::Reg<user_kek_key_code_user_kek_key_code7::UserKekKeyCodeUserKekKeyCode7Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code7;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body6`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY6")]
pub type UserKekKeyCodeUserKekBody6 =
    crate::Reg<user_kek_key_code_user_kek_body6::UserKekKeyCodeUserKekBody6Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body6;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code8`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE8")]
pub type UserKekKeyCodeUserKekKeyCode8 =
    crate::Reg<user_kek_key_code_user_kek_key_code8::UserKekKeyCodeUserKekKeyCode8Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code8;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body7`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY7")]
pub type UserKekKeyCodeUserKekBody7 =
    crate::Reg<user_kek_key_code_user_kek_body7::UserKekKeyCodeUserKekBody7Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body7;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code9`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE9")]
pub type UserKekKeyCodeUserKekKeyCode9 =
    crate::Reg<user_kek_key_code_user_kek_key_code9::UserKekKeyCodeUserKekKeyCode9Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code9;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body8`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY8")]
pub type UserKekKeyCodeUserKekBody8 =
    crate::Reg<user_kek_key_code_user_kek_body8::UserKekKeyCodeUserKekBody8Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body8;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code10`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE10")]
pub type UserKekKeyCodeUserKekKeyCode10 =
    crate::Reg<user_kek_key_code_user_kek_key_code10::UserKekKeyCodeUserKekKeyCode10Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code10;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body9`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY9")]
pub type UserKekKeyCodeUserKekBody9 =
    crate::Reg<user_kek_key_code_user_kek_body9::UserKekKeyCodeUserKekBody9Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body9;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code11`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE11")]
pub type UserKekKeyCodeUserKekKeyCode11 =
    crate::Reg<user_kek_key_code_user_kek_key_code11::UserKekKeyCodeUserKekKeyCode11Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code11;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body10`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY10")]
pub type UserKekKeyCodeUserKekBody10 =
    crate::Reg<user_kek_key_code_user_kek_body10::UserKekKeyCodeUserKekBody10Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body10;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code12`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE12")]
pub type UserKekKeyCodeUserKekKeyCode12 =
    crate::Reg<user_kek_key_code_user_kek_key_code12::UserKekKeyCodeUserKekKeyCode12Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code12;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_body11`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_BODY11")]
pub type UserKekKeyCodeUserKekBody11 =
    crate::Reg<user_kek_key_code_user_kek_body11::UserKekKeyCodeUserKekBody11Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_body11;
#[doc = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code_user_kek_key_code13`] module"]
#[doc(alias = "USER_KEK_KEY_CODE_USER_KEK_KEY_CODE13")]
pub type UserKekKeyCodeUserKekKeyCode13 =
    crate::Reg<user_kek_key_code_user_kek_key_code13::UserKekKeyCodeUserKekKeyCode13Spec>;
#[doc = "."]
pub mod user_kek_key_code_user_kek_key_code13;
#[doc = "UDS_KEY_CODE_UDS_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_header0`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_HEADER0")]
pub type UdsKeyCodeUdsHeader0 = crate::Reg<uds_key_code_uds_header0::UdsKeyCodeUdsHeader0Spec>;
#[doc = "."]
pub mod uds_key_code_uds_header0;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code0`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE0")]
pub type UdsKeyCodeUdsKeyCode0 = crate::Reg<uds_key_code_uds_key_code0::UdsKeyCodeUdsKeyCode0Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code0;
#[doc = "UDS_KEY_CODE_UDS_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_header1`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_HEADER1")]
pub type UdsKeyCodeUdsHeader1 = crate::Reg<uds_key_code_uds_header1::UdsKeyCodeUdsHeader1Spec>;
#[doc = "."]
pub mod uds_key_code_uds_header1;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code1`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE1")]
pub type UdsKeyCodeUdsKeyCode1 = crate::Reg<uds_key_code_uds_key_code1::UdsKeyCodeUdsKeyCode1Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code1;
#[doc = "UDS_KEY_CODE_UDS_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body0`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY0")]
pub type UdsKeyCodeUdsBody0 = crate::Reg<uds_key_code_uds_body0::UdsKeyCodeUdsBody0Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body0;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code2`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE2")]
pub type UdsKeyCodeUdsKeyCode2 = crate::Reg<uds_key_code_uds_key_code2::UdsKeyCodeUdsKeyCode2Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code2;
#[doc = "UDS_KEY_CODE_UDS_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body1`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY1")]
pub type UdsKeyCodeUdsBody1 = crate::Reg<uds_key_code_uds_body1::UdsKeyCodeUdsBody1Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body1;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code3`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE3")]
pub type UdsKeyCodeUdsKeyCode3 = crate::Reg<uds_key_code_uds_key_code3::UdsKeyCodeUdsKeyCode3Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code3;
#[doc = "UDS_KEY_CODE_UDS_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body2`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY2")]
pub type UdsKeyCodeUdsBody2 = crate::Reg<uds_key_code_uds_body2::UdsKeyCodeUdsBody2Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body2;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code4`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE4")]
pub type UdsKeyCodeUdsKeyCode4 = crate::Reg<uds_key_code_uds_key_code4::UdsKeyCodeUdsKeyCode4Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code4;
#[doc = "UDS_KEY_CODE_UDS_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body3`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY3")]
pub type UdsKeyCodeUdsBody3 = crate::Reg<uds_key_code_uds_body3::UdsKeyCodeUdsBody3Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body3;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code5`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE5")]
pub type UdsKeyCodeUdsKeyCode5 = crate::Reg<uds_key_code_uds_key_code5::UdsKeyCodeUdsKeyCode5Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code5;
#[doc = "UDS_KEY_CODE_UDS_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body4`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY4")]
pub type UdsKeyCodeUdsBody4 = crate::Reg<uds_key_code_uds_body4::UdsKeyCodeUdsBody4Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body4;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code6`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE6")]
pub type UdsKeyCodeUdsKeyCode6 = crate::Reg<uds_key_code_uds_key_code6::UdsKeyCodeUdsKeyCode6Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code6;
#[doc = "UDS_KEY_CODE_UDS_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body5`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY5")]
pub type UdsKeyCodeUdsBody5 = crate::Reg<uds_key_code_uds_body5::UdsKeyCodeUdsBody5Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body5;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code7`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE7")]
pub type UdsKeyCodeUdsKeyCode7 = crate::Reg<uds_key_code_uds_key_code7::UdsKeyCodeUdsKeyCode7Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code7;
#[doc = "UDS_KEY_CODE_UDS_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body6`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY6")]
pub type UdsKeyCodeUdsBody6 = crate::Reg<uds_key_code_uds_body6::UdsKeyCodeUdsBody6Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body6;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code8`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE8")]
pub type UdsKeyCodeUdsKeyCode8 = crate::Reg<uds_key_code_uds_key_code8::UdsKeyCodeUdsKeyCode8Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code8;
#[doc = "UDS_KEY_CODE_UDS_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body7`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY7")]
pub type UdsKeyCodeUdsBody7 = crate::Reg<uds_key_code_uds_body7::UdsKeyCodeUdsBody7Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body7;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code9`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE9")]
pub type UdsKeyCodeUdsKeyCode9 = crate::Reg<uds_key_code_uds_key_code9::UdsKeyCodeUdsKeyCode9Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code9;
#[doc = "UDS_KEY_CODE_UDS_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body8`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY8")]
pub type UdsKeyCodeUdsBody8 = crate::Reg<uds_key_code_uds_body8::UdsKeyCodeUdsBody8Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body8;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code10`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE10")]
pub type UdsKeyCodeUdsKeyCode10 =
    crate::Reg<uds_key_code_uds_key_code10::UdsKeyCodeUdsKeyCode10Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code10;
#[doc = "UDS_KEY_CODE_UDS_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body9`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY9")]
pub type UdsKeyCodeUdsBody9 = crate::Reg<uds_key_code_uds_body9::UdsKeyCodeUdsBody9Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body9;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code11`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE11")]
pub type UdsKeyCodeUdsKeyCode11 =
    crate::Reg<uds_key_code_uds_key_code11::UdsKeyCodeUdsKeyCode11Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code11;
#[doc = "UDS_KEY_CODE_UDS_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body10`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY10")]
pub type UdsKeyCodeUdsBody10 = crate::Reg<uds_key_code_uds_body10::UdsKeyCodeUdsBody10Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body10;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code12`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE12")]
pub type UdsKeyCodeUdsKeyCode12 =
    crate::Reg<uds_key_code_uds_key_code12::UdsKeyCodeUdsKeyCode12Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code12;
#[doc = "UDS_KEY_CODE_UDS_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_body11`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_BODY11")]
pub type UdsKeyCodeUdsBody11 = crate::Reg<uds_key_code_uds_body11::UdsKeyCodeUdsBody11Spec>;
#[doc = "."]
pub mod uds_key_code_uds_body11;
#[doc = "UDS_KEY_CODE_UDS_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code_uds_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code_uds_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code_uds_key_code13`] module"]
#[doc(alias = "UDS_KEY_CODE_UDS_KEY_CODE13")]
pub type UdsKeyCodeUdsKeyCode13 =
    crate::Reg<uds_key_code_uds_key_code13::UdsKeyCodeUdsKeyCode13Spec>;
#[doc = "."]
pub mod uds_key_code_uds_key_code13;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_header0`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_HEADER0")]
pub type PrinceRegion0KeyCodePrinceRegion0Header0 = crate::Reg<
    prince_region0_key_code_prince_region0_header0::PrinceRegion0KeyCodePrinceRegion0Header0Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_header0;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code0`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE0")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode0 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code0::PrinceRegion0KeyCodePrinceRegion0KeyCode0Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code0;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_header1`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_HEADER1")]
pub type PrinceRegion0KeyCodePrinceRegion0Header1 = crate::Reg<
    prince_region0_key_code_prince_region0_header1::PrinceRegion0KeyCodePrinceRegion0Header1Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_header1;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code1`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE1")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode1 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code1::PrinceRegion0KeyCodePrinceRegion0KeyCode1Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code1;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body0`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY0")]
pub type PrinceRegion0KeyCodePrinceRegion0Body0 = crate::Reg<
    prince_region0_key_code_prince_region0_body0::PrinceRegion0KeyCodePrinceRegion0Body0Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body0;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code2`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE2")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode2 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code2::PrinceRegion0KeyCodePrinceRegion0KeyCode2Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code2;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body1`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY1")]
pub type PrinceRegion0KeyCodePrinceRegion0Body1 = crate::Reg<
    prince_region0_key_code_prince_region0_body1::PrinceRegion0KeyCodePrinceRegion0Body1Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body1;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code3`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE3")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode3 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code3::PrinceRegion0KeyCodePrinceRegion0KeyCode3Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code3;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body2`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY2")]
pub type PrinceRegion0KeyCodePrinceRegion0Body2 = crate::Reg<
    prince_region0_key_code_prince_region0_body2::PrinceRegion0KeyCodePrinceRegion0Body2Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body2;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code4`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE4")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode4 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code4::PrinceRegion0KeyCodePrinceRegion0KeyCode4Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code4;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body3`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY3")]
pub type PrinceRegion0KeyCodePrinceRegion0Body3 = crate::Reg<
    prince_region0_key_code_prince_region0_body3::PrinceRegion0KeyCodePrinceRegion0Body3Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body3;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code5`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE5")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode5 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code5::PrinceRegion0KeyCodePrinceRegion0KeyCode5Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code5;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body4`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY4")]
pub type PrinceRegion0KeyCodePrinceRegion0Body4 = crate::Reg<
    prince_region0_key_code_prince_region0_body4::PrinceRegion0KeyCodePrinceRegion0Body4Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body4;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code6`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE6")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode6 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code6::PrinceRegion0KeyCodePrinceRegion0KeyCode6Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code6;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body5`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY5")]
pub type PrinceRegion0KeyCodePrinceRegion0Body5 = crate::Reg<
    prince_region0_key_code_prince_region0_body5::PrinceRegion0KeyCodePrinceRegion0Body5Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body5;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code7`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE7")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode7 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code7::PrinceRegion0KeyCodePrinceRegion0KeyCode7Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code7;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body6`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY6")]
pub type PrinceRegion0KeyCodePrinceRegion0Body6 = crate::Reg<
    prince_region0_key_code_prince_region0_body6::PrinceRegion0KeyCodePrinceRegion0Body6Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body6;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code8`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE8")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode8 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code8::PrinceRegion0KeyCodePrinceRegion0KeyCode8Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code8;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body7`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY7")]
pub type PrinceRegion0KeyCodePrinceRegion0Body7 = crate::Reg<
    prince_region0_key_code_prince_region0_body7::PrinceRegion0KeyCodePrinceRegion0Body7Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body7;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code9`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE9")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode9 = crate::Reg<
    prince_region0_key_code_prince_region0_key_code9::PrinceRegion0KeyCodePrinceRegion0KeyCode9Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code9;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body8`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY8")]
pub type PrinceRegion0KeyCodePrinceRegion0Body8 = crate::Reg<
    prince_region0_key_code_prince_region0_body8::PrinceRegion0KeyCodePrinceRegion0Body8Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body8;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code10`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE10")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode10 = crate :: Reg < prince_region0_key_code_prince_region0_key_code10 :: PrinceRegion0KeyCodePrinceRegion0KeyCode10Spec > ;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code10;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body9`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY9")]
pub type PrinceRegion0KeyCodePrinceRegion0Body9 = crate::Reg<
    prince_region0_key_code_prince_region0_body9::PrinceRegion0KeyCodePrinceRegion0Body9Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body9;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code11`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE11")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode11 = crate :: Reg < prince_region0_key_code_prince_region0_key_code11 :: PrinceRegion0KeyCodePrinceRegion0KeyCode11Spec > ;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code11;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body10`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY10")]
pub type PrinceRegion0KeyCodePrinceRegion0Body10 = crate::Reg<
    prince_region0_key_code_prince_region0_body10::PrinceRegion0KeyCodePrinceRegion0Body10Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body10;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code12`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE12")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode12 = crate :: Reg < prince_region0_key_code_prince_region0_key_code12 :: PrinceRegion0KeyCodePrinceRegion0KeyCode12Spec > ;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code12;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_body11`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_BODY11")]
pub type PrinceRegion0KeyCodePrinceRegion0Body11 = crate::Reg<
    prince_region0_key_code_prince_region0_body11::PrinceRegion0KeyCodePrinceRegion0Body11Spec,
>;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_body11;
#[doc = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code_prince_region0_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code_prince_region0_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code_prince_region0_key_code13`] module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE_PRINCE_REGION0_KEY_CODE13")]
pub type PrinceRegion0KeyCodePrinceRegion0KeyCode13 = crate :: Reg < prince_region0_key_code_prince_region0_key_code13 :: PrinceRegion0KeyCodePrinceRegion0KeyCode13Spec > ;
#[doc = "."]
pub mod prince_region0_key_code_prince_region0_key_code13;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_header0`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_HEADER0")]
pub type PrinceRegion1KeyCodePrinceRegion1Header0 = crate::Reg<
    prince_region1_key_code_prince_region1_header0::PrinceRegion1KeyCodePrinceRegion1Header0Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_header0;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code0`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE0")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode0 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code0::PrinceRegion1KeyCodePrinceRegion1KeyCode0Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code0;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_header1`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_HEADER1")]
pub type PrinceRegion1KeyCodePrinceRegion1Header1 = crate::Reg<
    prince_region1_key_code_prince_region1_header1::PrinceRegion1KeyCodePrinceRegion1Header1Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_header1;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code1`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE1")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode1 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code1::PrinceRegion1KeyCodePrinceRegion1KeyCode1Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code1;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body0`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY0")]
pub type PrinceRegion1KeyCodePrinceRegion1Body0 = crate::Reg<
    prince_region1_key_code_prince_region1_body0::PrinceRegion1KeyCodePrinceRegion1Body0Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body0;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code2`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE2")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode2 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code2::PrinceRegion1KeyCodePrinceRegion1KeyCode2Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code2;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body1`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY1")]
pub type PrinceRegion1KeyCodePrinceRegion1Body1 = crate::Reg<
    prince_region1_key_code_prince_region1_body1::PrinceRegion1KeyCodePrinceRegion1Body1Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body1;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code3`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE3")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode3 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code3::PrinceRegion1KeyCodePrinceRegion1KeyCode3Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code3;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body2`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY2")]
pub type PrinceRegion1KeyCodePrinceRegion1Body2 = crate::Reg<
    prince_region1_key_code_prince_region1_body2::PrinceRegion1KeyCodePrinceRegion1Body2Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body2;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code4`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE4")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode4 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code4::PrinceRegion1KeyCodePrinceRegion1KeyCode4Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code4;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body3`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY3")]
pub type PrinceRegion1KeyCodePrinceRegion1Body3 = crate::Reg<
    prince_region1_key_code_prince_region1_body3::PrinceRegion1KeyCodePrinceRegion1Body3Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body3;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code5`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE5")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode5 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code5::PrinceRegion1KeyCodePrinceRegion1KeyCode5Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code5;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body4`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY4")]
pub type PrinceRegion1KeyCodePrinceRegion1Body4 = crate::Reg<
    prince_region1_key_code_prince_region1_body4::PrinceRegion1KeyCodePrinceRegion1Body4Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body4;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code6`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE6")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode6 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code6::PrinceRegion1KeyCodePrinceRegion1KeyCode6Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code6;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body5`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY5")]
pub type PrinceRegion1KeyCodePrinceRegion1Body5 = crate::Reg<
    prince_region1_key_code_prince_region1_body5::PrinceRegion1KeyCodePrinceRegion1Body5Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body5;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code7`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE7")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode7 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code7::PrinceRegion1KeyCodePrinceRegion1KeyCode7Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code7;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body6`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY6")]
pub type PrinceRegion1KeyCodePrinceRegion1Body6 = crate::Reg<
    prince_region1_key_code_prince_region1_body6::PrinceRegion1KeyCodePrinceRegion1Body6Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body6;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code8`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE8")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode8 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code8::PrinceRegion1KeyCodePrinceRegion1KeyCode8Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code8;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body7`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY7")]
pub type PrinceRegion1KeyCodePrinceRegion1Body7 = crate::Reg<
    prince_region1_key_code_prince_region1_body7::PrinceRegion1KeyCodePrinceRegion1Body7Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body7;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code9`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE9")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode9 = crate::Reg<
    prince_region1_key_code_prince_region1_key_code9::PrinceRegion1KeyCodePrinceRegion1KeyCode9Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code9;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body8`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY8")]
pub type PrinceRegion1KeyCodePrinceRegion1Body8 = crate::Reg<
    prince_region1_key_code_prince_region1_body8::PrinceRegion1KeyCodePrinceRegion1Body8Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body8;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code10`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE10")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode10 = crate :: Reg < prince_region1_key_code_prince_region1_key_code10 :: PrinceRegion1KeyCodePrinceRegion1KeyCode10Spec > ;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code10;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body9`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY9")]
pub type PrinceRegion1KeyCodePrinceRegion1Body9 = crate::Reg<
    prince_region1_key_code_prince_region1_body9::PrinceRegion1KeyCodePrinceRegion1Body9Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body9;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code11`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE11")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode11 = crate :: Reg < prince_region1_key_code_prince_region1_key_code11 :: PrinceRegion1KeyCodePrinceRegion1KeyCode11Spec > ;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code11;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body10`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY10")]
pub type PrinceRegion1KeyCodePrinceRegion1Body10 = crate::Reg<
    prince_region1_key_code_prince_region1_body10::PrinceRegion1KeyCodePrinceRegion1Body10Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body10;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code12`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE12")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode12 = crate :: Reg < prince_region1_key_code_prince_region1_key_code12 :: PrinceRegion1KeyCodePrinceRegion1KeyCode12Spec > ;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code12;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_body11`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_BODY11")]
pub type PrinceRegion1KeyCodePrinceRegion1Body11 = crate::Reg<
    prince_region1_key_code_prince_region1_body11::PrinceRegion1KeyCodePrinceRegion1Body11Spec,
>;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_body11;
#[doc = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code_prince_region1_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code_prince_region1_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code_prince_region1_key_code13`] module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE_PRINCE_REGION1_KEY_CODE13")]
pub type PrinceRegion1KeyCodePrinceRegion1KeyCode13 = crate :: Reg < prince_region1_key_code_prince_region1_key_code13 :: PrinceRegion1KeyCodePrinceRegion1KeyCode13Spec > ;
#[doc = "."]
pub mod prince_region1_key_code_prince_region1_key_code13;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_header0`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_HEADER0")]
pub type PrinceRegion2KeyCodePrinceRegion2Header0 = crate::Reg<
    prince_region2_key_code_prince_region2_header0::PrinceRegion2KeyCodePrinceRegion2Header0Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_header0;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code0`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE0")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode0 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code0::PrinceRegion2KeyCodePrinceRegion2KeyCode0Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code0;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_header1`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_HEADER1")]
pub type PrinceRegion2KeyCodePrinceRegion2Header1 = crate::Reg<
    prince_region2_key_code_prince_region2_header1::PrinceRegion2KeyCodePrinceRegion2Header1Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_header1;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code1`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE1")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode1 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code1::PrinceRegion2KeyCodePrinceRegion2KeyCode1Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code1;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body0`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY0")]
pub type PrinceRegion2KeyCodePrinceRegion2Body0 = crate::Reg<
    prince_region2_key_code_prince_region2_body0::PrinceRegion2KeyCodePrinceRegion2Body0Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body0;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code2`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE2")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode2 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code2::PrinceRegion2KeyCodePrinceRegion2KeyCode2Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code2;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body1`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY1")]
pub type PrinceRegion2KeyCodePrinceRegion2Body1 = crate::Reg<
    prince_region2_key_code_prince_region2_body1::PrinceRegion2KeyCodePrinceRegion2Body1Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body1;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code3`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE3")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode3 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code3::PrinceRegion2KeyCodePrinceRegion2KeyCode3Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code3;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body2`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY2")]
pub type PrinceRegion2KeyCodePrinceRegion2Body2 = crate::Reg<
    prince_region2_key_code_prince_region2_body2::PrinceRegion2KeyCodePrinceRegion2Body2Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body2;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code4`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE4")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode4 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code4::PrinceRegion2KeyCodePrinceRegion2KeyCode4Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code4;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body3`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY3")]
pub type PrinceRegion2KeyCodePrinceRegion2Body3 = crate::Reg<
    prince_region2_key_code_prince_region2_body3::PrinceRegion2KeyCodePrinceRegion2Body3Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body3;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code5`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE5")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode5 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code5::PrinceRegion2KeyCodePrinceRegion2KeyCode5Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code5;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body4`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY4")]
pub type PrinceRegion2KeyCodePrinceRegion2Body4 = crate::Reg<
    prince_region2_key_code_prince_region2_body4::PrinceRegion2KeyCodePrinceRegion2Body4Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body4;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code6`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE6")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode6 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code6::PrinceRegion2KeyCodePrinceRegion2KeyCode6Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code6;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body5`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY5")]
pub type PrinceRegion2KeyCodePrinceRegion2Body5 = crate::Reg<
    prince_region2_key_code_prince_region2_body5::PrinceRegion2KeyCodePrinceRegion2Body5Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body5;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code7`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE7")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode7 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code7::PrinceRegion2KeyCodePrinceRegion2KeyCode7Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code7;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body6`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY6")]
pub type PrinceRegion2KeyCodePrinceRegion2Body6 = crate::Reg<
    prince_region2_key_code_prince_region2_body6::PrinceRegion2KeyCodePrinceRegion2Body6Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body6;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code8`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE8")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode8 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code8::PrinceRegion2KeyCodePrinceRegion2KeyCode8Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code8;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body7`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY7")]
pub type PrinceRegion2KeyCodePrinceRegion2Body7 = crate::Reg<
    prince_region2_key_code_prince_region2_body7::PrinceRegion2KeyCodePrinceRegion2Body7Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body7;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code9`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE9")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode9 = crate::Reg<
    prince_region2_key_code_prince_region2_key_code9::PrinceRegion2KeyCodePrinceRegion2KeyCode9Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code9;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body8`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY8")]
pub type PrinceRegion2KeyCodePrinceRegion2Body8 = crate::Reg<
    prince_region2_key_code_prince_region2_body8::PrinceRegion2KeyCodePrinceRegion2Body8Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body8;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code10`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE10")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode10 = crate :: Reg < prince_region2_key_code_prince_region2_key_code10 :: PrinceRegion2KeyCodePrinceRegion2KeyCode10Spec > ;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code10;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body9`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY9")]
pub type PrinceRegion2KeyCodePrinceRegion2Body9 = crate::Reg<
    prince_region2_key_code_prince_region2_body9::PrinceRegion2KeyCodePrinceRegion2Body9Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body9;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code11`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE11")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode11 = crate :: Reg < prince_region2_key_code_prince_region2_key_code11 :: PrinceRegion2KeyCodePrinceRegion2KeyCode11Spec > ;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code11;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body10`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY10")]
pub type PrinceRegion2KeyCodePrinceRegion2Body10 = crate::Reg<
    prince_region2_key_code_prince_region2_body10::PrinceRegion2KeyCodePrinceRegion2Body10Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body10;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code12`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE12")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode12 = crate :: Reg < prince_region2_key_code_prince_region2_key_code12 :: PrinceRegion2KeyCodePrinceRegion2KeyCode12Spec > ;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code12;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_body11`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_BODY11")]
pub type PrinceRegion2KeyCodePrinceRegion2Body11 = crate::Reg<
    prince_region2_key_code_prince_region2_body11::PrinceRegion2KeyCodePrinceRegion2Body11Spec,
>;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_body11;
#[doc = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code_prince_region2_key_code13`] module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE_PRINCE_REGION2_KEY_CODE13")]
pub type PrinceRegion2KeyCodePrinceRegion2KeyCode13 = crate :: Reg < prince_region2_key_code_prince_region2_key_code13 :: PrinceRegion2KeyCodePrinceRegion2KeyCode13Spec > ;
#[doc = "."]
pub mod prince_region2_key_code_prince_region2_key_code13;

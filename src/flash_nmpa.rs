#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_gpo0_gpo0: [u8; 0x04],
    _reserved_1_gpo0_gpo0: [u8; 0x04],
    _reserved_2_gpo0_gpo0: [u8; 0x04],
    _reserved_3_gpo0_gpo0: [u8; 0x04],
    _reserved_4_gpo1_gpo1: [u8; 0x04],
    _reserved_5_gpo1_gpo1: [u8; 0x04],
    _reserved_6_gpo1_gpo1: [u8; 0x04],
    _reserved_7_gpo1_gpo1: [u8; 0x04],
    _reserved_8_gpo2_gpo2: [u8; 0x04],
    _reserved_9_gpo2_gpo2: [u8; 0x04],
    _reserved_10_gpo2_gpo2: [u8; 0x04],
    _reserved_11_gpo2_gpo2: [u8; 0x04],
    _reserved_12_gpo3_gpo3: [u8; 0x04],
    _reserved_13_gpo3_gpo3: [u8; 0x04],
    _reserved_14_gpo3_gpo3: [u8; 0x04],
    _reserved_15_gpo3_gpo3: [u8; 0x04],
    _reserved_16_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_17_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_18_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_19_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_20_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_21_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_22_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_23_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    device_type: DeviceType,
    final_test_program_version: FinalTestProgramVersion,
    final_test_date: FinalTestDate,
    final_test_time: FinalTestTime,
    _reserved_28_uuid_uuid: [u8; 0x04],
    _reserved_29_uuid_uuid: [u8; 0x04],
    _reserved_30_uuid_uuid: [u8; 0x04],
    _reserved_31_uuid_uuid: [u8; 0x04],
    wafer_test1_program_version: WaferTest1ProgramVersion,
    wafer_test1_date: WaferTest1Date,
    wafer_test1_time: WaferTest1Time,
    _reserved35: [u8; 0x04],
    wafer_test2_program_version: WaferTest2ProgramVersion,
    wafer_test2_date: WaferTest2Date,
    wafer_test2_time: WaferTest2Time,
    usbcfg: Usbcfg,
    periphencfg: Periphencfg,
    ramsizecfg: Ramsizecfg,
    flashsizecfg: Flashsizecfg,
    _reserved42: [u8; 0x04],
    ringo_0: Ringo0,
    ringo_1: Ringo1,
    ringo_2: Ringo2,
    _reserved45: [u8; 0x04],
    fro_192mhz: Fro192mhz,
    _reserved46: [u8; 0x04],
    xo_32mhz: Xo32mhz,
    xo_32khz: Xo32khz,
    fro_1mhz: Fro1mhz,
    _reserved49: [u8; 0x04],
    _reserved_49_dcdc_power_profile_high_dcdc_power_profile_high: [u8; 0x04],
    _reserved_50_dcdc_power_profile_high_dcdc_power_profile_high: [u8; 0x04],
    _reserved_51_dcdc_power_profile_low_dcdc_power_profile_low: [u8; 0x04],
    _reserved_52_dcdc_power_profile_low_dcdc_power_profile_low: [u8; 0x04],
    _reserved_53_dcdc_power_profile_medium_dcdc_power_profile_medium: [u8; 0x04],
    _reserved_54_dcdc_power_profile_medium_dcdc_power_profile_medium: [u8; 0x04],
    bod: Bod,
    ldo_ao: LdoAo,
    sdio_delay: SdioDelay,
    _reserved58: [u8; 0x04],
    _reserved_58_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_59_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_60_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_61_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_62_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_63_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_64_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_65_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    temp_sens_vbe1vbe8_ref_1: TempSensVbe1vbe8Ref1,
    temp_sens_vbe1vbe8_ref_2: TempSensVbe1vbe8Ref2,
    temp_sens_slope: TempSensSlope,
    temp_sens_offset: TempSensOffset,
    _reserved_70_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved_71_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved_72_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved73: [u8; 0x04],
    _reserved_73_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    _reserved_74_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    _reserved_75_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    nxp_device_private_key: [NxpDevicePrivateKey; 13],
    nxp_device_certificate_0: [NxpDeviceCertificate0; 4],
    nxp_device_certificate_1: [NxpDeviceCertificate1; 4],
    nxp_device_certificate_2: [NxpDeviceCertificate2; 4],
    nxp_device_certificate_3: [NxpDeviceCertificate3; 4],
    sha256_digest: [Sha256Digest; 8],
    _reserved_82_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_83_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_84_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_85_ecid_backup_ecid_backup: [u8; 0x04],
    checksum: [Checksum; 4],
    _reserved87: [u8; 0x0aac],
    dis_rom_hiding: DisRomHiding,
    _reserved88: [u8; 0x0c],
    puf_sram: PufSram,
}
impl RegisterBlock {
    #[doc = "0x00 - GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_array0(&self) -> &Gpo0Gpo0Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - GPO0 register 0 description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_0(&self) -> &Gpo0Gpo0_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_array1(&self) -> &Gpo0Gpo0Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - GPO0 register 1 description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_1(&self) -> &Gpo0Gpo0_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_array2(&self) -> &Gpo0Gpo0Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - GPO0 register 2 description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_2(&self) -> &Gpo0Gpo0_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_array3(&self) -> &Gpo0Gpo0Array3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - GPO0 register 3 description"]
    #[inline(always)]
    pub const fn gpo0_gpo0_3(&self) -> &Gpo0Gpo0_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_array0(&self) -> &Gpo1Gpo1Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - GPO1 register 0 description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_0(&self) -> &Gpo1Gpo1_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_array1(&self) -> &Gpo1Gpo1Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - GPO1 register 1 description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_1(&self) -> &Gpo1Gpo1_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_array2(&self) -> &Gpo1Gpo1Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - GPO1 register 2 description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_2(&self) -> &Gpo1Gpo1_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_array3(&self) -> &Gpo1Gpo1Array3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - GPO1 register 3 description"]
    #[inline(always)]
    pub const fn gpo1_gpo1_3(&self) -> &Gpo1Gpo1_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_array0(&self) -> &Gpo2Gpo2Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - GPO2 register 0 description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_0(&self) -> &Gpo2Gpo2_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_array1(&self) -> &Gpo2Gpo2Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - GPO2 register 1 description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_1(&self) -> &Gpo2Gpo2_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x28 - GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_array2(&self) -> &Gpo2Gpo2Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - GPO2 register 2 description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_2(&self) -> &Gpo2Gpo2_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_array3(&self) -> &Gpo2Gpo2Array3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - GPO2 register 3 description"]
    #[inline(always)]
    pub const fn gpo2_gpo2_3(&self) -> &Gpo2Gpo2_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_array0(&self) -> &Gpo3Gpo3Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - GPO3 register 0 description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_0(&self) -> &Gpo3Gpo3_0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_array1(&self) -> &Gpo3Gpo3Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - GPO3 register 1 description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_1(&self) -> &Gpo3Gpo3_1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_array2(&self) -> &Gpo3Gpo3Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - GPO3 register 2 description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_2(&self) -> &Gpo3Gpo3_2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_array3(&self) -> &Gpo3Gpo3Array3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - GPO3 register 3 description"]
    #[inline(always)]
    pub const fn gpo3_gpo3_3(&self) -> &Gpo3Gpo3_3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_array0(&self) -> &GpoChecksumGpoChecksumArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - checksum of the GPO data in words 0"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_0(&self) -> &GpoChecksumGpoChecksum0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_array1(&self) -> &GpoChecksumGpoChecksumArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - checksum of the GPO data in words 1"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_1(&self) -> &GpoChecksumGpoChecksum1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_array2(&self) -> &GpoChecksumGpoChecksumArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - checksum of the GPO data in words 2"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_2(&self) -> &GpoChecksumGpoChecksum2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_array3(&self) -> &GpoChecksumGpoChecksumArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - checksum of the GPO data in words 3"]
    #[inline(always)]
    pub const fn gpo_checksum_gpo_checksum_3(&self) -> &GpoChecksumGpoChecksum3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_array0(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchIdArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_0(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchId0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_array1(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchIdArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_1(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchId1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_array2(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchIdArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_2(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchId2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_array3(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchIdArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_final_test_batch_id_3(
        &self,
    ) -> &FinalTestBatchIdFinalTestBatchId3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn device_type(&self) -> &DeviceType {
        &self.device_type
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn final_test_program_version(&self) -> &FinalTestProgramVersion {
        &self.final_test_program_version
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn final_test_date(&self) -> &FinalTestDate {
        &self.final_test_date
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn final_test_time(&self) -> &FinalTestTime {
        &self.final_test_time
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_array0(&self) -> &UuidUuidArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_0(&self) -> &UuidUuid0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_array1(&self) -> &UuidUuidArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_1(&self) -> &UuidUuid1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_array2(&self) -> &UuidUuidArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_2(&self) -> &UuidUuid2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_array3(&self) -> &UuidUuidArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn uuid_uuid_3(&self) -> &UuidUuid3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn wafer_test1_program_version(&self) -> &WaferTest1ProgramVersion {
        &self.wafer_test1_program_version
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn wafer_test1_date(&self) -> &WaferTest1Date {
        &self.wafer_test1_date
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn wafer_test1_time(&self) -> &WaferTest1Time {
        &self.wafer_test1_time
    }
    #[doc = "0x90 - no description available"]
    #[inline(always)]
    pub const fn wafer_test2_program_version(&self) -> &WaferTest2ProgramVersion {
        &self.wafer_test2_program_version
    }
    #[doc = "0x94 - no description available"]
    #[inline(always)]
    pub const fn wafer_test2_date(&self) -> &WaferTest2Date {
        &self.wafer_test2_date
    }
    #[doc = "0x98 - no description available"]
    #[inline(always)]
    pub const fn wafer_test2_time(&self) -> &WaferTest2Time {
        &self.wafer_test2_time
    }
    #[doc = "0x9c - no description available"]
    #[inline(always)]
    pub const fn usbcfg(&self) -> &Usbcfg {
        &self.usbcfg
    }
    #[doc = "0xa0 - no description available"]
    #[inline(always)]
    pub const fn periphencfg(&self) -> &Periphencfg {
        &self.periphencfg
    }
    #[doc = "0xa4 - no description available"]
    #[inline(always)]
    pub const fn ramsizecfg(&self) -> &Ramsizecfg {
        &self.ramsizecfg
    }
    #[doc = "0xa8 - no description available"]
    #[inline(always)]
    pub const fn flashsizecfg(&self) -> &Flashsizecfg {
        &self.flashsizecfg
    }
    #[doc = "0xb0 - no description available"]
    #[inline(always)]
    pub const fn ringo_0(&self) -> &Ringo0 {
        &self.ringo_0
    }
    #[doc = "0xb4 - no description available"]
    #[inline(always)]
    pub const fn ringo_1(&self) -> &Ringo1 {
        &self.ringo_1
    }
    #[doc = "0xb8 - no description available"]
    #[inline(always)]
    pub const fn ringo_2(&self) -> &Ringo2 {
        &self.ringo_2
    }
    #[doc = "0xc0 - no description available"]
    #[inline(always)]
    pub const fn fro_192mhz(&self) -> &Fro192mhz {
        &self.fro_192mhz
    }
    #[doc = "0xc8 - no description available"]
    #[inline(always)]
    pub const fn xo_32mhz(&self) -> &Xo32mhz {
        &self.xo_32mhz
    }
    #[doc = "0xcc - no description available"]
    #[inline(always)]
    pub const fn xo_32khz(&self) -> &Xo32khz {
        &self.xo_32khz
    }
    #[doc = "0xd0 - no description available"]
    #[inline(always)]
    pub const fn fro_1mhz(&self) -> &Fro1mhz {
        &self.fro_1mhz
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_dcdc_power_profile_high_array0(
        &self,
    ) -> &DcdcPowerProfileHighDcdcPowerProfileHighArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_dcdc_power_profile_high_0(
        &self,
    ) -> &DcdcPowerProfileHighDcdcPowerProfileHigh0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_dcdc_power_profile_high_array1(
        &self,
    ) -> &DcdcPowerProfileHighDcdcPowerProfileHighArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_dcdc_power_profile_high_1(
        &self,
    ) -> &DcdcPowerProfileHighDcdcPowerProfileHigh1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_dcdc_power_profile_low_array0(
        &self,
    ) -> &DcdcPowerProfileLowDcdcPowerProfileLowArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_dcdc_power_profile_low_0(
        &self,
    ) -> &DcdcPowerProfileLowDcdcPowerProfileLow0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_dcdc_power_profile_low_array1(
        &self,
    ) -> &DcdcPowerProfileLowDcdcPowerProfileLowArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_dcdc_power_profile_low_1(
        &self,
    ) -> &DcdcPowerProfileLowDcdcPowerProfileLow1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_dcdc_power_profile_medium_array0(
        &self,
    ) -> &DcdcPowerProfileMediumDcdcPowerProfileMediumArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_dcdc_power_profile_medium_0(
        &self,
    ) -> &DcdcPowerProfileMediumDcdcPowerProfileMedium0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_dcdc_power_profile_medium_array1(
        &self,
    ) -> &DcdcPowerProfileMediumDcdcPowerProfileMediumArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_dcdc_power_profile_medium_1(
        &self,
    ) -> &DcdcPowerProfileMediumDcdcPowerProfileMedium1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xf0 - no description available"]
    #[inline(always)]
    pub const fn bod(&self) -> &Bod {
        &self.bod
    }
    #[doc = "0xf4 - no description available"]
    #[inline(always)]
    pub const fn ldo_ao(&self) -> &LdoAo {
        &self.ldo_ao
    }
    #[doc = "0xf8 - no description available"]
    #[inline(always)]
    pub const fn sdio_delay(&self) -> &SdioDelay {
        &self.sdio_delay
    }
    #[doc = "0x100 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_array0(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbientArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_0(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbient0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x104 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_array1(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbientArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_1(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbient1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_array2(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbientArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_2(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbient2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_array3(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbientArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_aux_bias_curve_ambient_3(
        &self,
    ) -> &AuxBiasCurveAmbientAuxBiasCurveAmbient3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_array0(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTempArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_0(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTemp0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x114 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_array1(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTempArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_1(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTemp1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_array2(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTempArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_2(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTemp2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_array3(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTempArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_aux_bias_curve_temp_3(
        &self,
    ) -> &AuxBiasCurveTempAuxBiasCurveTemp3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn temp_sens_vbe1vbe8_ref_1(&self) -> &TempSensVbe1vbe8Ref1 {
        &self.temp_sens_vbe1vbe8_ref_1
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn temp_sens_vbe1vbe8_ref_2(&self) -> &TempSensVbe1vbe8Ref2 {
        &self.temp_sens_vbe1vbe8_ref_2
    }
    #[doc = "0x128 - no description available"]
    #[inline(always)]
    pub const fn temp_sens_slope(&self) -> &TempSensSlope {
        &self.temp_sens_slope
    }
    #[doc = "0x12c - no description available"]
    #[inline(always)]
    pub const fn temp_sens_offset(&self) -> &TempSensOffset {
        &self.temp_sens_offset
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_ringo(&self) -> &PvtMonitor0PvtMonitor0Ringo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_array0(&self) -> &PvtMonitor0PvtMonitor0Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_delays_lsb(&self) -> &PvtMonitor0PvtMonitor0DelaysLsb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_array1(&self) -> &PvtMonitor0PvtMonitor0Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_delays_msb(&self) -> &PvtMonitor0PvtMonitor0DelaysMsb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_pvt_monitor_0_array2(&self) -> &PvtMonitor0PvtMonitor0Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_ringo(&self) -> &PvtMonitor1PvtMonitor1Ringo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_array0(&self) -> &PvtMonitor1PvtMonitor1Array0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_delays_lsb(&self) -> &PvtMonitor1PvtMonitor1DelaysLsb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_array1(&self) -> &PvtMonitor1PvtMonitor1Array1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_delays_msb(&self) -> &PvtMonitor1PvtMonitor1DelaysMsb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_pvt_monitor_1_array2(&self) -> &PvtMonitor1PvtMonitor1Array2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c..0x180 - no description available"]
    #[inline(always)]
    pub const fn nxp_device_private_key(&self, n: usize) -> &NxpDevicePrivateKey {
        &self.nxp_device_private_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14c..0x180 - no description available"]
    #[inline(always)]
    pub fn nxp_device_private_key_iter(&self) -> impl Iterator<Item = &NxpDevicePrivateKey> {
        self.nxp_device_private_key.iter()
    }
    #[doc = "0x180..0x190 - NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_0(&self, n: usize) -> &NxpDeviceCertificate0 {
        &self.nxp_device_certificate_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x190 - NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
    #[inline(always)]
    pub fn nxp_device_certificate_0_iter(&self) -> impl Iterator<Item = &NxpDeviceCertificate0> {
        self.nxp_device_certificate_0.iter()
    }
    #[doc = "0x190..0x1a0 - NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_1(&self, n: usize) -> &NxpDeviceCertificate1 {
        &self.nxp_device_certificate_1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1a0 - NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
    #[inline(always)]
    pub fn nxp_device_certificate_1_iter(&self) -> impl Iterator<Item = &NxpDeviceCertificate1> {
        self.nxp_device_certificate_1.iter()
    }
    #[doc = "0x1a0..0x1b0 - NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_2(&self, n: usize) -> &NxpDeviceCertificate2 {
        &self.nxp_device_certificate_2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1b0 - NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
    #[inline(always)]
    pub fn nxp_device_certificate_2_iter(&self) -> impl Iterator<Item = &NxpDeviceCertificate2> {
        self.nxp_device_certificate_2.iter()
    }
    #[doc = "0x1b0..0x1c0 - NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_3(&self, n: usize) -> &NxpDeviceCertificate3 {
        &self.nxp_device_certificate_3[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1c0 - NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
    #[inline(always)]
    pub fn nxp_device_certificate_3_iter(&self) -> impl Iterator<Item = &NxpDeviceCertificate3> {
        self.nxp_device_certificate_3.iter()
    }
    #[doc = "0x1c0..0x1e0 - SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(&self, n: usize) -> &Sha256Digest {
        &self.sha256_digest[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e0 - SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub fn sha256_digest_iter(&self) -> impl Iterator<Item = &Sha256Digest> {
        self.sha256_digest.iter()
    }
    #[doc = "0x1e0 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_array0(&self) -> &EcidBackupEcidBackupArray0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_0(&self) -> &EcidBackupEcidBackup0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e4 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_array1(&self) -> &EcidBackupEcidBackupArray1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_1(&self) -> &EcidBackupEcidBackup1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e8 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_array2(&self) -> &EcidBackupEcidBackupArray2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_2(&self) -> &EcidBackupEcidBackup2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1ec - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_array3(&self) -> &EcidBackupEcidBackupArray3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub const fn ecid_backup_ecid_backup_3(&self) -> &EcidBackupEcidBackup3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1f0..0x200 - Checksum of the whole page"]
    #[inline(always)]
    pub const fn checksum(&self, n: usize) -> &Checksum {
        &self.checksum[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x200 - Checksum of the whole page"]
    #[inline(always)]
    pub fn checksum_iter(&self) -> impl Iterator<Item = &Checksum> {
        self.checksum.iter()
    }
    #[doc = "0xcac - no description available"]
    #[inline(always)]
    pub const fn dis_rom_hiding(&self) -> &DisRomHiding {
        &self.dis_rom_hiding
    }
    #[doc = "0xcbc - no description available"]
    #[inline(always)]
    pub const fn puf_sram(&self) -> &PufSram {
        &self.puf_sram
    }
}
#[doc = "GPO0_GPO0_0 (rw) register accessor: GPO0 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_0`] module"]
#[doc(alias = "GPO0_GPO0_0")]
pub type Gpo0Gpo0_0 = crate::Reg<gpo0_gpo0_0::Gpo0Gpo0_0Spec>;
#[doc = "GPO0 register 0 description"]
pub mod gpo0_gpo0_0;
#[doc = "GPO0_GPO0_ARRAY0 (rw) register accessor: GPO0 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_array0`] module"]
#[doc(alias = "GPO0_GPO0_ARRAY0")]
pub type Gpo0Gpo0Array0 = crate::Reg<gpo0_gpo0_array0::Gpo0Gpo0Array0Spec>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array0;
#[doc = "GPO0_GPO0_1 (rw) register accessor: GPO0 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_1`] module"]
#[doc(alias = "GPO0_GPO0_1")]
pub type Gpo0Gpo0_1 = crate::Reg<gpo0_gpo0_1::Gpo0Gpo0_1Spec>;
#[doc = "GPO0 register 1 description"]
pub mod gpo0_gpo0_1;
#[doc = "GPO0_GPO0_ARRAY1 (rw) register accessor: GPO0 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_array1`] module"]
#[doc(alias = "GPO0_GPO0_ARRAY1")]
pub type Gpo0Gpo0Array1 = crate::Reg<gpo0_gpo0_array1::Gpo0Gpo0Array1Spec>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array1;
#[doc = "GPO0_GPO0_2 (rw) register accessor: GPO0 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_2`] module"]
#[doc(alias = "GPO0_GPO0_2")]
pub type Gpo0Gpo0_2 = crate::Reg<gpo0_gpo0_2::Gpo0Gpo0_2Spec>;
#[doc = "GPO0 register 2 description"]
pub mod gpo0_gpo0_2;
#[doc = "GPO0_GPO0_ARRAY2 (rw) register accessor: GPO0 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_array2`] module"]
#[doc(alias = "GPO0_GPO0_ARRAY2")]
pub type Gpo0Gpo0Array2 = crate::Reg<gpo0_gpo0_array2::Gpo0Gpo0Array2Spec>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array2;
#[doc = "GPO0_GPO0_3 (rw) register accessor: GPO0 register 3 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_3`] module"]
#[doc(alias = "GPO0_GPO0_3")]
pub type Gpo0Gpo0_3 = crate::Reg<gpo0_gpo0_3::Gpo0Gpo0_3Spec>;
#[doc = "GPO0 register 3 description"]
pub mod gpo0_gpo0_3;
#[doc = "GPO0_GPO0_ARRAY3 (rw) register accessor: GPO0 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo0_gpo0_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo0_gpo0_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo0_gpo0_array3`] module"]
#[doc(alias = "GPO0_GPO0_ARRAY3")]
pub type Gpo0Gpo0Array3 = crate::Reg<gpo0_gpo0_array3::Gpo0Gpo0Array3Spec>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array3;
#[doc = "GPO1_GPO1_0 (rw) register accessor: GPO1 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_0`] module"]
#[doc(alias = "GPO1_GPO1_0")]
pub type Gpo1Gpo1_0 = crate::Reg<gpo1_gpo1_0::Gpo1Gpo1_0Spec>;
#[doc = "GPO1 register 0 description"]
pub mod gpo1_gpo1_0;
#[doc = "GPO1_GPO1_ARRAY0 (rw) register accessor: GPO1 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_array0`] module"]
#[doc(alias = "GPO1_GPO1_ARRAY0")]
pub type Gpo1Gpo1Array0 = crate::Reg<gpo1_gpo1_array0::Gpo1Gpo1Array0Spec>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array0;
#[doc = "GPO1_GPO1_1 (rw) register accessor: GPO1 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_1`] module"]
#[doc(alias = "GPO1_GPO1_1")]
pub type Gpo1Gpo1_1 = crate::Reg<gpo1_gpo1_1::Gpo1Gpo1_1Spec>;
#[doc = "GPO1 register 1 description"]
pub mod gpo1_gpo1_1;
#[doc = "GPO1_GPO1_ARRAY1 (rw) register accessor: GPO1 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_array1`] module"]
#[doc(alias = "GPO1_GPO1_ARRAY1")]
pub type Gpo1Gpo1Array1 = crate::Reg<gpo1_gpo1_array1::Gpo1Gpo1Array1Spec>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array1;
#[doc = "GPO1_GPO1_2 (rw) register accessor: GPO1 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_2`] module"]
#[doc(alias = "GPO1_GPO1_2")]
pub type Gpo1Gpo1_2 = crate::Reg<gpo1_gpo1_2::Gpo1Gpo1_2Spec>;
#[doc = "GPO1 register 2 description"]
pub mod gpo1_gpo1_2;
#[doc = "GPO1_GPO1_ARRAY2 (rw) register accessor: GPO1 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_array2`] module"]
#[doc(alias = "GPO1_GPO1_ARRAY2")]
pub type Gpo1Gpo1Array2 = crate::Reg<gpo1_gpo1_array2::Gpo1Gpo1Array2Spec>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array2;
#[doc = "GPO1_GPO1_3 (rw) register accessor: GPO1 register 3 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_3`] module"]
#[doc(alias = "GPO1_GPO1_3")]
pub type Gpo1Gpo1_3 = crate::Reg<gpo1_gpo1_3::Gpo1Gpo1_3Spec>;
#[doc = "GPO1 register 3 description"]
pub mod gpo1_gpo1_3;
#[doc = "GPO1_GPO1_ARRAY3 (rw) register accessor: GPO1 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo1_gpo1_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo1_gpo1_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo1_gpo1_array3`] module"]
#[doc(alias = "GPO1_GPO1_ARRAY3")]
pub type Gpo1Gpo1Array3 = crate::Reg<gpo1_gpo1_array3::Gpo1Gpo1Array3Spec>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array3;
#[doc = "GPO2_GPO2_0 (rw) register accessor: GPO2 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_0`] module"]
#[doc(alias = "GPO2_GPO2_0")]
pub type Gpo2Gpo2_0 = crate::Reg<gpo2_gpo2_0::Gpo2Gpo2_0Spec>;
#[doc = "GPO2 register 0 description"]
pub mod gpo2_gpo2_0;
#[doc = "GPO2_GPO2_ARRAY0 (rw) register accessor: GPO2 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_array0`] module"]
#[doc(alias = "GPO2_GPO2_ARRAY0")]
pub type Gpo2Gpo2Array0 = crate::Reg<gpo2_gpo2_array0::Gpo2Gpo2Array0Spec>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array0;
#[doc = "GPO2_GPO2_1 (rw) register accessor: GPO2 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_1`] module"]
#[doc(alias = "GPO2_GPO2_1")]
pub type Gpo2Gpo2_1 = crate::Reg<gpo2_gpo2_1::Gpo2Gpo2_1Spec>;
#[doc = "GPO2 register 1 description"]
pub mod gpo2_gpo2_1;
#[doc = "GPO2_GPO2_ARRAY1 (rw) register accessor: GPO2 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_array1`] module"]
#[doc(alias = "GPO2_GPO2_ARRAY1")]
pub type Gpo2Gpo2Array1 = crate::Reg<gpo2_gpo2_array1::Gpo2Gpo2Array1Spec>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array1;
#[doc = "GPO2_GPO2_2 (rw) register accessor: GPO2 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_2`] module"]
#[doc(alias = "GPO2_GPO2_2")]
pub type Gpo2Gpo2_2 = crate::Reg<gpo2_gpo2_2::Gpo2Gpo2_2Spec>;
#[doc = "GPO2 register 2 description"]
pub mod gpo2_gpo2_2;
#[doc = "GPO2_GPO2_ARRAY2 (rw) register accessor: GPO2 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_array2`] module"]
#[doc(alias = "GPO2_GPO2_ARRAY2")]
pub type Gpo2Gpo2Array2 = crate::Reg<gpo2_gpo2_array2::Gpo2Gpo2Array2Spec>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array2;
#[doc = "GPO2_GPO2_3 (rw) register accessor: GPO2 register 3 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_3`] module"]
#[doc(alias = "GPO2_GPO2_3")]
pub type Gpo2Gpo2_3 = crate::Reg<gpo2_gpo2_3::Gpo2Gpo2_3Spec>;
#[doc = "GPO2 register 3 description"]
pub mod gpo2_gpo2_3;
#[doc = "GPO2_GPO2_ARRAY3 (rw) register accessor: GPO2 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo2_gpo2_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo2_gpo2_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo2_gpo2_array3`] module"]
#[doc(alias = "GPO2_GPO2_ARRAY3")]
pub type Gpo2Gpo2Array3 = crate::Reg<gpo2_gpo2_array3::Gpo2Gpo2Array3Spec>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array3;
#[doc = "GPO3_GPO3_0 (rw) register accessor: GPO3 register 0 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_0`] module"]
#[doc(alias = "GPO3_GPO3_0")]
pub type Gpo3Gpo3_0 = crate::Reg<gpo3_gpo3_0::Gpo3Gpo3_0Spec>;
#[doc = "GPO3 register 0 description"]
pub mod gpo3_gpo3_0;
#[doc = "GPO3_GPO3_ARRAY0 (rw) register accessor: GPO3 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_array0`] module"]
#[doc(alias = "GPO3_GPO3_ARRAY0")]
pub type Gpo3Gpo3Array0 = crate::Reg<gpo3_gpo3_array0::Gpo3Gpo3Array0Spec>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array0;
#[doc = "GPO3_GPO3_1 (rw) register accessor: GPO3 register 1 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_1`] module"]
#[doc(alias = "GPO3_GPO3_1")]
pub type Gpo3Gpo3_1 = crate::Reg<gpo3_gpo3_1::Gpo3Gpo3_1Spec>;
#[doc = "GPO3 register 1 description"]
pub mod gpo3_gpo3_1;
#[doc = "GPO3_GPO3_ARRAY1 (rw) register accessor: GPO3 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_array1`] module"]
#[doc(alias = "GPO3_GPO3_ARRAY1")]
pub type Gpo3Gpo3Array1 = crate::Reg<gpo3_gpo3_array1::Gpo3Gpo3Array1Spec>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array1;
#[doc = "GPO3_GPO3_2 (rw) register accessor: GPO3 register 2 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_2`] module"]
#[doc(alias = "GPO3_GPO3_2")]
pub type Gpo3Gpo3_2 = crate::Reg<gpo3_gpo3_2::Gpo3Gpo3_2Spec>;
#[doc = "GPO3 register 2 description"]
pub mod gpo3_gpo3_2;
#[doc = "GPO3_GPO3_ARRAY2 (rw) register accessor: GPO3 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_array2`] module"]
#[doc(alias = "GPO3_GPO3_ARRAY2")]
pub type Gpo3Gpo3Array2 = crate::Reg<gpo3_gpo3_array2::Gpo3Gpo3Array2Spec>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array2;
#[doc = "GPO3_GPO3_3 (rw) register accessor: GPO3 register 3 description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_3`] module"]
#[doc(alias = "GPO3_GPO3_3")]
pub type Gpo3Gpo3_3 = crate::Reg<gpo3_gpo3_3::Gpo3Gpo3_3Spec>;
#[doc = "GPO3 register 3 description"]
pub mod gpo3_gpo3_3;
#[doc = "GPO3_GPO3_ARRAY3 (rw) register accessor: GPO3 array description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo3_gpo3_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo3_gpo3_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo3_gpo3_array3`] module"]
#[doc(alias = "GPO3_GPO3_ARRAY3")]
pub type Gpo3Gpo3Array3 = crate::Reg<gpo3_gpo3_array3::Gpo3Gpo3Array3Spec>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array3;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_0 (rw) register accessor: checksum of the GPO data in words 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_0`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_0")]
pub type GpoChecksumGpoChecksum0 =
    crate::Reg<gpo_checksum_gpo_checksum_0::GpoChecksumGpoChecksum0Spec>;
#[doc = "checksum of the GPO data in words 0"]
pub mod gpo_checksum_gpo_checksum_0;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0 (rw) register accessor: checksum of the GPO data in words \\[3:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_array0`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0")]
pub type GpoChecksumGpoChecksumArray0 =
    crate::Reg<gpo_checksum_gpo_checksum_array0::GpoChecksumGpoChecksumArray0Spec>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array0;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_1 (rw) register accessor: checksum of the GPO data in words 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_1`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_1")]
pub type GpoChecksumGpoChecksum1 =
    crate::Reg<gpo_checksum_gpo_checksum_1::GpoChecksumGpoChecksum1Spec>;
#[doc = "checksum of the GPO data in words 1"]
pub mod gpo_checksum_gpo_checksum_1;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1 (rw) register accessor: checksum of the GPO data in words \\[3:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_array1`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1")]
pub type GpoChecksumGpoChecksumArray1 =
    crate::Reg<gpo_checksum_gpo_checksum_array1::GpoChecksumGpoChecksumArray1Spec>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array1;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_2 (rw) register accessor: checksum of the GPO data in words 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_2`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_2")]
pub type GpoChecksumGpoChecksum2 =
    crate::Reg<gpo_checksum_gpo_checksum_2::GpoChecksumGpoChecksum2Spec>;
#[doc = "checksum of the GPO data in words 2"]
pub mod gpo_checksum_gpo_checksum_2;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2 (rw) register accessor: checksum of the GPO data in words \\[3:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_array2`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2")]
pub type GpoChecksumGpoChecksumArray2 =
    crate::Reg<gpo_checksum_gpo_checksum_array2::GpoChecksumGpoChecksumArray2Spec>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array2;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_3 (rw) register accessor: checksum of the GPO data in words 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_3`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_3")]
pub type GpoChecksumGpoChecksum3 =
    crate::Reg<gpo_checksum_gpo_checksum_3::GpoChecksumGpoChecksum3Spec>;
#[doc = "checksum of the GPO data in words 3"]
pub mod gpo_checksum_gpo_checksum_3;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3 (rw) register accessor: checksum of the GPO data in words \\[3:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gpo_checksum_gpo_checksum_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpo_checksum_gpo_checksum_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_checksum_gpo_checksum_array3`] module"]
#[doc(alias = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3")]
pub type GpoChecksumGpoChecksumArray3 =
    crate::Reg<gpo_checksum_gpo_checksum_array3::GpoChecksumGpoChecksumArray3Spec>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array3;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_0`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0")]
pub type FinalTestBatchIdFinalTestBatchId0 =
    crate::Reg<final_test_batch_id_final_test_batch_id_0::FinalTestBatchIdFinalTestBatchId0Spec>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_0;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_array0`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0")]
pub type FinalTestBatchIdFinalTestBatchIdArray0 = crate::Reg<
    final_test_batch_id_final_test_batch_id_array0::FinalTestBatchIdFinalTestBatchIdArray0Spec,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array0;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_1`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1")]
pub type FinalTestBatchIdFinalTestBatchId1 =
    crate::Reg<final_test_batch_id_final_test_batch_id_1::FinalTestBatchIdFinalTestBatchId1Spec>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_1;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_array1`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1")]
pub type FinalTestBatchIdFinalTestBatchIdArray1 = crate::Reg<
    final_test_batch_id_final_test_batch_id_array1::FinalTestBatchIdFinalTestBatchIdArray1Spec,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array1;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_2`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2")]
pub type FinalTestBatchIdFinalTestBatchId2 =
    crate::Reg<final_test_batch_id_final_test_batch_id_2::FinalTestBatchIdFinalTestBatchId2Spec>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_2;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_array2`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2")]
pub type FinalTestBatchIdFinalTestBatchIdArray2 = crate::Reg<
    final_test_batch_id_final_test_batch_id_array2::FinalTestBatchIdFinalTestBatchIdArray2Spec,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array2;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_3`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3")]
pub type FinalTestBatchIdFinalTestBatchId3 =
    crate::Reg<final_test_batch_id_final_test_batch_id_3::FinalTestBatchIdFinalTestBatchId3Spec>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_3;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_batch_id_final_test_batch_id_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_batch_id_final_test_batch_id_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_batch_id_final_test_batch_id_array3`] module"]
#[doc(alias = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3")]
pub type FinalTestBatchIdFinalTestBatchIdArray3 = crate::Reg<
    final_test_batch_id_final_test_batch_id_array3::FinalTestBatchIdFinalTestBatchIdArray3Spec,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array3;
#[doc = "DEVICE_TYPE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`device_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_type`] module"]
#[doc(alias = "DEVICE_TYPE")]
pub type DeviceType = crate::Reg<device_type::DeviceTypeSpec>;
#[doc = "no description available"]
pub mod device_type;
#[doc = "FINAL_TEST_PROGRAM_VERSION (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_program_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_program_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_program_version`] module"]
#[doc(alias = "FINAL_TEST_PROGRAM_VERSION")]
pub type FinalTestProgramVersion =
    crate::Reg<final_test_program_version::FinalTestProgramVersionSpec>;
#[doc = "no description available"]
pub mod final_test_program_version;
#[doc = "FINAL_TEST_DATE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_date`] module"]
#[doc(alias = "FINAL_TEST_DATE")]
pub type FinalTestDate = crate::Reg<final_test_date::FinalTestDateSpec>;
#[doc = "no description available"]
pub mod final_test_date;
#[doc = "FINAL_TEST_TIME (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`final_test_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`final_test_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@final_test_time`] module"]
#[doc(alias = "FINAL_TEST_TIME")]
pub type FinalTestTime = crate::Reg<final_test_time::FinalTestTimeSpec>;
#[doc = "no description available"]
pub mod final_test_time;
#[doc = "UUID_UUID_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_0`] module"]
#[doc(alias = "UUID_UUID_0")]
pub type UuidUuid0 = crate::Reg<uuid_uuid_0::UuidUuid0Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_0;
#[doc = "UUID_UUID_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_array0`] module"]
#[doc(alias = "UUID_UUID_ARRAY0")]
pub type UuidUuidArray0 = crate::Reg<uuid_uuid_array0::UuidUuidArray0Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_array0;
#[doc = "UUID_UUID_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_1`] module"]
#[doc(alias = "UUID_UUID_1")]
pub type UuidUuid1 = crate::Reg<uuid_uuid_1::UuidUuid1Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_1;
#[doc = "UUID_UUID_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_array1`] module"]
#[doc(alias = "UUID_UUID_ARRAY1")]
pub type UuidUuidArray1 = crate::Reg<uuid_uuid_array1::UuidUuidArray1Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_array1;
#[doc = "UUID_UUID_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_2`] module"]
#[doc(alias = "UUID_UUID_2")]
pub type UuidUuid2 = crate::Reg<uuid_uuid_2::UuidUuid2Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_2;
#[doc = "UUID_UUID_ARRAY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_array2`] module"]
#[doc(alias = "UUID_UUID_ARRAY2")]
pub type UuidUuidArray2 = crate::Reg<uuid_uuid_array2::UuidUuidArray2Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_array2;
#[doc = "UUID_UUID_3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_3`] module"]
#[doc(alias = "UUID_UUID_3")]
pub type UuidUuid3 = crate::Reg<uuid_uuid_3::UuidUuid3Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_3;
#[doc = "UUID_UUID_ARRAY3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid_uuid_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid_uuid_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid_uuid_array3`] module"]
#[doc(alias = "UUID_UUID_ARRAY3")]
pub type UuidUuidArray3 = crate::Reg<uuid_uuid_array3::UuidUuidArray3Spec>;
#[doc = "no description available"]
pub mod uuid_uuid_array3;
#[doc = "WAFER_TEST1_PROGRAM_VERSION (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_program_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_program_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test1_program_version`] module"]
#[doc(alias = "WAFER_TEST1_PROGRAM_VERSION")]
pub type WaferTest1ProgramVersion =
    crate::Reg<wafer_test1_program_version::WaferTest1ProgramVersionSpec>;
#[doc = "no description available"]
pub mod wafer_test1_program_version;
#[doc = "WAFER_TEST1_DATE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test1_date`] module"]
#[doc(alias = "WAFER_TEST1_DATE")]
pub type WaferTest1Date = crate::Reg<wafer_test1_date::WaferTest1DateSpec>;
#[doc = "no description available"]
pub mod wafer_test1_date;
#[doc = "WAFER_TEST1_TIME (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test1_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test1_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test1_time`] module"]
#[doc(alias = "WAFER_TEST1_TIME")]
pub type WaferTest1Time = crate::Reg<wafer_test1_time::WaferTest1TimeSpec>;
#[doc = "no description available"]
pub mod wafer_test1_time;
#[doc = "WAFER_TEST2_PROGRAM_VERSION (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_program_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_program_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test2_program_version`] module"]
#[doc(alias = "WAFER_TEST2_PROGRAM_VERSION")]
pub type WaferTest2ProgramVersion =
    crate::Reg<wafer_test2_program_version::WaferTest2ProgramVersionSpec>;
#[doc = "no description available"]
pub mod wafer_test2_program_version;
#[doc = "WAFER_TEST2_DATE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test2_date`] module"]
#[doc(alias = "WAFER_TEST2_DATE")]
pub type WaferTest2Date = crate::Reg<wafer_test2_date::WaferTest2DateSpec>;
#[doc = "no description available"]
pub mod wafer_test2_date;
#[doc = "WAFER_TEST2_TIME (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_test2_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wafer_test2_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_test2_time`] module"]
#[doc(alias = "WAFER_TEST2_TIME")]
pub type WaferTest2Time = crate::Reg<wafer_test2_time::WaferTest2TimeSpec>;
#[doc = "no description available"]
pub mod wafer_test2_time;
#[doc = "USBCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcfg`] module"]
#[doc(alias = "USBCFG")]
pub type Usbcfg = crate::Reg<usbcfg::UsbcfgSpec>;
#[doc = "no description available"]
pub mod usbcfg;
#[doc = "PERIPHENCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`periphencfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periphencfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphencfg`] module"]
#[doc(alias = "PERIPHENCFG")]
pub type Periphencfg = crate::Reg<periphencfg::PeriphencfgSpec>;
#[doc = "no description available"]
pub mod periphencfg;
#[doc = "RAMSIZECFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ramsizecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramsizecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramsizecfg`] module"]
#[doc(alias = "RAMSIZECFG")]
pub type Ramsizecfg = crate::Reg<ramsizecfg::RamsizecfgSpec>;
#[doc = "no description available"]
pub mod ramsizecfg;
#[doc = "FLASHSIZECFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`flashsizecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashsizecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashsizecfg`] module"]
#[doc(alias = "FLASHSIZECFG")]
pub type Flashsizecfg = crate::Reg<flashsizecfg::FlashsizecfgSpec>;
#[doc = "no description available"]
pub mod flashsizecfg;
#[doc = "RINGO_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo_0`] module"]
#[doc(alias = "RINGO_0")]
pub type Ringo0 = crate::Reg<ringo_0::Ringo0Spec>;
#[doc = "no description available"]
pub mod ringo_0;
#[doc = "RINGO_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo_1`] module"]
#[doc(alias = "RINGO_1")]
pub type Ringo1 = crate::Reg<ringo_1::Ringo1Spec>;
#[doc = "no description available"]
pub mod ringo_1;
#[doc = "RINGO_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ringo_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ringo_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringo_2`] module"]
#[doc(alias = "RINGO_2")]
pub type Ringo2 = crate::Reg<ringo_2::Ringo2Spec>;
#[doc = "no description available"]
pub mod ringo_2;
#[doc = "FRO_192MHZ (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`fro_192mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro_192mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro_192mhz`] module"]
#[doc(alias = "FRO_192MHZ")]
pub type Fro192mhz = crate::Reg<fro_192mhz::Fro192mhzSpec>;
#[doc = "no description available"]
pub mod fro_192mhz;
#[doc = "XO_32MHZ (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`xo_32mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo_32mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xo_32mhz`] module"]
#[doc(alias = "XO_32MHZ")]
pub type Xo32mhz = crate::Reg<xo_32mhz::Xo32mhzSpec>;
#[doc = "no description available"]
pub mod xo_32mhz;
#[doc = "XO_32KHZ (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`xo_32khz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo_32khz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xo_32khz`] module"]
#[doc(alias = "XO_32KHZ")]
pub type Xo32khz = crate::Reg<xo_32khz::Xo32khzSpec>;
#[doc = "no description available"]
pub mod xo_32khz;
#[doc = "FRO_1MHZ (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`fro_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro_1mhz`] module"]
#[doc(alias = "FRO_1MHZ")]
pub type Fro1mhz = crate::Reg<fro_1mhz::Fro1mhzSpec>;
#[doc = "no description available"]
pub mod fro_1mhz;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_high_dcdc_power_profile_high_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_high_dcdc_power_profile_high_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_high_dcdc_power_profile_high_0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0")]
pub type DcdcPowerProfileHighDcdcPowerProfileHigh0 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_0 :: DcdcPowerProfileHighDcdcPowerProfileHigh0Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_0;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_high_dcdc_power_profile_high_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_high_dcdc_power_profile_high_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_high_dcdc_power_profile_high_array0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0")]
pub type DcdcPowerProfileHighDcdcPowerProfileHighArray0 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_array0 :: DcdcPowerProfileHighDcdcPowerProfileHighArray0Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_array0;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_high_dcdc_power_profile_high_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_high_dcdc_power_profile_high_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_high_dcdc_power_profile_high_1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1")]
pub type DcdcPowerProfileHighDcdcPowerProfileHigh1 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_1 :: DcdcPowerProfileHighDcdcPowerProfileHigh1Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_1;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_high_dcdc_power_profile_high_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_high_dcdc_power_profile_high_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_high_dcdc_power_profile_high_array1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1")]
pub type DcdcPowerProfileHighDcdcPowerProfileHighArray1 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_array1 :: DcdcPowerProfileHighDcdcPowerProfileHighArray1Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_array1;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_low_dcdc_power_profile_low_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_low_dcdc_power_profile_low_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_low_dcdc_power_profile_low_0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0")]
pub type DcdcPowerProfileLowDcdcPowerProfileLow0 = crate::Reg<
    dcdc_power_profile_low_dcdc_power_profile_low_0::DcdcPowerProfileLowDcdcPowerProfileLow0Spec,
>;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_0;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_low_dcdc_power_profile_low_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_low_dcdc_power_profile_low_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_low_dcdc_power_profile_low_array0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0")]
pub type DcdcPowerProfileLowDcdcPowerProfileLowArray0 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_array0 :: DcdcPowerProfileLowDcdcPowerProfileLowArray0Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_array0;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_low_dcdc_power_profile_low_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_low_dcdc_power_profile_low_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_low_dcdc_power_profile_low_1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1")]
pub type DcdcPowerProfileLowDcdcPowerProfileLow1 = crate::Reg<
    dcdc_power_profile_low_dcdc_power_profile_low_1::DcdcPowerProfileLowDcdcPowerProfileLow1Spec,
>;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_1;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_low_dcdc_power_profile_low_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_low_dcdc_power_profile_low_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_low_dcdc_power_profile_low_array1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1")]
pub type DcdcPowerProfileLowDcdcPowerProfileLowArray1 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_array1 :: DcdcPowerProfileLowDcdcPowerProfileLowArray1Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_array1;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_medium_dcdc_power_profile_medium_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_medium_dcdc_power_profile_medium_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_medium_dcdc_power_profile_medium_0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0")]
pub type DcdcPowerProfileMediumDcdcPowerProfileMedium0 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_0 :: DcdcPowerProfileMediumDcdcPowerProfileMedium0Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_0;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_medium_dcdc_power_profile_medium_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_medium_dcdc_power_profile_medium_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_medium_dcdc_power_profile_medium_array0`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0")]
pub type DcdcPowerProfileMediumDcdcPowerProfileMediumArray0 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_array0 :: DcdcPowerProfileMediumDcdcPowerProfileMediumArray0Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_array0;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_medium_dcdc_power_profile_medium_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_medium_dcdc_power_profile_medium_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_medium_dcdc_power_profile_medium_1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1")]
pub type DcdcPowerProfileMediumDcdcPowerProfileMedium1 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_1 :: DcdcPowerProfileMediumDcdcPowerProfileMedium1Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_1;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_power_profile_medium_dcdc_power_profile_medium_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_power_profile_medium_dcdc_power_profile_medium_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_power_profile_medium_dcdc_power_profile_medium_array1`] module"]
#[doc(alias = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1")]
pub type DcdcPowerProfileMediumDcdcPowerProfileMediumArray1 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_array1 :: DcdcPowerProfileMediumDcdcPowerProfileMediumArray1Spec > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_array1;
#[doc = "BOD (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`bod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod`] module"]
#[doc(alias = "BOD")]
pub type Bod = crate::Reg<bod::BodSpec>;
#[doc = "no description available"]
pub mod bod;
#[doc = "LDO_AO (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo_ao::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_ao::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo_ao`] module"]
#[doc(alias = "LDO_AO")]
pub type LdoAo = crate::Reg<ldo_ao::LdoAoSpec>;
#[doc = "no description available"]
pub mod ldo_ao;
#[doc = "SDIO_DELAY (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_delay`] module"]
#[doc(alias = "SDIO_DELAY")]
pub type SdioDelay = crate::Reg<sdio_delay::SdioDelaySpec>;
#[doc = "no description available"]
pub mod sdio_delay;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_0`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbient0 = crate::Reg<
    aux_bias_curve_ambient_aux_bias_curve_ambient_0::AuxBiasCurveAmbientAuxBiasCurveAmbient0Spec,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_0;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0 (rw) register accessor: Aux Bias Curve Ambient (30degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_array0`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbientArray0 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array0 :: AuxBiasCurveAmbientAuxBiasCurveAmbientArray0Spec > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array0;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_1`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbient1 = crate::Reg<
    aux_bias_curve_ambient_aux_bias_curve_ambient_1::AuxBiasCurveAmbientAuxBiasCurveAmbient1Spec,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_1;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1 (rw) register accessor: Aux Bias Curve Ambient (30degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_array1`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbientArray1 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array1 :: AuxBiasCurveAmbientAuxBiasCurveAmbientArray1Spec > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array1;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_2`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbient2 = crate::Reg<
    aux_bias_curve_ambient_aux_bias_curve_ambient_2::AuxBiasCurveAmbientAuxBiasCurveAmbient2Spec,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_2;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2 (rw) register accessor: Aux Bias Curve Ambient (30degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_array2`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbientArray2 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array2 :: AuxBiasCurveAmbientAuxBiasCurveAmbientArray2Spec > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array2;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_3`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbient3 = crate::Reg<
    aux_bias_curve_ambient_aux_bias_curve_ambient_3::AuxBiasCurveAmbientAuxBiasCurveAmbient3Spec,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_3;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3 (rw) register accessor: Aux Bias Curve Ambient (30degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_ambient_aux_bias_curve_ambient_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_ambient_aux_bias_curve_ambient_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_ambient_aux_bias_curve_ambient_array3`] module"]
#[doc(alias = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3")]
pub type AuxBiasCurveAmbientAuxBiasCurveAmbientArray3 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array3 :: AuxBiasCurveAmbientAuxBiasCurveAmbientArray3Spec > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array3;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_0`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0")]
pub type AuxBiasCurveTempAuxBiasCurveTemp0 =
    crate::Reg<aux_bias_curve_temp_aux_bias_curve_temp_0::AuxBiasCurveTempAuxBiasCurveTemp0Spec>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_0;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0 (rw) register accessor: Aux Bias Curve TEMP (105degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_array0`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0")]
pub type AuxBiasCurveTempAuxBiasCurveTempArray0 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_array0::AuxBiasCurveTempAuxBiasCurveTempArray0Spec,
>;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array0;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_1`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1")]
pub type AuxBiasCurveTempAuxBiasCurveTemp1 =
    crate::Reg<aux_bias_curve_temp_aux_bias_curve_temp_1::AuxBiasCurveTempAuxBiasCurveTemp1Spec>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_1;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1 (rw) register accessor: Aux Bias Curve TEMP (105degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_array1`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1")]
pub type AuxBiasCurveTempAuxBiasCurveTempArray1 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_array1::AuxBiasCurveTempAuxBiasCurveTempArray1Spec,
>;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array1;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_2`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2")]
pub type AuxBiasCurveTempAuxBiasCurveTemp2 =
    crate::Reg<aux_bias_curve_temp_aux_bias_curve_temp_2::AuxBiasCurveTempAuxBiasCurveTemp2Spec>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_2;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2 (rw) register accessor: Aux Bias Curve TEMP (105degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_array2`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2")]
pub type AuxBiasCurveTempAuxBiasCurveTempArray2 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_array2::AuxBiasCurveTempAuxBiasCurveTempArray2Spec,
>;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array2;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_3`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3")]
pub type AuxBiasCurveTempAuxBiasCurveTemp3 =
    crate::Reg<aux_bias_curve_temp_aux_bias_curve_temp_3::AuxBiasCurveTempAuxBiasCurveTemp3Spec>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_3;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3 (rw) register accessor: Aux Bias Curve TEMP (105degC)\n\nYou can [`read`](crate::Reg::read) this register and get [`aux_bias_curve_temp_aux_bias_curve_temp_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux_bias_curve_temp_aux_bias_curve_temp_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux_bias_curve_temp_aux_bias_curve_temp_array3`] module"]
#[doc(alias = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3")]
pub type AuxBiasCurveTempAuxBiasCurveTempArray3 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_array3::AuxBiasCurveTempAuxBiasCurveTempArray3Spec,
>;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array3;
#[doc = "TEMP_SENS_VBE1VBE8_REF_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_vbe1vbe8_ref_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_vbe1vbe8_ref_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sens_vbe1vbe8_ref_1`] module"]
#[doc(alias = "TEMP_SENS_VBE1VBE8_REF_1")]
pub type TempSensVbe1vbe8Ref1 = crate::Reg<temp_sens_vbe1vbe8_ref_1::TempSensVbe1vbe8Ref1Spec>;
#[doc = "no description available"]
pub mod temp_sens_vbe1vbe8_ref_1;
#[doc = "TEMP_SENS_VBE1VBE8_REF_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_vbe1vbe8_ref_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_vbe1vbe8_ref_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sens_vbe1vbe8_ref_2`] module"]
#[doc(alias = "TEMP_SENS_VBE1VBE8_REF_2")]
pub type TempSensVbe1vbe8Ref2 = crate::Reg<temp_sens_vbe1vbe8_ref_2::TempSensVbe1vbe8Ref2Spec>;
#[doc = "no description available"]
pub mod temp_sens_vbe1vbe8_ref_2;
#[doc = "TEMP_SENS_SLOPE (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_slope::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_slope::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sens_slope`] module"]
#[doc(alias = "TEMP_SENS_SLOPE")]
pub type TempSensSlope = crate::Reg<temp_sens_slope::TempSensSlopeSpec>;
#[doc = "no description available"]
pub mod temp_sens_slope;
#[doc = "TEMP_SENS_OFFSET (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_sens_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_sens_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sens_offset`] module"]
#[doc(alias = "TEMP_SENS_OFFSET")]
pub type TempSensOffset = crate::Reg<temp_sens_offset::TempSensOffsetSpec>;
#[doc = "no description available"]
pub mod temp_sens_offset;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_array0`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0")]
pub type PvtMonitor0PvtMonitor0Array0 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array0::PvtMonitor0PvtMonitor0Array0Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array0;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_RINGO (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_ringo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_ringo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_ringo`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_RINGO")]
pub type PvtMonitor0PvtMonitor0Ringo =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_ringo::PvtMonitor0PvtMonitor0RingoSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_ringo;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_array1`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1")]
pub type PvtMonitor0PvtMonitor0Array1 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array1::PvtMonitor0PvtMonitor0Array1Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array1;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_delays_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_delays_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_delays_lsb`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB")]
pub type PvtMonitor0PvtMonitor0DelaysLsb =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_delays_lsb::PvtMonitor0PvtMonitor0DelaysLsbSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_delays_lsb;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_array2`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2")]
pub type PvtMonitor0PvtMonitor0Array2 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array2::PvtMonitor0PvtMonitor0Array2Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array2;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_0_pvt_monitor_0_delays_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_0_pvt_monitor_0_delays_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_0_pvt_monitor_0_delays_msb`] module"]
#[doc(alias = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB")]
pub type PvtMonitor0PvtMonitor0DelaysMsb =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_delays_msb::PvtMonitor0PvtMonitor0DelaysMsbSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_delays_msb;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_array0`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0")]
pub type PvtMonitor1PvtMonitor1Array0 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array0::PvtMonitor1PvtMonitor1Array0Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array0;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_RINGO (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_ringo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_ringo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_ringo`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_RINGO")]
pub type PvtMonitor1PvtMonitor1Ringo =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_ringo::PvtMonitor1PvtMonitor1RingoSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_ringo;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_array1`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1")]
pub type PvtMonitor1PvtMonitor1Array1 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array1::PvtMonitor1PvtMonitor1Array1Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array1;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_delays_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_delays_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_delays_lsb`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB")]
pub type PvtMonitor1PvtMonitor1DelaysLsb =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_delays_lsb::PvtMonitor1PvtMonitor1DelaysLsbSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_delays_lsb;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_array2`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2")]
pub type PvtMonitor1PvtMonitor1Array2 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array2::PvtMonitor1PvtMonitor1Array2Spec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array2;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_1_pvt_monitor_1_delays_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_1_pvt_monitor_1_delays_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_1_pvt_monitor_1_delays_msb`] module"]
#[doc(alias = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB")]
pub type PvtMonitor1PvtMonitor1DelaysMsb =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_delays_msb::PvtMonitor1PvtMonitor1DelaysMsbSpec>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_delays_msb;
#[doc = "NXP_DEVICE_PRIVATE_KEY (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_private_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_private_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxp_device_private_key`] module"]
#[doc(alias = "NXP_DEVICE_PRIVATE_KEY")]
pub type NxpDevicePrivateKey = crate::Reg<nxp_device_private_key::NxpDevicePrivateKeySpec>;
#[doc = "no description available"]
pub mod nxp_device_private_key;
#[doc = "NXP_DEVICE_CERTIFICATE_0 (rw) register accessor: NXP Device Certificate (ECDSA_sign - r\\[255:128\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_certificate_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_certificate_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxp_device_certificate_0`] module"]
#[doc(alias = "NXP_DEVICE_CERTIFICATE_0")]
pub type NxpDeviceCertificate0 = crate::Reg<nxp_device_certificate_0::NxpDeviceCertificate0Spec>;
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
pub mod nxp_device_certificate_0;
#[doc = "NXP_DEVICE_CERTIFICATE_1 (rw) register accessor: NXP Device Certificate (ECDSA_sign - r\\[127:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_certificate_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_certificate_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxp_device_certificate_1`] module"]
#[doc(alias = "NXP_DEVICE_CERTIFICATE_1")]
pub type NxpDeviceCertificate1 = crate::Reg<nxp_device_certificate_1::NxpDeviceCertificate1Spec>;
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
pub mod nxp_device_certificate_1;
#[doc = "NXP_DEVICE_CERTIFICATE_2 (rw) register accessor: NXP Device Certificate (ECDSA_sign - s\\[255:128\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_certificate_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_certificate_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxp_device_certificate_2`] module"]
#[doc(alias = "NXP_DEVICE_CERTIFICATE_2")]
pub type NxpDeviceCertificate2 = crate::Reg<nxp_device_certificate_2::NxpDeviceCertificate2Spec>;
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
pub mod nxp_device_certificate_2;
#[doc = "NXP_DEVICE_CERTIFICATE_3 (rw) register accessor: NXP Device Certificate (ECDSA_sign - s\\[127:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`nxp_device_certificate_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nxp_device_certificate_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxp_device_certificate_3`] module"]
#[doc(alias = "NXP_DEVICE_CERTIFICATE_3")]
pub type NxpDeviceCertificate3 = crate::Reg<nxp_device_certificate_3::NxpDeviceCertificate3Spec>;
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
pub mod nxp_device_certificate_3;
#[doc = "SHA256_DIGEST (rw) register accessor: SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha256_digest`] module"]
#[doc(alias = "SHA256_DIGEST")]
pub type Sha256Digest = crate::Reg<sha256_digest::Sha256DigestSpec>;
#[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
pub mod sha256_digest;
#[doc = "ECID_BACKUP_ECID_BACKUP_0 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_0`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_0")]
pub type EcidBackupEcidBackup0 = crate::Reg<ecid_backup_ecid_backup_0::EcidBackupEcidBackup0Spec>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_0;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY0 (rw) register accessor: ECID backup (the original is in page n-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_array0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_array0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_array0`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_ARRAY0")]
pub type EcidBackupEcidBackupArray0 =
    crate::Reg<ecid_backup_ecid_backup_array0::EcidBackupEcidBackupArray0Spec>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array0;
#[doc = "ECID_BACKUP_ECID_BACKUP_1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_1`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_1")]
pub type EcidBackupEcidBackup1 = crate::Reg<ecid_backup_ecid_backup_1::EcidBackupEcidBackup1Spec>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_1;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY1 (rw) register accessor: ECID backup (the original is in page n-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_array1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_array1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_array1`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_ARRAY1")]
pub type EcidBackupEcidBackupArray1 =
    crate::Reg<ecid_backup_ecid_backup_array1::EcidBackupEcidBackupArray1Spec>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array1;
#[doc = "ECID_BACKUP_ECID_BACKUP_2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_2`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_2")]
pub type EcidBackupEcidBackup2 = crate::Reg<ecid_backup_ecid_backup_2::EcidBackupEcidBackup2Spec>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_2;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY2 (rw) register accessor: ECID backup (the original is in page n-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_array2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_array2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_array2`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_ARRAY2")]
pub type EcidBackupEcidBackupArray2 =
    crate::Reg<ecid_backup_ecid_backup_array2::EcidBackupEcidBackupArray2Spec>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array2;
#[doc = "ECID_BACKUP_ECID_BACKUP_3 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_3`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_3")]
pub type EcidBackupEcidBackup3 = crate::Reg<ecid_backup_ecid_backup_3::EcidBackupEcidBackup3Spec>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_3;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY3 (rw) register accessor: ECID backup (the original is in page n-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ecid_backup_ecid_backup_array3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecid_backup_ecid_backup_array3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecid_backup_ecid_backup_array3`] module"]
#[doc(alias = "ECID_BACKUP_ECID_BACKUP_ARRAY3")]
pub type EcidBackupEcidBackupArray3 =
    crate::Reg<ecid_backup_ecid_backup_array3::EcidBackupEcidBackupArray3Spec>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array3;
#[doc = "CHECKSUM (rw) register accessor: Checksum of the whole page\n\nYou can [`read`](crate::Reg::read) this register and get [`checksum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`checksum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@checksum`] module"]
#[doc(alias = "CHECKSUM")]
pub type Checksum = crate::Reg<checksum::ChecksumSpec>;
#[doc = "Checksum of the whole page"]
pub mod checksum;
#[doc = "DIS_ROM_HIDING (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`dis_rom_hiding::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis_rom_hiding::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis_rom_hiding`] module"]
#[doc(alias = "DIS_ROM_HIDING")]
pub type DisRomHiding = crate::Reg<dis_rom_hiding::DisRomHidingSpec>;
#[doc = "no description available"]
pub mod dis_rom_hiding;
#[doc = "PUF_SRAM (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_sram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_sram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@puf_sram`] module"]
#[doc(alias = "PUF_SRAM")]
pub type PufSram = crate::Reg<puf_sram::PufSramSpec>;
#[doc = "no description available"]
pub mod puf_sram;

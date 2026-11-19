use std::sync::Arc;
use binder::{BinderFeatures, Interface, Strong};
use crate::IEngineer::BnEngineer;
use crate::IEngineer::IEngineer as IEngineerTrait;
use crate::hal::Hal;
use crate::{logi, loge, VibratorResult, ReadDataResult, RFNodeStatus};

pub const SERVICE_NAME: &str = "vendor.oplus.hardware.engineer.IEngineer/default";

pub struct EngineerService {
    hal: Arc<Hal>,
}

impl EngineerService {
    pub fn new(hal: Arc<Hal>) -> Self {
        Self { hal }
    }

    pub fn create_binder(self) -> Strong<dyn IEngineerTrait> {
        BnEngineer::new_binder(self, BinderFeatures::default())
    }
}

impl Interface for EngineerService {}

impl IEngineerTrait for EngineerService {
    fn saveEngineerData(&self, data_type: i32, data: &[u8]) -> binder::Result<bool> {
        Ok(self.hal.save_engineer_data(data_type, data))
    }

    fn readEngineerData(&self, data_type: i32, data: &[u8]) -> binder::Result<VibratorResult> {
        Ok(self.hal.read_engineer_data(data_type, data))
    }

    fn getPartitionWriteProtectState(&self) -> binder::Result<bool> {
        Ok(self.hal.get_partition_write_protect_state())
    }

    fn setPartitionWriteProtectState(&self, enable: bool) -> binder::Result<bool> {
        Ok(self.hal.set_partition_write_protect_state(enable))
    }

    fn getProductLineTestResult(&self) -> binder::Result<Vec<u8>> {
        Ok(self.hal.get_product_line_test_result())
    }

    fn setProductLineTestResult(&self, index: i32, value: i32) -> binder::Result<bool> {
        Ok(self.hal.set_product_line_test_result(index, value))
    }

    fn resetProductLineTestResult(&self) -> binder::Result<bool> {
        Ok(self.hal.reset_product_line_test_result())
    }

    fn loadSecrecyConfig(&self) -> binder::Result<Vec<u8>> {
        Ok(self.hal.load_secrecy_config())
    }

    fn saveSecrecyConfig(&self, config: &str) -> binder::Result<bool> {
        Ok(self.hal.save_secrecy_config(config))
    }

    fn setVibratorCalibrateData(&self, key: &str, value: &str) -> binder::Result<VibratorResult> {
        Ok(self.hal.set_vibrator_calibrate_data(key, value))
    }

    fn getVibratorCalibrateResult(&self, key: &str) -> binder::Result<VibratorResult> {
        Ok(self.hal.get_vibrator_calibrate_result(key))
    }

    fn getImpedance(&self) -> binder::Result<VibratorResult> {
        Ok(self.hal.get_impedance())
    }

    fn startVibrate(&self, mode: i32, index: i32, value: i32) -> binder::Result<i32> {
        Ok(self.hal.start_vibrate(mode, index, value))
    }

    fn startAgeVibrate(&self, mode: i32, index: i32) -> binder::Result<i32> {
        Ok(self.hal.start_age_vibrate(mode, index))
    }

    fn setProperties(&self, key: &str, value: &str) -> binder::Result<bool> {
        Ok(self.hal.set_properties(key, value))
    }

    fn queryMountPointMounted(&self, path: &str) -> binder::Result<bool> {
        Ok(self.hal.query_mount_point_mounted(path))
    }

    fn queryDeviceInfo(&self) -> binder::Result<String> {
        Ok(self.hal.query_device_info())
    }

    fn saveEngineerPersistData(&self, key: &str, value: &str) -> binder::Result<bool> {
        Ok(self.hal.save_engineer_persist_data(key, value))
    }

    fn queryEngineerPersistData(&self, key: &str) -> binder::Result<String> {
        Ok(self.hal.query_engineer_persist_data(key))
    }

    fn clearEngineerPersistData(&self, key: &str) -> binder::Result<bool> {
        Ok(self.hal.clear_engineer_persist_data(key))
    }

    fn listEngineerPersistData(&self, prefix: &str) -> binder::Result<Vec<String>> {
        Ok(self.hal.list_engineer_persist_data(prefix))
    }

    fn readVendorFile(&self, path: &str) -> binder::Result<Vec<u8>> {
        Ok(self.hal.read_vendor_file(path))
    }

    fn saveDataToVendorFile(&self, path: &str, data: &str, offset: i32) -> binder::Result<i32> {
        Ok(self.hal.save_data_to_vendor_file(path, data, offset))
    }

    fn writeData(&self, path: &str, offset: i32, append: bool, length: i32, data: &[u8]) -> binder::Result<i32> {
        Ok(self.hal.write_data(path, offset, append, length, data))
    }

    fn readData(&self, path: &str, offset: i32, length: i32) -> binder::Result<ReadDataResult> {
        Ok(self.hal.read_data(path, offset, length))
    }

    fn getQcRfCableStatus(&self) -> binder::Result<RFNodeStatus> {
        Ok(self.hal.get_qc_rf_cable_status())
    }

    fn getMtkSwtpStatus(&self) -> binder::Result<RFNodeStatus> {
        Ok(self.hal.get_mtk_swtp_status())
    }

    fn getQcPdsStatus(&self) -> binder::Result<RFNodeStatus> {
        Ok(self.hal.get_qc_pds_status())
    }

    fn getInterfaceVersion(&self) -> binder::Result<i32> {
        Ok(crate::IEngineer::VERSION)
    }

    fn getInterfaceHash(&self) -> binder::Result<String> {
        Ok(crate::IEngineer::HASH.to_string())
    }
}
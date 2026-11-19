#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)]
use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;

#[cfg(feature = "vintf_stability")]
const SERVICE_STABILITY: binder::binder_impl::Stability = binder::binder_impl::Stability::Vintf;
#[cfg(not(feature = "vintf_stability"))]
const SERVICE_STABILITY: binder::binder_impl::Stability = binder::binder_impl::Stability::Local;

declare_binder_interface! {
  IEngineer["vendor.oplus.hardware.engineer.IEngineer"] {
    native: BnEngineer(on_transact),
    proxy: BpEngineer {
      cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
      cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
    },
    stability: SERVICE_STABILITY,
  }
}

use crate::ReadDataResult;
use crate::RFNodeStatus;
use crate::VibratorResult;

pub trait IEngineer: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "vendor.oplus.hardware.engineer.IEngineer"
    }
    fn r#saveEngineerData(&self, _arg_type: i32, _arg_data: &[u8]) -> binder::Result<bool>;
    fn r#readEngineerData(&self, _arg_type: i32, _arg_data: &[u8])
        -> binder::Result<VibratorResult>;
    fn r#getPartitionWriteProtectState(&self) -> binder::Result<bool>;
    fn r#setPartitionWriteProtectState(&self, _arg_enable: bool) -> binder::Result<bool>;
    fn r#getProductLineTestResult(&self) -> binder::Result<Vec<u8>>;
    fn r#setProductLineTestResult(&self, _arg_index: i32, _arg_value: i32)
        -> binder::Result<bool>;
    fn r#resetProductLineTestResult(&self) -> binder::Result<bool>;
    fn r#loadSecrecyConfig(&self) -> binder::Result<Vec<u8>>;
    fn r#saveSecrecyConfig(&self, _arg_config: &str) -> binder::Result<bool>;
    fn r#setVibratorCalibrateData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<VibratorResult>;
    fn r#getVibratorCalibrateResult(&self, _arg_key: &str) -> binder::Result<VibratorResult>;
    fn r#getImpedance(&self) -> binder::Result<VibratorResult>;
    fn r#startVibrate(
        &self,
        _arg_mode: i32,
        _arg_index: i32,
        _arg_value: i32,
    ) -> binder::Result<i32>;
    fn r#startAgeVibrate(&self, _arg_mode: i32, _arg_index: i32) -> binder::Result<i32>;
    fn r#setProperties(&self, _arg_key: &str, _arg_value: &str) -> binder::Result<bool>;
    fn r#queryMountPointMounted(&self, _arg_path: &str) -> binder::Result<bool>;
    fn r#queryDeviceInfo(&self) -> binder::Result<String>;
    fn r#saveEngineerPersistData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<bool>;
    fn r#queryEngineerPersistData(&self, _arg_key: &str) -> binder::Result<String>;
    fn r#clearEngineerPersistData(&self, _arg_key: &str) -> binder::Result<bool>;
    fn r#listEngineerPersistData(&self, _arg_prefix: &str) -> binder::Result<Vec<String>>;
    fn r#readVendorFile(&self, _arg_path: &str) -> binder::Result<Vec<u8>>;
    fn r#saveDataToVendorFile(
        &self,
        _arg_path: &str,
        _arg_data: &str,
        _arg_offset: i32,
    ) -> binder::Result<i32>;
    fn r#writeData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_append: bool,
        _arg_length: i32,
        _arg_data: &[u8],
    ) -> binder::Result<i32>;
    fn r#readData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_length: i32,
    ) -> binder::Result<ReadDataResult>;
    fn r#getQcRfCableStatus(&self) -> binder::Result<RFNodeStatus>;
    fn r#getMtkSwtpStatus(&self) -> binder::Result<RFNodeStatus>;
    fn r#getQcPdsStatus(&self) -> binder::Result<RFNodeStatus>;
    fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
        Ok(VERSION)
    }
    fn r#getInterfaceHash(&self) -> binder::Result<String> {
        Ok(HASH.into())
    }
    fn getDefaultImpl() -> IEngineerDefaultRef
    where
        Self: Sized,
    {
        DEFAULT_IMPL.lock().unwrap().clone()
    }
    fn setDefaultImpl(d: IEngineerDefaultRef) -> IEngineerDefaultRef
    where
        Self: Sized,
    {
        std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
    }
}

pub mod transactions {
    pub const r#saveEngineerData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
    pub const r#readEngineerData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
    pub const r#getPartitionWriteProtectState: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
    pub const r#setPartitionWriteProtectState: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
    pub const r#getProductLineTestResult: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
    pub const r#setProductLineTestResult: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
    pub const r#resetProductLineTestResult: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
    pub const r#loadSecrecyConfig: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
    pub const r#saveSecrecyConfig: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
    pub const r#setVibratorCalibrateData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
    pub const r#getVibratorCalibrateResult: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
    pub const r#getImpedance: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
    pub const r#startVibrate: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
    pub const r#startAgeVibrate: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
    pub const r#setProperties: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
    pub const r#queryMountPointMounted: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
    pub const r#queryDeviceInfo: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
    pub const r#saveEngineerPersistData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
    pub const r#queryEngineerPersistData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
    pub const r#clearEngineerPersistData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
    pub const r#listEngineerPersistData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
    pub const r#readVendorFile: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
    pub const r#saveDataToVendorFile: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
    pub const r#writeData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
    pub const r#readData: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
    pub const r#getQcRfCableStatus: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
    pub const r#getMtkSwtpStatus: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
    pub const r#getQcPdsStatus: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
    pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
    pub const r#getInterfaceHash: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
}

pub type IEngineerDefaultRef = Option<std::sync::Arc<dyn IEngineer>>;
static DEFAULT_IMPL: std::sync::Mutex<IEngineerDefaultRef> = std::sync::Mutex::new(None);
pub const VERSION: i32 = 1;
pub const HASH: &str = "beafa669f2d959dea8f2191a42c2dec41af2e9c2";

impl BpEngineer {
    fn read_response_void(
        &self,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<()> {
        let _aidl_reply = _aidl_reply?;
        let _aidl_status: binder::Status = _aidl_reply.read()?;
        if !_aidl_status.is_ok() {
            return Err(_aidl_status);
        }
        Ok(())
    }

    fn read_response_value<T: binder::binder_impl::Deserialize>(
        &self,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<T> {
        let _aidl_reply = _aidl_reply?;
        let _aidl_status: binder::Status = _aidl_reply.read()?;
        if !_aidl_status.is_ok() {
            return Err(_aidl_status);
        }
        let _aidl_return: T = _aidl_reply.read()?;
        Ok(_aidl_return)
    }
}

impl IEngineer for BpEngineer {
    fn r#saveEngineerData(&self, _arg_type: i32, _arg_data: &[u8]) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_type)?;
        _aidl_data.write(_arg_data)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#saveEngineerData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#readEngineerData(
        &self,
        _arg_type: i32,
        _arg_data: &[u8],
    ) -> binder::Result<VibratorResult> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_type)?;
        _aidl_data.write(_arg_data)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#readEngineerData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getPartitionWriteProtectState(&self) -> binder::Result<bool> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getPartitionWriteProtectState,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#setPartitionWriteProtectState(&self, _arg_enable: bool) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_enable)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#setPartitionWriteProtectState,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getProductLineTestResult(&self) -> binder::Result<Vec<u8>> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getProductLineTestResult,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#setProductLineTestResult(
        &self,
        _arg_index: i32,
        _arg_value: i32,
    ) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_index)?;
        _aidl_data.write(&_arg_value)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#setProductLineTestResult,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#resetProductLineTestResult(&self) -> binder::Result<bool> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#resetProductLineTestResult,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#loadSecrecyConfig(&self) -> binder::Result<Vec<u8>> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#loadSecrecyConfig,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#saveSecrecyConfig(&self, _arg_config: &str) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_config)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#saveSecrecyConfig,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#setVibratorCalibrateData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<VibratorResult> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        _aidl_data.write(_arg_value)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#setVibratorCalibrateData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getVibratorCalibrateResult(&self, _arg_key: &str) -> binder::Result<VibratorResult> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getVibratorCalibrateResult,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getImpedance(&self) -> binder::Result<VibratorResult> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getImpedance,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#startVibrate(
        &self,
        _arg_mode: i32,
        _arg_index: i32,
        _arg_value: i32,
    ) -> binder::Result<i32> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_mode)?;
        _aidl_data.write(&_arg_index)?;
        _aidl_data.write(&_arg_value)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#startVibrate,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#startAgeVibrate(&self, _arg_mode: i32, _arg_index: i32) -> binder::Result<i32> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(&_arg_mode)?;
        _aidl_data.write(&_arg_index)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#startAgeVibrate,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#setProperties(&self, _arg_key: &str, _arg_value: &str) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        _aidl_data.write(_arg_value)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#setProperties,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#queryMountPointMounted(&self, _arg_path: &str) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_path)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#queryMountPointMounted,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#queryDeviceInfo(&self) -> binder::Result<String> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#queryDeviceInfo,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#saveEngineerPersistData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        _aidl_data.write(_arg_value)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#saveEngineerPersistData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#queryEngineerPersistData(&self, _arg_key: &str) -> binder::Result<String> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#queryEngineerPersistData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#clearEngineerPersistData(&self, _arg_key: &str) -> binder::Result<bool> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_key)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#clearEngineerPersistData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#listEngineerPersistData(&self, _arg_prefix: &str) -> binder::Result<Vec<String>> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_prefix)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#listEngineerPersistData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#readVendorFile(&self, _arg_path: &str) -> binder::Result<Vec<u8>> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_path)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#readVendorFile,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#saveDataToVendorFile(
        &self,
        _arg_path: &str,
        _arg_data: &str,
        _arg_offset: i32,
    ) -> binder::Result<i32> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_path)?;
        _aidl_data.write(_arg_data)?;
        _aidl_data.write(&_arg_offset)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#saveDataToVendorFile,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#writeData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_append: bool,
        _arg_length: i32,
        _arg_data: &[u8],
    ) -> binder::Result<i32> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_path)?;
        _aidl_data.write(&_arg_offset)?;
        _aidl_data.write(&_arg_append)?;
        _aidl_data.write(&_arg_length)?;
        _aidl_data.write(_arg_data)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#writeData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#readData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_length: i32,
    ) -> binder::Result<ReadDataResult> {
        let mut _aidl_data = self.binder.prepare_transact()?;
        _aidl_data.write(_arg_path)?;
        _aidl_data.write(&_arg_offset)?;
        _aidl_data.write(&_arg_length)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#readData,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getQcRfCableStatus(&self) -> binder::Result<RFNodeStatus> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getQcRfCableStatus,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getMtkSwtpStatus(&self) -> binder::Result<RFNodeStatus> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getMtkSwtpStatus,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getQcPdsStatus(&self) -> binder::Result<RFNodeStatus> {
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getQcPdsStatus,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_value(_aidl_reply)
    }
    fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
        let _cached = self
            .cached_version
            .load(std::sync::atomic::Ordering::Relaxed);
        if _cached != -1 {
            return Ok(_cached);
        }
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getInterfaceVersion,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        let _aidl_return: i32 = self.read_response_value(_aidl_reply)?;
        self.cached_version
            .store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
        Ok(_aidl_return)
    }
    fn r#getInterfaceHash(&self) -> binder::Result<String> {
        {
            let _cached = self.cached_hash.lock().unwrap();
            if let Some(ref _hash) = *_cached {
                return Ok(_hash.clone());
            }
        }
        let _aidl_data = self.binder.prepare_transact()?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#getInterfaceHash,
            _aidl_data,
            binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        let _aidl_return: String = self.read_response_value(_aidl_reply)?;
        *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
        Ok(_aidl_return)
    }
}

impl IEngineer for binder::binder_impl::Binder<BnEngineer> {
    fn r#saveEngineerData(&self, _arg_type: i32, _arg_data: &[u8]) -> binder::Result<bool> {
        self.0.r#saveEngineerData(_arg_type, _arg_data)
    }
    fn r#readEngineerData(
        &self,
        _arg_type: i32,
        _arg_data: &[u8],
    ) -> binder::Result<VibratorResult> {
        self.0.r#readEngineerData(_arg_type, _arg_data)
    }
    fn r#getPartitionWriteProtectState(&self) -> binder::Result<bool> {
        self.0.r#getPartitionWriteProtectState()
    }
    fn r#setPartitionWriteProtectState(&self, _arg_enable: bool) -> binder::Result<bool> {
        self.0.r#setPartitionWriteProtectState(_arg_enable)
    }
    fn r#getProductLineTestResult(&self) -> binder::Result<Vec<u8>> {
        self.0.r#getProductLineTestResult()
    }
    fn r#setProductLineTestResult(
        &self,
        _arg_index: i32,
        _arg_value: i32,
    ) -> binder::Result<bool> {
        self.0.r#setProductLineTestResult(_arg_index, _arg_value)
    }
    fn r#resetProductLineTestResult(&self) -> binder::Result<bool> {
        self.0.r#resetProductLineTestResult()
    }
    fn r#loadSecrecyConfig(&self) -> binder::Result<Vec<u8>> {
        self.0.r#loadSecrecyConfig()
    }
    fn r#saveSecrecyConfig(&self, _arg_config: &str) -> binder::Result<bool> {
        self.0.r#saveSecrecyConfig(_arg_config)
    }
    fn r#setVibratorCalibrateData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<VibratorResult> {
        self.0.r#setVibratorCalibrateData(_arg_key, _arg_value)
    }
    fn r#getVibratorCalibrateResult(&self, _arg_key: &str) -> binder::Result<VibratorResult> {
        self.0.r#getVibratorCalibrateResult(_arg_key)
    }
    fn r#getImpedance(&self) -> binder::Result<VibratorResult> {
        self.0.r#getImpedance()
    }
    fn r#startVibrate(
        &self,
        _arg_mode: i32,
        _arg_index: i32,
        _arg_value: i32,
    ) -> binder::Result<i32> {
        self.0.r#startVibrate(_arg_mode, _arg_index, _arg_value)
    }
    fn r#startAgeVibrate(&self, _arg_mode: i32, _arg_index: i32) -> binder::Result<i32> {
        self.0.r#startAgeVibrate(_arg_mode, _arg_index)
    }
    fn r#setProperties(&self, _arg_key: &str, _arg_value: &str) -> binder::Result<bool> {
        self.0.r#setProperties(_arg_key, _arg_value)
    }
    fn r#queryMountPointMounted(&self, _arg_path: &str) -> binder::Result<bool> {
        self.0.r#queryMountPointMounted(_arg_path)
    }
    fn r#queryDeviceInfo(&self) -> binder::Result<String> {
        self.0.r#queryDeviceInfo()
    }
    fn r#saveEngineerPersistData(
        &self,
        _arg_key: &str,
        _arg_value: &str,
    ) -> binder::Result<bool> {
        self.0.r#saveEngineerPersistData(_arg_key, _arg_value)
    }
    fn r#queryEngineerPersistData(&self, _arg_key: &str) -> binder::Result<String> {
        self.0.r#queryEngineerPersistData(_arg_key)
    }
    fn r#clearEngineerPersistData(&self, _arg_key: &str) -> binder::Result<bool> {
        self.0.r#clearEngineerPersistData(_arg_key)
    }
    fn r#listEngineerPersistData(&self, _arg_prefix: &str) -> binder::Result<Vec<String>> {
        self.0.r#listEngineerPersistData(_arg_prefix)
    }
    fn r#readVendorFile(&self, _arg_path: &str) -> binder::Result<Vec<u8>> {
        self.0.r#readVendorFile(_arg_path)
    }
    fn r#saveDataToVendorFile(
        &self,
        _arg_path: &str,
        _arg_data: &str,
        _arg_offset: i32,
    ) -> binder::Result<i32> {
        self.0
            .r#saveDataToVendorFile(_arg_path, _arg_data, _arg_offset)
    }
    fn r#writeData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_append: bool,
        _arg_length: i32,
        _arg_data: &[u8],
    ) -> binder::Result<i32> {
        self.0
            .r#writeData(_arg_path, _arg_offset, _arg_append, _arg_length, _arg_data)
    }
    fn r#readData(
        &self,
        _arg_path: &str,
        _arg_offset: i32,
        _arg_length: i32,
    ) -> binder::Result<ReadDataResult> {
        self.0.r#readData(_arg_path, _arg_offset, _arg_length)
    }
    fn r#getQcRfCableStatus(&self) -> binder::Result<RFNodeStatus> {
        self.0.r#getQcRfCableStatus()
    }
    fn r#getMtkSwtpStatus(&self) -> binder::Result<RFNodeStatus> {
        self.0.r#getMtkSwtpStatus()
    }
    fn r#getQcPdsStatus(&self) -> binder::Result<RFNodeStatus> {
        self.0.r#getQcPdsStatus()
    }
    fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
        self.0.r#getInterfaceVersion()
    }
    fn r#getInterfaceHash(&self) -> binder::Result<String> {
        self.0.r#getInterfaceHash()
    }
}

fn on_transact(
    _aidl_service: &dyn IEngineer,
    _aidl_code: binder::binder_impl::TransactionCode,
    _aidl_data: &binder::binder_impl::BorrowedParcel<'_>,
    _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>,
) -> std::result::Result<(), binder::StatusCode> {
    match _aidl_code {
        transactions::r#saveEngineerData => {
            let _arg_type: i32 = _aidl_data.read()?;
            let _arg_data: Vec<u8> = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#saveEngineerData(_arg_type, &_arg_data);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#readEngineerData => {
            let _arg_type: i32 = _aidl_data.read()?;
            let _arg_data: Vec<u8> = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#readEngineerData(_arg_type, &_arg_data);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getPartitionWriteProtectState => {
            let _aidl_return = _aidl_service.r#getPartitionWriteProtectState();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#setPartitionWriteProtectState => {
            let _arg_enable: bool = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#setPartitionWriteProtectState(_arg_enable);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getProductLineTestResult => {
            let _aidl_return = _aidl_service.r#getProductLineTestResult();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#setProductLineTestResult => {
            let _arg_index: i32 = _aidl_data.read()?;
            let _arg_value: i32 = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#setProductLineTestResult(_arg_index, _arg_value);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#resetProductLineTestResult => {
            let _aidl_return = _aidl_service.r#resetProductLineTestResult();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#loadSecrecyConfig => {
            let _aidl_return = _aidl_service.r#loadSecrecyConfig();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#saveSecrecyConfig => {
            let _arg_config: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#saveSecrecyConfig(&_arg_config);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#setVibratorCalibrateData => {
            let _arg_key: String = _aidl_data.read()?;
            let _arg_value: String = _aidl_data.read()?;
            let _aidl_return =
                _aidl_service.r#setVibratorCalibrateData(&_arg_key, &_arg_value);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getVibratorCalibrateResult => {
            let _arg_key: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#getVibratorCalibrateResult(&_arg_key);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getImpedance => {
            let _aidl_return = _aidl_service.r#getImpedance();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#startVibrate => {
            let _arg_mode: i32 = _aidl_data.read()?;
            let _arg_index: i32 = _aidl_data.read()?;
            let _arg_value: i32 = _aidl_data.read()?;
            let _aidl_return =
                _aidl_service.r#startVibrate(_arg_mode, _arg_index, _arg_value);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#startAgeVibrate => {
            let _arg_mode: i32 = _aidl_data.read()?;
            let _arg_index: i32 = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#startAgeVibrate(_arg_mode, _arg_index);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#setProperties => {
            let _arg_key: String = _aidl_data.read()?;
            let _arg_value: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#setProperties(&_arg_key, &_arg_value);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#queryMountPointMounted => {
            let _arg_path: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#queryMountPointMounted(&_arg_path);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#queryDeviceInfo => {
            let _aidl_return = _aidl_service.r#queryDeviceInfo();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#saveEngineerPersistData => {
            let _arg_key: String = _aidl_data.read()?;
            let _arg_value: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#saveEngineerPersistData(&_arg_key, &_arg_value);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#queryEngineerPersistData => {
            let _arg_key: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#queryEngineerPersistData(&_arg_key);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#clearEngineerPersistData => {
            let _arg_key: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#clearEngineerPersistData(&_arg_key);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#listEngineerPersistData => {
            let _arg_prefix: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#listEngineerPersistData(&_arg_prefix);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#readVendorFile => {
            let _arg_path: String = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#readVendorFile(&_arg_path);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#saveDataToVendorFile => {
            let _arg_path: String = _aidl_data.read()?;
            let _arg_data: String = _aidl_data.read()?;
            let _arg_offset: i32 = _aidl_data.read()?;
            let _aidl_return =
                _aidl_service.r#saveDataToVendorFile(&_arg_path, &_arg_data, _arg_offset);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#writeData => {
            let _arg_path: String = _aidl_data.read()?;
            let _arg_offset: i32 = _aidl_data.read()?;
            let _arg_append: bool = _aidl_data.read()?;
            let _arg_length: i32 = _aidl_data.read()?;
            let _arg_data: Vec<u8> = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#writeData(
                &_arg_path,
                _arg_offset,
                _arg_append,
                _arg_length,
                &_arg_data,
            );
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#readData => {
            let _arg_path: String = _aidl_data.read()?;
            let _arg_offset: i32 = _aidl_data.read()?;
            let _arg_length: i32 = _aidl_data.read()?;
            let _aidl_return =
                _aidl_service.r#readData(&_arg_path, _arg_offset, _arg_length);
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getQcRfCableStatus => {
            let _aidl_return = _aidl_service.r#getQcRfCableStatus();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getMtkSwtpStatus => {
            let _aidl_return = _aidl_service.r#getMtkSwtpStatus();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getQcPdsStatus => {
            let _aidl_return = _aidl_service.r#getQcPdsStatus();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getInterfaceVersion => {
            let _aidl_return = _aidl_service.r#getInterfaceVersion();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        transactions::r#getInterfaceHash => {
            let _aidl_return = _aidl_service.r#getInterfaceHash();
            match &_aidl_return {
                Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                }
                Err(_aidl_status) => _aidl_reply.write(_aidl_status)?,
            }
            Ok(())
        }
        _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION),
    }
}

pub(crate) mod mangled {
    pub use super::r#IEngineer as _6_vendor_5_oplus_8_hardware_8_engineer_9_IEngineer;
}

use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use libc::{open, close, read, write, lseek, fsync, O_RDONLY, O_WRONLY, O_CREAT, O_TRUNC, O_APPEND};
use crate::{logi, logw, loge, VibratorResult, ReadDataResult, RFNodeStatus};

extern "C" {
    fn __system_property_set(key: *const i8, value: *const i8) -> i32;
}

pub struct Hal;

impl Hal {
    pub fn open() -> Option<Self> {
        Some(Self)
    }

    fn get_reserve_path(&self) -> String {
        if Path::new("/dev/block/by-name/opporeserve1").exists() {
            "/dev/block/by-name/opporeserve1".to_string()
        } else {
            "/dev/block/by-name/oplusreserve1".to_string()
        }
    }

    fn read_file_at(&self, path: &str, offset: u64, buf: &mut [u8]) -> Result<usize, String> {
        let fd = unsafe { open(path.as_ptr() as *const libc::c_char, O_RDONLY) };
        if fd < 0 {
            return Err(format!("open {} failed", path));
        }
        let ret = unsafe { lseek(fd, offset as i64, 0) };
        if ret < 0 {
            unsafe { close(fd) };
            return Err("lseek failed".to_string());
        }
        let n = unsafe { read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        unsafe { close(fd) };
        if n < 0 {
            Err("read failed".to_string())
        } else {
            Ok(n as usize)
        }
    }

    fn write_file_at(&self, path: &str, offset: u64, data: &[u8], truncate: bool) -> Result<usize, String> {
        let flags = if truncate { O_WRONLY | O_CREAT | O_TRUNC } else { O_WRONLY | O_CREAT };
        let fd = unsafe { open(path.as_ptr() as *const libc::c_char, flags, 0o644) };
        if fd < 0 {
            return Err(format!("open {} failed", path));
        }
        let ret = unsafe { lseek(fd, offset as i64, 0) };
        if ret < 0 {
            unsafe { close(fd) };
            return Err("lseek failed".to_string());
        }
        let n = unsafe { write(fd, data.as_ptr() as *const libc::c_void, data.len()) };
        if n >= 0 {
            unsafe { fsync(fd) };
        }
        unsafe { close(fd) };
        if n < 0 {
            Err("write failed".to_string())
        } else {
            Ok(n as usize)
        }
    }

    pub fn save_engineer_data(&self, data_type: i32, data: &[u8]) -> bool {
        logi!("saveEngineerData type={} length={}", data_type, data.len());
        let path = "/mnt/vendor/persist/engineermode/engineermode_config";
        match self.write_file_at(path, 0, data, true) {
            Ok(n) => {
                logi!("saveEngineerData wrote {} bytes", n);
                true
            }
            Err(e) => {
                loge!("saveEngineerData failed: {}", e);
                false
            }
        }
    }

    pub fn read_engineer_data(&self, data_type: i32, _data: &[u8]) -> VibratorResult {
        logi!("readEngineerData type={}", data_type);
        let path = "/mnt/vendor/persist/engineermode/engineermode_config";
        let mut buf = vec![0u8; 4096];
        match self.read_file_at(path, 0, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                VibratorResult { result: 0, message: String::from_utf8_lossy(&buf).to_string() }
            }
            Err(e) => {
                loge!("readEngineerData failed: {}", e);
                VibratorResult { result: -1, message: e }
            }
        }
    }

    pub fn get_partition_write_protect_state(&self) -> bool {
        logi!("getPartitionWriteProtectState");
        let path = "/mnt/vendor/oplusreserve/system/config/secrecy.cfg";
        fs::read_to_string(path).map(|s| s.trim() == "1").unwrap_or(false)
    }

    pub fn set_partition_write_protect_state(&self, enable: bool) -> bool {
        logi!("setPartitionWriteProtectState enable={}", enable);
        let path = "/mnt/vendor/oplusreserve/system/config/secrecy.cfg";
        let data = if enable { b"1" } else { b"0" };
        match self.write_file_at(path, 0, data, true) {
            Ok(_) => true,
            Err(e) => {
                loge!("setPartitionWriteProtectState failed: {}", e);
                false
            }
        }
    }

    pub fn get_product_line_test_result(&self) -> Vec<u8> {
        logi!("getProductLineTestResult");
        let path = "/mnt/vendor/oplusreserve/radio/rule_list.config";
        fs::read(path).unwrap_or_default()
    }

    pub fn set_product_line_test_result(&self, index: i32, value: i32) -> bool {
        logi!("setProductLineTestResult index={} value={}", index, value);
        true
    }

    pub fn reset_product_line_test_result(&self) -> bool {
        logi!("resetProductLineTestResult");
        true
    }

    pub fn load_secrecy_config(&self) -> Vec<u8> {
        logi!("loadSecrecyConfig");
        let path = "/mnt/vendor/oplusreserve/system/config/secrecy.cfg";
        fs::read(path).unwrap_or_default()
    }

    pub fn save_secrecy_config(&self, config: &str) -> bool {
        logi!("saveSecrecyConfig len={}", config.len());
        let path = "/mnt/vendor/oplusreserve/system/config/secrecy.cfg";
        match self.write_file_at(path, 0, config.as_bytes(), true) {
            Ok(_) => true,
            Err(e) => {
                loge!("saveSecrecyConfig failed: {}", e);
                false
            }
        }
    }

    pub fn set_vibrator_calibrate_data(&self, key: &str, value: &str) -> VibratorResult {
        logi!("setVibratorCalibrateData key={} value={}", key, value);
        let path = format!("/sys/class/leds/vibrator/{}", key);
        match self.write_file_at(&path, 0, value.as_bytes(), false) {
            Ok(n) => VibratorResult { result: 0, message: format!("wrote {} bytes", n) },
            Err(e) => VibratorResult { result: -1, message: e }
        }
    }

    pub fn get_vibrator_calibrate_result(&self, key: &str) -> VibratorResult {
        logi!("getVibratorCalibrateResult key={}", key);
        let path = format!("/sys/class/leds/vibrator/{}", key);
        let mut buf = vec![0u8; 256];
        match self.read_file_at(&path, 0, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                VibratorResult { result: 0, message: String::from_utf8_lossy(&buf).to_string() }
            }
            Err(e) => VibratorResult { result: -1, message: e }
        }
    }

    pub fn get_impedance(&self) -> VibratorResult {
        logi!("getImpedance");
        let path = "/sys/class/leds/vibrator/oplus_impedance";
        let mut buf = vec![0u8; 64];
        match self.read_file_at(path, 0, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                VibratorResult { result: 0, message: String::from_utf8_lossy(&buf).to_string() }
            }
            Err(e) => VibratorResult { result: -1, message: e }
        }
    }

    pub fn start_vibrate(&self, mode: i32, index: i32, value: i32) -> i32 {
        logi!("startVibrate mode={} index={} value={}", mode, index, value);
        let path = "/sys/class/leds/vibrator/oplus_activate";
        let data = format!("{} {} {}", mode, index, value);
        match self.write_file_at(path, 0, data.as_bytes(), false) {
            Ok(_) => 0,
            Err(e) => {
                loge!("startVibrate failed: {}", e);
                -1
            }
        }
    }

    pub fn start_age_vibrate(&self, mode: i32, index: i32) -> i32 {
        logi!("startAgeVibrate mode={} index={}", mode, index);
        let path = "/sys/class/leds/vibrator/oplus_duration";
        let data = format!("{} {}", mode, index);
        match self.write_file_at(path, 0, data.as_bytes(), false) {
            Ok(_) => 0,
            Err(e) => {
                loge!("startAgeVibrate failed: {}", e);
                -1
            }
        }
    }

    pub fn set_properties(&self, key: &str, value: &str) -> bool {
        logi!("setProperties key={} value={}", key, value);
        let prop_name = format!("vendor.oplus.engineer.{}", key);
        unsafe {
            __system_property_set(prop_name.as_ptr() as *const i8, value.as_ptr() as *const i8);
        }
        true
    }

    pub fn query_mount_point_mounted(&self, path: &str) -> bool {
        logi!("queryMountPointMounted path={}", path);
        Path::new(path).exists()
    }

    pub fn query_device_info(&self) -> String {
        logi!("queryDeviceInfo");
        let mut info = String::new();
        if let Ok(serial) = fs::read_to_string("/proc/oplusVersion/serialID") {
            info.push_str(&format!("serial:{}", serial.trim()));
        }
        info
    }

    pub fn save_engineer_persist_data(&self, key: &str, value: &str) -> bool {
        logi!("saveEngineerPersistData key={} value={}", key, value);
        let path = format!("/mnt/vendor/persist/engineermode/{}", key);
        match self.write_file_at(&path, 0, value.as_bytes(), true) {
            Ok(_) => true,
            Err(e) => {
                loge!("saveEngineerPersistData failed: {}", e);
                false
            }
        }
    }

    pub fn query_engineer_persist_data(&self, key: &str) -> String {
        logi!("queryEngineerPersistData key={}", key);
        let path = format!("/mnt/vendor/persist/engineermode/{}", key);
        fs::read_to_string(path).unwrap_or_default()
    }

    pub fn clear_engineer_persist_data(&self, key: &str) -> bool {
        logi!("clearEngineerPersistData key={}", key);
        let path = format!("/mnt/vendor/persist/engineermode/{}", key);
        fs::remove_file(path).is_ok()
    }

    pub fn list_engineer_persist_data(&self, prefix: &str) -> Vec<String> {
        logi!("listEngineerPersistData prefix={}", prefix);
        let path = "/mnt/vendor/persist/engineermode";
        fs::read_dir(path)
            .map(|entries| {
                entries.filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok())
                    .filter(|name| name.starts_with(prefix))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn read_vendor_file(&self, path: &str) -> Vec<u8> {
        logi!("readVendorFile path={}", path);
        fs::read(path).unwrap_or_default()
    }

    pub fn save_data_to_vendor_file(&self, path: &str, data: &str, offset: i32) -> i32 {
        logi!("saveDataToVendorFile path={} offset={}", path, offset);
        match self.write_file_at(path, offset as u64, data.as_bytes(), false) {
            Ok(n) => n as i32,
            Err(e) => {
                loge!("saveDataToVendorFile failed: {}", e);
                -1
            }
        }
    }

    pub fn write_data(&self, path: &str, offset: i32, append: bool, length: i32, data: &[u8]) -> i32 {
        logi!("writeData path={} offset={} append={} length={}", path, offset, append, length);
        let flags = if append { O_WRONLY | O_CREAT | O_APPEND } else { O_WRONLY | O_CREAT };
        let fd = unsafe { open(path.as_ptr() as *const libc::c_char, flags, 0o644) };
        if fd < 0 {
            return -1;
        }
        let ret = unsafe { lseek(fd, offset as i64, 0) };
        if ret < 0 {
            unsafe { close(fd) };
            return -1;
        }
        let n = unsafe { write(fd, data.as_ptr() as *const libc::c_void, data.len().min(length as usize)) };
        if n >= 0 {
            unsafe { fsync(fd) };
        }
        unsafe { close(fd) };
        n as i32
    }

    pub fn read_data(&self, path: &str, offset: i32, length: i32) -> ReadDataResult {
        logi!("readData path={} offset={} length={}", path, offset, length);
        let mut buf = vec![0u8; length as usize];
        match self.read_file_at(path, offset as u64, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                ReadDataResult { result: 0, data: buf }
            }
            Err(e) => {
                loge!("readData failed: {}", e);
                ReadDataResult { result: -1, data: vec![] }
            }
        }
    }

    pub fn get_qc_rf_cable_status(&self) -> RFNodeStatus {
        logi!("getQcRfCableStatus");
        let mut buf = vec![0u8; 64];
        match self.read_file_at("/proc/oplus_rf/rf_cable", 0, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                let status = String::from_utf8_lossy(&buf).trim().parse().unwrap_or(0);
                RFNodeStatus { status, value: 0 }
            }
            Err(_) => RFNodeStatus { status: -1, value: 0 }
        }
    }

    pub fn get_mtk_swtp_status(&self) -> RFNodeStatus {
        logi!("getMtkSwtpStatus");
        RFNodeStatus { status: 0, value: 0 }
    }

    pub fn get_qc_pds_status(&self) -> RFNodeStatus {
        logi!("getQcPdsStatus");
        let mut buf = vec![0u8; 64];
        match self.read_file_at("/proc/oplus_rf/rf_pds", 0, &mut buf) {
            Ok(n) => {
                buf.truncate(n);
                let status = String::from_utf8_lossy(&buf).trim().parse().unwrap_or(0);
                RFNodeStatus { status, value: 0 }
            }
            Err(_) => RFNodeStatus { status: -1, value: 0 }
        }
    }
}
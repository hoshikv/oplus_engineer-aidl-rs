# oplus_engineer-aidl-rs

rust engineer aidl HAL.
Reverse engineered from the stock `vendor-oplus-hardware-engineer-V1-service` 

## Binary Analysis 
- **Interface**: `vendor.oplus.hardware.engineer.IEngineer` (AIDL)
- **Service name**: `vendor.oplus.hardware.engineer.IEngineer/default`
- **Transport**: HIDL/AIDL via binder (libbinder_ndk.so)
- **Architecture**: ARM64 (aarch64), Android 35

## Implemented Methods

The HAL provides these functions:

### Engineer Data Management
- `saveEngineerData(type, data)` - Save engineer mode data
- `readEngineerData(type, data)` - Read engineer mode data

### Partition Protection
- `getPartitionWriteProtectState()` - Get write protect state
- `setPartitionWriteProtectState(enable)` - Set write protect state

### Product Line Test
- `getProductLineTestResult()` - Get test results
- `setProductLineTestResult(index, value)` - Set test result
- `resetProductLineTestResult()` - Reset test results

### Secrecy Config
- `loadSecrecyConfig()` - Load secrecy configuration
- `saveSecrecyConfig(config)` - Save secrecy configuration

### Vibrator Calibration
- `setVibratorCalibrateData(key, value)` - Set calibration data
- `getVibratorCalibrateResult(key)` - Get calibration result
- `getImpedance()` - Get vibrator impedance
- `startVibrate(mode, index, value)` - Start vibration
- `startAgeVibrate(mode, index)` - Start aging vibration

### Properties & System
- `setProperties(key, value)` - Set system property
- `queryMountPointMounted(path)` - Check if mount point is mounted
- `queryDeviceInfo()` - Get device info (serial number)

### Engineer Persist Data
- `saveEngineerPersistData(key, value)` - Save persistent data
- `queryEngineerPersistData(key)` - Query persistent data
- `clearEngineerPersistData(key)` - Clear persistent data
- `listEngineerPersistData(prefix)` - List persistent data keys

### Vendor File Operations
- `readVendorFile(path)` - Read vendor file
- `saveDataToVendorFile(path, data, offset)` - Save data to vendor file
- `writeData(path, offset, append, length, data)` - Write data
- `readData(path, offset, length)` - Read data with result

### RF Status
- `getQcRfCableStatus()` - Get QC RF cable status
- `getMtkSwtpStatus()` - Get MTK SWTP status
- `getQcPdsStatus()` - Get QC PDS status

## Parcelable Types

- `VibratorResult` - Result with status code and message
- `ReadDataResult` - Result with status code and byte array
- `RFNodeStatus` - RF node status with status and value

## Building

```bash
export NDK_HOME=/opt/android-sdk/ndk/29.0.14206865
export ANDROID_NDK_HOME=$NDK_HOME
export PATH=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

rustup target add aarch64-linux-android

cargo build --release --target aarch64-linux-android
```

The instance build can only register service names that are declared in the device
manifest. `vendor.oplus.hardware.engineer.IEngineer/default` 

The build expects the NDK paths configured in `.cargo/config.toml`. Change them if your
NDK lives elsewhere.

## Structure

| Path | Purpose |
| --- | --- |
| `src/gen/vendor/oplus/hardware/engineer/` | Hand-written AIDL bindings matching the stock interface |
| `src/hal.rs` | All 28 method implementations (sysfs, procfs, reserve partitions) |
| `src/service.rs` | `BnEngineer` implementation wired to `Hal` |
| `src/main.rs` | Process entry, `--selftest`, instance name override |
| `aidl/` | AIDL interface definitions recovered from the device |
| `vendor/binder_ndk`, `vendor/binder_ndk_sys` | Patched binder crates (NDK backend and bindings) |

## Deployment

```bash
adb push target/aarch64-linux-android/release/engineer-hal-service /data/local/tmp/
adb shell su -c "chmod 755 /data/local/tmp/engineer-hal-service"

# proxy -> on_transact -> HAL
adb shell su -c "/data/local/tmp/engineer-hal-service --selftest"

# register test instance
adb shell su -c "setsid /data/local/tmp/engineer-hal-service vendor.oplus.hardware.engineer.IEngineer/test"
```

Instances:
```bash
adb shell service list | grep engineer.IEngineer
```

package vendor.oplus.hardware.engineer;

import vendor.oplus.hardware.engineer.VibratorResult;
import vendor.oplus.hardware.engineer.ReadDataResult;
import vendor.oplus.hardware.engineer.RFNodeStatus;

@VintfStability
interface IEngineer {
    boolean saveEngineerData(int32_t type, in byte[] data);
    VibratorResult readEngineerData(int32_t type, in byte[] data);
    boolean getPartitionWriteProtectState();
    boolean setPartitionWriteProtectState(boolean enable);
    byte[] getProductLineTestResult();
    boolean setProductLineTestResult(int32_t index, int32_t value);
    boolean resetProductLineTestResult();
    byte[] loadSecrecyConfig();
    boolean saveSecrecyConfig(String config);
    VibratorResult setVibratorCalibrateData(String key, String value);
    VibratorResult getVibratorCalibrateResult(String key);
    VibratorResult getImpedance();
    int32_t startVibrate(int32_t mode, int32_t index, int32_t value);
    int32_t startAgeVibrate(int32_t mode, int32_t index);
    boolean setProperties(String key, String value);
    boolean queryMountPointMounted(String path);
    String queryDeviceInfo();
    boolean saveEngineerPersistData(String key, String value);
    String queryEngineerPersistData(String key);
    boolean clearEngineerPersistData(String key);
    String[] listEngineerPersistData(String prefix);
    byte[] readVendorFile(String path);
    int32_t saveDataToVendorFile(String path, String data, int32_t offset);
    int32_t writeData(String path, int32_t offset, boolean append, int32_t length, in byte[] data);
    ReadDataResult readData(String path, int32_t offset, int32_t length);
    RFNodeStatus getQcRfCableStatus();
    RFNodeStatus getMtkSwtpStatus();
    RFNodeStatus getQcPdsStatus();
    int32_t getInterfaceVersion();
    String getInterfaceHash();
}
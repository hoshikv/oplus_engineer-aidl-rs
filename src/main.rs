use std::sync::Arc;

use engineer_hal::hal::Hal;
use engineer_hal::service::{EngineerService, SERVICE_NAME};
use engineer_hal::{loge, logi};

fn run_selftest(hal: Arc<Hal>) -> i32 {
    use binder::binder_impl::Proxy;
    use engineer_hal::IEngineer::IEngineer as IEngineerTrait;

    let service = EngineerService::new(hal).create_binder();
    let proxy = match engineer_hal::IEngineer::BpEngineer::from_binder(service.as_binder()) {
        Ok(proxy) => proxy,
        Err(error) => {
            loge!("selftest: from_binder failed: {error:?}");
            return 1;
        }
    };

    let mut failures = 0;
    macro_rules! check {
        ($name:literal, $expr:expr) => {
            match $expr {
                Ok(value) => logi!("selftest {} ok: {:?}", $name, value),
                Err(error) => {
                    failures += 1;
                    loge!("selftest {} failed: {error:?}", $name);
                }
            }
        };
    }

    check!("getInterfaceVersion", proxy.getInterfaceVersion());
    check!("getInterfaceHash", proxy.getInterfaceHash());
    check!(
        "getPartitionWriteProtectState",
        proxy.getPartitionWriteProtectState()
    );
    check!("queryDeviceInfo", proxy.queryDeviceInfo());
    check!("queryMountPointMounted", proxy.queryMountPointMounted("/data"));
    check!("getQcRfCableStatus", proxy.getQcRfCableStatus());
    check!("getMtkSwtpStatus", proxy.getMtkSwtpStatus());
    check!("getQcPdsStatus", proxy.getQcPdsStatus());
    check!(
        "listEngineerPersistData",
        proxy.listEngineerPersistData("")
    );

    if failures == 0 {
        logi!("selftest: all checks passed");
        0
    } else {
        loge!("selftest: {failures} checks failed");
        1
    }
}

fn main() {
    logi!("engineer AIDL HAL starting");

    let arg = std::env::args().nth(1);

    if arg.as_deref() == Some("--selftest") {
        let Some(hal) = Hal::open().map(Arc::new) else {
            loge!("failed to open engineer HAL");
            std::process::exit(1);
        };
        std::process::exit(run_selftest(hal));
    }

    let service_name = arg.unwrap_or_else(|| SERVICE_NAME.to_string());
    logi!("using service name {service_name}");

    let Some(hal) = Hal::open().map(Arc::new) else {
        loge!("failed to open engineer HAL");
        std::process::exit(1);
    };

    let service = EngineerService::new(hal).create_binder();

    binder::ProcessState::set_thread_pool_max_thread_count(4);
    binder::ProcessState::start_thread_pool();

    if let Err(error) = binder::add_service(&service_name, service.as_binder()) {
        loge!(
            "failed to register {service_name}: {error:?} (raw {})",
            error as i32
        );
        std::process::exit(1);
    }

    logi!("engineer AIDL HAL ready");
    binder::ProcessState::join_thread_pool();
}
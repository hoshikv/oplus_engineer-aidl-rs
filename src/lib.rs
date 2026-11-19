pub mod hal;
pub mod log;
pub mod service;

#[allow(unexpected_cfgs, unused_mut)]
#[path = "gen/vendor/oplus/hardware/engineer/IEngineer.rs"]
pub mod IEngineer;

#[allow(unexpected_cfgs, unused_mut)]
#[path = "gen/vendor/oplus/hardware/engineer/VibratorResult.rs"]
pub mod VibratorResultModule;

#[allow(unexpected_cfgs, unused_mut)]
#[path = "gen/vendor/oplus/hardware/engineer/ReadDataResult.rs"]
pub mod ReadDataResultModule;

#[allow(unexpected_cfgs, unused_mut)]
#[path = "gen/vendor/oplus/hardware/engineer/RFNodeStatus.rs"]
pub mod RFNodeStatusModule;

pub use crate::VibratorResultModule::VibratorResult;
pub use crate::ReadDataResultModule::ReadDataResult;
pub use crate::RFNodeStatusModule::RFNodeStatus;

pub mod mangled {
    pub use crate::VibratorResultModule::VibratorResult;
    pub use crate::ReadDataResultModule::ReadDataResult;
    pub use crate::RFNodeStatusModule::RFNodeStatus;
    pub use crate::IEngineer::mangled::*;
}
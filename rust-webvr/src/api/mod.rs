#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use self::mock::MockServiceCreator;

#[cfg(all(any(target_os="windows", target_os = "linux"), feature = "openvr"))]
mod openvr;
#[cfg(all(any(target_os="windows", target_os = "linux"), feature = "openvr"))]
pub use self::openvr::OpenVRServiceCreator;

#[cfg(all(feature = "googlevr", target_os= "android"))]
mod googlevr;
#[cfg(all(feature = "googlevr", target_os= "android"))]
pub use self::googlevr::GoogleVRServiceCreator;
#[cfg(all(feature = "googlevr", target_os= "android"))]
pub use self::googlevr::jni::*;

#[cfg(all(feature = "oculusvr", target_os= "android"))]
mod oculusvr;
#[cfg(all(feature = "oculusvr", target_os= "android"))]
pub use self::oculusvr::OculusVRServiceCreator;
#[cfg(all(feature = "oculusvr", target_os= "android"))]
pub use self::oculusvr::jni::*;

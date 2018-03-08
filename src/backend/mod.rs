//! TLS backend-specific functionality.

#[cfg(all(any(target_os = "macos", target_os = "ios"), not(feature = "force_openssl")))]
pub mod security_framework;

#[cfg(all(target_os = "windows", not(feature = "force_openssl")))]
pub mod schannel;

#[cfg(any(not(any(target_os = "macos", target_os = "windows", target_os = "ios")), feature = "force_openssl"))]
pub mod openssl;

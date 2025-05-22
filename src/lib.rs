//! This is a placeholder crate to reserve the `ap33772` namespace.
//! Actual functionality will be published in a future release.
#![no_std]
#![deny(unsafe_code)]

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

pub mod ap33772s;

#[cfg(not(feature = "register"))]
mod commands;

#[cfg(feature = "register")]
pub mod commands;



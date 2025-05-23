// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Internal module with code generated by [`wit-bindgen`](https://github.com/jvff/wit-bindgen).

#![allow(missing_docs)]

// Export the service interface.
wit_bindgen::generate!({
    world: "service",
    export_macro_name: "export_service",
    pub_export_macro: true,
});

pub use self::linera::app::{base_runtime_api, service_runtime_api};

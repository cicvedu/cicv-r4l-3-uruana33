// SPDX-License-Identifier: GPL-2.0

//! Rust hello world sample.

use kernel::prelude::*;
use kernel::{str::CStr, ThisModule};

module! {
  type: RustHelloWorld,
  name: "rust_helloworld",
  author: "whocare",
  description: "hello world module in rust",
  license: "GPL",
}

struct RustHelloWorld {}

impl kernel::Module for RustHelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust module\n");
        Ok(RustHelloWorld {})
    }
}

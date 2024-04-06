// SPDX-License-Identifier: GPL-2.0

//! Character devices.
//!
//! Also called "char devices", `chrdev`, `cdev`.
//!
//! C header: [`include/linux/cdev.h`](../../../../include/linux/completion.h)
//!
//! Reference: <https://www.kernel.org/doc/html/latest/core-api/kernel-api.html#char-devices>

use crate::bindings;
use crate::Opaque;
use crate::str::CStr;
use core::{marker::PhantomPinned, pin::Pin};
use crate::sync::NeedsLockClass;
use crate::sync::LockClassKey;
use crate::pr_info;

/// completion

// pub struct Completion(*mut bindings::completion);

pub struct Completion {
    pub(crate) complete: Opaque<bindings::completion>,
    pub(crate) wait_list: Opaque<bindings::swait_queue_head>,
    _pin: PhantomPinned,
}

impl Completion {
    #[allow(dead_code)]
    /// new
    pub const unsafe fn new() -> Self {
        // Completion( (&mut bindings::completion::default()) as *mut bindings::completion)
        Self { complete: Opaque::uninit(), wait_list:Opaque::uninit(), _pin: PhantomPinned,}
    }

    #[allow(dead_code)]
    /// init
    pub fn init(&self) {
        // unsafe { bindings::init_completion(self.0) };
        unsafe { bindings::init_completion(self.complete.get()) };
    }

    #[allow(dead_code)]
    /// wait_state
    pub fn wait_for_completion(&self) {
        // unsafe { bindings::wait_for_completion(self.0) };
        unsafe { bindings::wait_for_completion(self.complete.get()) };
    }

    #[allow(dead_code)]
    /// complete
    pub fn complete_all(&self) {
        // unsafe { bindings::complete(self.0) };
        unsafe { bindings::complete_all(self.complete.get()) };
    }

    #[allow(dead_code)]
    /// complete
    pub fn completion_done(&self) -> bool {
        // unsafe { bindings::complete(self.0) };
        unsafe { bindings::completion_done(self.complete.get()) }
    }

}

unsafe impl Sync for Completion {}

unsafe impl Send for Completion {}

impl NeedsLockClass for Completion {
    fn init(
        self: Pin<&mut Self>,
        name: &'static CStr,
        key: &'static LockClassKey,
        _: &'static LockClassKey,
    ) {
        unsafe {
            bindings::__init_swait_queue_head(self.wait_list.get(), name.as_char_ptr(), key.get())
        };
    }
}

impl Drop for Completion {
    fn drop(&mut self) {
        pr_info!("---------completion release----------\n");
    }
}


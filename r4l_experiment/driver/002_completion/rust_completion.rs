// SPDX-License-Identifier: GPL-2.0

//! Rust Completion.

use core::result::Result::Err;
use kernel::prelude::*;
use kernel::{chrdev, file, complete, task::Task};

module! {
    type: RustCompletion,
    name: "rust_completion",
    author: "Rust for Linux Contributors",
    description: "Rust Completion module",
    license: "GPL",
}

kernel::init_static_sync! {
    static COMPLETION: complete::Completion;
}

struct RustFile {
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        pr_info!("---------打开字符设备----------\n");
        Ok(
            Box::try_new(RustFile {})?
        )
    }

    fn write(_this: &Self, _file: &file::File, _reader: &mut impl kernel::io_buffer::IoBufferReader, _offset:u64,) -> Result<usize> {
        pr_info!("---------数据写入字符设备----------\n");
        pr_info!("---------开始设置completion状态----------\n");

        COMPLETION.complete_all();

        pr_info!("---------completion状态 => 已完成({}), 唤醒reader----------\n", COMPLETION.completion_done());
        pr_info!("process {} awakening the readers...\n", Task::current().pid());
        pr_info!("---------数据长度{}----------\n", _reader.len());
        Ok(_reader.len())
    }

    fn read(_this: &Self, _file: &file::File, _writer: &mut impl kernel::io_buffer::IoBufferWriter, _offset:u64,) -> Result<usize> {
        pr_info!("---------从字符设备读取数据----------\n");
        pr_info!("process {} is going to sleep\n",Task::current().pid());
        pr_info!("---------阻塞中,等待completion状态----------\n");

        COMPLETION.wait_for_completion();

        pr_info!("---------reader 收到completion状态 => 已完成----------\n");
        pr_info!("process {} awoken\n",Task::current().pid());
        Ok(_offset.try_into().unwrap())
    }
}

struct RustCompletion {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl kernel::Module for RustCompletion {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Completion module (init)\n");

        COMPLETION.init();

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustCompletion { _dev: chrdev_reg })
    }
}

impl Drop for RustCompletion {
    fn drop(&mut self) {
        pr_info!("Rust Completion module (exit)\n");
    }
}

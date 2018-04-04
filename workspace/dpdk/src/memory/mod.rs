// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::phys_addr_t;
use ::libc::c_void;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::rust_extra::u31;
use ::std::mem::forget;
use ::std::mem::uninitialized;


include!("MemoryDetails.rs");
include!("DpdkAllocatedMemory.rs");
